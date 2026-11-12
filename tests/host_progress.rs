//! A serial public-API consumer that observes fresh samples after work.

use std::cell::Cell;
use std::convert::Infallible;
use std::error::Error;
use std::fmt;
use std::time::Duration;

use rust_flight_framework::{
    Application, ApplicationId, ApplicationState, ApplicationWorkContext, Clock, Event,
    EventEmitOutcome, EventQueue, EventSeverity, EventSource, EventTimestamp, FrameworkInstant,
    LifecycleError, ManualClock, Runtime, RuntimeWorkError, RuntimeWorkEventError,
};

const SAMPLE_WINDOW: Duration = Duration::from_secs(2);

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

fn overdue_event(application_id: ApplicationId, seconds: u64) -> Event<Diagnostic> {
    Event::new(
        EventSource::Application(application_id),
        EventSeverity::Warning,
        Diagnostic::SampleOverdue,
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
