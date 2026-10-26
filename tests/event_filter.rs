//! Public-only optional diagnostic consumer of explicit severity filtering.

use std::cell::{Cell, RefCell};
use std::convert::Infallible;
use std::error::Error;
use std::fmt;
use std::time::Duration;

use rust_flight_framework::{
    Application, ApplicationId, ApplicationState, ApplicationWorkContext, Clock, Event,
    EventEmitOutcome, EventFilter, EventQueue, EventSeverity, EventSource, EventTimestamp,
    FilteredEventOutcome, ManualClock, Runtime, RuntimeWorkError,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DiagnosticError {
    ClockBusy,
    QueueBusy,
    QueueFull { capacity: usize },
}

struct WorkObservations {
    severity: Cell<EventSeverity>,
    completed_samples: Cell<u32>,
    attempted: Cell<Option<Event<u8>>>,
    outcome: Cell<Option<FilteredEventOutcome>>,
}

struct OptionalDiagnosticProducer<'a> {
    observations: &'a WorkObservations,
    events: &'a RefCell<EventQueue<u8>>,
    clock: &'a RefCell<ManualClock>,
}

impl fmt::Display for DiagnosticError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ClockBusy => formatter.write_str("mission clock is borrowed"),
            Self::QueueBusy => formatter.write_str("mission event queue is borrowed"),
            Self::QueueFull { capacity } => {
                write!(formatter, "mission event queue is full at {capacity}")
            }
        }
    }
}

impl Error for DiagnosticError {}

impl WorkObservations {
    fn new(severity: EventSeverity) -> Self {
        Self {
            severity: Cell::new(severity),
            completed_samples: Cell::new(0),
            attempted: Cell::new(None),
            outcome: Cell::new(None),
        }
    }
}

impl Application for OptionalDiagnosticProducer<'_> {
    type StartError = Infallible;
    type WorkError = DiagnosticError;
    type StopError = Infallible;
    type RestartError = Infallible;

    fn start(&mut self) -> Result<(), Self::StartError> {
        Ok(())
    }

    fn work(&mut self, context: ApplicationWorkContext<'_>) -> Result<(), Self::WorkError> {
        let clock = self
            .clock
            .try_borrow()
            .map_err(|_| DiagnosticError::ClockBusy)?;
        let event = Event::new(
            EventSource::Application(context.application_id()),
            self.observations.severity.get(),
            41,
            EventTimestamp::from_clock(&*clock),
        );
        self.observations.attempted.set(Some(event));
        let outcome = self
            .events
            .try_borrow_mut()
            .map_err(|_| DiagnosticError::QueueBusy)?
            .emit_filtered(&event, EventFilter::new(EventSeverity::Error));
        self.observations.outcome.set(Some(outcome));
        match outcome {
            FilteredEventOutcome::FilteredOut
            | FilteredEventOutcome::Emitted(EventEmitOutcome::Recorded) => {
                // This mission permits its effect after optional omission.
                self.observations
                    .completed_samples
                    .set(self.observations.completed_samples.get() + 1);
                Ok(())
            }
            FilteredEventOutcome::Emitted(EventEmitOutcome::QueueFull { capacity }) => {
                Err(DiagnosticError::QueueFull { capacity })
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

fn framework_event(identifier: u8, severity: EventSeverity, seconds: u64) -> Event<u8> {
    Event::new(
        EventSource::Framework,
        severity,
        identifier,
        EventTimestamp::from_elapsed(Duration::from_secs(seconds)),
    )
}

fn registered_mission<'a>(
    observations: &'a [WorkObservations; 2],
    events: &'a RefCell<EventQueue<u8>>,
    clock: &'a RefCell<ManualClock>,
) -> (Runtime<OptionalDiagnosticProducer<'a>>, [ApplicationId; 2]) {
    let mut runtime = Runtime::new(2).unwrap();
    let identities = observations.each_ref().map(|observations| {
        let identity = runtime
            .register(OptionalDiagnosticProducer {
                observations,
                events,
                clock,
            })
            .unwrap();
        runtime.start(identity).unwrap();
        identity
    });
    assert_ne!(identities[0], identities[1]);
    (runtime, identities)
}

#[test]
fn severity_minima_have_an_explicit_nine_case_selection_table() {
    let severities = [
        EventSeverity::Informational,
        EventSeverity::Warning,
        EventSeverity::Error,
    ];
    let cases = [
        (EventSeverity::Informational, [true, true, true]),
        (EventSeverity::Warning, [false, true, true]),
        (EventSeverity::Error, [false, false, true]),
    ];
    for (minimum, expected) in cases {
        let filter = EventFilter::new(minimum);
        assert_eq!(filter.minimum_severity(), minimum);
        for (severity, allowed) in severities.into_iter().zip(expected) {
            assert_eq!(
                filter.allows(severity),
                allowed,
                "{minimum:?}, {severity:?}"
            );
        }
    }
}

#[test]
fn suppression_preserves_empty_partial_and_full_queue_records() {
    let older = [
        framework_event(1, EventSeverity::Warning, 12),
        framework_event(2, EventSeverity::Error, 3),
    ];
    let omitted = framework_event(3, EventSeverity::Informational, 99);
    let filter = EventFilter::new(EventSeverity::Warning);
    for retained in 0..=2 {
        let mut queue = EventQueue::new(2).unwrap();
        for event in older.iter().take(retained) {
            assert_eq!(queue.emit(event), EventEmitOutcome::Recorded);
        }
        assert_eq!(
            queue.emit_filtered(&omitted, filter),
            FilteredEventOutcome::FilteredOut
        );
        assert_eq!(queue.capacity(), 2);
        assert_eq!(queue.pending(), retained);
        assert_eq!(queue.is_empty(), retained == 0);
        for event in older.iter().take(retained) {
            assert_eq!(queue.dequeue(), Some(*event));
        }
        assert_eq!(queue.dequeue(), None);
    }
}

#[test]
fn allowed_full_rejection_retains_the_borrowed_record_for_explicit_retry() {
    let older = [
        framework_event(1, EventSeverity::Error, 8),
        framework_event(2, EventSeverity::Warning, 2),
    ];
    let retry = framework_event(3, EventSeverity::Error, 19);
    let filter = EventFilter::new(EventSeverity::Error);
    let mut queue = EventQueue::new(2).unwrap();
    for event in &older {
        assert_eq!(queue.emit(event), EventEmitOutcome::Recorded);
    }
    assert_eq!(
        queue.emit_filtered(&retry, filter),
        FilteredEventOutcome::Emitted(EventEmitOutcome::QueueFull { capacity: 2 })
    );
    assert_eq!(queue.pending(), 2);
    assert_eq!(queue.dequeue(), Some(older[0]));
    assert_eq!(
        queue.emit_filtered(&retry, filter),
        FilteredEventOutcome::Emitted(EventEmitOutcome::Recorded)
    );
    assert_eq!(queue.dequeue(), Some(older[1]));
    assert_eq!(queue.dequeue(), Some(retry));
    assert_eq!(queue.dequeue(), None);
}

#[test]
fn mixed_filtered_and_unfiltered_emissions_do_not_install_queue_policy() {
    let info = framework_event(1, EventSeverity::Informational, 4);
    let error = framework_event(2, EventSeverity::Error, 9);
    let warning = framework_event(3, EventSeverity::Warning, 6);
    let filter = EventFilter::new(EventSeverity::Error);
    let mut queue = EventQueue::new(3).unwrap();
    assert_eq!(
        queue.emit_filtered(&info, filter),
        FilteredEventOutcome::FilteredOut
    );
    assert_eq!(queue.emit(&info), EventEmitOutcome::Recorded);
    assert_eq!(
        queue.emit_filtered(&error, filter),
        FilteredEventOutcome::Emitted(EventEmitOutcome::Recorded)
    );
    assert_eq!(queue.emit(&warning), EventEmitOutcome::Recorded);
    for event in [info, error, warning] {
        assert_eq!(queue.dequeue(), Some(event));
    }
    assert!(queue.is_empty());
}

#[test]
fn changing_each_attempts_filter_does_not_prune_older_accepted_records() {
    let before = framework_event(1, EventSeverity::Informational, 1);
    let excluded = framework_event(2, EventSeverity::Warning, 2);
    let error = framework_event(3, EventSeverity::Error, 3);
    let after = framework_event(4, EventSeverity::Informational, 4);
    let all = EventFilter::new(EventSeverity::Informational);
    let errors = EventFilter::new(EventSeverity::Error);
    let mut queue = EventQueue::new(3).unwrap();
    for (event, filter, outcome) in [
        (
            before,
            all,
            FilteredEventOutcome::Emitted(EventEmitOutcome::Recorded),
        ),
        (excluded, errors, FilteredEventOutcome::FilteredOut),
        (
            error,
            errors,
            FilteredEventOutcome::Emitted(EventEmitOutcome::Recorded),
        ),
        (
            after,
            all,
            FilteredEventOutcome::Emitted(EventEmitOutcome::Recorded),
        ),
    ] {
        assert_eq!(queue.emit_filtered(&event, filter), outcome);
    }
    assert_eq!(queue.pending(), 3);
    for event in [before, error, after] {
        assert_eq!(queue.dequeue(), Some(event));
    }
    assert!(queue.is_empty());
}

#[test]
fn optional_diagnostics_allow_work_and_leave_room_for_a_later_error() {
    let observations = [
        WorkObservations::new(EventSeverity::Informational),
        WorkObservations::new(EventSeverity::Informational),
    ];
    let events = RefCell::new(EventQueue::new(1).unwrap());
    let clock = RefCell::new(ManualClock::from_elapsed(Duration::from_secs(7)));
    let (mut runtime, identities) = registered_mission(&observations, &events, &clock);
    for completed in 1..=3 {
        runtime.work(identities[0]).unwrap();
        assert_eq!(observations[0].completed_samples.get(), completed);
        assert_eq!(
            observations[0].outcome.get(),
            Some(FilteredEventOutcome::FilteredOut)
        );
        assert!(events.borrow().is_empty());
        assert_eq!(runtime.state(identities[0]), Ok(ApplicationState::Running));
    }
    assert_eq!(
        observations[0].attempted.get(),
        Some(Event::new(
            EventSource::Application(identities[0]),
            EventSeverity::Informational,
            41,
            EventTimestamp::from_elapsed(Duration::from_secs(7)),
        ))
    );
    observations[0].severity.set(EventSeverity::Error);
    clock.borrow_mut().advance(Duration::from_secs(5)).unwrap();
    runtime.work(identities[0]).unwrap();
    assert_eq!(observations[0].completed_samples.get(), 4);
    assert_eq!(
        observations[0].outcome.get(),
        Some(FilteredEventOutcome::Emitted(EventEmitOutcome::Recorded))
    );
    let accepted = events.borrow_mut().dequeue().unwrap();
    assert_eq!(accepted.source(), EventSource::Application(identities[0]));
    assert_eq!(accepted.severity(), EventSeverity::Error);
    assert_eq!(accepted.identifier(), &41);
    assert_eq!(accepted.timestamp().instant(), clock.borrow().now());
    assert_eq!(accepted.timestamp().elapsed(), Duration::from_secs(12));
    assert!(events.borrow().is_empty());
}

#[test]
fn allowed_saturation_is_terminal_but_preserves_older_records_and_peer_work() {
    let observations = [
        WorkObservations::new(EventSeverity::Error),
        WorkObservations::new(EventSeverity::Informational),
    ];
    let events = RefCell::new(EventQueue::new(1).unwrap());
    let clock = RefCell::new(ManualClock::from_elapsed(Duration::from_secs(23)));
    let retained = framework_event(9, EventSeverity::Error, 2);
    assert_eq!(
        events.borrow_mut().emit(&retained),
        EventEmitOutcome::Recorded
    );
    let (mut runtime, identities) = registered_mission(&observations, &events, &clock);
    let error = runtime.work(identities[0]).unwrap_err();
    assert_eq!(
        error,
        RuntimeWorkError::Application {
            application_id: identities[0],
            source: DiagnosticError::QueueFull { capacity: 1 },
        }
    );
    assert_eq!(
        Error::source(&error).and_then(|source| source.downcast_ref()),
        Some(&DiagnosticError::QueueFull { capacity: 1 })
    );
    assert_eq!(observations[0].completed_samples.get(), 0);
    assert_eq!(
        observations[0].outcome.get(),
        Some(FilteredEventOutcome::Emitted(EventEmitOutcome::QueueFull {
            capacity: 1
        }))
    );
    assert_eq!(runtime.state(identities[0]), Ok(ApplicationState::Failed));
    runtime.work(identities[1]).unwrap();
    assert_eq!(observations[1].completed_samples.get(), 1);
    assert_eq!(
        observations[1].outcome.get(),
        Some(FilteredEventOutcome::FilteredOut)
    );
    assert_eq!(runtime.state(identities[1]), Ok(ApplicationState::Running));
    assert_eq!(events.borrow().pending(), 1);
    assert_eq!(events.borrow_mut().dequeue(), Some(retained));
    assert!(events.borrow().is_empty());
}
