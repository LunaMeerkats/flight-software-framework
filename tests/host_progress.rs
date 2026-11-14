//! Serial public-API consumers that observe fresh samples after work.

use std::cell::Cell;
use std::convert::Infallible;
use std::error::Error;
use std::fmt;
use std::time::Duration;

use rust_flight_framework::{
    Application, ApplicationId, ApplicationState, ApplicationWorkContext, Clock, Event,
    EventEmitOutcome, EventQueue, EventSeverity, EventSource, EventTimestamp, FrameworkInstant,
    LifecycleError, ManualClock, PeriodicWork, PeriodicWorkOutcome, Runtime, RuntimeWorkError,
    RuntimeWorkEventError, ScheduledWork,
};

const SAMPLE_WINDOW: Duration = Duration::from_secs(2);
const WORK_INTERVAL: Duration = Duration::from_secs(1);
const CONGESTION_WINDOW: Duration = Duration::from_secs(10);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SampleInput {
    Empty,
    Ready(u8),
    ReadFailed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ReadFailed;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Diagnostic {
    SampleOverdue,
    DownstreamCongested,
    WorkReturnedError,
}

#[derive(Debug, Eq, PartialEq)]
enum TurnProgress {
    Sample(u8),
    NoSample,
    Overdue {
        event: Event<Diagnostic>,
        outcome: EventEmitOutcome,
    },
}

#[derive(Debug, Eq, PartialEq)]
enum PeriodicTurn {
    Deferred,
    Congested {
        event: Event<Diagnostic>,
        outcome: EventEmitOutcome,
    },
    Disabled,
    Idle(PeriodicWorkOutcome),
    Worked {
        released: ScheduledWork,
        progress: TurnProgress,
    },
}

#[derive(Debug)]
struct Sampler<'a> {
    input: &'a Cell<SampleInput>,
    output: &'a Cell<Option<u8>>,
}

struct SampleHost<'a> {
    runtime: Runtime<Sampler<'a>>,
    application_id: ApplicationId,
    output: &'a Cell<Option<u8>>,
    clock: ManualClock,
    events: EventQueue<Diagnostic>,
    window_started_at: FrameworkInstant,
    warning_attempted: bool,
}

struct Deferral {
    started_at: FrameworkInstant,
    warning_attempted: bool,
}

struct PeriodicSampleHost<'a> {
    sampler: SampleHost<'a>,
    cursor: Option<PeriodicWork>,
    pending_sample: Option<u8>,
    deferral: Option<Deferral>,
}

impl Application for Sampler<'_> {
    type StartError = Infallible;
    type WorkError = ReadFailed;
    type StopError = Infallible;
    type RestartError = Infallible;

    fn start(&mut self) -> Result<(), Self::StartError> {
        Ok(())
    }

    fn work(&mut self, _context: ApplicationWorkContext<'_>) -> Result<(), Self::WorkError> {
        match self.input.replace(SampleInput::Empty) {
            SampleInput::Empty => Ok(()),
            SampleInput::Ready(value) => {
                self.output.set(Some(value));
                Ok(())
            }
            SampleInput::ReadFailed => Err(ReadFailed),
        }
    }

    fn stop(&mut self) -> Result<(), Self::StopError> {
        Ok(())
    }

    fn restart(&mut self) -> Result<(), Self::RestartError> {
        Ok(())
    }
}

impl fmt::Display for ReadFailed {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("sample input failed")
    }
}

impl Error for ReadFailed {}

impl<'a> SampleHost<'a> {
    fn start(input: &'a Cell<SampleInput>, output: &'a Cell<Option<u8>>) -> Self {
        let mut runtime = Runtime::new(1).expect("one application record");
        let events = EventQueue::new(1).expect("one event record");
        let clock = ManualClock::new();
        let application_id = runtime.register(Sampler { input, output }).unwrap();
        runtime.start(application_id).unwrap();
        output.set(None);

        Self {
            runtime,
            application_id,
            output,
            window_started_at: clock.now(),
            clock,
            events,
            warning_attempted: false,
        }
    }

    fn work_turn(&mut self) -> Result<TurnProgress, RuntimeWorkEventError<ReadFailed, Diagnostic>> {
        let work = self.runtime.work_with_failure_event(
            self.application_id,
            Diagnostic::WorkReturnedError,
            &self.clock,
            &mut self.events,
        );
        let sample = self.output.take();
        work?;

        let now = self.clock.now();
        if let Some(value) = sample {
            self.window_started_at = now;
            self.warning_attempted = false;
            return Ok(TurnProgress::Sample(value));
        }

        // Both instants come from the host's one nondecreasing ManualClock.
        let elapsed = now.elapsed() - self.window_started_at.elapsed();
        if self.warning_attempted || elapsed < SAMPLE_WINDOW {
            return Ok(TurnProgress::NoSample);
        }

        self.warning_attempted = true;
        let event = Event::new(
            EventSource::Application(self.application_id),
            EventSeverity::Warning,
            Diagnostic::SampleOverdue,
            EventTimestamp::from_instant(now),
        );
        let outcome = self.events.emit(&event);
        Ok(TurnProgress::Overdue { event, outcome })
    }
}

impl<'a> PeriodicSampleHost<'a> {
    fn start(input: &'a Cell<SampleInput>, output: &'a Cell<Option<u8>>) -> Self {
        let sampler = SampleHost::start(input, output);
        let first = ScheduledWork::new(sampler.application_id, sampler.clock.now());
        Self {
            sampler,
            cursor: Some(PeriodicWork::new(first, WORK_INTERVAL).unwrap()),
            pending_sample: None,
            deferral: None,
        }
    }

    fn turn(&mut self) -> Result<PeriodicTurn, RuntimeWorkEventError<ReadFailed, Diagnostic>> {
        let Some(cursor) = &mut self.cursor else {
            return Ok(PeriodicTurn::Disabled);
        };
        if self.pending_sample.is_some() {
            return Ok(self.deferred_turn());
        }
        if let Some(deferral) = self.deferral.take() {
            // These readings share one clock: window start <= deferral <= now.
            let elapsed_before_deferral =
                deferral.started_at.elapsed() - self.sampler.window_started_at.elapsed();
            let now = self.sampler.clock.now();
            self.sampler.window_started_at =
                FrameworkInstant::from_elapsed(now.elapsed() - elapsed_before_deferral);
        }

        let release = cursor.poll(&self.sampler.clock);
        let PeriodicWorkOutcome::Released { scheduled_work, .. } = release else {
            return Ok(PeriodicTurn::Idle(release));
        };
        let progress = self
            .sampler
            .work_turn()
            .inspect_err(|_| self.cursor = None)?;
        if let TurnProgress::Sample(value) = progress {
            self.pending_sample = Some(value);
        }
        Ok(PeriodicTurn::Worked {
            released: scheduled_work,
            progress,
        })
    }

    fn take_pending_sample(&mut self) -> Option<u8> {
        self.pending_sample.take()
    }

    fn deferred_turn(&mut self) -> PeriodicTurn {
        let now = self.sampler.clock.now();
        let deferral = self.deferral.get_or_insert(Deferral {
            started_at: now,
            warning_attempted: false,
        });
        // Both times are observations on the same nondecreasing clock.
        let elapsed = now.elapsed() - deferral.started_at.elapsed();
        if deferral.warning_attempted || elapsed < CONGESTION_WINDOW {
            return PeriodicTurn::Deferred;
        }

        deferral.warning_attempted = true;
        let event = Event::new(
            EventSource::Application(self.sampler.application_id),
            EventSeverity::Warning,
            Diagnostic::DownstreamCongested,
            EventTimestamp::from_instant(now),
        );
        let outcome = self.sampler.events.emit(&event);
        PeriodicTurn::Congested { event, outcome }
    }
}

fn overdue_event(application_id: ApplicationId, seconds: u64) -> Event<Diagnostic> {
    Event::new(
        EventSource::Application(application_id),
        EventSeverity::Warning,
        Diagnostic::SampleOverdue,
        EventTimestamp::from_elapsed(Duration::from_secs(seconds)),
    )
}

fn congestion_event(application_id: ApplicationId, seconds: u64) -> Event<Diagnostic> {
    Event::new(
        EventSource::Application(application_id),
        EventSeverity::Warning,
        Diagnostic::DownstreamCongested,
        EventTimestamp::from_elapsed(Duration::from_secs(seconds)),
    )
}

#[test]
fn fresh_sample_at_threshold_resets_window_and_rearms_warning() {
    let input = Cell::new(SampleInput::Empty);
    let output = Cell::new(None);
    let mut host = SampleHost::start(&input, &output);

    host.clock.advance(Duration::from_millis(1_999)).unwrap();
    assert_eq!(host.work_turn(), Ok(TurnProgress::NoSample));
    assert!(host.events.is_empty());

    host.clock.advance(Duration::from_millis(1)).unwrap();
    input.set(SampleInput::Ready(12));
    assert_eq!(host.work_turn(), Ok(TurnProgress::Sample(12)));
    assert!(host.events.is_empty());

    host.clock.advance(Duration::from_millis(1_999)).unwrap();
    assert_eq!(host.work_turn(), Ok(TurnProgress::NoSample));
    assert!(host.events.is_empty());

    host.clock.advance(Duration::from_millis(1)).unwrap();
    let first = overdue_event(host.application_id, 4);
    assert_eq!(
        host.work_turn(),
        Ok(TurnProgress::Overdue {
            event: first,
            outcome: EventEmitOutcome::Recorded,
        })
    );
    assert_eq!(
        host.runtime.state(host.application_id),
        Ok(ApplicationState::Running)
    );
    assert_eq!(host.events.dequeue(), Some(first));
    assert_eq!(host.work_turn(), Ok(TurnProgress::NoSample));
    assert!(host.events.is_empty());

    input.set(SampleInput::Ready(12));
    assert_eq!(host.work_turn(), Ok(TurnProgress::Sample(12)));
    host.clock.advance(Duration::from_secs(3)).unwrap();
    let second = overdue_event(host.application_id, 7);
    assert_eq!(
        host.work_turn(),
        Ok(TurnProgress::Overdue {
            event: second,
            outcome: EventEmitOutcome::Recorded,
        })
    );
    assert_eq!(host.events.dequeue(), Some(second));
    assert!(host.events.is_empty());
}

#[test]
fn full_queue_preserves_prior_warning_and_exposes_rejected_attempt() {
    let input = Cell::new(SampleInput::Empty);
    let output = Cell::new(None);
    let mut host = SampleHost::start(&input, &output);
    host.clock.advance(SAMPLE_WINDOW).unwrap();
    let prior = overdue_event(host.application_id, 2);
    assert_eq!(
        host.work_turn(),
        Ok(TurnProgress::Overdue {
            event: prior,
            outcome: EventEmitOutcome::Recorded,
        })
    );

    input.set(SampleInput::Ready(21));
    assert_eq!(host.work_turn(), Ok(TurnProgress::Sample(21)));
    host.clock.advance(SAMPLE_WINDOW).unwrap();
    let rejected = overdue_event(host.application_id, 4);
    assert_eq!(
        host.work_turn(),
        Ok(TurnProgress::Overdue {
            event: rejected,
            outcome: EventEmitOutcome::QueueFull { capacity: 1 },
        })
    );
    assert_eq!(
        host.runtime.state(host.application_id),
        Ok(ApplicationState::Running)
    );
    assert_eq!(host.events.dequeue(), Some(prior));
    assert_eq!(host.work_turn(), Ok(TurnProgress::NoSample));
    assert!(host.events.is_empty());
}

#[test]
fn returned_failure_at_threshold_skips_overdue_observation() {
    let input = Cell::new(SampleInput::ReadFailed);
    let output = Cell::new(None);
    let mut host = SampleHost::start(&input, &output);
    host.clock.advance(SAMPLE_WINDOW).unwrap();

    let error = host.work_turn().unwrap_err();
    assert_eq!(
        error.work_error(),
        &RuntimeWorkError::Application {
            application_id: host.application_id,
            source: ReadFailed,
        }
    );
    assert_eq!(
        host.runtime.state(host.application_id),
        Ok(ApplicationState::Failed)
    );
    let expected = Event::new(
        EventSource::Application(host.application_id),
        EventSeverity::Error,
        Diagnostic::WorkReturnedError,
        EventTimestamp::from_elapsed(Duration::from_secs(2)),
    );
    let attempt = error.failure_event_attempt().unwrap();
    assert_eq!(attempt.event(), &expected);
    assert_eq!(attempt.outcome(), EventEmitOutcome::Recorded);
    assert_eq!(host.events.dequeue(), Some(expected));

    input.set(SampleInput::Ready(31));
    let rejection = host.work_turn().unwrap_err();
    assert_eq!(
        rejection.work_error(),
        &RuntimeWorkError::Lifecycle(LifecycleError::NotRunning {
            application_id: host.application_id,
            state: ApplicationState::Failed,
        })
    );
    assert_eq!(input.get(), SampleInput::Ready(31));
    assert!(rejection.failure_event_attempt().is_none());
    assert!(host.events.is_empty());
}

#[test]
fn periodic_deferral_preserves_release_and_pre_deferral_elapsed_time() {
    let input = Cell::new(SampleInput::Ready(7));
    let output = Cell::new(None);
    let mut host = PeriodicSampleHost::start(&input, &output);
    let id = host.sampler.application_id;
    assert_eq!(
        host.turn(),
        Ok(PeriodicTurn::Worked {
            released: ScheduledWork::new(id, FrameworkInstant::ZERO),
            progress: TurnProgress::Sample(7),
        })
    );

    let due = host.sampler.clock.advance(WORK_INTERVAL).unwrap();
    let retained = ScheduledWork::new(id, due);
    input.set(SampleInput::ReadFailed);
    assert_eq!(host.turn(), Ok(PeriodicTurn::Deferred));
    let resumed_at = host.sampler.clock.advance(Duration::from_secs(9)).unwrap();
    assert_eq!(host.turn(), Ok(PeriodicTurn::Deferred));
    assert_eq!(input.replace(SampleInput::Empty), SampleInput::ReadFailed);
    assert_eq!(
        host.cursor.as_ref().unwrap().next_scheduled_work(),
        Some(retained)
    );
    assert!(host.sampler.events.is_empty());
    assert_eq!(host.take_pending_sample(), Some(7));
    assert_eq!(
        host.turn(),
        Ok(PeriodicTurn::Worked {
            released: retained,
            progress: TurnProgress::NoSample,
        })
    );
    assert!(host.sampler.events.is_empty());

    let next_at = FrameworkInstant::from_elapsed(Duration::from_secs(11));
    let next = ScheduledWork::new(id, next_at);
    input.set(SampleInput::ReadFailed);
    assert_eq!(
        host.turn(),
        Ok(PeriodicTurn::Idle(PeriodicWorkOutcome::Waiting {
            next,
            observed_at: resumed_at,
        }))
    );
    assert_eq!(input.replace(SampleInput::Empty), SampleInput::ReadFailed);
    host.sampler.clock.advance(WORK_INTERVAL).unwrap();
    let event = overdue_event(id, 11);
    assert_eq!(
        host.turn(),
        Ok(PeriodicTurn::Worked {
            released: next,
            progress: TurnProgress::Overdue {
                event,
                outcome: EventEmitOutcome::Recorded,
            },
        })
    );
    assert_eq!(host.sampler.events.dequeue(), Some(event));
    assert_eq!(
        host.sampler.runtime.state(id),
        Ok(ApplicationState::Running)
    );
}

#[test]
fn periodic_failure_at_adjusted_threshold_disables_release() {
    let input = Cell::new(SampleInput::Ready(7));
    let output = Cell::new(None);
    let mut host = PeriodicSampleHost::start(&input, &output);
    let id = host.sampler.application_id;
    host.turn().unwrap();
    host.sampler.clock.advance(WORK_INTERVAL).unwrap();
    assert_eq!(host.turn(), Ok(PeriodicTurn::Deferred));
    host.sampler.clock.advance(Duration::from_secs(9)).unwrap();
    assert_eq!(host.take_pending_sample(), Some(7));
    host.turn().unwrap();

    host.sampler.clock.advance(WORK_INTERVAL).unwrap();
    input.set(SampleInput::ReadFailed);
    let error = host.turn().unwrap_err();
    assert_eq!(
        error.work_error(),
        &RuntimeWorkError::Application {
            application_id: id,
            source: ReadFailed,
        }
    );
    let expected = Event::new(
        EventSource::Application(id),
        EventSeverity::Error,
        Diagnostic::WorkReturnedError,
        EventTimestamp::from_elapsed(Duration::from_secs(11)),
    );
    let attempt = error.failure_event_attempt().unwrap();
    assert_eq!(attempt.event(), &expected);
    assert_eq!(attempt.outcome(), EventEmitOutcome::Recorded);
    assert_eq!(host.sampler.events.dequeue(), Some(expected));
    assert_eq!(host.sampler.runtime.state(id), Ok(ApplicationState::Failed));

    host.sampler.clock.advance(WORK_INTERVAL).unwrap();
    input.set(SampleInput::Ready(8));
    assert_eq!(host.turn(), Ok(PeriodicTurn::Disabled));
    assert_eq!(input.get(), SampleInput::Ready(8));
    assert!(host.sampler.events.is_empty());
}

#[test]
fn congestion_warns_at_threshold_and_rearms_after_room_is_observed() {
    let input = Cell::new(SampleInput::Ready(7));
    let output = Cell::new(None);
    let mut host = PeriodicSampleHost::start(&input, &output);
    let id = host.sampler.application_id;
    host.turn().unwrap();

    host.sampler.clock.advance(Duration::from_secs(3)).unwrap();
    input.set(SampleInput::ReadFailed);
    assert_eq!(host.turn(), Ok(PeriodicTurn::Deferred));
    host.sampler
        .clock
        .advance(Duration::from_millis(9_999))
        .unwrap();
    assert_eq!(host.turn(), Ok(PeriodicTurn::Deferred));
    assert!(host.sampler.events.is_empty());
    host.sampler
        .clock
        .advance(Duration::from_millis(1))
        .unwrap();
    let first = congestion_event(id, 13);
    assert_eq!(
        host.turn(),
        Ok(PeriodicTurn::Congested {
            event: first,
            outcome: EventEmitOutcome::Recorded,
        })
    );
    assert_eq!(host.sampler.events.dequeue(), Some(first));
    assert_eq!(host.turn(), Ok(PeriodicTurn::Deferred));
    assert!(host.sampler.events.is_empty());
    assert_eq!(host.sampler.window_started_at, FrameworkInstant::ZERO);
    assert_eq!(
        host.sampler.runtime.state(id),
        Ok(ApplicationState::Running)
    );
    assert_eq!(
        input.replace(SampleInput::Ready(8)),
        SampleInput::ReadFailed
    );
    assert_eq!(host.take_pending_sample(), Some(7));

    let retained = ScheduledWork::new(id, FrameworkInstant::from_elapsed(WORK_INTERVAL));
    assert_eq!(
        host.turn(),
        Ok(PeriodicTurn::Worked {
            released: retained,
            progress: TurnProgress::Sample(8),
        })
    );
    assert_eq!(host.turn(), Ok(PeriodicTurn::Deferred));
    host.sampler.clock.advance(Duration::from_secs(11)).unwrap();
    let second = congestion_event(id, 24);
    assert_eq!(
        host.turn(),
        Ok(PeriodicTurn::Congested {
            event: second,
            outcome: EventEmitOutcome::Recorded,
        })
    );
    assert_eq!(host.sampler.events.dequeue(), Some(second));
}

#[test]
fn rejected_congestion_warning_preserves_older_event_and_does_not_retry() {
    let input = Cell::new(SampleInput::Empty);
    let output = Cell::new(None);
    let mut host = PeriodicSampleHost::start(&input, &output);
    let id = host.sampler.application_id;
    host.turn().unwrap();
    host.sampler.clock.advance(SAMPLE_WINDOW).unwrap();
    host.turn().unwrap();
    let prior = overdue_event(id, 2);

    input.set(SampleInput::Ready(7));
    host.sampler.clock.advance(WORK_INTERVAL).unwrap();
    host.turn().unwrap();
    assert_eq!(host.turn(), Ok(PeriodicTurn::Deferred));
    host.sampler.clock.advance(CONGESTION_WINDOW).unwrap();
    assert_eq!(
        host.turn(),
        Ok(PeriodicTurn::Congested {
            event: congestion_event(id, 13),
            outcome: EventEmitOutcome::QueueFull { capacity: 1 },
        })
    );
    assert_eq!(host.sampler.events.dequeue(), Some(prior));
    assert_eq!(host.turn(), Ok(PeriodicTurn::Deferred));
    assert!(host.sampler.events.is_empty());
    assert_eq!(host.take_pending_sample(), Some(7));

    input.set(SampleInput::Ready(8));
    host.turn().unwrap();
    assert_eq!(host.turn(), Ok(PeriodicTurn::Deferred));
    host.sampler.clock.advance(CONGESTION_WINDOW).unwrap();
    assert_eq!(host.take_pending_sample(), Some(8));
    assert!(matches!(
        host.turn(),
        Ok(PeriodicTurn::Worked {
            progress: TurnProgress::NoSample,
            ..
        })
    ));
    assert!(host.sampler.events.is_empty());
}
