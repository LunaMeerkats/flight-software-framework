//! Public-API evidence for returned-stop failure and retained peer deliveries.

use std::cell::RefCell;
use std::convert::Infallible;
use std::error::Error;
use std::fmt;
use std::rc::Rc;

use rust_flight_framework::{
    Application, ApplicationId, ApplicationMessageContext, ApplicationState,
    ApplicationWorkContext, DeliveryStatus, LifecycleError, LifecycleOperation, Message,
    MessageDispatchError, MessageDispatchOutcome, MessagingApplication, MessagingOperationError,
    MessagingRuntime, PublishClassification, Runtime, RuntimeInboxConfig, RuntimeRestartError,
    RuntimeStopError, RuntimeWorkError,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MissionTopic {
    Command,
}

const COMMAND_ONLY: [MissionTopic; 1] = [MissionTopic::Command];

type MissionMessage = Message<MissionTopic, 2>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct MissionStopError {
    code: u8,
}

impl fmt::Display for MissionStopError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "requested stop failure {}", self.code)
    }
}

impl Error for MissionStopError {}

#[derive(Clone, Copy, Debug)]
enum StopBehavior {
    Complete,
    Reject(MissionStopError),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CallbackObservation {
    Start,
    Work,
    Stop,
    Restart,
    Message {
        application_id: ApplicationId,
        message: MissionMessage,
    },
}

type CallbackTrace = Rc<RefCell<Vec<CallbackObservation>>>;

#[derive(Debug)]
struct MissionApplication {
    stop_behavior: StopBehavior,
    trace: CallbackTrace,
}

impl MissionApplication {
    fn new(stop_behavior: StopBehavior) -> (Self, CallbackTrace) {
        let trace = Rc::new(RefCell::new(Vec::new()));
        (
            Self {
                stop_behavior,
                trace: Rc::clone(&trace),
            },
            trace,
        )
    }
}

impl Application for MissionApplication {
    type StartError = Infallible;

    fn start(&mut self) -> Result<(), Self::StartError> {
        self.trace.borrow_mut().push(CallbackObservation::Start);
        Ok(())
    }

    type WorkError = Infallible;

    fn work(&mut self, _context: ApplicationWorkContext<'_>) -> Result<(), Self::WorkError> {
        self.trace.borrow_mut().push(CallbackObservation::Work);
        Ok(())
    }

    type StopError = MissionStopError;

    fn stop(&mut self) -> Result<(), Self::StopError> {
        self.trace.borrow_mut().push(CallbackObservation::Stop);
        match self.stop_behavior {
            StopBehavior::Complete => Ok(()),
            StopBehavior::Reject(error) => Err(error),
        }
    }

    type RestartError = Infallible;

    fn restart(&mut self) -> Result<(), Self::RestartError> {
        self.trace.borrow_mut().push(CallbackObservation::Restart);
        Ok(())
    }
}

impl MessagingApplication<MissionTopic, 2> for MissionApplication {
    type MessageError = Infallible;

    fn handle_message(
        &mut self,
        message: &MissionMessage,
        context: &mut ApplicationMessageContext<'_, MissionTopic, 2>,
    ) -> Result<(), Self::MessageError> {
        self.trace.borrow_mut().push(CallbackObservation::Message {
            application_id: context.application_id(),
            message: *message,
        });
        Ok(())
    }
}

type MissionRuntime = MessagingRuntime<MissionApplication, MissionTopic, 2>;

struct MissionFixture {
    runtime: MissionRuntime,
    selected_id: ApplicationId,
    peer_id: ApplicationId,
    selected_trace: CallbackTrace,
    peer_trace: CallbackTrace,
}

impl MissionFixture {
    fn running() -> Self {
        let (selected, selected_trace) =
            MissionApplication::new(StopBehavior::Reject(MissionStopError { code: 29 }));
        let (peer, peer_trace) = MissionApplication::new(StopBehavior::Complete);
        let mut runtime = Runtime::new(2).expect("two runtime records can be reserved");
        let selected_id = runtime
            .register(selected)
            .expect("selected application fits");
        let peer_id = runtime.register(peer).expect("peer application fits");
        let configurations =
            [2, 3].map(|capacity| RuntimeInboxConfig::new(capacity, &COMMAND_ONLY));
        let mut runtime =
            MissionRuntime::new(runtime, &configurations).expect("test inbox topology is valid");
        assert_eq!(runtime.start(selected_id), Ok(ApplicationState::Running));
        assert_eq!(runtime.start(peer_id), Ok(ApplicationState::Running));
        Self {
            runtime,
            selected_id,
            peer_id,
            selected_trace,
            peer_trace,
        }
    }

    fn publish(
        &mut self,
        message: &MissionMessage,
        classification: PublishClassification,
        statuses: [DeliveryStatus; 2],
    ) {
        let report = self.runtime.publish(message).expect("report storage fits");
        assert_eq!(report.classification(), classification);
        assert_eq!(
            report
                .outcomes()
                .iter()
                .map(|outcome| (outcome.application_id(), outcome.status()))
                .collect::<Vec<_>>(),
            [(self.selected_id, statuses[0]), (self.peer_id, statuses[1])]
        );
    }

    fn assert_stop_failure(
        &self,
        error: &MessagingOperationError<RuntimeStopError<MissionStopError>>,
    ) {
        let expected = RuntimeStopError::Application {
            application_id: self.selected_id,
            source: MissionStopError { code: 29 },
        };
        assert_eq!(error.operation_error(), &expected);
        let operation_source = Error::source(error)
            .and_then(|source| source.downcast_ref::<RuntimeStopError<MissionStopError>>())
            .expect("the returned wrapper retains its concrete stop error");
        assert_eq!(operation_source, &expected);
        let mission_source = Error::source(operation_source)
            .and_then(|source| source.downcast_ref::<MissionStopError>())
            .expect("the stop error retains the original mission cause");
        assert_eq!(mission_source, &MissionStopError { code: 29 });
        assert!(Error::source(mission_source).is_none());
        assert_eq!(error.discarded_deliveries(), 2);
        assert_eq!(
            self.runtime.state(self.selected_id),
            Ok(ApplicationState::Failed)
        );
        assert_eq!(
            self.runtime.state(self.peer_id),
            Ok(ApplicationState::Running)
        );
        assert_eq!(self.runtime.pending(self.selected_id), Ok(0));
        assert_eq!(self.runtime.pending(self.peer_id), Ok(2));
        assert_eq!(
            *self.selected_trace.borrow(),
            [CallbackObservation::Start, CallbackObservation::Stop]
        );
        assert_eq!(*self.peer_trace.borrow(), [CallbackObservation::Start]);
    }

    fn assert_failed_lifecycle_requests(&mut self) {
        let invalid_transition = |operation| LifecycleError::InvalidTransition {
            application_id: self.selected_id,
            state: ApplicationState::Failed,
            operation,
        };
        let stop = self
            .runtime
            .stop(self.selected_id)
            .expect_err("Failed cannot stop");
        assert_eq!(
            stop.operation_error(),
            &RuntimeStopError::Lifecycle(invalid_transition(LifecycleOperation::Stop))
        );
        assert_eq!(stop.discarded_deliveries(), 0);
        self.assert_terminal_state_and_retained_peer_inbox();
        let restart = self
            .runtime
            .restart(self.selected_id)
            .expect_err("Failed cannot restart");
        assert_eq!(
            restart.operation_error(),
            &RuntimeRestartError::Lifecycle(invalid_transition(LifecycleOperation::Restart))
        );
        assert_eq!(restart.discarded_deliveries(), 0);
        self.assert_terminal_state_and_retained_peer_inbox();
    }

    fn assert_failed_work_and_dispatch_requests(&mut self) {
        let not_running = LifecycleError::NotRunning {
            application_id: self.selected_id,
            state: ApplicationState::Failed,
        };
        let work = self
            .runtime
            .work(self.selected_id)
            .expect_err("Failed cannot work");
        assert_eq!(
            work.operation_error(),
            &RuntimeWorkError::Lifecycle(not_running)
        );
        assert_eq!(work.discarded_deliveries(), 0);
        self.assert_terminal_state_and_retained_peer_inbox();
        let dispatch = self
            .runtime
            .dispatch_one(self.selected_id)
            .expect_err("Failed cannot dispatch");
        assert_eq!(
            dispatch.operation_error(),
            &MessageDispatchError::Lifecycle(not_running)
        );
        assert_eq!(dispatch.discarded_deliveries(), 0);
        self.assert_terminal_state_and_retained_peer_inbox();
    }

    fn assert_terminal_state_and_retained_peer_inbox(&self) {
        assert_eq!(
            *self.selected_trace.borrow(),
            [CallbackObservation::Start, CallbackObservation::Stop]
        );
        assert_eq!(*self.peer_trace.borrow(), [CallbackObservation::Start]);
        assert_eq!(self.runtime.pending(self.selected_id), Ok(0));
        assert_eq!(self.runtime.pending(self.peer_id), Ok(3));
        assert_eq!(
            self.runtime.state(self.selected_id),
            Ok(ApplicationState::Failed)
        );
        assert_eq!(
            self.runtime.state(self.peer_id),
            Ok(ApplicationState::Running)
        );
    }

    fn assert_peer_fifo_and_work_progress(&mut self, expected_messages: [MissionMessage; 3]) {
        let mut expected_trace = vec![CallbackObservation::Start];
        for (index, message) in expected_messages.into_iter().enumerate() {
            assert_eq!(
                self.runtime.dispatch_one(self.peer_id),
                Ok(MessageDispatchOutcome::Dispatched)
            );
            expected_trace.push(CallbackObservation::Message {
                application_id: self.peer_id,
                message,
            });
            assert_eq!(*self.peer_trace.borrow(), expected_trace);
            assert_eq!(self.runtime.pending(self.peer_id), Ok(2 - index));
        }
        assert_eq!(
            self.runtime.dispatch_one(self.peer_id),
            Ok(MessageDispatchOutcome::InboxEmpty)
        );
        assert_eq!(*self.peer_trace.borrow(), expected_trace);
        assert_eq!(
            self.runtime.work(self.peer_id),
            Ok(ApplicationState::Running)
        );
        expected_trace.push(CallbackObservation::Work);
        assert_eq!(*self.peer_trace.borrow(), expected_trace);
        assert_eq!(self.runtime.pending(self.peer_id), Ok(0));
        assert_eq!(
            self.runtime.state(self.peer_id),
            Ok(ApplicationState::Running)
        );
        assert_eq!(
            self.runtime.state(self.selected_id),
            Ok(ApplicationState::Failed)
        );
        assert_eq!(self.runtime.pending(self.selected_id), Ok(0));
        assert_eq!(
            *self.selected_trace.borrow(),
            [CallbackObservation::Start, CallbackObservation::Stop]
        );
    }
}

#[test]
fn returned_stop_failure_preserves_peer_fifo_and_terminal_callback_gates() {
    let mut mission = MissionFixture::running();
    let messages = [&[0x10, 1][..], &[0x20][..], &[0x30, 3][..]].map(|payload| {
        MissionMessage::try_new(MissionTopic::Command, payload).expect("test payload fits")
    });
    for message in &messages[..2] {
        mission.publish(
            message,
            PublishClassification::Complete,
            [DeliveryStatus::Delivered, DeliveryStatus::Delivered],
        );
    }
    assert_eq!(mission.runtime.inbox_capacity(mission.selected_id), Ok(2));
    assert_eq!(mission.runtime.pending(mission.selected_id), Ok(2));
    assert_eq!(mission.runtime.inbox_capacity(mission.peer_id), Ok(3));
    assert_eq!(mission.runtime.pending(mission.peer_id), Ok(2));

    let error = mission
        .runtime
        .stop(mission.selected_id)
        .expect_err("stop returns its concrete failure");
    mission.assert_stop_failure(&error);
    mission.publish(
        &messages[2],
        PublishClassification::Partial,
        [DeliveryStatus::Unavailable, DeliveryStatus::Delivered],
    );
    mission.assert_failed_lifecycle_requests();
    mission.assert_failed_work_and_dispatch_requests();
    mission.assert_peer_fifo_and_work_progress(messages);
}
