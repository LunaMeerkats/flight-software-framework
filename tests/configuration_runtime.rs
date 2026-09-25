//! Public-API evidence for runtime-owned configuration at work safe points.

use std::cell::{Cell, RefCell};
use std::convert::Infallible;
use std::error::Error;
use std::fmt;
use std::rc::Rc;
use std::time::Duration;

use rust_flight_framework::{
    Application, ApplicationId, ApplicationState, ApplicationWorkContext, Clock,
    ConfigurationError, ConfigurationTable, DeliveryStatus, EventEmitOutcome, EventQueue,
    EventSeverity, EventSource, EventTimestamp, FrameworkInstant, LifecycleError, Message,
    MessageBusCreateError, MessagingRuntime, MessagingRuntimeCreateErrorKind,
    PublishClassification, Runtime, RuntimeConfigurationError, RuntimeCreateError,
    RuntimeInboxConfig, RuntimeWorkError, ScheduledWork, ScheduledWorkOutcome, WorkSchedule,
};

#[derive(Clone, Debug, Eq, PartialEq)]
struct CopiedConfiguration {
    revision: u64,
    bytes: Vec<u8>,
}

type ObservationLog = Rc<RefCell<Vec<Option<CopiedConfiguration>>>>;

#[derive(Debug)]
struct ObservingApplication {
    observations: ObservationLog,
    work_calls: Rc<Cell<usize>>,
    fail_on_call: Option<usize>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct MissionWorkError {
    call: usize,
}

impl fmt::Display for MissionWorkError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "mission work failed on call {}", self.call)
    }
}

impl Error for MissionWorkError {}

impl Application for ObservingApplication {
    type StartError = Infallible;
    type WorkError = MissionWorkError;
    type StopError = Infallible;
    type RestartError = Infallible;

    fn start(&mut self) -> Result<(), Self::StartError> {
        Ok(())
    }

    fn work(&mut self, context: ApplicationWorkContext<'_>) -> Result<(), Self::WorkError> {
        let call = self.work_calls.get() + 1;
        self.work_calls.set(call);
        let copied = context.configuration().map(|view| CopiedConfiguration {
            revision: view.revision(),
            bytes: view.bytes().to_vec(),
        });
        self.observations.borrow_mut().push(copied);
        if self.fail_on_call == Some(call) {
            return Err(MissionWorkError { call });
        }
        Ok(())
    }

    fn stop(&mut self) -> Result<(), Self::StopError> {
        Ok(())
    }

    fn restart(&mut self) -> Result<(), Self::RestartError> {
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MissionValidationError {
    InvalidLength(usize),
    UnknownMode(u8),
}

impl fmt::Display for MissionValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLength(length) => write!(formatter, "expected two bytes, got {length}"),
            Self::UnknownMode(mode) => write!(formatter, "unknown mission mode {mode}"),
        }
    }
}

impl Error for MissionValidationError {}

fn validate_mission(bytes: &[u8]) -> Result<(), MissionValidationError> {
    let [mode, _period] = bytes else {
        return Err(MissionValidationError::InvalidLength(bytes.len()));
    };
    if *mode > 2 {
        return Err(MissionValidationError::UnknownMode(*mode));
    }
    Ok(())
}

fn mission_table(initial: &[u8]) -> ConfigurationTable<MissionValidationError, 2> {
    ConfigurationTable::new(initial, validate_mission).expect("test configuration is valid")
}

fn application(
    fail_on_call: Option<usize>,
) -> (ObservingApplication, ObservationLog, Rc<Cell<usize>>) {
    let observations = Rc::new(RefCell::new(Vec::new()));
    let work_calls = Rc::new(Cell::new(0));
    (
        ObservingApplication {
            observations: Rc::clone(&observations),
            work_calls: Rc::clone(&work_calls),
            fail_on_call,
        },
        observations,
        work_calls,
    )
}

fn assert_observation(observations: &ObservationLog, index: usize, revision: u64, bytes: &[u8]) {
    assert_eq!(
        observations.borrow()[index],
        Some(CopiedConfiguration {
            revision,
            bytes: bytes.to_vec(),
        })
    );
}

type ConfiguredRuntime = Runtime<ObservingApplication, MissionValidationError, 2>;

#[test]
fn unconfigured_work_reports_absence_and_rejects_table_operations() {
    let (application, observations, work_calls) = application(None);
    let mut runtime = Runtime::new(1).expect("one runtime record can be reserved");
    let application_id = runtime.register(application).expect("application fits");
    runtime.start(application_id).expect("application starts");

    assert_eq!(runtime.work(application_id), Ok(ApplicationState::Running));
    assert_eq!(&*observations.borrow(), &[None]);
    assert_eq!(work_calls.get(), 1);
    assert_eq!(
        runtime.replace_configuration(&[]),
        Err(RuntimeConfigurationError::NotConfigured)
    );
    assert_eq!(
        runtime.rollback_configuration(),
        Err(RuntimeConfigurationError::NotConfigured)
    );
}

#[test]
fn zero_byte_configured_runtime_is_distinct_from_absence() {
    let table = ConfigurationTable::<Infallible, 0>::new(&[], |_| Ok(()))
        .expect("empty zero-byte configuration is valid");
    let (application, observations, _) = application(None);
    let mut runtime = Runtime::<ObservingApplication>::with_configuration(1, table)
        .expect("one configured record can be reserved");
    let application_id = runtime.register(application).expect("application fits");
    runtime.start(application_id).expect("application starts");

    assert_eq!(runtime.work(application_id), Ok(ApplicationState::Running));
    assert_observation(&observations, 0, 1, &[]);
    assert_eq!(runtime.replace_configuration(&[]), Ok(2));
    assert_eq!(runtime.rollback_configuration(), Ok(1));
}

#[test]
fn work_context_exposes_only_the_used_configuration_prefix() {
    let table = ConfigurationTable::<Infallible, 4>::new(&[4, 5], |_| Ok(()))
        .expect("two bytes fit a four-byte table");
    let (application, observations, _) = application(None);
    let mut runtime = Runtime::<ObservingApplication, Infallible, 4>::with_configuration(1, table)
        .expect("one configured record can be reserved");
    let application_id = runtime.register(application).expect("application fits");
    runtime.start(application_id).expect("application starts");

    assert_eq!(runtime.work(application_id), Ok(ApplicationState::Running));
    assert_observation(&observations, 0, 1, &[4, 5]);
}

fn assert_failed_construction_preserves_table(capacity: usize, expected: RuntimeCreateError) {
    let mut table = mission_table(&[0, 1]);
    assert_eq!(table.replace(&[1, 2]), Ok(2));
    let error = ConfiguredRuntime::with_configuration(capacity, table)
        .expect_err("configured runtime storage must be rejected");

    assert_eq!(error.kind(), expected);
    assert_eq!(error.configuration().active().revision(), 2);
    assert_eq!(error.configuration().active().bytes(), &[1, 2]);
    let mut recovered = error.into_configuration();
    assert_eq!(recovered.rollback(), Ok(1));
    assert_eq!(recovered.active().bytes(), &[0, 1]);
    assert_eq!(recovered.replace(&[2, 3]), Ok(3));
}

#[test]
fn configured_construction_failures_return_the_unchanged_table_lineage() {
    assert_failed_construction_preserves_table(0, RuntimeCreateError::ZeroCapacity);
    assert_failed_construction_preserves_table(
        usize::MAX,
        RuntimeCreateError::CapacityAllocationFailed {
            requested: usize::MAX,
        },
    );
}

fn started_runtime(
    table: ConfigurationTable<MissionValidationError, 2>,
    fail_on_call: Option<usize>,
) -> (
    ConfiguredRuntime,
    ApplicationId,
    ObservationLog,
    Rc<Cell<usize>>,
) {
    let (application, observations, work_calls) = application(fail_on_call);
    let mut runtime = ConfiguredRuntime::with_configuration(2, table)
        .expect("configured runtime storage can be reserved");
    let application_id = runtime.register(application).expect("application fits");
    runtime.start(application_id).expect("application starts");
    (runtime, application_id, observations, work_calls)
}

#[test]
fn work_observes_activation_rejection_rollback_and_revision_non_reuse() {
    let (mut runtime, application_id, observations, _) =
        started_runtime(mission_table(&[0, 1]), None);

    runtime.work(application_id).expect("initial work succeeds");
    assert_eq!(runtime.replace_configuration(&[1, 2]), Ok(2));
    runtime
        .work(application_id)
        .expect("replacement is visible");
    assert_eq!(
        runtime.replace_configuration(&[9, 2]),
        Err(RuntimeConfigurationError::Table(
            ConfigurationError::Rejected(MissionValidationError::UnknownMode(9))
        ))
    );
    runtime
        .work(application_id)
        .expect("rejection retains active");
    assert_eq!(runtime.rollback_configuration(), Ok(1));
    runtime.work(application_id).expect("rollback is visible");
    assert_eq!(
        runtime.rollback_configuration(),
        Err(RuntimeConfigurationError::Table(
            ConfigurationError::NoRollbackAvailable
        ))
    );
    assert_eq!(runtime.replace_configuration(&[2, 3]), Ok(3));
    runtime
        .work(application_id)
        .expect("revision three is visible");

    assert_observation(&observations, 0, 1, &[0, 1]);
    assert_observation(&observations, 1, 2, &[1, 2]);
    assert_observation(&observations, 2, 2, &[1, 2]);
    assert_observation(&observations, 3, 1, &[0, 1]);
    assert_observation(&observations, 4, 3, &[2, 3]);
}

#[test]
fn lifecycle_rejection_suppresses_work_and_restart_retains_configuration_history() {
    let mut table = mission_table(&[0, 1]);
    table.replace(&[1, 2]).expect("replacement is valid");
    let (mut runtime, application_id, observations, work_calls) = started_runtime(table, None);
    runtime.work(application_id).expect("running work succeeds");
    runtime.stop(application_id).expect("application stops");

    assert_eq!(
        runtime.work(application_id),
        Err(RuntimeWorkError::Lifecycle(LifecycleError::NotRunning {
            application_id,
            state: ApplicationState::Stopped,
        }))
    );
    assert_eq!(work_calls.get(), 1);
    runtime
        .restart(application_id)
        .expect("application restarts");
    runtime
        .work(application_id)
        .expect("restarted work succeeds");
    assert_eq!(runtime.rollback_configuration(), Ok(1));
    runtime
        .work(application_id)
        .expect("retained rollback is visible");

    assert_observation(&observations, 0, 2, &[1, 2]);
    assert_observation(&observations, 1, 2, &[1, 2]);
    assert_observation(&observations, 2, 1, &[0, 1]);
}

#[test]
fn returned_work_error_retains_configuration_history_and_peer_progress() {
    let mut table = mission_table(&[0, 1]);
    table.replace(&[1, 2]).expect("replacement is valid");
    let (faulting, faulting_observations, _) = application(Some(1));
    let (peer, peer_observations, peer_calls) = application(None);
    let mut runtime = ConfiguredRuntime::with_configuration(2, table)
        .expect("configured runtime storage can be reserved");
    let faulting_id = runtime.register(faulting).expect("faulting app fits");
    let peer_id = runtime.register(peer).expect("peer fits");
    runtime.start(faulting_id).expect("faulting app starts");
    runtime.start(peer_id).expect("peer starts");

    let error = runtime.work(faulting_id).expect_err("first work fails");
    assert_eq!(
        error,
        RuntimeWorkError::Application {
            application_id: faulting_id,
            source: MissionWorkError { call: 1 },
        }
    );
    assert_eq!(runtime.state(faulting_id), Ok(ApplicationState::Failed));
    assert_eq!(runtime.work(peer_id), Ok(ApplicationState::Running));
    assert_eq!(peer_calls.get(), 1);
    assert_eq!(runtime.rollback_configuration(), Ok(1));
    assert_eq!(runtime.work(peer_id), Ok(ApplicationState::Running));

    assert_observation(&faulting_observations, 0, 2, &[1, 2]);
    assert_observation(&peer_observations, 0, 2, &[1, 2]);
    assert_observation(&peer_observations, 1, 1, &[0, 1]);
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
}

impl Clock for CountingClock {
    fn now(&self) -> FrameworkInstant {
        self.reads.set(self.reads.get() + 1);
        self.instant
    }
}

#[test]
fn scheduled_work_observes_the_active_configuration_through_runtime_work() {
    let (mut runtime, application_id, observations, _) =
        started_runtime(mission_table(&[0, 1]), None);
    assert_eq!(runtime.replace_configuration(&[1, 2]), Ok(2));
    let scheduled_at = FrameworkInstant::from_elapsed(Duration::from_secs(5));
    let item = ScheduledWork::new(application_id, scheduled_at);
    let mut schedule = WorkSchedule::new(&[item]).expect("one ordered item is valid");
    let clock = CountingClock::at(Duration::from_secs(5));

    assert_eq!(
        runtime.run_next_scheduled_work(&mut schedule, &clock),
        Ok(ScheduledWorkOutcome::Completed {
            scheduled_work: item,
            observed_at: scheduled_at,
            state: ApplicationState::Running,
        })
    );
    assert_eq!(clock.reads.get(), 1);
    assert!(schedule.is_complete());
    assert_observation(&observations, 0, 2, &[1, 2]);
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MissionEventId {
    WorkReturnedError,
}

#[test]
fn failure_event_work_observes_configuration_and_retains_history() {
    let mut table = mission_table(&[0, 1]);
    table.replace(&[1, 2]).expect("replacement is valid");
    let (mut runtime, application_id, observations, _) = started_runtime(table, Some(1));
    let clock = CountingClock::at(Duration::from_secs(7));
    let mut events = EventQueue::new(1).expect("one event slot can be reserved");

    let error = runtime
        .work_with_failure_event(
            application_id,
            MissionEventId::WorkReturnedError,
            &clock,
            &mut events,
        )
        .expect_err("configured work returns its requested failure");
    let attempt = error
        .failure_event_attempt()
        .expect("application error attempts one event");
    assert_eq!(attempt.outcome(), EventEmitOutcome::Recorded);
    assert_eq!(
        attempt.event().source(),
        EventSource::Application(application_id)
    );
    assert_eq!(attempt.event().severity(), EventSeverity::Error);
    assert_eq!(
        attempt.event().identifier(),
        &MissionEventId::WorkReturnedError
    );
    assert_eq!(
        attempt.event().timestamp(),
        EventTimestamp::from_elapsed(Duration::from_secs(7))
    );
    assert_eq!(clock.reads.get(), 1);
    assert_eq!(events.pending(), 1);
    assert_eq!(runtime.state(application_id), Ok(ApplicationState::Failed));
    assert_eq!(runtime.rollback_configuration(), Ok(1));
    assert_observation(&observations, 0, 2, &[1, 2]);
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MissionTopic {
    Command,
}

const COMMAND_ONLY: [MissionTopic; 1] = [MissionTopic::Command];

type ConfiguredMessagingRuntime =
    MessagingRuntime<ObservingApplication, MissionTopic, 1, MissionValidationError, 2>;

#[test]
fn failed_messaging_attachment_preserves_configuration_lineage() {
    let mut table = mission_table(&[0, 1]);
    assert_eq!(table.replace(&[1, 2]), Ok(2));
    let (application, observations, _) = application(None);
    let mut runtime = ConfiguredRuntime::with_configuration(1, table)
        .expect("configured runtime storage can be reserved");
    let application_id = runtime.register(application).expect("application fits");
    let invalid = [RuntimeInboxConfig::new(usize::MAX, &COMMAND_ONLY)];

    let error = ConfiguredMessagingRuntime::new(runtime, &invalid)
        .expect_err("the inbox capacity is unrepresentable");
    assert_eq!(
        error.kind(),
        MessagingRuntimeCreateErrorKind::MessageBus(
            MessageBusCreateError::InboxStorageAllocationFailed {
                application_id,
                requested: usize::MAX,
            }
        )
    );
    assert_eq!(
        error.runtime().state(application_id),
        Ok(ApplicationState::Registered)
    );

    let corrected = [RuntimeInboxConfig::new(1, &COMMAND_ONLY)];
    let mut recovered = ConfiguredMessagingRuntime::new(error.into_runtime(), &corrected)
        .expect("the returned runtime accepts a corrected inbox");
    recovered.start(application_id).expect("application starts");
    recovered
        .work(application_id)
        .expect("revision two is visible");
    assert_eq!(recovered.rollback_configuration(), Ok(1));
    recovered.work(application_id).expect("rollback is visible");
    assert_eq!(recovered.replace_configuration(&[2, 3]), Ok(3));
    recovered
        .work(application_id)
        .expect("revision three is visible");

    assert_observation(&observations, 0, 2, &[1, 2]);
    assert_observation(&observations, 1, 1, &[0, 1]);
    assert_observation(&observations, 2, 3, &[2, 3]);
}

struct StartedMessagingRuntime {
    runtime: ConfiguredMessagingRuntime,
    faulting_id: ApplicationId,
    peer_id: ApplicationId,
    faulting_observations: ObservationLog,
    peer_observations: ObservationLog,
}

fn started_messaging_runtime() -> StartedMessagingRuntime {
    let mut table = mission_table(&[0, 1]);
    table.replace(&[1, 2]).expect("replacement is valid");
    let (faulting, faulting_observations, _) = application(Some(1));
    let (peer, peer_observations, _) = application(None);
    let mut runtime = ConfiguredRuntime::with_configuration(2, table)
        .expect("configured runtime storage can be reserved");
    let faulting_id = runtime.register(faulting).expect("faulting app fits");
    let peer_id = runtime.register(peer).expect("peer fits");
    let configurations = [
        RuntimeInboxConfig::new(1, &COMMAND_ONLY),
        RuntimeInboxConfig::new(1, &COMMAND_ONLY),
    ];
    let mut runtime = MessagingRuntime::new(runtime, &configurations)
        .expect("configured messaging topology is valid");
    runtime.start(faulting_id).expect("faulting app starts");
    runtime.start(peer_id).expect("peer starts");
    StartedMessagingRuntime {
        runtime,
        faulting_id,
        peer_id,
        faulting_observations,
        peer_observations,
    }
}

#[test]
fn messaging_work_preserves_configuration_and_failed_inbox_cleanup() {
    let mut started = started_messaging_runtime();
    let message = Message::try_new(MissionTopic::Command, &[7]).expect("payload fits");
    let report = started
        .runtime
        .publish(&message)
        .expect("report can be reserved");
    assert_eq!(report.classification(), PublishClassification::Complete);
    assert!(
        report
            .outcomes()
            .iter()
            .all(|outcome| { outcome.status() == DeliveryStatus::Delivered })
    );

    let error = started
        .runtime
        .work(started.faulting_id)
        .expect_err("faulting work returns through messaging owner");
    assert_eq!(
        error.operation_error(),
        &RuntimeWorkError::Application {
            application_id: started.faulting_id,
            source: MissionWorkError { call: 1 },
        }
    );
    assert_eq!(error.discarded_deliveries(), 1);
    assert_eq!(started.runtime.pending(started.faulting_id), Ok(0));
    assert_eq!(started.runtime.pending(started.peer_id), Ok(1));
    assert_eq!(
        started.runtime.work(started.peer_id),
        Ok(ApplicationState::Running)
    );
    assert_eq!(started.runtime.replace_configuration(&[2, 3]), Ok(3));
    assert_eq!(
        started.runtime.work(started.peer_id),
        Ok(ApplicationState::Running)
    );
    assert_eq!(started.runtime.rollback_configuration(), Ok(2));
    assert_eq!(
        started.runtime.work(started.peer_id),
        Ok(ApplicationState::Running)
    );

    assert_observation(&started.faulting_observations, 0, 2, &[1, 2]);
    assert_observation(&started.peer_observations, 0, 2, &[1, 2]);
    assert_observation(&started.peer_observations, 1, 3, &[2, 3]);
    assert_observation(&started.peer_observations, 2, 2, &[1, 2]);
}
