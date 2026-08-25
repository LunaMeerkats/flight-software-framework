//! Public-API evidence for caller-selected one-message application dispatch.

use std::cell::RefCell;
use std::convert::Infallible;
use std::error::Error;
use std::fmt;
use std::rc::Rc;

use rust_flight_framework::{
    Application, ApplicationId, ApplicationMessageContext, ApplicationState, DeliveryStatus,
    LifecycleError, LifecycleRegistry, Message, MessageDispatchError, MessageDispatchOutcome,
    MessagingApplication, MessagingOperationError, MessagingRuntime, PublishClassification,
    PublishError, PublishReport, Runtime, RuntimeInboxConfig,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MissionTopic {
    Input,
    Reply,
}

const INPUT_ONLY: [MissionTopic; 1] = [MissionTopic::Input];
const REPLY_ONLY: [MissionTopic; 1] = [MissionTopic::Reply];
const INPUT_AND_REPLY: [MissionTopic; 2] = [MissionTopic::Input, MissionTopic::Reply];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DispatchBehavior {
    Passive,
    PublishOnce,
    PublishTwice,
    PublishThenFail,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DispatchFailure {
    Requested,
    Publish(PublishError),
}

impl fmt::Display for DispatchFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Requested => formatter.write_str("requested message callback failure"),
            Self::Publish(error) => error.fmt(formatter),
        }
    }
}

impl Error for DispatchFailure {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Requested => None,
            Self::Publish(error) => Some(error),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct HandledMessage {
    application_id: ApplicationId,
    topic: MissionTopic,
    payload: u8,
}

#[derive(Debug, Eq, PartialEq)]
struct PublicationObservation {
    classification: PublishClassification,
    outcomes: Vec<(ApplicationId, DeliveryStatus)>,
}

impl PublicationObservation {
    fn from_report(report: &PublishReport) -> Self {
        Self {
            classification: report.classification(),
            outcomes: report
                .outcomes()
                .iter()
                .map(|outcome| (outcome.application_id(), outcome.status()))
                .collect(),
        }
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
struct DispatchTrace {
    handled: Vec<HandledMessage>,
    publications: Vec<PublicationObservation>,
}

#[derive(Debug)]
struct DispatchApplication {
    behavior: DispatchBehavior,
    trace: Rc<RefCell<DispatchTrace>>,
}

impl DispatchApplication {
    fn publish_reply(
        &self,
        context: &mut ApplicationMessageContext<'_, MissionTopic, 1>,
        payload: u8,
    ) -> Result<(), DispatchFailure> {
        let report = context
            .publish(&message(MissionTopic::Reply, payload))
            .map_err(DispatchFailure::Publish)?;
        self.trace
            .borrow_mut()
            .publications
            .push(PublicationObservation::from_report(&report));
        Ok(())
    }
}

impl Application for DispatchApplication {
    type StartError = Infallible;

    fn start(&mut self) -> Result<(), Self::StartError> {
        Ok(())
    }

    type WorkError = Infallible;

    fn work(&mut self) -> Result<(), Self::WorkError> {
        Ok(())
    }

    type StopError = Infallible;

    fn stop(&mut self) -> Result<(), Self::StopError> {
        Ok(())
    }

    type RestartError = Infallible;

    fn restart(&mut self) -> Result<(), Self::RestartError> {
        Ok(())
    }
}

impl MessagingApplication<MissionTopic, 1> for DispatchApplication {
    type MessageError = DispatchFailure;

    fn handle_message(
        &mut self,
        message: &Message<MissionTopic, 1>,
        context: &mut ApplicationMessageContext<'_, MissionTopic, 1>,
    ) -> Result<(), Self::MessageError> {
        let payload = message.payload().first().copied().unwrap_or_default();
        self.trace.borrow_mut().handled.push(HandledMessage {
            application_id: context.application_id(),
            topic: *message.topic(),
            payload,
        });

        if *message.topic() != MissionTopic::Input {
            return Ok(());
        }
        match self.behavior {
            DispatchBehavior::Passive => Ok(()),
            DispatchBehavior::PublishOnce => self.publish_reply(context, payload.wrapping_add(1)),
            DispatchBehavior::PublishTwice => {
                self.publish_reply(context, payload.wrapping_add(1))?;
                self.publish_reply(context, payload.wrapping_add(2))
            }
            DispatchBehavior::PublishThenFail => {
                self.publish_reply(context, payload.wrapping_add(1))?;
                Err(DispatchFailure::Requested)
            }
        }
    }
}

type DispatchRuntime = MessagingRuntime<DispatchApplication, MissionTopic, 1>;

fn configured_runtime<const COUNT: usize>(
    applications: [DispatchApplication; COUNT],
    capacities: [usize; COUNT],
    subscriptions: [&'static [MissionTopic]; COUNT],
) -> (DispatchRuntime, [ApplicationId; COUNT]) {
    let mut runtime = Runtime::new(COUNT).expect("test runtime capacity is positive");
    let application_ids = applications.map(|application| {
        runtime
            .register(application)
            .expect("test application fits")
    });
    let configurations: [RuntimeInboxConfig<'static, MissionTopic>; COUNT] =
        std::array::from_fn(|index| {
            RuntimeInboxConfig::new(capacities[index], subscriptions[index])
        });
    let runtime = MessagingRuntime::new(runtime, &configurations).expect("test topology is valid");
    (runtime, application_ids)
}

fn application(behavior: DispatchBehavior) -> (DispatchApplication, Rc<RefCell<DispatchTrace>>) {
    let trace = Rc::new(RefCell::new(DispatchTrace::default()));
    (
        DispatchApplication {
            behavior,
            trace: Rc::clone(&trace),
        },
        trace,
    )
}

fn message(topic: MissionTopic, payload: u8) -> Message<MissionTopic, 1> {
    Message::try_new(topic, &[payload]).expect("one-byte test payload fits")
}

fn operation_error(
    application_id: ApplicationId,
    state: ApplicationState,
) -> MessageDispatchError<DispatchFailure> {
    MessageDispatchError::Lifecycle(LifecycleError::NotRunning {
        application_id,
        state,
    })
}

#[test]
fn lifecycle_gate_and_empty_inbox_never_invoke_application_code() {
    let (application, trace) = application(DispatchBehavior::Passive);
    let (mut runtime, [application_id]) = configured_runtime([application], [1], [&INPUT_ONLY]);

    let registered = runtime
        .dispatch_one(application_id)
        .expect_err("registered applications cannot dispatch");
    assert_eq!(
        registered.operation_error(),
        &operation_error(application_id, ApplicationState::Registered)
    );
    assert_eq!(registered.discarded_deliveries(), 0);

    let mut identity_source = LifecycleRegistry::new(2).expect("test capacity is positive");
    identity_source
        .register()
        .expect("first test identity fits");
    let unknown_id = identity_source
        .register()
        .expect("second test identity fits");
    let unknown = runtime
        .dispatch_one(unknown_id)
        .expect_err("identity outside the topology is rejected");
    assert_eq!(
        unknown.operation_error(),
        &MessageDispatchError::Lifecycle(LifecycleError::UnknownApplication {
            application_id: unknown_id
        })
    );
    assert_eq!(unknown.discarded_deliveries(), 0);

    assert_eq!(runtime.start(application_id), Ok(ApplicationState::Running));
    assert_eq!(
        runtime.dispatch_one(application_id),
        Ok(MessageDispatchOutcome::InboxEmpty)
    );
    assert!(trace.borrow().handled.is_empty());
    assert_eq!(runtime.pending(application_id), Ok(0));

    runtime.stop(application_id).expect("test stop succeeds");
    let stopped = runtime
        .dispatch_one(application_id)
        .expect_err("stopped applications cannot dispatch");
    assert_eq!(
        stopped.operation_error(),
        &operation_error(application_id, ApplicationState::Stopped)
    );
    assert_eq!(stopped.discarded_deliveries(), 0);
    assert!(trace.borrow().handled.is_empty());
}

#[test]
fn dispatch_presents_one_oldest_message_per_call() {
    let (application, trace) = application(DispatchBehavior::Passive);
    let (mut runtime, [application_id]) = configured_runtime([application], [2], [&INPUT_ONLY]);
    assert_eq!(runtime.start(application_id), Ok(ApplicationState::Running));
    runtime
        .publish(&message(MissionTopic::Input, 1))
        .expect("first report can be reserved");
    runtime
        .publish(&message(MissionTopic::Input, 2))
        .expect("second report can be reserved");

    assert_eq!(
        runtime.dispatch_one(application_id),
        Ok(MessageDispatchOutcome::Dispatched)
    );
    assert_eq!(runtime.pending(application_id), Ok(1));
    assert_eq!(trace.borrow().handled[0].payload, 1);

    assert_eq!(
        runtime.dispatch_one(application_id),
        Ok(MessageDispatchOutcome::Dispatched)
    );
    assert_eq!(runtime.pending(application_id), Ok(0));
    assert_eq!(
        trace
            .borrow()
            .handled
            .iter()
            .map(|handled| handled.payload)
            .collect::<Vec<_>>(),
        [1, 2]
    );
    assert_eq!(
        runtime.dispatch_one(application_id),
        Ok(MessageDispatchOutcome::InboxEmpty)
    );
    assert_eq!(trace.borrow().handled.len(), 2);
}

#[test]
fn self_publication_uses_the_freed_slot_and_preserves_peer_outcomes() {
    let (publisher, publisher_trace) = application(DispatchBehavior::PublishTwice);
    let (peer, _peer_trace) = application(DispatchBehavior::Passive);
    let (unavailable, _unavailable_trace) = application(DispatchBehavior::Passive);
    let (mut runtime, application_ids) = configured_runtime(
        [publisher, peer, unavailable],
        [1, 2, 1],
        [&INPUT_AND_REPLY, &REPLY_ONLY, &REPLY_ONLY],
    );
    runtime.start(application_ids[0]).expect("publisher starts");
    runtime.start(application_ids[1]).expect("peer starts");
    runtime.start(application_ids[2]).expect("third app starts");
    runtime.stop(application_ids[2]).expect("third app stops");
    runtime
        .publish(&message(MissionTopic::Input, 7))
        .expect("input report can be reserved");

    assert_eq!(
        runtime.dispatch_one(application_ids[0]),
        Ok(MessageDispatchOutcome::Dispatched)
    );
    assert_eq!(
        runtime.state(application_ids[0]),
        Ok(ApplicationState::Running)
    );
    assert_eq!(runtime.pending(application_ids[0]), Ok(1));
    assert_eq!(runtime.pending(application_ids[1]), Ok(2));
    assert_eq!(runtime.pending(application_ids[2]), Ok(0));

    let trace = publisher_trace.borrow();
    assert_eq!(trace.handled[0].application_id, application_ids[0]);
    assert_eq!(trace.handled[0].payload, 7);
    assert_publish_twice_outcomes(&trace.publications, application_ids);
    drop(trace);

    assert_eq!(
        runtime.dispatch_one(application_ids[0]),
        Ok(MessageDispatchOutcome::Dispatched)
    );
    assert_eq!(runtime.pending(application_ids[0]), Ok(0));
    assert_eq!(publisher_trace.borrow().handled[1].payload, 8);
}

fn assert_publish_twice_outcomes(
    publications: &[PublicationObservation],
    application_ids: [ApplicationId; 3],
) {
    assert_eq!(publications.len(), 2);
    assert_eq!(
        publications[0],
        PublicationObservation {
            classification: PublishClassification::Partial,
            outcomes: vec![
                (application_ids[0], DeliveryStatus::Delivered),
                (application_ids[1], DeliveryStatus::Delivered),
                (application_ids[2], DeliveryStatus::Unavailable),
            ],
        }
    );
    assert_eq!(
        publications[1],
        PublicationObservation {
            classification: PublishClassification::Partial,
            outcomes: vec![
                (application_ids[0], DeliveryStatus::InboxFull),
                (application_ids[1], DeliveryStatus::Delivered),
                (application_ids[2], DeliveryStatus::Unavailable),
            ],
        }
    );
}

#[test]
fn dispatch_refreshes_peer_availability_after_lifecycle_change() {
    let (publisher, publisher_trace) = application(DispatchBehavior::PublishOnce);
    let (peer, _peer_trace) = application(DispatchBehavior::Passive);
    let (mut runtime, application_ids) =
        configured_runtime([publisher, peer], [1, 1], [&INPUT_ONLY, &REPLY_ONLY]);
    runtime.start(application_ids[0]).expect("publisher starts");
    runtime.start(application_ids[1]).expect("peer starts");
    runtime
        .publish(&message(MissionTopic::Input, 1))
        .expect("first input report can be reserved");
    runtime
        .dispatch_one(application_ids[0])
        .expect("first callback succeeds");
    assert_eq!(runtime.pending(application_ids[1]), Ok(1));
    assert_eq!(
        runtime
            .stop(application_ids[1])
            .expect("peer stop succeeds")
            .discarded_deliveries(),
        1
    );

    runtime
        .publish(&message(MissionTopic::Input, 2))
        .expect("second input report can be reserved");
    runtime
        .dispatch_one(application_ids[0])
        .expect("second callback succeeds");
    let trace = publisher_trace.borrow();
    assert_eq!(trace.publications.len(), 2);
    assert_eq!(
        trace.publications[0].outcomes,
        [(application_ids[1], DeliveryStatus::Delivered)]
    );
    assert_eq!(
        trace.publications[1].outcomes,
        [(application_ids[1], DeliveryStatus::Unavailable)]
    );
    assert_eq!(runtime.pending(application_ids[1]), Ok(0));
}

fn assert_callback_failure_error(
    error: &MessagingOperationError<MessageDispatchError<DispatchFailure>>,
    application_id: ApplicationId,
) {
    let expected = MessageDispatchError::Application {
        application_id,
        source: DispatchFailure::Requested,
    };
    assert_eq!(error.operation_error(), &expected);
    assert_eq!(error.discarded_deliveries(), 2);
    assert_eq!(
        Error::source(error).and_then(|source| source.downcast_ref()),
        Some(&expected)
    );
    assert_eq!(
        Error::source(&expected).and_then(|source| source.downcast_ref()),
        Some(&DispatchFailure::Requested)
    );
}

#[test]
fn callback_error_clears_selected_queue_but_retains_peer_publication() {
    let (failing, failing_trace) = application(DispatchBehavior::PublishThenFail);
    let (peer, peer_trace) = application(DispatchBehavior::Passive);
    let (mut runtime, application_ids) = configured_runtime(
        [failing, peer],
        [2, 3],
        [&INPUT_AND_REPLY, &INPUT_AND_REPLY],
    );
    runtime.start(application_ids[0]).expect("publisher starts");
    runtime.start(application_ids[1]).expect("peer starts");
    runtime
        .publish(&message(MissionTopic::Input, 1))
        .expect("first input report can be reserved");
    runtime
        .publish(&message(MissionTopic::Input, 2))
        .expect("second input report can be reserved");

    let error = runtime
        .dispatch_one(application_ids[0])
        .expect_err("selected callback returns its concrete error");
    assert_callback_failure_error(&error, application_ids[0]);
    assert_eq!(
        runtime.state(application_ids[0]),
        Ok(ApplicationState::Failed)
    );
    assert_eq!(runtime.pending(application_ids[0]), Ok(0));
    assert_eq!(
        runtime.state(application_ids[1]),
        Ok(ApplicationState::Running)
    );
    assert_eq!(runtime.pending(application_ids[1]), Ok(3));

    let publication = &failing_trace.borrow().publications[0];
    assert_eq!(publication.classification, PublishClassification::Complete);
    assert_eq!(
        publication.outcomes,
        [
            (application_ids[0], DeliveryStatus::Delivered),
            (application_ids[1], DeliveryStatus::Delivered),
        ]
    );

    assert_eq!(
        runtime.dispatch_one(application_ids[1]),
        Ok(MessageDispatchOutcome::Dispatched)
    );
    assert_eq!(peer_trace.borrow().handled[0].payload, 1);
    assert_eq!(runtime.pending(application_ids[1]), Ok(2));
    let failed = runtime
        .dispatch_one(application_ids[0])
        .expect_err("terminal failed application remains ineligible");
    assert_eq!(
        failed.operation_error(),
        &operation_error(application_ids[0], ApplicationState::Failed)
    );
    assert_eq!(failed.discarded_deliveries(), 0);
}
