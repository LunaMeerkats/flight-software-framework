//! Keep a healthy application's periodic work going after a peer fails.
//!
//! The host selects each release and drains diagnostics on a separate turn.
//! The first trace leaves the backup idle; the second retains both failures.
//! The third prioritizes retained admission before new sampler work.

#![forbid(unsafe_code)]

use std::cell::Cell;
use std::convert::Infallible;
use std::error::Error;
use std::fmt;
use std::io::{self, Write};
use std::process::ExitCode;
use std::time::Duration;

use rust_flight_framework::{
    Application, ApplicationId, ApplicationWorkContext, Event, EventEmitOutcome, EventQueue,
    EventSeverity, EventSource, EventTimestamp, FrameworkInstant, ManualClock, PeriodicWork,
    PeriodicWorkOutcome, Runtime, RuntimeWorkError, RuntimeWorkEventError, ScheduledWork,
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
    EarlierWarning,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Role {
    Sampler,
    BackupSampler,
    Heartbeat,
}

#[derive(Debug, Eq, PartialEq)]
enum Turn {
    Disabled,
    DeferredForDiagnostics,
    Waiting { next_at: FrameworkInstant },
    Exhausted,
    Worked,
}

struct Host<'a> {
    runtime: Runtime<MissionApplication<'a>>,
    clock: ManualClock,
    events: EventQueue<Diagnostic>,
    sampler_id: ApplicationId,
    backup_sampler_id: ApplicationId,
    sampler: Option<PeriodicWork>,
    backup_sampler: Option<PeriodicWork>,
    heartbeat: Option<PeriodicWork>,
    pending_failures: [Option<Event<Diagnostic>>; 2],
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
        let mut runtime = Runtime::new(3)?;
        let sampler = runtime.register(MissionApplication::Sampler)?;
        let backup_sampler = runtime.register(MissionApplication::Sampler)?;
        let heartbeat = runtime.register(MissionApplication::Heartbeat(heartbeats))?;
        runtime.start(sampler)?;
        runtime.start(backup_sampler)?;
        runtime.start(heartbeat)?;
        let sampler_work = PeriodicWork::new(
            ScheduledWork::new(sampler, instant(1)),
            Duration::from_secs(2),
        )?;
        let heartbeat_work = PeriodicWork::new(
            ScheduledWork::new(heartbeat, instant(2)),
            Duration::from_secs(1),
        )?;
        let backup_sampler_work = PeriodicWork::new(
            ScheduledWork::new(backup_sampler, instant(1)),
            Duration::from_secs(2),
        )?;

        Ok(Self {
            runtime,
            clock: ManualClock::new(),
            events: EventQueue::new(1)?,
            sampler_id: sampler,
            backup_sampler_id: backup_sampler,
            sampler: Some(sampler_work),
            backup_sampler: Some(backup_sampler_work),
            heartbeat: Some(heartbeat_work),
            pending_failures: [None, None],
        })
    }

    fn turn(
        &mut self,
        role: Role,
    ) -> Result<Turn, RuntimeWorkEventError<SampleUnavailable, Diagnostic>> {
        let selected = match role {
            Role::Sampler => &mut self.sampler,
            Role::BackupSampler => &mut self.backup_sampler,
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
                .inspect_err(|error| {
                    *selected = None;
                    if let Some(attempt) = error.failure_event_attempt()
                        && matches!(attempt.outcome(), EventEmitOutcome::QueueFull { .. })
                    {
                        // Only two samplers can fail, once each. The two slots
                        // suffice until explicit retry removes retained events.
                        let slot = if self.pending_failures[0].is_none() {
                            &mut self.pending_failures[0]
                        } else {
                            &mut self.pending_failures[1]
                        };
                        *slot = Some(*attempt.event());
                    }
                })
                .map(|_| Turn::Worked),
        }
    }

    fn turn_after_retained_failures(
        &mut self,
        role: Role,
    ) -> Result<Turn, RuntimeWorkEventError<SampleUnavailable, Diagnostic>> {
        let selected = match role {
            Role::Sampler => &self.sampler,
            Role::BackupSampler => &self.backup_sampler,
            // Only the samplers can fail in this fixed mission.
            Role::Heartbeat => return self.turn(role),
        };
        if selected.is_none() {
            return Ok(Turn::Disabled);
        }
        // Defer before polling so the original release remains available.
        // Retry compacts the slots; an empty head means no retained event.
        if self.pending_failures[0].is_some() {
            return Ok(Turn::DeferredForDiagnostics);
        }
        self.turn(role)
    }

    fn retry_failure_event(&mut self) -> Option<EventEmitOutcome> {
        let outcome = self.events.emit(self.pending_failures[0].as_ref()?);
        if outcome == EventEmitOutcome::Recorded {
            self.pending_failures[0] = self.pending_failures[1].take();
        }
        Some(outcome)
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
    run_rejected_pair(heartbeats, output)?;
    run_retained_priority(heartbeats, output)
}

fn run_rejected_pair<'a>(
    heartbeats: &'a Cell<u8>,
    output: &mut impl Write,
) -> Result<(), Box<dyn Error + 'a>> {
    heartbeats.set(0);
    let mut host = Host::new(heartbeats)?;
    let older = Event::new(
        EventSource::Framework,
        EventSeverity::Warning,
        Diagnostic::EarlierWarning,
        EventTimestamp::from_elapsed(Duration::ZERO),
    );
    if host.events.emit(&older) != EventEmitOutcome::Recorded {
        return Err("expected room for the initial warning".into());
    }
    for (role, name) in [(Role::Sampler, "sampler"), (Role::BackupSampler, "backup")] {
        host.clock.advance(Duration::from_secs(1))?;
        let failure = host
            .turn(role)
            .err()
            .ok_or("expected the selected sampler's injected failure")?;
        if let RuntimeWorkError::Lifecycle(_) = failure.work_error() {
            return Err(failure.into());
        }
        let attempt = failure
            .failure_event_attempt()
            .ok_or("missing failure event")?;
        writeln!(output, "retry {name}: {failure}, {:?}", attempt.outcome())?;
    }
    let heartbeat = host.turn(Role::Heartbeat)?;
    let full_retry = host.retry_failure_event();
    writeln!(
        output,
        "retry heartbeat {:?} ({}), backup {:?}, full {:?}",
        heartbeat,
        heartbeats.get(),
        host.runtime.state(host.backup_sampler_id)?,
        full_retry
    )?;
    host.events.dequeue().ok_or("missing older warning")?;
    for _ in 0..2 {
        let admission = host.retry_failure_event();
        writeln!(output, "retry admission {admission:?}")?;
        let event = host.events.dequeue().ok_or("missing retried failure")?;
        writeln!(
            output,
            "retry drained {:?} from {}s",
            event.source(),
            event.timestamp().elapsed().as_secs()
        )?;
    }
    writeln!(output, "retry empty {:?}", host.retry_failure_event())?;
    Ok(())
}

fn run_retained_priority<'a>(
    heartbeats: &'a Cell<u8>,
    output: &mut impl Write,
) -> Result<(), Box<dyn Error + 'a>> {
    heartbeats.set(0);
    let mut host = Host::new(heartbeats)?;
    let warning = Event::new(
        EventSource::Framework,
        EventSeverity::Warning,
        Diagnostic::EarlierWarning,
        EventTimestamp::from_elapsed(Duration::ZERO),
    );
    if host.events.emit(&warning) != EventEmitOutcome::Recorded {
        return Err("expected room for the initial warning".into());
    }
    host.clock.advance(Duration::from_secs(1))?;
    let failure = host
        .turn_after_retained_failures(Role::Sampler)
        .err()
        .ok_or("expected the sampler's injected failure")?;
    writeln!(output, "priority sampler: {failure}")?;
    host.clock.advance(Duration::from_secs(1))?;
    writeln!(
        output,
        "priority backup {:?}",
        host.turn_after_retained_failures(Role::BackupSampler)?
    )?;
    writeln!(
        output,
        "priority heartbeat {:?} ({})",
        host.turn_after_retained_failures(Role::Heartbeat)?,
        heartbeats.get()
    )?;
    host.events.dequeue().ok_or("missing older warning")?;
    writeln!(
        output,
        "priority admission {:?}",
        host.retry_failure_event()
    )?;
    let failure = host
        .turn_after_retained_failures(Role::BackupSampler)
        .err()
        .ok_or("expected the backup's injected failure")?;
    writeln!(output, "priority backup: {failure}")?;
    for _ in 0..2 {
        let event = host.events.dequeue().ok_or("missing ordered failure")?;
        writeln!(
            output,
            "priority drained {:?} from {}s",
            event.source(),
            event.timestamp().elapsed().as_secs()
        )?;
        writeln!(output, "priority retry {:?}", host.retry_failure_event())?;
    }
    Ok(())
}

fn instant(seconds: u64) -> FrameworkInstant {
    FrameworkInstant::from_elapsed(Duration::from_secs(seconds))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_flight_framework::{ApplicationState, Clock};

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
        assert_eq!(host.retry_failure_event(), None);
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

    #[test]
    fn two_rejected_failures_retry_in_observation_order_without_duplicates() {
        for order in [
            [Role::Sampler, Role::BackupSampler],
            [Role::BackupSampler, Role::Sampler],
        ] {
            let heartbeats = Cell::new(0);
            let (mut host, older, rejected) = capture_rejected_pair(&heartbeats, order);
            assert_eq!(rejected[0].timestamp().instant(), instant(1));
            assert_eq!(rejected[1].timestamp().instant(), instant(2));
            let full = Some(EventEmitOutcome::QueueFull { capacity: 1 });
            for _ in 0..2 {
                assert_eq!(host.retry_failure_event(), full);
                assert_eq!(host.events.pending(), 1);
            }
            assert_eq!(host.turn(Role::Heartbeat).unwrap(), Turn::Worked);
            assert_eq!(heartbeats.get(), 1);
            for role in order {
                assert_eq!(host.turn(role).unwrap(), Turn::Disabled);
            }
            assert_eq!(host.events.dequeue(), Some(older));
            assert_eq!(host.retry_failure_event(), Some(EventEmitOutcome::Recorded));
            assert_eq!(host.retry_failure_event(), full);
            assert_eq!(host.events.dequeue(), Some(rejected[0]));
            assert!(host.events.is_empty());
            assert_eq!(host.retry_failure_event(), Some(EventEmitOutcome::Recorded));
            assert_eq!(host.retry_failure_event(), None);
            assert_eq!(host.events.dequeue(), Some(rejected[1]));
            assert_eq!(host.retry_failure_event(), None);
            assert!(host.events.is_empty());
            host.clock.advance(Duration::from_secs(1)).unwrap();
            assert_eq!(host.turn(Role::Heartbeat).unwrap(), Turn::Worked);
            assert_eq!(heartbeats.get(), 2);
            for role in order {
                assert_eq!(host.turn(role).unwrap(), Turn::Disabled);
            }
            assert_eq!(host.retry_failure_event(), None);
        }
    }

    #[test]
    fn lifecycle_rejection_disables_only_selected_cursor_without_retry() {
        let heartbeats = Cell::new(0);
        let mut host = Host::new(&heartbeats).unwrap();
        host.runtime.stop(host.backup_sampler_id).unwrap();
        host.clock.advance(Duration::from_secs(1)).unwrap();
        let failure = host.turn(Role::BackupSampler).unwrap_err();
        assert!(matches!(
            failure.work_error(),
            RuntimeWorkError::Lifecycle(_)
        ));
        assert!(failure.failure_event_attempt().is_none());
        assert_eq!(host.retry_failure_event(), None);
        assert_eq!(host.turn(Role::BackupSampler).unwrap(), Turn::Disabled);
        assert!(host.events.is_empty());
        host.clock.advance(Duration::from_secs(1)).unwrap();
        assert_eq!(host.turn(Role::Heartbeat).unwrap(), Turn::Worked);
        assert_eq!(heartbeats.get(), 1);
    }

    #[test]
    fn priority_deferral_preserves_release_and_keeps_heartbeat_selectable() {
        for (failed, waiting) in [
            (Role::Sampler, Role::BackupSampler),
            (Role::BackupSampler, Role::Sampler),
        ] {
            let heartbeats = Cell::new(0);
            let (mut host, warning, retained) = capture_one_rejected_failure(&heartbeats, failed);
            let next = sampler_release(&host, waiting);
            assert_eq!(next.unwrap().scheduled_at(), instant(1));
            for seconds in [2, 3] {
                host.clock.advance(Duration::from_secs(1)).unwrap();
                assert_eq!(
                    host.turn_after_retained_failures(waiting).unwrap(),
                    Turn::DeferredForDiagnostics
                );
                assert_eq!(sampler_release(&host, waiting), next);
                assert_eq!(
                    host.runtime.state(next.unwrap().application_id()).unwrap(),
                    ApplicationState::Running
                );
                assert_eq!(
                    host.turn_after_retained_failures(failed).unwrap(),
                    Turn::Disabled
                );
                assert_eq!(
                    host.turn_after_retained_failures(Role::Heartbeat).unwrap(),
                    Turn::Worked
                );
                assert_eq!(heartbeats.get(), seconds - 1);
                assert_eq!(
                    host.retry_failure_event(),
                    Some(EventEmitOutcome::QueueFull { capacity: 1 })
                );
                assert_eq!(host.pending_failures, [Some(retained), None]);
                assert_eq!(host.events.pending(), 1);
            }
            assert_eq!(host.events.dequeue(), Some(warning));
            assert_eq!(
                host.turn_after_retained_failures(waiting).unwrap(),
                Turn::DeferredForDiagnostics
            );
            assert_eq!(sampler_release(&host, waiting), next);
            assert_eq!(host.pending_failures, [Some(retained), None]);
            assert!(host.events.is_empty());
        }
    }

    #[test]
    fn priority_admission_allows_later_failure_behind_the_older_queued_event() {
        for (first, later) in [
            (Role::Sampler, Role::BackupSampler),
            (Role::BackupSampler, Role::Sampler),
        ] {
            let heartbeats = Cell::new(0);
            let (mut host, warning, retained) = capture_one_rejected_failure(&heartbeats, first);
            let later_id = sampler_release(&host, later).unwrap().application_id();
            host.clock.advance(Duration::from_secs(5)).unwrap();
            assert_eq!(host.events.dequeue(), Some(warning));
            assert_eq!(host.retry_failure_event(), Some(EventEmitOutcome::Recorded));
            assert_eq!(host.pending_failures, [None, None]);
            let failure = host.turn_after_retained_failures(later).unwrap_err();
            let expected = Event::new(
                EventSource::Application(later_id),
                EventSeverity::Error,
                Diagnostic::WorkFailed,
                EventTimestamp::from_elapsed(Duration::from_secs(6)),
            );
            assert_eq!(
                failure.work_error(),
                &RuntimeWorkError::Application {
                    application_id: later_id,
                    source: SampleUnavailable,
                }
            );
            let attempt = failure.failure_event_attempt().unwrap();
            assert_eq!(*attempt.event(), expected);
            assert_eq!(
                attempt.outcome(),
                EventEmitOutcome::QueueFull { capacity: 1 }
            );
            assert_eq!(
                host.runtime.state(later_id).unwrap(),
                ApplicationState::Failed
            );
            assert_eq!(host.pending_failures, [Some(expected), None]);
            assert_eq!(host.events.dequeue(), Some(retained));
            assert_eq!(host.retry_failure_event(), Some(EventEmitOutcome::Recorded));
            assert_eq!(host.events.dequeue(), Some(expected));
            for role in [first, later] {
                assert_eq!(
                    host.turn_after_retained_failures(role).unwrap(),
                    Turn::Disabled
                );
            }
            assert_eq!(host.retry_failure_event(), None);
            assert!(host.events.is_empty());
        }
    }

    #[test]
    fn priority_without_retention_keeps_waiting_and_lifecycle_outcomes() {
        let heartbeats = Cell::new(0);
        let mut host = Host::new(&heartbeats).unwrap();
        for role in [Role::Sampler, Role::BackupSampler] {
            assert_eq!(
                host.turn_after_retained_failures(role).unwrap(),
                Turn::Waiting {
                    next_at: instant(1)
                }
            );
        }
        assert_eq!(
            host.turn_after_retained_failures(Role::Heartbeat).unwrap(),
            Turn::Waiting {
                next_at: instant(2)
            }
        );
        host.runtime.stop(host.backup_sampler_id).unwrap();
        host.clock.advance(Duration::from_secs(1)).unwrap();
        let failure = host
            .turn_after_retained_failures(Role::BackupSampler)
            .unwrap_err();
        assert!(matches!(
            failure.work_error(),
            RuntimeWorkError::Lifecycle(_)
        ));
        assert!(failure.failure_event_attempt().is_none());
        assert_eq!(host.retry_failure_event(), None);
        assert_eq!(
            host.turn_after_retained_failures(Role::BackupSampler)
                .unwrap(),
            Turn::Disabled
        );
        assert!(host.events.is_empty());
        assert_eq!(heartbeats.get(), 0);
    }

    #[test]
    fn runnable_trace_preserves_original_prefix_and_finishes_two_retries() {
        let heartbeats = Cell::new(0);
        let mut output = Vec::new();
        run(&heartbeats, &mut output).unwrap();
        let trace = std::str::from_utf8(&output).unwrap();
        assert!(trace.starts_with(concat!(
            "1s sampler Failed, application ApplicationId(0) returned a work error: ",
            "sample unavailable, diagnostic Recorded\n",
            "2s heartbeat Worked (1)\n",
            "3s heartbeat Worked (2)\n",
            "3s sampler Disabled\n",
            "3s drained WorkFailed from 1s\n",
            "4s heartbeat Worked (3)\n",
        )));
        let (original, _) = trace.split_once("priority sampler:").unwrap();
        assert_eq!(original.lines().count(), 14);
        assert_eq!(
            original.matches("retry admission Some(Recorded)\n").count(),
            2
        );
        assert!(original.ends_with("retry empty None\n"));
    }

    #[test]
    fn runnable_priority_trace_defers_then_admits_in_order() {
        let heartbeats = Cell::new(0);
        let mut output = Vec::new();
        run_retained_priority(&heartbeats, &mut output).unwrap();
        assert_eq!(
            std::str::from_utf8(&output).unwrap(),
            concat!(
                "priority sampler: application ApplicationId(0) returned a work error: ",
                "sample unavailable; failure event was rejected at queue capacity 1\n",
                "priority backup DeferredForDiagnostics\n",
                "priority heartbeat Worked (1)\n",
                "priority admission Some(Recorded)\n",
                "priority backup: application ApplicationId(1) returned a work error: ",
                "sample unavailable; failure event was rejected at queue capacity 1\n",
                "priority drained Application(ApplicationId(0)) from 1s\n",
                "priority retry Some(Recorded)\n",
                "priority drained Application(ApplicationId(1)) from 2s\n",
                "priority retry None\n",
            )
        );
    }

    fn sampler_release(host: &Host<'_>, role: Role) -> Option<ScheduledWork> {
        match role {
            Role::Sampler => host.sampler.as_ref().unwrap().next_scheduled_work(),
            Role::BackupSampler => host.backup_sampler.as_ref().unwrap().next_scheduled_work(),
            Role::Heartbeat => panic!("expected a sampler role"),
        }
    }

    fn capture_one_rejected_failure(
        heartbeats: &Cell<u8>,
        role: Role,
    ) -> (Host<'_>, Event<Diagnostic>, Event<Diagnostic>) {
        let mut host = Host::new(heartbeats).unwrap();
        let id = sampler_release(&host, role).unwrap().application_id();
        let warning = Event::new(
            EventSource::Framework,
            EventSeverity::Warning,
            Diagnostic::EarlierWarning,
            EventTimestamp::from_elapsed(Duration::ZERO),
        );
        assert_eq!(host.events.emit(&warning), EventEmitOutcome::Recorded);
        host.clock.advance(Duration::from_secs(1)).unwrap();
        let failure = host.turn_after_retained_failures(role).unwrap_err();
        assert_eq!(
            failure.work_error(),
            &RuntimeWorkError::Application {
                application_id: id,
                source: SampleUnavailable,
            }
        );
        let expected = Event::new(
            EventSource::Application(id),
            EventSeverity::Error,
            Diagnostic::WorkFailed,
            EventTimestamp::from_elapsed(Duration::from_secs(1)),
        );
        let attempt = failure.failure_event_attempt().unwrap();
        assert_eq!(*attempt.event(), expected);
        assert_eq!(
            attempt.outcome(),
            EventEmitOutcome::QueueFull { capacity: 1 }
        );
        assert_eq!(host.runtime.state(id).unwrap(), ApplicationState::Failed);
        (host, warning, expected)
    }

    fn capture_rejected_pair(
        heartbeats: &Cell<u8>,
        order: [Role; 2],
    ) -> (Host<'_>, Event<Diagnostic>, [Event<Diagnostic>; 2]) {
        let mut host = Host::new(heartbeats).unwrap();
        let older = Event::new(
            EventSource::Framework,
            EventSeverity::Warning,
            Diagnostic::EarlierWarning,
            EventTimestamp::from_elapsed(Duration::ZERO),
        );
        assert_eq!(host.events.emit(&older), EventEmitOutcome::Recorded);
        let rejected = order.map(|role| {
            let id = match role {
                Role::Sampler => host.sampler_id,
                Role::BackupSampler => host.backup_sampler_id,
                Role::Heartbeat => panic!("the selected roles must be samplers"),
            };
            host.clock.advance(Duration::from_secs(1)).unwrap();
            let expected = Event::new(
                EventSource::Application(id),
                EventSeverity::Error,
                Diagnostic::WorkFailed,
                EventTimestamp::from_instant(host.clock.now()),
            );
            let failure = host.turn(role).unwrap_err();
            assert_eq!(
                failure.work_error(),
                &RuntimeWorkError::Application {
                    application_id: id,
                    source: SampleUnavailable,
                }
            );
            assert_eq!(host.runtime.state(id).unwrap(), ApplicationState::Failed);
            let attempt = failure.failure_event_attempt().unwrap();
            assert_eq!(
                attempt.outcome(),
                EventEmitOutcome::QueueFull { capacity: 1 }
            );
            assert_eq!(*attempt.event(), expected);
            expected
        });
        (host, older, rejected)
    }
}
