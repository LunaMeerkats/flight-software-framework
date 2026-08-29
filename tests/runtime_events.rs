use std::cell::Cell;
use std::convert::Infallible;
use std::error::Error;
use std::fmt;
use std::rc::Rc;
use std::time::Duration;

use rust_flight_framework::{
    Application, ApplicationId, ApplicationState, Clock, Event, EventEmitOutcome, EventQueue,
    EventSeverity, EventSource, EventTimestamp, FrameworkInstant, LifecycleError,
    LifecycleRegistry, Runtime, RuntimeWorkError, RuntimeWorkEventError,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MissionEventId {
    RetainedDiagnostic,
    WorkReturnedError,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct MissionWorkError {
    code: u16,
}

impl fmt::Display for MissionWorkError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "mission work rejected with code {}", self.code)
    }
}

impl Error for MissionWorkError {}

#[derive(Debug)]
enum MissionApplication {
    Healthy {
        work_calls: Rc<Cell<usize>>,
    },
    Faulting {
        work_calls: Rc<Cell<usize>>,
        error_code: u16,
    },
}

impl Application for MissionApplication {
    type StartError = Infallible;
    type WorkError = MissionWorkError;
    type StopError = Infallible;
    type RestartError = Infallible;

    fn start(&mut self) -> Result<(), Self::StartError> {
        Ok(())
    }

    fn work(&mut self) -> Result<(), Self::WorkError> {
        match self {
            Self::Healthy { work_calls } => {
                work_calls.set(work_calls.get() + 1);
                Ok(())
            }
            Self::Faulting {
                work_calls,
                error_code,
            } => {
                work_calls.set(work_calls.get() + 1);
                Err(MissionWorkError { code: *error_code })
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

#[derive(Debug)]
struct CountingClock {
    instant: FrameworkInstant,
    reads: Cell<usize>,
}

impl CountingClock {
    fn at(elapsed: Duration) -> Self {
        Self {
            instant: FrameworkInstant::from_elapsed(elapsed),
            reads: Cell::new(0),
        }
    }

    fn reads(&self) -> usize {
        self.reads.get()
    }
}

impl Clock for CountingClock {
    fn now(&self) -> FrameworkInstant {
        self.reads.set(self.reads.get() + 1);
        self.instant
    }
}

struct StartedRuntime {
    runtime: Runtime<MissionApplication>,
    faulting_id: ApplicationId,
    peer_id: ApplicationId,
    faulting_work_calls: Rc<Cell<usize>>,
    peer_work_calls: Rc<Cell<usize>>,
}

fn started_runtime(error_code: u16) -> StartedRuntime {
    let faulting_work_calls = Rc::new(Cell::new(0));
    let peer_work_calls = Rc::new(Cell::new(0));
    let mut runtime = Runtime::new(2).expect("two runtime records can be reserved");
    let faulting_id = runtime
        .register(MissionApplication::Faulting {
            work_calls: Rc::clone(&faulting_work_calls),
            error_code,
        })
        .expect("faulting application fits");
    let peer_id = runtime
        .register(MissionApplication::Healthy {
            work_calls: Rc::clone(&peer_work_calls),
        })
        .expect("healthy peer fits");
    runtime
        .start(faulting_id)
        .expect("faulting application starts");
    runtime.start(peer_id).expect("healthy peer starts");

    StartedRuntime {
        runtime,
        faulting_id,
        peer_id,
        faulting_work_calls,
        peer_work_calls,
    }
}

fn registered_healthy_runtime() -> (Runtime<MissionApplication>, ApplicationId, Rc<Cell<usize>>) {
    let work_calls = Rc::new(Cell::new(0));
    let mut runtime = Runtime::new(1).expect("one runtime record can be reserved");
    let application_id = runtime
        .register(MissionApplication::Healthy {
            work_calls: Rc::clone(&work_calls),
        })
        .expect("healthy application fits");
    (runtime, application_id, work_calls)
}

fn expected_work_error(
    application_id: ApplicationId,
    code: u16,
) -> RuntimeWorkError<MissionWorkError> {
    RuntimeWorkError::Application {
        application_id,
        source: MissionWorkError { code },
    }
}

fn expected_failure_event(
    application_id: ApplicationId,
    elapsed: Duration,
) -> Event<MissionEventId> {
    Event::new(
        EventSource::Application(application_id),
        EventSeverity::Error,
        MissionEventId::WorkReturnedError,
        EventTimestamp::from_elapsed(elapsed),
    )
}

fn assert_error_source_chain(
    error: &RuntimeWorkEventError<MissionWorkError, MissionEventId>,
    expected_code: u16,
) {
    let runtime_error = Error::source(error)
        .and_then(|source| source.downcast_ref::<RuntimeWorkError<MissionWorkError>>())
        .expect("integration error source is the complete runtime work error");
    assert_eq!(runtime_error, error.work_error());
    assert_eq!(
        Error::source(runtime_error).and_then(|source| source.downcast_ref()),
        Some(&MissionWorkError {
            code: expected_code
        })
    );
}

#[test]
fn returned_work_error_records_clock_captured_event_and_preserves_peer_progress() {
    let mut fixture = started_runtime(17);
    let clock = CountingClock::at(Duration::from_secs(9));
    let mut events = EventQueue::new(2).expect("two event records can be reserved");

    let returned_error = fixture
        .runtime
        .work_with_failure_event(
            fixture.faulting_id,
            MissionEventId::WorkReturnedError,
            &clock,
            &mut events,
        )
        .expect_err("faulting work returns its concrete error");
    let expected_error = expected_work_error(fixture.faulting_id, 17);
    assert_eq!(returned_error.work_error(), &expected_error);
    assert_error_source_chain(&returned_error, 17);
    assert_eq!(clock.reads(), 1);
    assert_eq!(fixture.faulting_work_calls.get(), 1);
    assert_eq!(
        fixture.runtime.state(fixture.faulting_id),
        Ok(ApplicationState::Failed)
    );

    let (work_error, attempt) = returned_error.into_parts();
    assert_eq!(work_error, expected_error);
    let attempt = attempt.expect("returned application error attempts one event");
    let expected_event = expected_failure_event(fixture.faulting_id, Duration::from_secs(9));
    assert_eq!(attempt.outcome(), EventEmitOutcome::Recorded);
    assert_eq!(attempt.event(), &expected_event);
    assert_eq!(events.dequeue(), Some(expected_event));
    assert!(events.is_empty());

    let repeated_error = fixture
        .runtime
        .work_with_failure_event(
            fixture.faulting_id,
            MissionEventId::WorkReturnedError,
            &clock,
            &mut events,
        )
        .expect_err("failed application is rejected before another callback");
    assert_eq!(
        repeated_error.work_error(),
        &RuntimeWorkError::Lifecycle(LifecycleError::NotRunning {
            application_id: fixture.faulting_id,
            state: ApplicationState::Failed,
        })
    );
    assert_eq!(repeated_error.failure_event_attempt(), None);
    assert_eq!(clock.reads(), 1);
    assert!(events.is_empty());
    assert_eq!(fixture.faulting_work_calls.get(), 1);

    assert_eq!(
        fixture.runtime.work(fixture.peer_id),
        Ok(ApplicationState::Running)
    );
    assert_eq!(fixture.peer_work_calls.get(), 1);
}

#[test]
fn saturated_failure_event_retains_error_and_exact_event_for_retry() {
    let mut fixture = started_runtime(23);
    let clock = CountingClock::at(Duration::from_secs(12));
    let mut events = EventQueue::new(1).expect("one event record can be reserved");
    let retained_event = Event::new(
        EventSource::Framework,
        EventSeverity::Warning,
        MissionEventId::RetainedDiagnostic,
        EventTimestamp::from_elapsed(Duration::from_secs(2)),
    );
    assert_eq!(events.emit(&retained_event), EventEmitOutcome::Recorded);

    let returned_error = fixture
        .runtime
        .work_with_failure_event(
            fixture.faulting_id,
            MissionEventId::WorkReturnedError,
            &clock,
            &mut events,
        )
        .expect_err("faulting work remains the primary error");
    let (work_error, attempt) = returned_error.into_parts();
    assert_eq!(work_error, expected_work_error(fixture.faulting_id, 23));
    let attempt = attempt.expect("returned application error attempts one event");
    let expected_event = expected_failure_event(fixture.faulting_id, Duration::from_secs(12));
    assert_eq!(attempt.event(), &expected_event);
    assert_eq!(
        attempt.outcome(),
        EventEmitOutcome::QueueFull { capacity: 1 }
    );
    assert_eq!(clock.reads(), 1);
    assert_eq!(
        fixture.runtime.state(fixture.faulting_id),
        Ok(ApplicationState::Failed)
    );
    assert_eq!(
        fixture.runtime.state(fixture.peer_id),
        Ok(ApplicationState::Running)
    );
    assert_eq!(
        fixture.runtime.work(fixture.peer_id),
        Ok(ApplicationState::Running)
    );
    assert_eq!(fixture.peer_work_calls.get(), 1);
    assert_eq!(events.dequeue(), Some(retained_event));

    assert_eq!(events.emit(attempt.event()), EventEmitOutcome::Recorded);
    assert_eq!(events.dequeue(), Some(expected_event));
}

#[test]
fn lifecycle_rejections_do_not_read_clock_or_emit() {
    let (mut runtime, application_id, work_calls) = registered_healthy_runtime();
    let clock = CountingClock::at(Duration::from_secs(15));
    let mut events = EventQueue::new(1).expect("one event record can be reserved");

    let rejection = runtime
        .work_with_failure_event(
            application_id,
            MissionEventId::WorkReturnedError,
            &clock,
            &mut events,
        )
        .expect_err("registered application is not running");
    assert_eq!(
        rejection.work_error(),
        &RuntimeWorkError::Lifecycle(LifecycleError::NotRunning {
            application_id,
            state: ApplicationState::Registered,
        })
    );
    assert_eq!(rejection.failure_event_attempt(), None);
    assert_eq!(clock.reads(), 0);
    assert!(events.is_empty());
    assert_eq!(work_calls.get(), 0);

    let mut issuing_registry = LifecycleRegistry::new(2).expect("two issuing records fit");
    issuing_registry
        .register()
        .expect("first issuing record fits");
    let out_of_range_id = issuing_registry
        .register()
        .expect("second issuing record fits");
    let unknown = runtime
        .work_with_failure_event(
            out_of_range_id,
            MissionEventId::WorkReturnedError,
            &clock,
            &mut events,
        )
        .expect_err("out-of-range identity is unknown to this runtime");
    assert_eq!(
        unknown.work_error(),
        &RuntimeWorkError::Lifecycle(LifecycleError::UnknownApplication {
            application_id: out_of_range_id,
        })
    );
    assert_eq!(unknown.failure_event_attempt(), None);
    assert_eq!(clock.reads(), 0);
    assert!(events.is_empty());
    assert_eq!(work_calls.get(), 0);
}

#[test]
fn successful_work_does_not_read_clock_or_emit() {
    let (mut runtime, application_id, work_calls) = registered_healthy_runtime();
    let clock = CountingClock::at(Duration::from_secs(15));
    let mut events = EventQueue::new(1).expect("one event record can be reserved");
    runtime
        .start(application_id)
        .expect("healthy application starts");
    assert_eq!(
        runtime.work_with_failure_event(
            application_id,
            MissionEventId::WorkReturnedError,
            &clock,
            &mut events,
        ),
        Ok(ApplicationState::Running)
    );
    assert_eq!(clock.reads(), 0);
    assert!(events.is_empty());
    assert_eq!(work_calls.get(), 1);
}
