//! Keep a healthy application's periodic work going after a peer fails.
//!
//! The host selects each release and drains diagnostics on a separate turn.

#![forbid(unsafe_code)]

use std::cell::Cell;
use std::convert::Infallible;
use std::error::Error;
use std::fmt;
use std::io::{self, Write};
use std::process::ExitCode;
use std::time::Duration;

use rust_flight_framework::{
    Application, ApplicationId, ApplicationWorkContext, EventQueue, FrameworkInstant, ManualClock,
    PeriodicWork, PeriodicWorkOutcome, Runtime, RuntimeWorkError, RuntimeWorkEventError,
    ScheduledWork,
};

enum MissionApplication<'a> {
    Sampler,
    Heartbeat(&'a Cell<u8>),
}

#[derive(Debug, Eq, PartialEq)]
struct SampleUnavailable;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Diagnostic {
    WorkFailed,
    #[cfg(test)]
    EarlierWarning,
}

#[derive(Clone, Copy)]
enum Role {
    Sampler,
    Heartbeat,
}

#[derive(Debug, Eq, PartialEq)]
enum Turn {
    Disabled,
    Waiting { next_at: FrameworkInstant },
    Exhausted,
    Worked,
}

struct Host<'a> {
    runtime: Runtime<MissionApplication<'a>>,
    clock: ManualClock,
    events: EventQueue<Diagnostic>,
    sampler_id: ApplicationId,
    sampler: Option<PeriodicWork>,
    heartbeat: Option<PeriodicWork>,
}

impl fmt::Display for SampleUnavailable {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("sample unavailable")
    }
}

impl Error for SampleUnavailable {}

impl Application for MissionApplication<'_> {
    type StartError = Infallible;
    type WorkError = SampleUnavailable;
    type StopError = Infallible;
    type RestartError = Infallible;

    fn start(&mut self) -> Result<(), Self::StartError> {
        Ok(())
    }

    fn work(&mut self, _context: ApplicationWorkContext<'_>) -> Result<(), Self::WorkError> {
        match self {
            Self::Sampler => Err(SampleUnavailable),
            Self::Heartbeat(count) => {
                count.set(count.get().saturating_add(1));
                Ok(())
            }
        }
    }

    fn stop(&mut self) -> Result<(), Self::StopError> {
        Ok(())
    }

    fn restart(&mut self) -> Result<(), Self::RestartError> {
        Ok(())
    }
}

impl<'a> Host<'a> {
    fn new(heartbeats: &'a Cell<u8>) -> Result<Self, Box<dyn Error + 'a>> {
        let mut runtime = Runtime::new(2)?;
        let sampler = runtime.register(MissionApplication::Sampler)?;
        let heartbeat = runtime.register(MissionApplication::Heartbeat(heartbeats))?;
        runtime.start(sampler)?;
        runtime.start(heartbeat)?;
        let sampler_work = PeriodicWork::new(
            ScheduledWork::new(sampler, instant(1)),
            Duration::from_secs(2),
        )?;
        let heartbeat_work = PeriodicWork::new(
            ScheduledWork::new(heartbeat, instant(2)),
            Duration::from_secs(1),
        )?;

        Ok(Self {
            runtime,
            clock: ManualClock::new(),
            events: EventQueue::new(1)?,
            sampler_id: sampler,
            sampler: Some(sampler_work),
            heartbeat: Some(heartbeat_work),
        })
    }

    fn turn(
        &mut self,
        role: Role,
    ) -> Result<Turn, RuntimeWorkEventError<SampleUnavailable, Diagnostic>> {
        let selected = match role {
            Role::Sampler => &mut self.sampler,
            Role::Heartbeat => &mut self.heartbeat,
        };
        let Some(cursor) = selected else {
            return Ok(Turn::Disabled);
        };
        match cursor.poll(&self.clock) {
            PeriodicWorkOutcome::Waiting { next, .. } => Ok(Turn::Waiting {
                next_at: next.scheduled_at(),
            }),
            PeriodicWorkOutcome::Exhausted => Ok(Turn::Exhausted),
            PeriodicWorkOutcome::Released { scheduled_work, .. } => self
                .runtime
                .work_with_failure_event(
                    scheduled_work.application_id(),
                    Diagnostic::WorkFailed,
                    &self.clock,
                    &mut self.events,
                )
                .inspect_err(|_| *selected = None)
                .map(|_| Turn::Worked),
        }
    }
}

fn main() -> ExitCode {
    let heartbeats = Cell::new(0);
    match run(&heartbeats, &mut io::stdout().lock()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("periodic peer example: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run<'a>(heartbeats: &'a Cell<u8>, output: &mut impl Write) -> Result<(), Box<dyn Error + 'a>> {
    let mut host = Host::new(heartbeats)?;
    let second = Duration::from_secs(1);
    host.clock.advance(second)?;
    let Err(failure) = host.turn(Role::Sampler) else {
        return Err("expected the sampler's injected failure".into());
    };
    if let RuntimeWorkError::Lifecycle(_) = failure.work_error() {
        return Err(failure.into());
    }
    let attempt = failure
        .failure_event_attempt()
        .ok_or("missing failure event")?;
    writeln!(
        output,
        "1s sampler {:?}, {failure}, diagnostic {:?}",
        host.runtime.state(host.sampler_id)?,
        attempt.outcome()
    )?;

    // Observe the failure here and keep selecting the peer's turns.
    for seconds in [2, 3] {
        host.clock.advance(second)?;
        writeln!(
            output,
            "{seconds}s heartbeat {:?} ({})",
            host.turn(Role::Heartbeat)?,
            heartbeats.get()
        )?;
    }
    writeln!(output, "3s sampler {:?}", host.turn(Role::Sampler)?)?;
    let event = host
        .events
        .dequeue()
        .ok_or("missing queued failure event")?;
    writeln!(
        output,
        "3s drained {:?} from {}s",
        event.identifier(),
        event.timestamp().elapsed().as_secs()
    )?;
    host.clock.advance(second)?;
    writeln!(
        output,
        "4s heartbeat {:?} ({})",
        host.turn(Role::Heartbeat)?,
        heartbeats.get()
    )?;
    Ok(())
}

fn instant(seconds: u64) -> FrameworkInstant {
    FrameworkInstant::from_elapsed(Duration::from_secs(seconds))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_flight_framework::{
        ApplicationState, Event, EventEmitOutcome, EventSeverity, EventSource, EventTimestamp,
    };

    #[test]
    fn heartbeat_keeps_its_release_and_runs_before_and_after_drain() {
        let heartbeats = Cell::new(0);
        let mut host = Host::new(&heartbeats).unwrap();
        let second = Duration::from_secs(1);
        let sampler = host.sampler_id;
        host.clock.advance(second).unwrap();
        let failure = host.turn(Role::Sampler).unwrap_err();
        let attempt = failure.failure_event_attempt().unwrap();
        assert_eq!(
            failure.work_error(),
            &RuntimeWorkError::Application {
                application_id: sampler,
                source: SampleUnavailable
            }
        );
        assert_eq!(
            host.runtime.state(sampler).unwrap(),
            ApplicationState::Failed
        );
        assert_eq!(attempt.outcome(), EventEmitOutcome::Recorded);
        assert_eq!(
            *attempt.event(),
            Event::new(
                EventSource::Application(sampler),
                EventSeverity::Error,
                Diagnostic::WorkFailed,
                EventTimestamp::from_elapsed(second),
            ),
        );
        assert_eq!(
            host.turn(Role::Heartbeat).unwrap(),
            Turn::Waiting {
                next_at: instant(2)
            }
        );
        assert_eq!(heartbeats.get(), 0);
        for expected in [1, 2] {
            host.clock.advance(second).unwrap();
            assert_eq!(host.turn(Role::Heartbeat).unwrap(), Turn::Worked);
            assert_eq!(heartbeats.get(), expected);
            assert_eq!(host.events.pending(), 1);
        }
        assert_eq!(host.turn(Role::Sampler).unwrap(), Turn::Disabled);
        assert_eq!(host.events.dequeue(), Some(*attempt.event()));
        host.clock.advance(second).unwrap();
        assert_eq!(host.turn(Role::Heartbeat).unwrap(), Turn::Worked);
        assert_eq!(heartbeats.get(), 3);
        assert!(host.events.is_empty());
    }

    #[test]
    fn rejected_failure_keeps_older_diagnostic_and_peer_work() {
        let heartbeats = Cell::new(0);
        let mut host = Host::new(&heartbeats).unwrap();
        let older = Event::new(
            EventSource::Framework,
            EventSeverity::Warning,
            Diagnostic::EarlierWarning,
            EventTimestamp::from_elapsed(Duration::ZERO),
        );
        assert_eq!(host.events.emit(&older), EventEmitOutcome::Recorded);
        host.clock.advance(Duration::from_secs(1)).unwrap();
        let failure = host.turn(Role::Sampler).unwrap_err();
        let attempt = failure.failure_event_attempt().unwrap();
        assert_eq!(
            failure.work_error(),
            &RuntimeWorkError::Application {
                application_id: host.sampler_id,
                source: SampleUnavailable,
            }
        );
        assert_eq!(
            host.runtime.state(host.sampler_id).unwrap(),
            ApplicationState::Failed
        );
        assert_eq!(
            attempt.outcome(),
            EventEmitOutcome::QueueFull { capacity: 1 }
        );
        assert_eq!(
            attempt.event().source(),
            EventSource::Application(host.sampler_id)
        );
        assert_eq!(attempt.event().timestamp().instant(), instant(1));
        host.clock.advance(Duration::from_secs(1)).unwrap();
        assert_eq!(host.turn(Role::Heartbeat).unwrap(), Turn::Worked);
        assert_eq!(heartbeats.get(), 1);
        assert_eq!(host.turn(Role::Sampler).unwrap(), Turn::Disabled);
        assert_eq!(host.events.dequeue(), Some(older));
        assert!(host.events.is_empty());
    }
}
