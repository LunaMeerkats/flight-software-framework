//! Public-API evidence for work events after lifecycle-owned inbox cleanup.

use std::cell::{Cell, RefCell};
use std::convert::Infallible;
use std::error::Error;
use std::fmt;
use std::rc::Rc;
use std::time::Duration;

use rust_flight_framework::{
    Application, ApplicationId, ApplicationMessageContext, ApplicationState,
    ApplicationWorkContext, Clock, ConfigurationError, ConfigurationTable, Event, EventEmitOutcome,
    EventQueue, EventSeverity, EventSource, EventTimestamp, FrameworkInstant, LifecycleError,
    LifecycleRegistry, Message, MessageDispatchOutcome, MessagingApplication,
    MessagingOperationError, MessagingRuntime, PublishClassification, Runtime,
    RuntimeConfigurationError, RuntimeInboxConfig, RuntimeWorkError, RuntimeWorkEventError,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MissionTopic {
    Command,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MissionEventId {
    OlderDiagnostic,
    WorkReturnedError,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct MissionWorkError(u16);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct InvalidConfiguration;

#[derive(Default)]
struct ApplicationObservations {
    work_calls: Cell<usize>,
    configuration: RefCell<Option<(u64, Vec<u8>)>>,
    messages: RefCell<Vec<u8>>,
}

struct MissionApplication {
    observations: Rc<ApplicationObservations>,
    failure: Option<MissionWorkError>,
}

struct CountingClock {
    reads: Cell<usize>,
}

type MissionRuntime =
    MessagingRuntime<MissionApplication, MissionTopic, 1, InvalidConfiguration, 1>;
type WorkEventError =
    MessagingOperationError<RuntimeWorkEventError<MissionWorkError, MissionEventId>>;

struct MissionFixture {
    runtime: MissionRuntime,
    faulting_id: ApplicationId,
    peer_id: ApplicationId,
    faulting: Rc<ApplicationObservations>,
    peer: Rc<ApplicationObservations>,
}

impl Application for MissionApplication {
    type StartError = Infallible;
    type WorkError = MissionWorkError;
    type StopError = Infallible;
    type RestartError = Infallible;

    fn start(&mut self) -> Result<(), Self::StartError> {
        Ok(())
    }

    fn work(&mut self, context: ApplicationWorkContext<'_>) -> Result<(), Self::WorkError> {
        self.observations
            .work_calls
            .set(self.observations.work_calls.get() + 1);
        *self.observations.configuration.borrow_mut() = context
            .configuration()
            .map(|view| (view.revision(), view.bytes().to_vec()));
        match self.failure {
            Some(error) => Err(error),
            None => Ok(()),
        }
    }

    fn stop(&mut self) -> Result<(), Self::StopError> {
        Ok(())
    }

    fn restart(&mut self) -> Result<(), Self::RestartError> {
        Ok(())
    }
}

impl MessagingApplication<MissionTopic, 1> for MissionApplication {
    type MessageError = Infallible;

    fn handle_message(
        &mut self,
        message: &Message<MissionTopic, 1>,
        _context: &mut ApplicationMessageContext<'_, MissionTopic, 1>,
    ) -> Result<(), Self::MessageError> {
        self.observations
            .messages
            .borrow_mut()
            .extend_from_slice(message.payload());
        Ok(())
    }
}

impl Clock for CountingClock {
    fn now(&self) -> FrameworkInstant {
        self.reads.set(self.reads.get() + 1);
        FrameworkInstant::from_elapsed(Duration::from_secs(9))
    }
}

impl fmt::Display for MissionWorkError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "mission work failure {}", self.0)
    }
}

impl Error for MissionWorkError {}

impl fmt::Display for InvalidConfiguration {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("configuration requires one percentage byte")
    }
}

impl Error for InvalidConfiguration {}

impl MissionFixture {
    fn registered() -> Self {
        let faulting = Rc::new(ApplicationObservations::default());
        let peer = Rc::new(ApplicationObservations::default());
        let table = ConfigurationTable::new(&[7], validate_configuration)
            .expect("initial test configuration is valid");
        let mut runtime = Runtime::with_configuration(2, table)
            .expect("two configured runtime records can be reserved");
        let faulting_id = runtime
            .register(MissionApplication {
                observations: Rc::clone(&faulting),
                failure: Some(MissionWorkError(17)),
            })
            .expect("faulting application fits");
        let peer_id = runtime
            .register(MissionApplication {
                observations: Rc::clone(&peer),
                failure: None,
            })
            .expect("peer application fits");
        let inboxes = [
            RuntimeInboxConfig::new(2, &[MissionTopic::Command]),
            RuntimeInboxConfig::new(2, &[MissionTopic::Command]),
        ];
        Self {
            runtime: MessagingRuntime::new(runtime, &inboxes)
                .expect("two matching bounded inboxes are valid"),
            faulting_id,
            peer_id,
            faulting,
            peer,
        }
    }

    fn started() -> Self {
        let mut fixture = Self::registered();
        fixture.runtime.start(fixture.faulting_id).unwrap();
        fixture.runtime.start(fixture.peer_id).unwrap();
        fixture
    }

    fn fill_inboxes(&mut self) {
        for value in [11, 22] {
            let message = Message::try_new(MissionTopic::Command, &[value]).unwrap();
            let report = self.runtime.publish(&message).unwrap();
            assert_eq!(report.classification(), PublishClassification::Complete);
        }
        assert_eq!(self.runtime.pending(self.faulting_id), Ok(2));
        assert_eq!(self.runtime.pending(self.peer_id), Ok(2));
    }
}

fn validate_configuration(bytes: &[u8]) -> Result<(), InvalidConfiguration> {
    match bytes {
        [value] if *value <= 100 => Ok(()),
        _ => Err(InvalidConfiguration),
    }
}

fn expected_work_error(application_id: ApplicationId) -> RuntimeWorkError<MissionWorkError> {
    RuntimeWorkError::Application {
        application_id,
        source: MissionWorkError(17),
    }
}

fn expected_failure_event(application_id: ApplicationId) -> Event<MissionEventId> {
    Event::new(
        EventSource::Application(application_id),
        EventSeverity::Error,
        MissionEventId::WorkReturnedError,
        EventTimestamp::from_elapsed(Duration::from_secs(9)),
    )
}

fn assert_error_source_chain(error: &WorkEventError) {
    let event_error = Error::source(error)
        .and_then(|source| {
            source.downcast_ref::<RuntimeWorkEventError<MissionWorkError, MissionEventId>>()
        })
        .expect("messaging source retains the full work-event error");
    assert_eq!(event_error, error.operation_error());
    let work_error = Error::source(event_error)
        .and_then(|source| source.downcast_ref::<RuntimeWorkError<MissionWorkError>>())
        .expect("event source retains the full runtime work error");
    assert_eq!(work_error, event_error.work_error());
    assert_eq!(
        Error::source(work_error).and_then(|source| source.downcast_ref()),
        Some(&MissionWorkError(17))
    );
}

#[test]
fn successful_work_preserves_inboxes_without_reading_clock_or_emitting() {
    let mut fixture = MissionFixture::started();
    fixture.fill_inboxes();
    let clock = CountingClock {
        reads: Cell::new(0),
    };
    let mut events = EventQueue::new(1).unwrap();

    assert_eq!(
        fixture.runtime.work_with_failure_event(
            fixture.peer_id,
            MissionEventId::WorkReturnedError,
            &clock,
            &mut events,
        ),
        Ok(ApplicationState::Running)
    );
    assert_eq!(clock.reads.get(), 0);
    assert!(events.is_empty());
    assert_eq!(fixture.peer.work_calls.get(), 1);
    assert_eq!(*fixture.peer.configuration.borrow(), Some((1, vec![7])));
    assert_eq!(fixture.faulting.work_calls.get(), 0);
    assert_eq!(fixture.runtime.pending(fixture.faulting_id), Ok(2));
    assert_eq!(fixture.runtime.pending(fixture.peer_id), Ok(2));
}

#[test]
fn non_running_work_preserves_peer_inbox_and_suppresses_event_effects() {
    for state in [
        ApplicationState::Registered,
        ApplicationState::Stopped,
        ApplicationState::Failed,
    ] {
        let mut fixture = MissionFixture::registered();
        match state {
            ApplicationState::Registered => {}
            ApplicationState::Stopped => {
                fixture.runtime.start(fixture.faulting_id).unwrap();
                fixture.runtime.stop(fixture.faulting_id).unwrap();
            }
            ApplicationState::Failed => {
                fixture.runtime.start(fixture.faulting_id).unwrap();
                fixture.runtime.work(fixture.faulting_id).unwrap_err();
            }
            ApplicationState::Running => unreachable!("not a rejection case"),
        }
        fixture.runtime.start(fixture.peer_id).unwrap();
        let message = Message::try_new(MissionTopic::Command, &[11]).unwrap();
        fixture.runtime.publish(&message).unwrap();
        let calls_before = fixture.faulting.work_calls.get();
        let clock = CountingClock {
            reads: Cell::new(0),
        };
        let mut events = EventQueue::new(1).unwrap();
        let error = fixture
            .runtime
            .work_with_failure_event(
                fixture.faulting_id,
                MissionEventId::WorkReturnedError,
                &clock,
                &mut events,
            )
            .unwrap_err();

        assert_eq!(error.discarded_deliveries(), 0);
        assert_eq!(
            error.operation_error().work_error(),
            &RuntimeWorkError::Lifecycle(LifecycleError::NotRunning {
                application_id: fixture.faulting_id,
                state,
            })
        );
        assert_eq!(error.operation_error().failure_event_attempt(), None);
        assert_eq!(clock.reads.get(), 0);
        assert!(events.is_empty());
        assert_eq!(fixture.faulting.work_calls.get(), calls_before);
        assert_eq!(fixture.runtime.state(fixture.faulting_id), Ok(state));
        assert_eq!(fixture.runtime.pending(fixture.faulting_id), Ok(0));
        assert_eq!(fixture.runtime.pending(fixture.peer_id), Ok(1));
    }
}

#[test]
fn unknown_work_preserves_all_inboxes_without_clock_or_event_effects() {
    let mut fixture = MissionFixture::started();
    fixture.fill_inboxes();
    let mut registry = LifecycleRegistry::new(3).unwrap();
    registry.register().unwrap();
    registry.register().unwrap();
    let unknown_id = registry.register().unwrap();
    let clock = CountingClock {
        reads: Cell::new(0),
    };
    let mut events = EventQueue::new(1).unwrap();
    let error = fixture
        .runtime
        .work_with_failure_event(
            unknown_id,
            MissionEventId::WorkReturnedError,
            &clock,
            &mut events,
        )
        .unwrap_err();

    assert_eq!(error.discarded_deliveries(), 0);
    assert_eq!(
        error.operation_error().work_error(),
        &RuntimeWorkError::Lifecycle(LifecycleError::UnknownApplication {
            application_id: unknown_id,
        })
    );
    assert_eq!(error.operation_error().failure_event_attempt(), None);
    assert_eq!(clock.reads.get(), 0);
    assert!(events.is_empty());
    assert_eq!(fixture.faulting.work_calls.get(), 0);
    assert_eq!(fixture.peer.work_calls.get(), 0);
    assert_eq!(fixture.runtime.pending(fixture.faulting_id), Ok(2));
    assert_eq!(fixture.runtime.pending(fixture.peer_id), Ok(2));
}

#[test]
fn returned_work_error_clears_selected_inbox_and_preserves_peer_dispatch() {
    let mut fixture = MissionFixture::started();
    fixture.fill_inboxes();
    let clock = CountingClock {
        reads: Cell::new(0),
    };
    let mut events = EventQueue::new(1).unwrap();
    let error = fixture
        .runtime
        .work_with_failure_event(
            fixture.faulting_id,
            MissionEventId::WorkReturnedError,
            &clock,
            &mut events,
        )
        .unwrap_err();

    assert_eq!(error.discarded_deliveries(), 2);
    assert_eq!(
        error.operation_error().work_error(),
        &expected_work_error(fixture.faulting_id)
    );
    assert_error_source_chain(&error);
    let attempt = error.operation_error().failure_event_attempt().unwrap();
    assert_eq!(attempt.outcome(), EventEmitOutcome::Recorded);
    assert_eq!(
        attempt.event(),
        &expected_failure_event(fixture.faulting_id)
    );
    assert_eq!(events.dequeue(), Some(*attempt.event()));
    assert_eq!(clock.reads.get(), 1);
    assert_eq!(fixture.faulting.work_calls.get(), 1);
    assert_eq!(fixture.runtime.pending(fixture.faulting_id), Ok(0));
    assert_eq!(fixture.runtime.pending(fixture.peer_id), Ok(2));
    assert_eq!(
        fixture.runtime.state(fixture.faulting_id),
        Ok(ApplicationState::Failed)
    );
    for _ in 0..2 {
        assert_eq!(
            fixture.runtime.dispatch_one(fixture.peer_id),
            Ok(MessageDispatchOutcome::Dispatched)
        );
    }
    assert_eq!(*fixture.peer.messages.borrow(), vec![11, 22]);
    assert_eq!(fixture.runtime.pending(fixture.peer_id), Ok(0));
    assert_eq!(
        fixture.runtime.work_with_failure_event(
            fixture.peer_id,
            MissionEventId::WorkReturnedError,
            &clock,
            &mut events,
        ),
        Ok(ApplicationState::Running)
    );
    assert_eq!(fixture.peer.work_calls.get(), 1);
    assert_eq!(clock.reads.get(), 1);
    assert!(events.is_empty());
}

#[test]
fn saturated_event_preserves_original_error_and_exact_event_for_explicit_retry() {
    let mut fixture = MissionFixture::started();
    fixture.fill_inboxes();
    let clock = CountingClock {
        reads: Cell::new(0),
    };
    let mut events = EventQueue::new(1).unwrap();
    let older = Event::new(
        EventSource::Framework,
        EventSeverity::Warning,
        MissionEventId::OlderDiagnostic,
        EventTimestamp::from_elapsed(Duration::from_secs(2)),
    );
    assert_eq!(events.emit(&older), EventEmitOutcome::Recorded);
    let error = fixture
        .runtime
        .work_with_failure_event(
            fixture.faulting_id,
            MissionEventId::WorkReturnedError,
            &clock,
            &mut events,
        )
        .unwrap_err();

    assert_error_source_chain(&error);
    assert_eq!(error.discarded_deliveries(), 2);
    let (work_error, attempt) = error.into_operation_error().into_parts();
    assert_eq!(work_error, expected_work_error(fixture.faulting_id));
    let attempt = attempt.unwrap();
    assert_eq!(
        attempt.outcome(),
        EventEmitOutcome::QueueFull { capacity: 1 }
    );
    assert_eq!(
        attempt.event(),
        &expected_failure_event(fixture.faulting_id)
    );
    assert_eq!(events.pending(), 1);
    assert_eq!(clock.reads.get(), 1);
    assert_eq!(fixture.runtime.pending(fixture.faulting_id), Ok(0));
    assert_eq!(fixture.runtime.pending(fixture.peer_id), Ok(2));
    assert_eq!(
        fixture.runtime.work(fixture.peer_id),
        Ok(ApplicationState::Running)
    );
    assert_eq!(fixture.peer.work_calls.get(), 1);
    assert_eq!(events.dequeue(), Some(older));
    assert_eq!(events.emit(attempt.event()), EventEmitOutcome::Recorded);
    assert_eq!(
        events.dequeue(),
        Some(expected_failure_event(fixture.faulting_id))
    );
    assert_eq!(clock.reads.get(), 1);
    assert!(events.is_empty());
}

#[test]
fn failed_work_observes_active_configuration_without_automatic_rollback() {
    let mut fixture = MissionFixture::started();
    assert_eq!(fixture.runtime.replace_configuration(&[42]), Ok(2));
    let clock = CountingClock {
        reads: Cell::new(0),
    };
    let mut events = EventQueue::new(1).unwrap();
    let error = fixture
        .runtime
        .work_with_failure_event(
            fixture.faulting_id,
            MissionEventId::WorkReturnedError,
            &clock,
            &mut events,
        )
        .unwrap_err();
    assert_eq!(error.discarded_deliveries(), 0);
    assert_eq!(
        *fixture.faulting.configuration.borrow(),
        Some((2, vec![42]))
    );
    fixture.runtime.work(fixture.peer_id).unwrap();
    assert_eq!(*fixture.peer.configuration.borrow(), Some((2, vec![42])));
    assert_eq!(
        fixture.runtime.replace_configuration(&[101]),
        Err(RuntimeConfigurationError::Table(
            ConfigurationError::Rejected(InvalidConfiguration)
        ))
    );
    assert_eq!(fixture.runtime.rollback_configuration(), Ok(1));
    fixture.runtime.work(fixture.peer_id).unwrap();
    assert_eq!(*fixture.peer.configuration.borrow(), Some((1, vec![7])));
    assert_eq!(
        fixture.runtime.rollback_configuration(),
        Err(RuntimeConfigurationError::Table(
            ConfigurationError::NoRollbackAvailable
        ))
    );
    assert_eq!(fixture.runtime.replace_configuration(&[50]), Ok(3));
    fixture.runtime.work(fixture.peer_id).unwrap();
    assert_eq!(*fixture.peer.configuration.borrow(), Some((3, vec![50])));
    assert_eq!(clock.reads.get(), 1);
    assert_eq!(
        events.dequeue(),
        Some(expected_failure_event(fixture.faulting_id))
    );
    assert!(events.is_empty());
}
