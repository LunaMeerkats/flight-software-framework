//! Public-API evidence for scheduled work through the lifecycle inbox owner.

use std::cell::{Cell, RefCell};
use std::convert::Infallible;
use std::error::Error;
use std::fmt;
use std::rc::Rc;
use std::time::Duration;

use rust_flight_framework::{
    Application, ApplicationId, ApplicationMessageContext, ApplicationState,
    ApplicationWorkContext, Clock, ConfigurationError, ConfigurationTable, FrameworkInstant,
    LifecycleError, LifecycleRegistry, Message, MessageDispatchOutcome, MessagingApplication,
    MessagingOperationError, MessagingRuntime, PublishClassification, Runtime,
    RuntimeConfigurationError, RuntimeInboxConfig, RuntimeWorkError, ScheduledWork,
    ScheduledWorkError, ScheduledWorkOutcome, WorkSchedule,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ApplicationName {
    Alpha,
    Beta,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MissionTopic {
    Command,
}

#[derive(Debug, Eq, PartialEq)]
struct MissionWorkError(u16);

#[derive(Debug, Eq, PartialEq)]
struct InvalidConfiguration;

#[derive(Clone, Debug, Eq, PartialEq)]
struct WorkObservation {
    application: ApplicationName,
    configuration: Option<(u64, Vec<u8>)>,
}

#[derive(Default)]
struct Observations {
    work: RefCell<Vec<WorkObservation>>,
    messages: RefCell<Vec<(ApplicationName, Vec<u8>)>>,
}

struct MissionApplication {
    name: ApplicationName,
    observations: Rc<Observations>,
    failure: Option<MissionWorkError>,
}

struct CountingClock {
    current: Cell<FrameworkInstant>,
    reads: Cell<usize>,
}

type MissionRuntime =
    MessagingRuntime<MissionApplication, MissionTopic, 1, InvalidConfiguration, 1>;
type MessagingScheduleError = MessagingOperationError<ScheduledWorkError<MissionWorkError>>;

struct MissionFixture {
    runtime: MissionRuntime,
    alpha: ApplicationId,
    beta: ApplicationId,
    observations: Rc<Observations>,
}

#[derive(Debug, Eq, PartialEq)]
struct ReplayTrace {
    outcomes: Vec<ScheduledWorkOutcome>,
    work: Vec<WorkObservation>,
    final_states: [ApplicationState; 2],
    clock_reads: usize,
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
        self.observations.work.borrow_mut().push(WorkObservation {
            application: self.name,
            configuration: context
                .configuration()
                .map(|view| (view.revision(), view.bytes().to_vec())),
        });
        match self.failure.take() {
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
            .push((self.name, message.payload().to_vec()));
        Ok(())
    }
}

impl Clock for CountingClock {
    fn now(&self) -> FrameworkInstant {
        self.reads.set(self.reads.get() + 1);
        self.current.get()
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

impl CountingClock {
    fn at(seconds: u64) -> Self {
        Self {
            current: Cell::new(instant(seconds)),
            reads: Cell::new(0),
        }
    }
}

impl MissionFixture {
    fn registered(failure: Option<MissionWorkError>) -> Self {
        let observations = Rc::new(Observations::default());
        let table = ConfigurationTable::new(&[7], validate_configuration).unwrap();
        let mut runtime = Runtime::with_configuration(2, table).unwrap();
        let alpha = runtime
            .register(MissionApplication {
                name: ApplicationName::Alpha,
                observations: Rc::clone(&observations),
                failure,
            })
            .unwrap();
        let beta = runtime
            .register(MissionApplication {
                name: ApplicationName::Beta,
                observations: Rc::clone(&observations),
                failure: None,
            })
            .unwrap();
        let inboxes = [
            RuntimeInboxConfig::new(2, &[MissionTopic::Command]),
            RuntimeInboxConfig::new(2, &[MissionTopic::Command]),
        ];
        Self {
            runtime: MessagingRuntime::new(runtime, &inboxes).unwrap(),
            alpha,
            beta,
            observations,
        }
    }

    fn running(failure: Option<MissionWorkError>) -> Self {
        let mut fixture = Self::registered(failure);
        fixture.runtime.start(fixture.alpha).unwrap();
        fixture.runtime.start(fixture.beta).unwrap();
        fixture
    }

    fn fill_inboxes(&mut self) {
        for value in [11, 22] {
            let message = Message::try_new(MissionTopic::Command, &[value]).unwrap();
            let report = self.runtime.publish(&message).unwrap();
            assert_eq!(report.classification(), PublishClassification::Complete);
        }
        assert_eq!(self.runtime.pending(self.alpha), Ok(2));
        assert_eq!(self.runtime.pending(self.beta), Ok(2));
    }

    fn work_order(&self) -> Vec<ApplicationName> {
        self.observations
            .work
            .borrow()
            .iter()
            .map(|observation| observation.application)
            .collect()
    }
}

fn instant(seconds: u64) -> FrameworkInstant {
    FrameworkInstant::from_elapsed(Duration::from_secs(seconds))
}

fn scheduled(application_id: ApplicationId, seconds: u64) -> ScheduledWork {
    ScheduledWork::new(application_id, instant(seconds))
}

fn validate_configuration(bytes: &[u8]) -> Result<(), InvalidConfiguration> {
    match bytes {
        [value] if *value <= 100 => Ok(()),
        _ => Err(InvalidConfiguration),
    }
}

fn rejected_alpha(state: ApplicationState) -> MissionFixture {
    let mut fixture = MissionFixture::registered(Some(MissionWorkError(17)));
    match state {
        ApplicationState::Registered => {}
        ApplicationState::Stopped => {
            fixture.runtime.start(fixture.alpha).unwrap();
            fixture.runtime.stop(fixture.alpha).unwrap();
        }
        ApplicationState::Failed => {
            fixture.runtime.start(fixture.alpha).unwrap();
            fixture.runtime.work(fixture.alpha).unwrap_err();
        }
        ApplicationState::Running => unreachable!("not a rejection case"),
    }
    fixture.runtime.start(fixture.beta).unwrap();
    fixture
}

fn assert_error_source_chain(error: &MessagingScheduleError) {
    let scheduled_error = Error::source(error)
        .and_then(|source| source.downcast_ref::<ScheduledWorkError<MissionWorkError>>())
        .expect("messaging source retains the full scheduled error");
    assert_eq!(scheduled_error, error.operation_error());
    let work_error = Error::source(scheduled_error)
        .and_then(|source| source.downcast_ref::<RuntimeWorkError<MissionWorkError>>())
        .expect("scheduled source retains the full runtime work error");
    assert_eq!(work_error, scheduled_error.work_error());
    assert_eq!(
        Error::source(work_error).and_then(|source| source.downcast_ref()),
        Some(&MissionWorkError(17))
    );
}

fn replay_scenario() -> ReplayTrace {
    let mut fixture = MissionFixture::running(None);
    let mut schedule = WorkSchedule::new(&[
        scheduled(fixture.beta, 2),
        scheduled(fixture.alpha, 2),
        scheduled(fixture.alpha, 6),
    ])
    .unwrap();
    let clock = CountingClock::at(0);
    let mut outcomes = Vec::new();
    for seconds in [0, 2, 2, 9, 9] {
        clock.current.set(instant(seconds));
        outcomes.push(
            fixture
                .runtime
                .run_next_scheduled_work(&mut schedule, &clock)
                .unwrap(),
        );
    }
    assert!(schedule.is_complete());
    let work = fixture.observations.work.borrow().clone();
    ReplayTrace {
        outcomes,
        work,
        final_states: [
            fixture.runtime.state(fixture.alpha).unwrap(),
            fixture.runtime.state(fixture.beta).unwrap(),
        ],
        clock_reads: clock.reads.get(),
    }
}

#[test]
fn complete_waiting_and_due_work_observe_clock_bounds_and_preserve_inboxes() {
    let mut fixture = MissionFixture::running(None);
    fixture.fill_inboxes();
    let clock = CountingClock::at(0);
    let mut empty = WorkSchedule::new(&[]).unwrap();
    assert_eq!(
        fixture.runtime.run_next_scheduled_work(&mut empty, &clock),
        Ok(ScheduledWorkOutcome::Complete)
    );
    assert_eq!(clock.reads.get(), 0);
    let item = scheduled(fixture.beta, 5);
    let mut schedule = WorkSchedule::new(&[item]).unwrap();
    assert_eq!(
        fixture
            .runtime
            .run_next_scheduled_work(&mut schedule, &clock),
        Ok(ScheduledWorkOutcome::Waiting {
            next: item,
            observed_at: instant(0),
        })
    );
    assert_eq!(clock.reads.get(), 1);
    assert_eq!(schedule.remaining(), 1);
    assert!(fixture.work_order().is_empty());
    assert_eq!(fixture.runtime.pending(fixture.alpha), Ok(2));
    assert_eq!(fixture.runtime.pending(fixture.beta), Ok(2));
    clock.current.set(instant(5));
    assert_eq!(
        fixture
            .runtime
            .run_next_scheduled_work(&mut schedule, &clock),
        Ok(ScheduledWorkOutcome::Completed {
            scheduled_work: item,
            observed_at: instant(5),
            state: ApplicationState::Running,
        })
    );
    assert_eq!(clock.reads.get(), 2);
    assert_eq!(fixture.work_order(), [ApplicationName::Beta]);
    assert_eq!(fixture.runtime.pending(fixture.alpha), Ok(2));
    assert_eq!(fixture.runtime.pending(fixture.beta), Ok(2));
    assert!(schedule.is_complete());
    assert_eq!(
        fixture
            .runtime
            .run_next_scheduled_work(&mut schedule, &clock),
        Ok(ScheduledWorkOutcome::Complete)
    );
    assert_eq!(clock.reads.get(), 2);
    assert_eq!(clock.current.get(), instant(5));
}

#[test]
fn equal_time_and_overdue_items_follow_caller_order_one_per_call() {
    let mut fixture = MissionFixture::running(None);
    let items = [
        scheduled(fixture.beta, 5),
        scheduled(fixture.alpha, 5),
        scheduled(fixture.alpha, 8),
    ];
    let mut schedule = WorkSchedule::new(&items).unwrap();
    let clock = CountingClock::at(5);
    let expected_order = [
        ApplicationName::Beta,
        ApplicationName::Alpha,
        ApplicationName::Alpha,
    ];
    for (index, item) in items.into_iter().enumerate() {
        if index == 2 {
            clock.current.set(instant(10));
        }
        assert_eq!(
            fixture
                .runtime
                .run_next_scheduled_work(&mut schedule, &clock),
            Ok(ScheduledWorkOutcome::Completed {
                scheduled_work: item,
                observed_at: clock.current.get(),
                state: ApplicationState::Running,
            })
        );
        assert_eq!(fixture.work_order(), expected_order[..=index]);
        assert_eq!(schedule.remaining(), 2 - index);
        assert_eq!(clock.reads.get(), index + 1);
    }
    assert!(schedule.is_complete());
}

#[test]
fn lifecycle_rejections_consume_once_and_leave_due_peer_available() {
    for state in [
        ApplicationState::Registered,
        ApplicationState::Stopped,
        ApplicationState::Failed,
    ] {
        let mut fixture = rejected_alpha(state);
        let message = Message::try_new(MissionTopic::Command, &[11]).unwrap();
        fixture.runtime.publish(&message).unwrap();
        let calls_before = fixture.work_order().len();
        let item = scheduled(fixture.alpha, 0);
        let mut schedule = WorkSchedule::new(&[item, scheduled(fixture.beta, 0)]).unwrap();
        let clock = CountingClock::at(0);
        let error = fixture
            .runtime
            .run_next_scheduled_work(&mut schedule, &clock)
            .unwrap_err();
        assert_eq!(error.discarded_deliveries(), 0);
        assert_eq!(error.operation_error().scheduled_work(), item);
        assert_eq!(error.operation_error().observed_at(), instant(0));
        assert_eq!(
            error.into_operation_error().into_work_error(),
            RuntimeWorkError::Lifecycle(LifecycleError::NotRunning {
                application_id: fixture.alpha,
                state,
            })
        );
        assert_eq!(schedule.remaining(), 1);
        assert_eq!(clock.reads.get(), 1);
        assert_eq!(fixture.work_order().len(), calls_before);
        assert_eq!(fixture.runtime.state(fixture.alpha), Ok(state));
        assert_eq!(fixture.runtime.pending(fixture.alpha), Ok(0));
        assert_eq!(fixture.runtime.pending(fixture.beta), Ok(1));
        fixture
            .runtime
            .run_next_scheduled_work(&mut schedule, &clock)
            .unwrap();
        assert_eq!(fixture.work_order().len(), calls_before + 1);
        assert_eq!(fixture.work_order().last(), Some(&ApplicationName::Beta));
        assert_eq!(fixture.runtime.pending(fixture.beta), Ok(1));
        assert_eq!(clock.reads.get(), 2);
        assert!(schedule.is_complete());
    }
}

#[test]
fn unknown_identity_is_consumed_without_touching_owned_inboxes() {
    let mut registry = LifecycleRegistry::new(3).unwrap();
    registry.register().unwrap();
    registry.register().unwrap();
    let unknown = registry.register().unwrap();
    let mut fixture = MissionFixture::running(None);
    fixture.fill_inboxes();
    let item = scheduled(unknown, 0);
    let mut schedule = WorkSchedule::new(&[item, scheduled(fixture.beta, 0)]).unwrap();
    let clock = CountingClock::at(3);
    let error = fixture
        .runtime
        .run_next_scheduled_work(&mut schedule, &clock)
        .unwrap_err();
    assert_eq!(error.discarded_deliveries(), 0);
    assert_eq!(error.operation_error().scheduled_work(), item);
    assert_eq!(error.operation_error().observed_at(), instant(3));
    assert_eq!(
        error.into_operation_error().into_work_error(),
        RuntimeWorkError::Lifecycle(LifecycleError::UnknownApplication {
            application_id: unknown,
        })
    );
    assert_eq!(fixture.runtime.pending(fixture.alpha), Ok(2));
    assert_eq!(fixture.runtime.pending(fixture.beta), Ok(2));
    assert!(fixture.work_order().is_empty());
    assert_eq!(schedule.remaining(), 1);
    assert_eq!(clock.reads.get(), 1);
    fixture
        .runtime
        .run_next_scheduled_work(&mut schedule, &clock)
        .unwrap();
    assert_eq!(fixture.work_order(), [ApplicationName::Beta]);
    assert_eq!(clock.reads.get(), 2);
    assert!(schedule.is_complete());
}

#[test]
fn returned_failure_clears_exact_selected_inbox_and_preserves_peer_fifo() {
    let mut fixture = MissionFixture::running(Some(MissionWorkError(17)));
    fixture.fill_inboxes();
    let item = scheduled(fixture.alpha, 5);
    let mut schedule = WorkSchedule::new(&[item, scheduled(fixture.beta, 5)]).unwrap();
    let clock = CountingClock::at(9);
    let error = fixture
        .runtime
        .run_next_scheduled_work(&mut schedule, &clock)
        .unwrap_err();
    assert_error_source_chain(&error);
    assert_eq!(error.discarded_deliveries(), 2);
    assert_eq!(error.operation_error().scheduled_work(), item);
    assert_eq!(error.operation_error().observed_at(), instant(9));
    assert_eq!(
        error.into_operation_error().into_work_error(),
        RuntimeWorkError::Application {
            application_id: fixture.alpha,
            source: MissionWorkError(17),
        }
    );
    assert_eq!(
        fixture.runtime.state(fixture.alpha),
        Ok(ApplicationState::Failed)
    );
    assert_eq!(
        fixture.runtime.state(fixture.beta),
        Ok(ApplicationState::Running)
    );
    assert_eq!(fixture.runtime.pending(fixture.alpha), Ok(0));
    assert_eq!(fixture.runtime.pending(fixture.beta), Ok(2));
    assert_eq!(schedule.remaining(), 1);
    assert_eq!(clock.reads.get(), 1);
    fixture
        .runtime
        .run_next_scheduled_work(&mut schedule, &clock)
        .unwrap();
    assert_eq!(
        fixture.work_order(),
        [ApplicationName::Alpha, ApplicationName::Beta]
    );
    assert_eq!(fixture.runtime.pending(fixture.beta), Ok(2));
    for _ in 0..2 {
        assert_eq!(
            fixture.runtime.dispatch_one(fixture.beta),
            Ok(MessageDispatchOutcome::Dispatched)
        );
    }
    assert_eq!(
        *fixture.observations.messages.borrow(),
        [
            (ApplicationName::Beta, vec![11]),
            (ApplicationName::Beta, vec![22])
        ]
    );
    assert_eq!(fixture.runtime.pending(fixture.beta), Ok(0));
    assert_eq!(clock.reads.get(), 2);
    assert!(schedule.is_complete());
}

#[test]
fn scheduled_failure_retains_active_configuration_and_consume_once_rollback() {
    let mut fixture = MissionFixture::running(Some(MissionWorkError(17)));
    assert_eq!(fixture.runtime.replace_configuration(&[42]), Ok(2));
    let mut schedule = WorkSchedule::new(&[
        scheduled(fixture.alpha, 0),
        scheduled(fixture.beta, 0),
        scheduled(fixture.beta, 0),
    ])
    .unwrap();
    let clock = CountingClock::at(0);
    let error = fixture
        .runtime
        .run_next_scheduled_work(&mut schedule, &clock)
        .unwrap_err();
    assert_eq!(error.discarded_deliveries(), 0);
    fixture
        .runtime
        .run_next_scheduled_work(&mut schedule, &clock)
        .unwrap();
    assert_eq!(
        fixture.runtime.replace_configuration(&[101]),
        Err(RuntimeConfigurationError::Table(
            ConfigurationError::Rejected(InvalidConfiguration)
        ))
    );
    assert_eq!(fixture.runtime.rollback_configuration(), Ok(1));
    fixture
        .runtime
        .run_next_scheduled_work(&mut schedule, &clock)
        .unwrap();
    assert_eq!(
        fixture.runtime.rollback_configuration(),
        Err(RuntimeConfigurationError::Table(
            ConfigurationError::NoRollbackAvailable
        ))
    );
    let observed = fixture.observations.work.borrow();
    assert_eq!(observed[0].configuration, Some((2, vec![42])));
    assert_eq!(observed[1].configuration, Some((2, vec![42])));
    assert_eq!(observed[2].configuration, Some((1, vec![7])));
    assert_eq!(
        fixture.runtime.state(fixture.alpha),
        Ok(ApplicationState::Failed)
    );
    assert_eq!(clock.reads.get(), 3);
    assert!(schedule.is_complete());
}

#[test]
fn identical_manual_readings_and_caller_order_replay_the_same_trace() {
    let first = replay_scenario();
    assert_eq!(first, replay_scenario());
    assert!(matches!(
        first.outcomes[0],
        ScheduledWorkOutcome::Waiting { .. }
    ));
    assert_eq!(first.outcomes[4], ScheduledWorkOutcome::Complete);
    assert_eq!(first.work.len(), 3);
    assert_eq!(first.clock_reads, 4);
    assert_eq!(first.final_states, [ApplicationState::Running; 2]);
}
