//! Public-API evidence for runtime-owned lifecycle-aware message delivery.

use std::error::Error;
use std::fmt;

use rust_flight_framework::{
    Application, ApplicationId, ApplicationState, DeliveryStatus, LifecycleError,
    LifecycleOperation, Message, MessageBusCreateError, MessagingRuntime,
    MessagingRuntimeCreateErrorKind, PublishClassification, PublishReport, Runtime,
    RuntimeInboxConfig, RuntimeRestartError, RuntimeStartError, RuntimeStopError, RuntimeWorkError,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MissionTopic {
    Command,
}

const COMMAND_ONLY: [MissionTopic; 1] = [MissionTopic::Command];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum FailurePoint {
    Start,
    Work,
    Stop,
    Restart,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct MissionFailure(FailurePoint);

impl fmt::Display for MissionFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "mission {:?} failure", self.0)
    }
}

impl Error for MissionFailure {}

#[derive(Debug)]
struct MissionApplication {
    failure: Option<FailurePoint>,
}

impl MissionApplication {
    fn run(&self, operation: FailurePoint) -> Result<(), MissionFailure> {
        if self.failure == Some(operation) {
            return Err(MissionFailure(operation));
        }
        Ok(())
    }
}

impl Application for MissionApplication {
    type StartError = MissionFailure;

    fn start(&mut self) -> Result<(), Self::StartError> {
        self.run(FailurePoint::Start)
    }

    type WorkError = MissionFailure;

    fn work(&mut self) -> Result<(), Self::WorkError> {
        self.run(FailurePoint::Work)
    }

    type StopError = MissionFailure;

    fn stop(&mut self) -> Result<(), Self::StopError> {
        self.run(FailurePoint::Stop)
    }

    type RestartError = MissionFailure;

    fn restart(&mut self) -> Result<(), Self::RestartError> {
        self.run(FailurePoint::Restart)
    }
}

type MissionRuntime = MessagingRuntime<MissionApplication, MissionTopic, 4>;

fn configured_runtime<const COUNT: usize>(
    failures: [Option<FailurePoint>; COUNT],
    capacities: [usize; COUNT],
) -> (MissionRuntime, [ApplicationId; COUNT]) {
    let mut runtime = Runtime::new(COUNT).expect("test runtime capacity is positive");
    let application_ids = failures.map(|failure| {
        runtime
            .register(MissionApplication { failure })
            .expect("test application fits")
    });
    let configurations =
        capacities.map(|capacity| RuntimeInboxConfig::new(capacity, &COMMAND_ONLY));
    let messaging_runtime =
        MessagingRuntime::new(runtime, &configurations).expect("test topology is valid");
    (messaging_runtime, application_ids)
}

fn command(payload: u8) -> Message<MissionTopic, 4> {
    Message::try_new(MissionTopic::Command, &[payload]).expect("test payload fits")
}

fn delivery_statuses(report: &PublishReport) -> Vec<DeliveryStatus> {
    report
        .outcomes()
        .iter()
        .map(|outcome| outcome.status())
        .collect()
}

#[test]
fn inbox_count_mismatch_preserves_the_owned_runtime() {
    let mut runtime = Runtime::new(1).expect("one runtime record can be reserved");
    let application_id = runtime
        .register(MissionApplication { failure: None })
        .expect("application fits");
    let no_inboxes: [RuntimeInboxConfig<'_, MissionTopic>; 0] = [];

    let error = MessagingRuntime::<_, MissionTopic, 4>::new(runtime, &no_inboxes)
        .expect_err("every registered application needs an inbox");
    assert_eq!(
        error.kind(),
        MessagingRuntimeCreateErrorKind::InboxCountMismatch {
            applications: 1,
            inboxes: 0,
        }
    );
    assert_eq!(
        error.runtime().state(application_id),
        Ok(ApplicationState::Registered)
    );

    let mut recovered_runtime = error.into_runtime();
    assert_eq!(
        recovered_runtime.start(application_id),
        Ok(ApplicationState::Running)
    );
}

#[test]
fn attachment_requires_registered_state_and_preserves_the_runtime() {
    let mut runtime = Runtime::new(2).expect("two runtime records can be reserved");
    let first_id = runtime
        .register(MissionApplication { failure: None })
        .expect("first application fits");
    let second_id = runtime
        .register(MissionApplication { failure: None })
        .expect("second application fits");
    assert_eq!(runtime.start(second_id), Ok(ApplicationState::Running));
    let configurations = [
        RuntimeInboxConfig::new(1, &COMMAND_ONLY),
        RuntimeInboxConfig::new(1, &COMMAND_ONLY),
    ];

    let error = MessagingRuntime::<_, MissionTopic, 4>::new(runtime, &configurations)
        .expect_err("an already-running application cannot be attached");
    assert_eq!(
        error.kind(),
        MessagingRuntimeCreateErrorKind::ApplicationNotRegistered {
            application_id: second_id,
            state: ApplicationState::Running,
        }
    );

    let mut recovered_runtime = error.into_runtime();
    assert_eq!(
        recovered_runtime.state(first_id),
        Ok(ApplicationState::Registered)
    );
    assert_eq!(
        recovered_runtime.stop(second_id),
        Ok(ApplicationState::Stopped)
    );
}

#[test]
fn invalid_inbox_configuration_preserves_the_owned_runtime() {
    let mut runtime = Runtime::new(1).expect("one runtime record can be reserved");
    let application_id = runtime
        .register(MissionApplication { failure: None })
        .expect("application fits");
    let configurations = [RuntimeInboxConfig::new(0, &COMMAND_ONLY)];

    let error = MessagingRuntime::<_, MissionTopic, 4>::new(runtime, &configurations)
        .expect_err("a zero-capacity inbox is invalid");
    assert_eq!(
        error.kind(),
        MessagingRuntimeCreateErrorKind::MessageBus(MessageBusCreateError::ZeroInboxCapacity {
            application_id
        })
    );
    assert_eq!(
        error.into_runtime().state(application_id),
        Ok(ApplicationState::Registered)
    );
}

#[test]
fn registered_subscribers_are_known_but_unavailable() {
    let (mut runtime, application_ids) = configured_runtime([None, None], [1, 2]);

    let report = runtime
        .publish(&command(1))
        .expect("report storage can be reserved");
    assert_eq!(
        report.classification(),
        PublishClassification::WhollyUndelivered
    );
    assert_eq!(
        delivery_statuses(&report),
        [DeliveryStatus::Unavailable, DeliveryStatus::Unavailable]
    );
    assert_eq!(report.outcomes()[0].application_id(), application_ids[0]);
    assert_eq!(report.outcomes()[1].application_id(), application_ids[1]);
    assert_eq!(runtime.inbox_capacity(application_ids[0]), Ok(1));
    assert_eq!(runtime.inbox_capacity(application_ids[1]), Ok(2));
    assert_eq!(runtime.pending(application_ids[0]), Ok(0));
    assert_eq!(runtime.pending(application_ids[1]), Ok(0));
}

#[test]
fn stop_clears_one_inbox_and_restart_reconnects_it_empty() {
    let (mut runtime, application_ids) = configured_runtime([None, None], [2, 4]);
    assert_eq!(
        runtime.start(application_ids[0]),
        Ok(ApplicationState::Running)
    );
    assert_eq!(
        runtime.start(application_ids[1]),
        Ok(ApplicationState::Running)
    );
    assert_eq!(
        runtime
            .publish(&command(1))
            .expect("report storage can be reserved")
            .classification(),
        PublishClassification::Complete
    );
    assert_eq!(
        runtime
            .publish(&command(2))
            .expect("report storage can be reserved")
            .classification(),
        PublishClassification::Complete
    );

    let stop = runtime.stop(application_ids[0]).expect("stop succeeds");
    assert_eq!(stop.state(), ApplicationState::Stopped);
    assert_eq!(stop.discarded_deliveries(), 2);
    assert_eq!(runtime.pending(application_ids[0]), Ok(0));
    assert_eq!(runtime.pending(application_ids[1]), Ok(2));

    let partial = runtime
        .publish(&command(3))
        .expect("report storage can be reserved");
    assert_eq!(partial.classification(), PublishClassification::Partial);
    assert_eq!(
        delivery_statuses(&partial),
        [DeliveryStatus::Unavailable, DeliveryStatus::Delivered]
    );
    assert_eq!(
        runtime.restart(application_ids[0]),
        Ok(ApplicationState::Running)
    );
    assert_eq!(runtime.pending(application_ids[0]), Ok(0));

    let reconnected = runtime
        .publish(&command(4))
        .expect("report storage can be reserved");
    assert_eq!(
        reconnected.classification(),
        PublishClassification::Complete
    );
    assert_eq!(runtime.pending(application_ids[0]), Ok(1));
    assert_eq!(runtime.pending(application_ids[1]), Ok(4));
}

#[test]
fn returned_work_error_clears_only_the_failed_endpoint() {
    let (mut runtime, application_ids) =
        configured_runtime([Some(FailurePoint::Work), None], [2, 2]);
    assert_eq!(
        runtime.start(application_ids[0]),
        Ok(ApplicationState::Running)
    );
    assert_eq!(
        runtime.start(application_ids[1]),
        Ok(ApplicationState::Running)
    );
    runtime
        .publish(&command(1))
        .expect("report storage can be reserved");

    let error = runtime
        .work(application_ids[0])
        .expect_err("selected application returns its work error");
    let expected = RuntimeWorkError::Application {
        application_id: application_ids[0],
        source: MissionFailure(FailurePoint::Work),
    };
    assert_eq!(error.operation_error(), &expected);
    assert_eq!(error.discarded_deliveries(), 1);
    assert_eq!(
        Error::source(&error).and_then(|source| source.downcast_ref()),
        Some(&expected)
    );
    assert_eq!(
        runtime.state(application_ids[0]),
        Ok(ApplicationState::Failed)
    );
    assert_eq!(runtime.pending(application_ids[0]), Ok(0));
    assert_eq!(runtime.pending(application_ids[1]), Ok(1));

    let later = runtime
        .publish(&command(2))
        .expect("report storage can be reserved");
    assert_eq!(later.classification(), PublishClassification::Partial);
    assert_eq!(
        delivery_statuses(&later),
        [DeliveryStatus::Unavailable, DeliveryStatus::Delivered]
    );
    assert_eq!(
        runtime.work(application_ids[1]),
        Ok(ApplicationState::Running)
    );
}

#[test]
fn returned_stop_error_clears_only_the_failed_endpoint() {
    let (mut runtime, application_ids) =
        configured_runtime([Some(FailurePoint::Stop), None], [2, 2]);
    assert_eq!(
        runtime.start(application_ids[0]),
        Ok(ApplicationState::Running)
    );
    assert_eq!(
        runtime.start(application_ids[1]),
        Ok(ApplicationState::Running)
    );
    runtime
        .publish(&command(1))
        .expect("report storage can be reserved");

    let error = runtime
        .stop(application_ids[0])
        .expect_err("selected application returns its stop error");
    assert_eq!(
        error.operation_error(),
        &RuntimeStopError::Application {
            application_id: application_ids[0],
            source: MissionFailure(FailurePoint::Stop),
        }
    );
    assert_eq!(error.discarded_deliveries(), 1);
    assert_eq!(
        runtime.state(application_ids[0]),
        Ok(ApplicationState::Failed)
    );
    assert_eq!(runtime.pending(application_ids[0]), Ok(0));
    assert_eq!(runtime.pending(application_ids[1]), Ok(1));

    let later = runtime
        .publish(&command(2))
        .expect("report storage can be reserved");
    assert_eq!(later.classification(), PublishClassification::Partial);
    assert_eq!(
        delivery_statuses(&later),
        [DeliveryStatus::Unavailable, DeliveryStatus::Delivered]
    );
}

#[test]
fn lifecycle_rejection_reports_zero_discard_without_clearing() {
    let (mut runtime, [application_id]) = configured_runtime([None], [2]);
    assert_eq!(runtime.start(application_id), Ok(ApplicationState::Running));
    runtime
        .publish(&command(1))
        .expect("report storage can be reserved");

    let error = runtime
        .start(application_id)
        .expect_err("a running application cannot start again");
    assert_eq!(
        error.operation_error(),
        &RuntimeStartError::Lifecycle(LifecycleError::InvalidTransition {
            application_id,
            state: ApplicationState::Running,
            operation: LifecycleOperation::Start,
        })
    );
    assert_eq!(error.discarded_deliveries(), 0);
    assert_eq!(runtime.state(application_id), Ok(ApplicationState::Running));
    assert_eq!(runtime.pending(application_id), Ok(1));
}

#[test]
fn start_failure_never_exposes_an_inbox() {
    let (mut runtime, [application_id]) = configured_runtime([Some(FailurePoint::Start)], [1]);

    let error = runtime
        .start(application_id)
        .expect_err("application returns its start error");
    assert_eq!(
        error.operation_error(),
        &RuntimeStartError::Application {
            application_id,
            source: MissionFailure(FailurePoint::Start),
        }
    );
    assert_eq!(error.discarded_deliveries(), 0);
    assert_eq!(runtime.state(application_id), Ok(ApplicationState::Failed));
    let report = runtime
        .publish(&command(1))
        .expect("report storage can be reserved");
    assert_eq!(delivery_statuses(&report), [DeliveryStatus::Unavailable]);
    assert_eq!(runtime.pending(application_id), Ok(0));
}

#[test]
fn restart_failure_remains_terminal_and_unavailable() {
    let (mut runtime, [application_id]) = configured_runtime([Some(FailurePoint::Restart)], [1]);
    assert_eq!(runtime.start(application_id), Ok(ApplicationState::Running));
    runtime
        .publish(&command(1))
        .expect("report storage can be reserved");
    assert_eq!(
        runtime
            .stop(application_id)
            .expect("stop succeeds before restart failure")
            .discarded_deliveries(),
        1
    );

    let error = runtime
        .restart(application_id)
        .expect_err("application returns its restart error");
    assert_eq!(
        error.operation_error(),
        &RuntimeRestartError::Application {
            application_id,
            source: MissionFailure(FailurePoint::Restart),
        }
    );
    assert_eq!(error.discarded_deliveries(), 0);
    assert_eq!(runtime.state(application_id), Ok(ApplicationState::Failed));
    let report = runtime
        .publish(&command(2))
        .expect("report storage can be reserved");
    assert_eq!(delivery_statuses(&report), [DeliveryStatus::Unavailable]);
    assert_eq!(runtime.pending(application_id), Ok(0));
}
