//! Public-only selective publication and fixed two-subscriber retry consumer.

use std::cell::Cell;
use std::convert::Infallible;
use std::error::Error;
use std::fmt;
use std::rc::Rc;

use rust_flight_framework::{
    Application, ApplicationId, ApplicationInboxConfig, ApplicationMessageContext,
    ApplicationState, ApplicationWorkContext, DeliveryStatus, LifecycleRegistry, Message,
    MessageBus, MessageDispatchOutcome, MessagingApplication, MessagingRuntime,
    PublishClassification, PublishReport, PublishToError, Runtime, RuntimeInboxConfig,
    RuntimeWorkError,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Topic {
    Output,
    Other,
    Unrouted,
}

const OUTPUT: [Topic; 1] = [Topic::Output];
const OTHER: [Topic; 1] = [Topic::Other];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum WorkPolicy {
    Healthy,
    Fail,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SinkError {
    RequestedWorkFailure,
    OutputOccupied,
    InvalidPayload,
}

impl fmt::Display for SinkError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RequestedWorkFailure => formatter.write_str("requested ordinary-work failure"),
            Self::OutputOccupied => formatter.write_str("host has not observed the older value"),
            Self::InvalidPayload => formatter.write_str("expected one output byte"),
        }
    }
}

impl Error for SinkError {}

#[derive(Debug)]
struct Sink {
    output: Rc<Cell<Option<u8>>>,
    work_policy: WorkPolicy,
}

impl Application for Sink {
    type StartError = Infallible;
    type WorkError = SinkError;
    type StopError = Infallible;
    type RestartError = Infallible;

    fn start(&mut self) -> Result<(), Self::StartError> {
        Ok(())
    }

    fn work(&mut self, _context: ApplicationWorkContext<'_>) -> Result<(), Self::WorkError> {
        match self.work_policy {
            WorkPolicy::Healthy => Ok(()),
            WorkPolicy::Fail => Err(SinkError::RequestedWorkFailure),
        }
    }

    fn stop(&mut self) -> Result<(), Self::StopError> {
        Ok(())
    }

    fn restart(&mut self) -> Result<(), Self::RestartError> {
        Ok(())
    }
}

impl MessagingApplication<Topic, 1> for Sink {
    type MessageError = SinkError;

    fn handle_message(
        &mut self,
        message: &Message<Topic, 1>,
        _context: &mut ApplicationMessageContext<'_, Topic, 1>,
    ) -> Result<(), Self::MessageError> {
        if self.output.get().is_some() {
            return Err(SinkError::OutputOccupied);
        }
        let [value] = message.payload() else {
            return Err(SinkError::InvalidPayload);
        };
        self.output.set(Some(*value));
        Ok(())
    }
}

type Owner = MessagingRuntime<Sink, Topic, 1>;

struct Mission {
    owner: Owner,
    ids: [ApplicationId; 2],
    outputs: [Rc<Cell<Option<u8>>>; 2],
}

// One record and two acceptance flags are retained until both inboxes accept.
// The record stores acceptance history, not callback or delivery evidence.
struct PendingFanout {
    message: Message<Topic, 1>,
    ids: [ApplicationId; 2],
    accepted: [bool; 2],
}

impl PendingFanout {
    fn from_report(
        message: Message<Topic, 1>,
        ids: [ApplicationId; 2],
        report: &PublishReport,
    ) -> Self {
        // This consumer fixes exactly these two subscribers at construction.
        assert_eq!(report.outcomes().len(), ids.len());
        let accepted = std::array::from_fn(|index| {
            let outcome = report.outcomes()[index];
            assert_eq!(outcome.application_id(), ids[index]);
            outcome.status() == DeliveryStatus::Delivered
        });
        Self {
            message,
            ids,
            accepted,
        }
    }

    fn retry_one(
        &mut self,
        owner: &mut Owner,
        index: usize,
    ) -> Result<Option<DeliveryStatus>, PublishToError> {
        if self.accepted[index] {
            return Ok(None);
        }
        let status = owner.publish_to(self.ids[index], &self.message)?;
        self.accepted[index] = status == DeliveryStatus::Delivered;
        Ok(Some(status))
    }

    fn replace_after_acceptance(
        &mut self,
        next: Message<Topic, 1>,
    ) -> Result<(), Message<Topic, 1>> {
        if self.accepted != [true; 2] {
            return Err(next);
        }
        self.message = next;
        self.accepted = [false; 2];
        Ok(())
    }
}

fn message(topic: Topic, value: u8) -> Message<Topic, 1> {
    Message::try_new(topic, &[value]).expect("one byte fits the fixed payload")
}

fn identities<const COUNT: usize>() -> [ApplicationId; COUNT] {
    let mut registry = LifecycleRegistry::new(COUNT).expect("positive test capacity");
    std::array::from_fn(|_| registry.register().expect("test record fits"))
}

fn mission(topics: [&[Topic]; 2], capacities: [usize; 2], policies: [WorkPolicy; 2]) -> Mission {
    let outputs = std::array::from_fn(|_| Rc::new(Cell::new(None)));
    let mut runtime = Runtime::new(2).expect("two application records fit");
    let ids = std::array::from_fn(|index| {
        runtime
            .register(Sink {
                output: Rc::clone(&outputs[index]),
                work_policy: policies[index],
            })
            .expect("sink fits fixed runtime capacity")
    });
    let configurations = std::array::from_fn::<_, 2, _>(|index| {
        RuntimeInboxConfig::new(capacities[index], topics[index])
    });
    let owner = Owner::new(runtime, &configurations).expect("fixed topology is valid");
    Mission {
        owner,
        ids,
        outputs,
    }
}

fn start_both(mission: &mut Mission) {
    for id in mission.ids {
        assert_eq!(mission.owner.start(id), Ok(ApplicationState::Running));
    }
}

fn dispatch_value(mission: &mut Mission, index: usize) -> u8 {
    assert_eq!(
        mission.owner.dispatch_one(mission.ids[index]),
        Ok(MessageDispatchOutcome::Dispatched)
    );
    mission.outputs[index]
        .take()
        .expect("one dispatched byte is available for host observation")
}

fn partial_fanout() -> (Mission, PendingFanout) {
    let mut mission = mission([&OUTPUT; 2], [2; 2], [WorkPolicy::Healthy; 2]);
    start_both(&mut mission);
    for value in [3, 4] {
        assert_eq!(
            mission
                .owner
                .publish_to(mission.ids[1], &message(Topic::Output, value)),
            Ok(DeliveryStatus::Delivered)
        );
    }
    let next = message(Topic::Output, 42);
    let report = mission
        .owner
        .publish(&next)
        .expect("two-outcome report fits");
    assert_eq!(report.classification(), PublishClassification::Partial);
    assert_eq!(report.outcomes()[0].status(), DeliveryStatus::Delivered);
    assert_eq!(report.outcomes()[1].status(), DeliveryStatus::InboxFull);
    let pending = PendingFanout::from_report(next, mission.ids, &report);
    (mission, pending)
}

#[test]
fn selected_publication_preserves_peer_inboxes_and_full_fifo() {
    let ids = identities::<2>();
    let configurations = ids.map(|id| ApplicationInboxConfig::new(id, 2, &OUTPUT));
    let mut bus = MessageBus::new(&configurations).expect("fixed topology is valid");
    let first = message(Topic::Output, 1);
    let second = message(Topic::Output, 2);
    let rejected = message(Topic::Output, 3);
    assert_eq!(
        bus.publish_to(ids[0], &first),
        Ok(DeliveryStatus::Delivered)
    );
    assert_eq!(bus.pending(ids[1]), Ok(0));
    let broadcast = bus.publish(&second).expect("two-outcome report fits");
    assert_eq!(broadcast.classification(), PublishClassification::Complete);
    assert_eq!(
        bus.publish_to(ids[0], &rejected),
        Ok(DeliveryStatus::InboxFull)
    );
    assert_eq!(bus.pending(ids[0]), Ok(2));
    assert_eq!(bus.pending(ids[1]), Ok(1));
    assert_eq!(bus.dequeue(ids[0]), Ok(Some(first)));
    assert_eq!(bus.dequeue(ids[0]), Ok(Some(second)));
    assert_eq!(bus.dequeue(ids[0]), Ok(None));
    assert_eq!(bus.dequeue(ids[1]), Ok(Some(second)));
    assert_eq!(bus.dequeue(ids[1]), Ok(None));
    assert_eq!(rejected.payload(), &[3]);
    assert_eq!(
        bus.publish_to(ids[1], &rejected),
        Ok(DeliveryStatus::Delivered)
    );
    assert_eq!(bus.dequeue(ids[1]), Ok(Some(rejected)));
}

#[test]
fn standalone_rejects_unknown_and_unsubscribed_before_full_inbox_checks() {
    let ids = identities::<3>();
    let configurations = [
        ApplicationInboxConfig::new(ids[0], 1, &OUTPUT),
        ApplicationInboxConfig::new(ids[1], 1, &OTHER),
    ];
    let mut bus = MessageBus::new(&configurations).expect("fixed topology is valid");
    let output = message(Topic::Output, 7);
    let other = message(Topic::Other, 8);
    assert_eq!(
        bus.publish_to(ids[0], &output),
        Ok(DeliveryStatus::Delivered)
    );
    assert_eq!(
        bus.publish_to(ids[1], &other),
        Ok(DeliveryStatus::Delivered)
    );
    for record in [output, message(Topic::Unrouted, 9)] {
        assert_eq!(
            bus.publish_to(ids[2], &record),
            Err(PublishToError::UnknownApplication {
                application_id: ids[2]
            })
        );
        assert_eq!(
            bus.publish_to(ids[1], &record),
            Err(PublishToError::NotSubscribed {
                application_id: ids[1]
            })
        );
    }
    assert_eq!(bus.pending(ids[0]), Ok(1));
    assert_eq!(bus.pending(ids[1]), Ok(1));
    assert_eq!(bus.dequeue(ids[0]), Ok(Some(output)));
    assert_eq!(bus.dequeue(ids[1]), Ok(Some(other)));
    let report = bus
        .publish(&message(Topic::Unrouted, 9))
        .expect("empty report fits");
    assert_eq!(
        report.classification(),
        PublishClassification::NoSubscribers
    );
}

#[test]
fn runtime_selective_publication_tracks_lifecycle_and_selected_clearing() {
    let mut mission = mission([&OUTPUT; 2], [1; 2], [WorkPolicy::Healthy; 2]);
    let selected = mission.ids[0];
    let peer = mission.ids[1];
    let first = message(Topic::Output, 1);
    assert_eq!(
        mission.owner.publish_to(selected, &first),
        Ok(DeliveryStatus::Unavailable)
    );
    assert_eq!(mission.owner.pending(selected), Ok(0));
    start_both(&mut mission);
    assert_eq!(
        mission.owner.publish_to(selected, &first),
        Ok(DeliveryStatus::Delivered)
    );
    assert_eq!(
        mission.owner.publish_to(selected, &first),
        Ok(DeliveryStatus::InboxFull)
    );
    assert_eq!(
        mission.owner.publish_to(peer, &first),
        Ok(DeliveryStatus::Delivered)
    );
    let stopped = mission
        .owner
        .stop(selected)
        .expect("cooperative stop succeeds");
    assert_eq!(stopped.discarded_deliveries(), 1);
    assert_eq!(
        mission.owner.publish_to(selected, &first),
        Ok(DeliveryStatus::Unavailable)
    );
    assert_eq!(
        mission.owner.restart(selected),
        Ok(ApplicationState::Running)
    );
    let second = message(Topic::Output, 2);
    assert_eq!(
        mission.owner.publish_to(selected, &second),
        Ok(DeliveryStatus::Delivered)
    );
    assert_eq!(mission.owner.pending(peer), Ok(1));
    assert_eq!(dispatch_value(&mut mission, 0), 2);
    assert_eq!(dispatch_value(&mut mission, 1), 1);
}

#[test]
fn failed_selected_subscriber_rejects_retry_and_preserves_healthy_peer() {
    let mut mission = mission(
        [&OUTPUT; 2],
        [1; 2],
        [WorkPolicy::Fail, WorkPolicy::Healthy],
    );
    start_both(&mut mission);
    let selected = mission.ids[0];
    let peer = mission.ids[1];
    let output = message(Topic::Output, 2);
    let report = mission
        .owner
        .publish(&output)
        .expect("two-outcome report fits");
    assert_eq!(report.classification(), PublishClassification::Complete);
    let failure = mission
        .owner
        .work(selected)
        .expect_err("selected ordinary work fails");
    assert_eq!(failure.discarded_deliveries(), 1);
    assert_eq!(
        failure.operation_error(),
        &RuntimeWorkError::Application {
            application_id: selected,
            source: SinkError::RequestedWorkFailure
        }
    );
    assert_eq!(mission.owner.state(selected), Ok(ApplicationState::Failed));
    assert_eq!(
        mission.owner.publish_to(selected, &output),
        Ok(DeliveryStatus::Unavailable)
    );
    assert_eq!(mission.owner.pending(selected), Ok(0));
    assert_eq!(dispatch_value(&mut mission, 1), 2);
    assert!(mission.owner.work(peer).is_ok());
}

#[test]
fn runtime_validates_subscription_before_registered_running_or_stopped_availability() {
    let mut mission = mission([&OUTPUT, &OTHER], [1; 2], [WorkPolicy::Healthy; 2]);
    let unknown = identities::<3>()[2];
    let output = message(Topic::Output, 6);
    let other = message(Topic::Other, 7);
    for phase in 0..3 {
        if phase == 1 {
            start_both(&mut mission);
            assert_eq!(
                mission.owner.publish_to(mission.ids[1], &other),
                Ok(DeliveryStatus::Delivered)
            );
        } else if phase == 2 {
            mission
                .owner
                .stop(mission.ids[1])
                .expect("stop clears the unrelated topic");
        }
        assert_eq!(
            mission.owner.publish_to(unknown, &output),
            Err(PublishToError::UnknownApplication {
                application_id: unknown
            })
        );
        assert_eq!(
            mission.owner.publish_to(mission.ids[1], &output),
            Err(PublishToError::NotSubscribed {
                application_id: mission.ids[1]
            })
        );
        assert_eq!(mission.owner.pending(mission.ids[0]), Ok(0));
    }
    assert_eq!(mission.owner.pending(mission.ids[1]), Ok(0));
    assert!(mission.outputs.iter().all(|output| output.get().is_none()));
}

#[test]
fn partial_retry_retains_one_record_and_never_repeats_an_accepted_copy() {
    let (mut mission, mut pending) = partial_fanout();
    let later = message(Topic::Output, 43);
    assert_eq!(pending.replace_after_acceptance(later), Err(later));
    for _ in 0..2 {
        assert_eq!(
            pending.retry_one(&mut mission.owner, 1),
            Ok(Some(DeliveryStatus::InboxFull))
        );
        assert_eq!(pending.message.payload(), &[42]);
        assert_eq!(pending.accepted, [true, false]);
    }
    assert_eq!(dispatch_value(&mut mission, 0), 42);
    assert_eq!(pending.retry_one(&mut mission.owner, 0), Ok(None));
    assert_eq!(mission.owner.pending(mission.ids[0]), Ok(0));
    assert_eq!(dispatch_value(&mut mission, 1), 3);
    assert_eq!(
        pending.retry_one(&mut mission.owner, 1),
        Ok(Some(DeliveryStatus::Delivered))
    );
    assert_eq!(pending.accepted, [true; 2]);
    assert_eq!(dispatch_value(&mut mission, 1), 4);
    assert_eq!(dispatch_value(&mut mission, 1), 42);
    for id in mission.ids {
        assert_eq!(
            mission.owner.dispatch_one(id),
            Ok(MessageDispatchOutcome::InboxEmpty)
        );
    }
    assert_eq!(pending.replace_after_acceptance(later), Ok(()));
    for index in 0..2 {
        assert_eq!(
            pending.retry_one(&mut mission.owner, index),
            Ok(Some(DeliveryStatus::Delivered))
        );
    }
    assert_eq!(dispatch_value(&mut mission, 0), 43);
    assert_eq!(dispatch_value(&mut mission, 1), 43);
}

#[test]
fn retry_after_unavailable_retains_acceptance_and_requires_explicit_restart() {
    let (mut mission, mut pending) = partial_fanout();
    let stopped = mission
        .owner
        .stop(mission.ids[1])
        .expect("caller explicitly stops sink");
    assert_eq!(stopped.discarded_deliveries(), 2);
    for _ in 0..2 {
        assert_eq!(
            pending.retry_one(&mut mission.owner, 1),
            Ok(Some(DeliveryStatus::Unavailable))
        );
        assert_eq!(pending.accepted, [true, false]);
        assert_eq!(pending.message.payload(), &[42]);
    }
    assert_eq!(dispatch_value(&mut mission, 0), 42);
    assert_eq!(
        mission.owner.restart(mission.ids[1]),
        Ok(ApplicationState::Running)
    );
    assert_eq!(
        pending.retry_one(&mut mission.owner, 1),
        Ok(Some(DeliveryStatus::Delivered))
    );
    assert_eq!(pending.retry_one(&mut mission.owner, 0), Ok(None));
    assert_eq!(mission.owner.pending(mission.ids[0]), Ok(0));
    assert_eq!(dispatch_value(&mut mission, 1), 42);
}

#[test]
fn an_acceptance_ledger_does_not_prove_processing_after_lifecycle_clearing() {
    let mut mission = mission([&OUTPUT; 2], [1; 2], [WorkPolicy::Healthy; 2]);
    start_both(&mut mission);
    let output = message(Topic::Output, 5);
    let report = mission
        .owner
        .publish(&output)
        .expect("two-outcome report fits");
    let mut pending = PendingFanout::from_report(output, mission.ids, &report);
    assert_eq!(pending.accepted, [true; 2]);
    let stopped = mission
        .owner
        .stop(mission.ids[0])
        .expect("caller explicitly stops sink");
    assert_eq!(stopped.discarded_deliveries(), 1);
    assert_eq!(mission.outputs[0].get(), None);
    assert_eq!(
        mission.owner.restart(mission.ids[0]),
        Ok(ApplicationState::Running)
    );
    assert_eq!(pending.retry_one(&mut mission.owner, 0), Ok(None));
    assert_eq!(
        mission.owner.dispatch_one(mission.ids[0]),
        Ok(MessageDispatchOutcome::InboxEmpty)
    );
    assert_eq!(dispatch_value(&mut mission, 1), 5);
}

#[test]
fn repeated_selected_publication_remains_a_new_enqueue_attempt() {
    let ids = identities::<1>();
    let configurations = [ApplicationInboxConfig::new(ids[0], 2, &OUTPUT)];
    let mut bus = MessageBus::new(&configurations).expect("fixed topology is valid");
    let output = message(Topic::Output, 8);
    for _ in 0..2 {
        assert_eq!(
            bus.publish_to(ids[0], &output),
            Ok(DeliveryStatus::Delivered)
        );
    }
    assert_eq!(bus.dequeue(ids[0]), Ok(Some(output)));
    assert_eq!(bus.dequeue(ids[0]), Ok(Some(output)));
    assert_eq!(bus.dequeue(ids[0]), Ok(None));
}
