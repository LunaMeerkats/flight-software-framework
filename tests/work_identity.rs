//! Public-only diagnostic consumer of runtime-issued ordinary-work identity.

use std::cell::{Cell, RefCell};
use std::convert::Infallible;
use std::error::Error;
use std::fmt;
use std::time::Duration;

use rust_flight_framework::{
    Application, ApplicationId, ApplicationMessageContext, ApplicationState,
    ApplicationWorkContext, Clock, ConfigurationTable, Event, EventEmitOutcome, EventQueue,
    EventSeverity, EventSource, EventTimestamp, FrameworkInstant, LifecycleError, Message,
    MessageDispatchOutcome, MessagingApplication, MessagingOperationError, MessagingRuntime,
    PublishClassification, Runtime, RuntimeInboxConfig, RuntimeWorkError, RuntimeWorkEventError,
    ScheduledWork, ScheduledWorkOutcome, WorkSchedule,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MissionEvent {
    OlderDiagnostic,
    WorkAccepted,
    WorkReturnedError,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DiagnosticError {
    QueueBusy,
    QueueFull { capacity: usize },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct WorkObservation {
    application_id: ApplicationId,
    configuration: Option<(u64, Option<u8>)>,
}

#[derive(Default)]
struct Observations {
    calls: Cell<usize>,
    attempted: Cell<Option<WorkObservation>>,
    committed: Cell<Option<WorkObservation>>,
    message: Cell<Option<(ApplicationId, u8)>>,
}

struct MissionClock {
    instant: Cell<FrameworkInstant>,
}

struct DiagnosticProducer<'a> {
    observations: &'a Observations,
    events: &'a RefCell<EventQueue<MissionEvent>>,
    clock: &'a MissionClock,
}

type DirectRuntime<'a> = Runtime<DiagnosticProducer<'a>, Infallible, 1>;
type MessageRuntime<'a> = MessagingRuntime<DiagnosticProducer<'a>, (), 1, Infallible, 1>;

impl fmt::Display for DiagnosticError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueueBusy => formatter.write_str("mission diagnostic queue is borrowed"),
            Self::QueueFull { capacity } => {
                write!(formatter, "mission diagnostic queue is full at {capacity}")
            }
        }
    }
}

impl Error for DiagnosticError {}

impl Clock for MissionClock {
    fn now(&self) -> FrameworkInstant {
        self.instant.get()
    }
}

impl Application for DiagnosticProducer<'_> {
    type StartError = Infallible;
    type WorkError = DiagnosticError;
    type StopError = Infallible;
    type RestartError = Infallible;

    fn start(&mut self) -> Result<(), Self::StartError> {
        Ok(())
    }

    fn work(&mut self, context: ApplicationWorkContext<'_>) -> Result<(), Self::WorkError> {
        self.observations
            .calls
            .set(self.observations.calls.get() + 1);
        let observation = WorkObservation {
            application_id: context.application_id(),
            configuration: context
                .configuration()
                .map(|view| (view.revision(), view.bytes().first().copied())),
        };
        self.observations.attempted.set(Some(observation));
        let event = Event::new(
            EventSource::Application(context.application_id()),
            EventSeverity::Informational,
            MissionEvent::WorkAccepted,
            EventTimestamp::from_clock(self.clock),
        );
        let mut events = self
            .events
            .try_borrow_mut()
            .map_err(|_| DiagnosticError::QueueBusy)?;
        match events.emit(&event) {
            EventEmitOutcome::Recorded => {
                // Commit the bounded effect only after actual queue acceptance.
                self.observations.committed.set(Some(observation));
                Ok(())
            }
            EventEmitOutcome::QueueFull { capacity } => {
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

impl MessagingApplication<(), 1> for DiagnosticProducer<'_> {
    type MessageError = Infallible;

    fn handle_message(
        &mut self,
        message: &Message<(), 1>,
        context: &mut ApplicationMessageContext<'_, (), 1>,
    ) -> Result<(), Self::MessageError> {
        self.observations.message.set(
            message
                .payload()
                .first()
                .copied()
                .map(|value| (context.application_id(), value)),
        );
        Ok(())
    }
}

fn mission_clock() -> MissionClock {
    MissionClock {
        instant: Cell::new(FrameworkInstant::ZERO),
    }
}

fn mission_events() -> RefCell<EventQueue<MissionEvent>> {
    RefCell::new(EventQueue::new(1).unwrap())
}

fn registered_runtime<'a>(
    observations: &'a [Observations; 2],
    events: &'a RefCell<EventQueue<MissionEvent>>,
    clock: &'a MissionClock,
) -> (DirectRuntime<'a>, [ApplicationId; 2]) {
    let table = ConfigurationTable::new(&[7], |_| Ok::<_, Infallible>(())).unwrap();
    let mut runtime = DirectRuntime::with_configuration(2, table).unwrap();
    let identities = observations.each_ref().map(|observations| {
        runtime
            .register(DiagnosticProducer {
                observations,
                events,
                clock,
            })
            .unwrap()
    });
    assert_ne!(identities[0], identities[1]);
    (runtime, identities)
}

fn message_runtime(runtime: DirectRuntime<'_>) -> MessageRuntime<'_> {
    let inboxes = [RuntimeInboxConfig::new(1, &[()]); 2];
    MessagingRuntime::new(runtime, &inboxes).unwrap()
}

fn expected_observation(application_id: ApplicationId, revision: u64, byte: u8) -> WorkObservation {
    WorkObservation {
        application_id,
        configuration: Some((revision, Some(byte))),
    }
}

fn expected_event(
    source: EventSource,
    identifier: MissionEvent,
    severity: EventSeverity,
    instant: FrameworkInstant,
) -> Event<MissionEvent> {
    Event::new(
        source,
        severity,
        identifier,
        EventTimestamp::from_instant(instant),
    )
}

fn assert_committed(
    observations: &Observations,
    events: &RefCell<EventQueue<MissionEvent>>,
    expected: WorkObservation,
    instant: FrameworkInstant,
) {
    assert_eq!(observations.attempted.get(), Some(expected));
    assert_eq!(observations.committed.get(), Some(expected));
    assert_eq!(
        events.borrow_mut().dequeue(),
        Some(expected_event(
            EventSource::Application(expected.application_id),
            MissionEvent::WorkAccepted,
            EventSeverity::Informational,
            instant,
        ))
    );
    assert!(events.borrow().is_empty());
}

fn fill_mission_events(events: &RefCell<EventQueue<MissionEvent>>) -> Event<MissionEvent> {
    let retained = expected_event(
        EventSource::Framework,
        MissionEvent::OlderDiagnostic,
        EventSeverity::Warning,
        FrameworkInstant::ZERO,
    );
    assert_eq!(
        events.borrow_mut().emit(&retained),
        EventEmitOutcome::Recorded
    );
    retained
}

fn assert_work_error(
    error: &RuntimeWorkError<DiagnosticError>,
    application_id: ApplicationId,
    expected: DiagnosticError,
) {
    assert_eq!(
        error,
        &RuntimeWorkError::Application {
            application_id,
            source: expected
        }
    );
    assert_eq!(
        Error::source(error).and_then(|source| source.downcast_ref()),
        Some(&expected)
    );
}

fn assert_failure_event(
    error: &RuntimeWorkEventError<DiagnosticError, MissionEvent>,
    events: &mut EventQueue<MissionEvent>,
    application_id: ApplicationId,
    instant: FrameworkInstant,
) {
    assert_work_error(
        error.work_error(),
        application_id,
        DiagnosticError::QueueFull { capacity: 1 },
    );
    assert_eq!(
        Error::source(error).and_then(|source| source.downcast_ref()),
        Some(error.work_error())
    );
    let expected = expected_event(
        EventSource::Application(application_id),
        MissionEvent::WorkReturnedError,
        EventSeverity::Error,
        instant,
    );
    let attempt = error.failure_event_attempt().unwrap();
    assert_eq!(attempt.event(), &expected);
    assert_eq!(attempt.outcome(), EventEmitOutcome::Recorded);
    assert_eq!(events.dequeue(), Some(expected));
    assert!(events.is_empty());
}

fn fill_inboxes(runtime: &mut MessageRuntime<'_>, identities: [ApplicationId; 2]) {
    let report = runtime
        .publish(&Message::try_new((), &[23]).unwrap())
        .unwrap();
    assert_eq!(report.classification(), PublishClassification::Complete);
    for identity in identities {
        assert_eq!(runtime.pending(identity), Ok(1));
    }
}

fn assert_messaging_failure_event(
    error: &MessagingOperationError<RuntimeWorkEventError<DiagnosticError, MissionEvent>>,
    events: &mut EventQueue<MissionEvent>,
    application_id: ApplicationId,
    instant: FrameworkInstant,
) {
    assert_eq!(error.discarded_deliveries(), 1);
    assert_eq!(
        Error::source(error).and_then(|source| source.downcast_ref()),
        Some(error.operation_error())
    );
    assert_failure_event(error.operation_error(), events, application_id, instant);
}

#[test]
fn direct_work_retains_identity_configuration_and_lifecycle_suppression() {
    let observations = [Observations::default(), Observations::default()];
    let events = mission_events();
    let clock = mission_clock();
    let (mut runtime, identities) = registered_runtime(&observations, &events, &clock);
    for (index, identity) in identities.into_iter().enumerate() {
        assert_eq!(
            runtime.work(identity),
            Err(RuntimeWorkError::Lifecycle(LifecycleError::NotRunning {
                application_id: identity,
                state: ApplicationState::Registered,
            }))
        );
        assert_eq!(observations[index].calls.get(), 0);
        runtime.start(identity).unwrap();
        runtime.work(identity).unwrap();
        assert_committed(
            &observations[index],
            &events,
            expected_observation(identity, 1, 7),
            clock.now(),
        );
    }
    assert_eq!(runtime.replace_configuration(&[42]), Ok(2));
    for (index, identity) in identities.into_iter().enumerate() {
        runtime.stop(identity).unwrap();
        assert_eq!(
            runtime.work(identity),
            Err(RuntimeWorkError::Lifecycle(LifecycleError::NotRunning {
                application_id: identity,
                state: ApplicationState::Stopped,
            }))
        );
        assert_eq!(observations[index].calls.get(), 1);
        runtime.restart(identity).unwrap();
        runtime.work(identity).unwrap();
        assert_committed(
            &observations[index],
            &events,
            expected_observation(identity, 2, 42),
            clock.now(),
        );
        assert_eq!(observations[index].calls.get(), 2);
    }
}

#[test]
fn unconfigured_work_exposes_both_selected_identities_without_a_configuration() {
    let observations = [Observations::default(), Observations::default()];
    let events = mission_events();
    let clock = mission_clock();
    let mut runtime = Runtime::new(2).unwrap();
    let mut identities = Vec::with_capacity(2);
    for observations in &observations {
        let identity = runtime
            .register(DiagnosticProducer {
                observations,
                events: &events,
                clock: &clock,
            })
            .unwrap();
        identities.push(identity);
        runtime.start(identity).unwrap();
        runtime.work(identity).unwrap();
        assert_committed(
            observations,
            &events,
            WorkObservation {
                application_id: identity,
                configuration: None,
            },
            clock.now(),
        );
    }
    assert_ne!(identities[0], identities[1]);
}

#[test]
fn messaging_work_keeps_both_inboxes_and_restart_retains_the_selected_identity() {
    let observations = [Observations::default(), Observations::default()];
    let events = mission_events();
    let clock = mission_clock();
    let (runtime, identities) = registered_runtime(&observations, &events, &clock);
    let mut runtime = message_runtime(runtime);
    for identity in identities {
        runtime.start(identity).unwrap();
    }
    fill_inboxes(&mut runtime, identities);
    for (index, identity) in identities.into_iter().enumerate() {
        runtime.work(identity).unwrap();
        assert_committed(
            &observations[index],
            &events,
            expected_observation(identity, 1, 7),
            clock.now(),
        );
        assert_eq!(runtime.pending(identity), Ok(1));
    }
    assert_eq!(runtime.replace_configuration(&[42]), Ok(2));
    for (index, identity) in identities.into_iter().enumerate() {
        assert_eq!(runtime.stop(identity).unwrap().discarded_deliveries(), 1);
        assert!(runtime.work(identity).is_err());
        assert_eq!(observations[index].calls.get(), 1);
        runtime.restart(identity).unwrap();
        runtime.work(identity).unwrap();
        assert_committed(
            &observations[index],
            &events,
            expected_observation(identity, 2, 42),
            clock.now(),
        );
    }
}

#[test]
fn checked_busy_queue_preserves_records_and_returns_the_selected_typed_error() {
    for selected in [0, 1] {
        let observations = [Observations::default(), Observations::default()];
        let events = mission_events();
        let clock = mission_clock();
        let (mut runtime, identities) = registered_runtime(&observations, &events, &clock);
        for identity in identities {
            runtime.start(identity).unwrap();
        }
        let retained = fill_mission_events(&events);
        let host_borrow = events.borrow();
        let error = runtime.work(identities[selected]).unwrap_err();
        assert_work_error(&error, identities[selected], DiagnosticError::QueueBusy);
        assert_eq!(host_borrow.pending(), 1);
        assert_eq!(observations[selected].committed.get(), None);
        assert_eq!(
            observations[selected].attempted.get(),
            Some(expected_observation(identities[selected], 1, 7))
        );
        assert_eq!(
            runtime.state(identities[selected]),
            Ok(ApplicationState::Failed)
        );
        drop(host_borrow);
        assert_eq!(events.borrow_mut().dequeue(), Some(retained));
        let peer = 1 - selected;
        runtime.work(identities[peer]).unwrap();
        assert_committed(
            &observations[peer],
            &events,
            expected_observation(identities[peer], 1, 7),
            clock.now(),
        );
    }
}

#[test]
fn direct_failure_event_wrapper_preserves_full_queue_error_and_peer_progress() {
    for selected in [0, 1] {
        let observations = [Observations::default(), Observations::default()];
        let events = mission_events();
        let clock = mission_clock();
        let (mut runtime, identities) = registered_runtime(&observations, &events, &clock);
        for identity in identities {
            runtime.start(identity).unwrap();
        }
        let retained = fill_mission_events(&events);
        let mut failures = EventQueue::new(1).unwrap();
        let error = runtime
            .work_with_failure_event(
                identities[selected],
                MissionEvent::WorkReturnedError,
                &clock,
                &mut failures,
            )
            .unwrap_err();
        assert_failure_event(&error, &mut failures, identities[selected], clock.now());
        assert_eq!(observations[selected].committed.get(), None);
        assert_eq!(
            observations[selected].attempted.get(),
            Some(expected_observation(identities[selected], 1, 7))
        );
        assert_eq!(
            runtime.state(identities[selected]),
            Ok(ApplicationState::Failed)
        );
        assert_eq!(events.borrow_mut().dequeue(), Some(retained));
        assert!(runtime.work(identities[selected]).is_err());
        assert_eq!(observations[selected].calls.get(), 1);
        let peer = 1 - selected;
        runtime
            .work_with_failure_event(
                identities[peer],
                MissionEvent::WorkReturnedError,
                &clock,
                &mut failures,
            )
            .unwrap();
        assert_committed(
            &observations[peer],
            &events,
            expected_observation(identities[peer], 1, 7),
            clock.now(),
        );
        assert!(failures.is_empty());
    }
}

#[test]
fn messaging_failure_event_wrapper_clears_only_the_selected_inbox() {
    for selected in [0, 1] {
        let observations = [Observations::default(), Observations::default()];
        let events = mission_events();
        let clock = mission_clock();
        let (runtime, identities) = registered_runtime(&observations, &events, &clock);
        let mut runtime = message_runtime(runtime);
        for identity in identities {
            runtime.start(identity).unwrap();
        }
        fill_inboxes(&mut runtime, identities);
        let retained = fill_mission_events(&events);
        let mut failures = EventQueue::new(1).unwrap();
        let error = runtime
            .work_with_failure_event(
                identities[selected],
                MissionEvent::WorkReturnedError,
                &clock,
                &mut failures,
            )
            .unwrap_err();
        assert_messaging_failure_event(&error, &mut failures, identities[selected], clock.now());
        assert_eq!(
            observations[selected].attempted.get(),
            Some(expected_observation(identities[selected], 1, 7))
        );
        assert_eq!(observations[selected].committed.get(), None);
        assert_eq!(
            runtime.state(identities[selected]),
            Ok(ApplicationState::Failed)
        );
        assert_eq!(runtime.pending(identities[selected]), Ok(0));
        assert_eq!(events.borrow_mut().dequeue(), Some(retained));
        let peer = 1 - selected;
        assert_eq!(runtime.pending(identities[peer]), Ok(1));
        runtime
            .work_with_failure_event(
                identities[peer],
                MissionEvent::WorkReturnedError,
                &clock,
                &mut failures,
            )
            .unwrap();
        assert_committed(
            &observations[peer],
            &events,
            expected_observation(identities[peer], 1, 7),
            clock.now(),
        );
        assert_eq!(
            runtime.dispatch_one(identities[peer]),
            Ok(MessageDispatchOutcome::Dispatched)
        );
        assert_eq!(
            observations[peer].message.get(),
            Some((identities[peer], 23))
        );
        assert!(failures.is_empty());
    }
}

#[test]
fn direct_schedule_issues_both_identities_only_when_each_item_is_due() {
    let observations = [Observations::default(), Observations::default()];
    let events = mission_events();
    let clock = mission_clock();
    let (mut runtime, identities) = registered_runtime(&observations, &events, &clock);
    for identity in identities {
        runtime.start(identity).unwrap();
    }
    let due = FrameworkInstant::from_elapsed(Duration::from_secs(1));
    let items = identities.map(|identity| ScheduledWork::new(identity, due));
    let mut schedule = WorkSchedule::new(&items).unwrap();
    assert_eq!(
        runtime.run_next_scheduled_work(&mut schedule, &clock),
        Ok(ScheduledWorkOutcome::Waiting {
            next: items[0],
            observed_at: FrameworkInstant::ZERO
        })
    );
    assert_eq!(schedule.remaining(), 2);
    assert!(
        observations
            .iter()
            .all(|observation| observation.calls.get() == 0)
    );
    clock.instant.set(due);
    for (index, identity) in identities.into_iter().enumerate() {
        assert_eq!(
            runtime.run_next_scheduled_work(&mut schedule, &clock),
            Ok(ScheduledWorkOutcome::Completed {
                scheduled_work: items[index],
                observed_at: due,
                state: ApplicationState::Running
            })
        );
        assert_committed(
            &observations[index],
            &events,
            expected_observation(identity, 1, 7),
            due,
        );
    }
    assert_eq!(
        runtime.run_next_scheduled_work(&mut schedule, &clock),
        Ok(ScheduledWorkOutcome::Complete)
    );
    assert!(schedule.is_complete());
}

#[test]
fn messaging_schedule_retains_fault_identity_and_allows_the_queued_peer_to_progress() {
    for selected in [0, 1] {
        let observations = [Observations::default(), Observations::default()];
        let events = mission_events();
        let clock = mission_clock();
        let (runtime, identities) = registered_runtime(&observations, &events, &clock);
        let mut runtime = message_runtime(runtime);
        for identity in identities {
            runtime.start(identity).unwrap();
        }
        fill_inboxes(&mut runtime, identities);
        let retained = fill_mission_events(&events);
        let peer = 1 - selected;
        let items =
            [selected, peer].map(|index| ScheduledWork::new(identities[index], clock.now()));
        let mut schedule = WorkSchedule::new(&items).unwrap();
        let error = runtime
            .run_next_scheduled_work(&mut schedule, &clock)
            .unwrap_err();
        assert_eq!(error.discarded_deliveries(), 1);
        assert_eq!(error.operation_error().scheduled_work(), items[0]);
        assert_work_error(
            error.operation_error().work_error(),
            identities[selected],
            DiagnosticError::QueueFull { capacity: 1 },
        );
        assert_eq!(
            observations[selected].attempted.get(),
            Some(expected_observation(identities[selected], 1, 7))
        );
        assert_eq!(observations[selected].committed.get(), None);
        assert_eq!(runtime.pending(identities[selected]), Ok(0));
        assert_eq!(runtime.pending(identities[peer]), Ok(1));
        assert_eq!(events.borrow_mut().dequeue(), Some(retained));
        assert_eq!(
            runtime.run_next_scheduled_work(&mut schedule, &clock),
            Ok(ScheduledWorkOutcome::Completed {
                scheduled_work: items[1],
                observed_at: clock.now(),
                state: ApplicationState::Running
            })
        );
        assert_committed(
            &observations[peer],
            &events,
            expected_observation(identities[peer], 1, 7),
            clock.now(),
        );
        assert_eq!(runtime.pending(identities[peer]), Ok(1));
        assert_eq!(
            runtime.dispatch_one(identities[peer]),
            Ok(MessageDispatchOutcome::Dispatched)
        );
        assert_eq!(
            observations[peer].message.get(),
            Some((identities[peer], 23))
        );
        assert!(schedule.is_complete());
    }
}
