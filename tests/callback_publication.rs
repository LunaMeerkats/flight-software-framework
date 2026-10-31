//! Public-only callback-owned reply retry and selected-publication evidence.

use std::cell::{Cell, OnceCell};
use std::convert::Infallible;
use std::error::Error;
use std::fmt;
use std::rc::Rc;

use rust_flight_framework::{
    Application, ApplicationId, ApplicationMessageContext, ApplicationState,
    ApplicationWorkContext, DeliveryStatus, LifecycleRegistry, Message, MessageDispatchError,
    MessageDispatchOutcome, MessagingApplication, MessagingRuntime, PublishError, PublishToError,
    Runtime, RuntimeInboxConfig,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Topic {
    NewReply,
    Retry,
    Probe,
    Reply,
    Other,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CallbackError {
    MissingSetup,
    InvalidInput,
    OutputOccupied,
    Broadcast(PublishError),
    Selected(PublishToError),
    Requested,
}

impl fmt::Display for CallbackError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingSetup => formatter.write_str("mission destinations were not configured"),
            Self::InvalidInput => formatter.write_str("unexpected topic or payload"),
            Self::OutputOccupied => formatter.write_str("host has not drained the output"),
            Self::Broadcast(error) => error.fmt(formatter),
            Self::Selected(error) => error.fmt(formatter),
            Self::Requested => formatter.write_str("requested callback failure"),
        }
    }
}

impl Error for CallbackError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Broadcast(error) => Some(error),
            Self::Selected(error) => Some(error),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct PendingReply {
    message: Message<Topic, 1>,
    accepted: [bool; 2],
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct ReplyObservation {
    pending: Option<PendingReply>,
    deferred: Option<u8>,
}

#[derive(Debug)]
struct ReplyProducer {
    // Mission setup fixes these IDs once, before the producer starts.
    destinations: Rc<OnceCell<[ApplicationId; 2]>>,
    pending: Option<PendingReply>,
    // Test-only copied observations never drive transfer or retry decisions.
    observation: Rc<Cell<ReplyObservation>>,
}

impl ReplyProducer {
    fn handle(
        &mut self,
        input: &Message<Topic, 1>,
        context: &mut ApplicationMessageContext<'_, Topic, 1>,
    ) -> Result<(), CallbackError> {
        let ids = self.destinations.get().ok_or(CallbackError::MissingSetup)?;
        let mut deferred = None;
        match input.topic() {
            Topic::NewReply => {
                let [value] = input.payload() else {
                    return Err(CallbackError::InvalidInput);
                };
                if self.pending.is_some() {
                    deferred = Some(*value);
                } else {
                    let message = message(Topic::Reply, *value);
                    let report = context
                        .publish(&message)
                        .map_err(CallbackError::Broadcast)?;
                    // Mission construction fixes exactly these two subscribers.
                    let accepted = std::array::from_fn(|index| {
                        report.outcomes().iter().any(|outcome| {
                            outcome.application_id() == ids[index]
                                && outcome.status() == DeliveryStatus::Delivered
                        })
                    });
                    self.pending = Some(PendingReply { message, accepted });
                }
            }
            Topic::Retry => {
                if let Some(reply) = &mut self.pending {
                    for (index, id) in ids.iter().enumerate() {
                        if !reply.accepted[index] {
                            let status = context
                                .publish_to(*id, &reply.message)
                                .map_err(CallbackError::Selected)?;
                            reply.accepted[index] = status == DeliveryStatus::Delivered;
                        }
                    }
                }
            }
            _ => return Err(CallbackError::InvalidInput),
        }
        self.observation.set(ReplyObservation {
            pending: self.pending,
            deferred,
        });
        if self
            .pending
            .is_some_and(|reply| reply.accepted == [true; 2])
        {
            self.pending = None;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
enum ProbeAction {
    Selected { target: ApplicationId, topic: Topic },
    SelfTwice,
    SelfPeerThenFail { peer: ApplicationId },
    ReturnRoutingError { target: ApplicationId },
}

type Attempt = Result<DeliveryStatus, PublishToError>;
type ProbeResults = [Option<Attempt>; 2];
type Output = Rc<Cell<Option<u8>>>;

#[derive(Debug)]
struct Probe {
    action: Rc<Cell<Option<ProbeAction>>>,
    results: Rc<Cell<ProbeResults>>,
}

impl Probe {
    fn handle(
        &self,
        context: &mut ApplicationMessageContext<'_, Topic, 1>,
    ) -> Result<(), CallbackError> {
        let action = self.action.get().ok_or(CallbackError::MissingSetup)?;
        let reply = message(Topic::Reply, 42);
        let mut results = [None; 2];
        match action {
            ProbeAction::Selected { target, topic } => {
                results[0] = Some(context.publish_to(target, &message(topic, 42)));
            }
            ProbeAction::SelfTwice => {
                for result in &mut results {
                    *result = Some(context.publish_to(context.application_id(), &reply));
                }
            }
            ProbeAction::SelfPeerThenFail { peer } => {
                results[0] = Some(context.publish_to(context.application_id(), &reply));
                results[1] = Some(context.publish_to(peer, &reply));
                self.results.set(results);
                return Err(CallbackError::Requested);
            }
            ProbeAction::ReturnRoutingError { target } => {
                return context
                    .publish_to(target, &reply)
                    .map(|_| ())
                    .map_err(CallbackError::Selected);
            }
        }
        self.results.set(results);
        Ok(())
    }
}

#[derive(Debug)]
enum Actor {
    Producer(ReplyProducer),
    Probe(Probe),
    Sink(Output),
}

impl Application for Actor {
    type StartError = Infallible;
    type WorkError = Infallible;
    type StopError = Infallible;
    type RestartError = Infallible;

    fn start(&mut self) -> Result<(), Self::StartError> {
        Ok(())
    }

    fn work(&mut self, _context: ApplicationWorkContext<'_>) -> Result<(), Self::WorkError> {
        Ok(())
    }

    fn stop(&mut self) -> Result<(), Self::StopError> {
        Ok(())
    }

    fn restart(&mut self) -> Result<(), Self::RestartError> {
        Ok(())
    }
}

impl MessagingApplication<Topic, 1> for Actor {
    type MessageError = CallbackError;

    fn handle_message(
        &mut self,
        input: &Message<Topic, 1>,
        context: &mut ApplicationMessageContext<'_, Topic, 1>,
    ) -> Result<(), Self::MessageError> {
        match self {
            Self::Producer(producer) => producer.handle(input, context),
            Self::Probe(probe) if input.topic() == &Topic::Probe => probe.handle(context),
            Self::Probe(_) => Ok(()),
            Self::Sink(output) => {
                let [value] = input.payload() else {
                    return Err(CallbackError::InvalidInput);
                };
                if output.get().is_some() {
                    return Err(CallbackError::OutputOccupied);
                }
                output.set(Some(*value));
                Ok(())
            }
        }
    }
}

type Owner = MessagingRuntime<Actor, Topic, 1>;

fn message(topic: Topic, value: u8) -> Message<Topic, 1> {
    Message::try_new(topic, &[value]).expect("one byte fits")
}

fn configured(
    actors: [Actor; 3],
    capacities: [usize; 3],
    topics: [&[Topic]; 3],
) -> (Owner, [ApplicationId; 3]) {
    let mut runtime = Runtime::new(3).expect("three records fit");
    let ids = actors.map(|actor| runtime.register(actor).expect("actor fits"));
    let configurations = std::array::from_fn::<_, 3, _>(|index| {
        RuntimeInboxConfig::new(capacities[index], topics[index])
    });
    let owner = MessagingRuntime::new(runtime, &configurations).expect("valid topology");
    (owner, ids)
}

fn sink() -> (Actor, Output) {
    let output = Rc::new(Cell::new(None));
    (Actor::Sink(Rc::clone(&output)), output)
}

fn invoke(owner: &mut Owner, id: ApplicationId, topic: Topic, value: u8) {
    assert_eq!(
        owner.publish_to(id, &message(topic, value)),
        Ok(DeliveryStatus::Delivered)
    );
    assert_eq!(
        owner.dispatch_one(id),
        Ok(MessageDispatchOutcome::Dispatched)
    );
}

fn drain(owner: &mut Owner, id: ApplicationId, output: &Output) -> u8 {
    assert_eq!(
        owner.dispatch_one(id),
        Ok(MessageDispatchOutcome::Dispatched)
    );
    output.take().expect("callback staged one byte")
}

struct ReplyMission {
    owner: Owner,
    ids: [ApplicationId; 3],
    outputs: [Output; 2],
    observation: Rc<Cell<ReplyObservation>>,
}

fn reply_mission() -> ReplyMission {
    let destinations = Rc::new(OnceCell::new());
    let observation = Rc::new(Cell::new(ReplyObservation::default()));
    let producer = Actor::Producer(ReplyProducer {
        destinations: Rc::clone(&destinations),
        pending: None,
        observation: Rc::clone(&observation),
    });
    let (first, first_output) = sink();
    let (second, second_output) = sink();
    let (mut owner, ids) = configured(
        [producer, first, second],
        [1, 2, 2],
        [
            &[Topic::NewReply, Topic::Retry],
            &[Topic::Reply],
            &[Topic::Reply],
        ],
    );
    destinations
        .set([ids[1], ids[2]])
        .expect("configure once before start");
    for id in ids {
        owner.start(id).unwrap();
    }
    ReplyMission {
        owner,
        ids,
        outputs: [first_output, second_output],
        observation,
    }
}

fn assert_reply(mission: &ReplyMission, value: u8, accepted: [bool; 2]) {
    assert_eq!(
        mission.observation.get().pending,
        Some(PendingReply {
            message: message(Topic::Reply, value),
            accepted
        })
    );
}

#[test]
fn callback_retry_preserves_fifo_and_never_repeats_accepted_peer() {
    let mut mission = reply_mission();
    let [producer, first, second] = mission.ids;
    for value in [3, 4] {
        mission
            .owner
            .publish_to(second, &message(Topic::Reply, value))
            .unwrap();
    }
    invoke(&mut mission.owner, producer, Topic::NewReply, 42);
    assert_reply(&mission, 42, [true, false]);
    assert_eq!(drain(&mut mission.owner, first, &mission.outputs[0]), 42);
    for _ in 0..2 {
        invoke(&mut mission.owner, producer, Topic::Retry, 0);
        assert_reply(&mission, 42, [true, false]);
        assert_eq!(mission.owner.pending(first), Ok(0));
    }
    invoke(&mut mission.owner, producer, Topic::NewReply, 43);
    assert_eq!(mission.observation.get().deferred, Some(43));
    assert_reply(&mission, 42, [true, false]);
    assert_eq!(drain(&mut mission.owner, second, &mission.outputs[1]), 3);
    invoke(&mut mission.owner, producer, Topic::Retry, 0);
    assert_reply(&mission, 42, [true; 2]);
    assert_eq!(mission.owner.pending(first), Ok(0));
    for value in [4, 42] {
        assert_eq!(
            drain(&mut mission.owner, second, &mission.outputs[1]),
            value
        );
    }
    invoke(&mut mission.owner, producer, Topic::Retry, 0);
    assert_eq!(mission.observation.get().pending, None);
    assert_eq!(mission.owner.pending(first), Ok(0));
    assert_eq!(mission.owner.pending(second), Ok(0));
    invoke(&mut mission.owner, producer, Topic::NewReply, 43);
    assert_reply(&mission, 43, [true; 2]);
    assert_eq!(drain(&mut mission.owner, first, &mission.outputs[0]), 43);
    assert_eq!(drain(&mut mission.owner, second, &mission.outputs[1]), 43);
    assert_eq!(mission.owner.state(producer), Ok(ApplicationState::Running));
}

#[test]
fn callback_retry_retains_reply_while_stopped_and_refreshes_after_restart() {
    let mut mission = reply_mission();
    let [producer, first, second] = mission.ids;
    mission.owner.stop(second).unwrap();
    invoke(&mut mission.owner, producer, Topic::NewReply, 42);
    assert_reply(&mission, 42, [true, false]);
    for _ in 0..2 {
        invoke(&mut mission.owner, producer, Topic::Retry, 0);
        assert_reply(&mission, 42, [true, false]);
    }
    assert_eq!(drain(&mut mission.owner, first, &mission.outputs[0]), 42);
    mission.owner.restart(second).unwrap();
    invoke(&mut mission.owner, producer, Topic::Retry, 0);
    assert_reply(&mission, 42, [true; 2]);
    assert_eq!(mission.owner.pending(first), Ok(0));
    assert_eq!(drain(&mut mission.owner, second, &mission.outputs[1]), 42);
}

struct ProbeMission {
    owner: Owner,
    ids: [ApplicationId; 3],
    action: Rc<Cell<Option<ProbeAction>>>,
    results: Rc<Cell<ProbeResults>>,
    output: Output,
    other_output: Output,
}

fn probe_mission(source_capacity: usize) -> ProbeMission {
    let action = Rc::new(Cell::new(None));
    let results = Rc::new(Cell::new([None; 2]));
    let probe = Actor::Probe(Probe {
        action: Rc::clone(&action),
        results: Rc::clone(&results),
    });
    let (peer, output) = sink();
    let (other, other_output) = sink();
    let (mut owner, ids) = configured(
        [probe, peer, other],
        [source_capacity, 1, 1],
        [
            &[Topic::Probe, Topic::Reply],
            &[Topic::Reply],
            &[Topic::Other],
        ],
    );
    owner.start(ids[0]).unwrap();
    ProbeMission {
        owner,
        ids,
        action,
        results,
        output,
        other_output,
    }
}

fn attempt(mission: &mut ProbeMission, target: ApplicationId, topic: Topic) -> Attempt {
    mission
        .action
        .set(Some(ProbeAction::Selected { target, topic }));
    invoke(&mut mission.owner, mission.ids[0], Topic::Probe, 0);
    mission.results.get()[0].expect("one selected attempt")
}

#[test]
fn callback_selection_refreshes_registered_running_stopped_and_failed_states() {
    let mut mission = probe_mission(1);
    let peer = mission.ids[1];
    assert_eq!(
        attempt(&mut mission, peer, Topic::Reply),
        Ok(DeliveryStatus::Unavailable)
    );
    mission.owner.start(peer).unwrap();
    assert_eq!(
        attempt(&mut mission, peer, Topic::Reply),
        Ok(DeliveryStatus::Delivered)
    );
    assert_eq!(
        attempt(&mut mission, peer, Topic::Reply),
        Ok(DeliveryStatus::InboxFull)
    );
    assert_eq!(mission.owner.stop(peer).unwrap().discarded_deliveries(), 1);
    assert_eq!(
        attempt(&mut mission, peer, Topic::Reply),
        Ok(DeliveryStatus::Unavailable)
    );
    mission.owner.restart(peer).unwrap();
    assert_eq!(
        attempt(&mut mission, peer, Topic::Reply),
        Ok(DeliveryStatus::Delivered)
    );
    assert_eq!(
        mission.owner.dispatch_one(peer),
        Ok(MessageDispatchOutcome::Dispatched)
    );
    assert_eq!(mission.output.get(), Some(42));
    assert_eq!(
        attempt(&mut mission, peer, Topic::Reply),
        Ok(DeliveryStatus::Delivered)
    );
    let error = mission
        .owner
        .dispatch_one(peer)
        .expect_err("occupied output fails sink");
    assert_eq!(
        error.operation_error(),
        &MessageDispatchError::Application {
            application_id: peer,
            source: CallbackError::OutputOccupied,
        }
    );
    assert_eq!(mission.owner.state(peer), Ok(ApplicationState::Failed));
    assert_eq!(
        attempt(&mut mission, peer, Topic::Reply),
        Ok(DeliveryStatus::Unavailable)
    );
    assert_eq!(mission.owner.pending(mission.ids[2]), Ok(0));
}

#[test]
fn callback_routing_errors_precede_availability_and_preserve_every_inbox() {
    let mut mission = probe_mission(1);
    let [source, peer, other] = mission.ids;
    let mut registry = LifecycleRegistry::new(4).unwrap();
    let unknown = (0..4).map(|_| registry.register().unwrap()).last().unwrap();
    assert_eq!(
        attempt(&mut mission, unknown, Topic::Reply),
        Err(PublishToError::UnknownApplication {
            application_id: unknown
        })
    );
    assert_eq!(
        attempt(&mut mission, other, Topic::Reply),
        Err(PublishToError::NotSubscribed {
            application_id: other
        })
    );
    mission.owner.start(other).unwrap();
    mission
        .owner
        .publish_to(other, &message(Topic::Other, 7))
        .unwrap();
    assert_eq!(
        attempt(&mut mission, other, Topic::Reply),
        Err(PublishToError::NotSubscribed {
            application_id: other
        })
    );
    assert_eq!(mission.owner.pending(source), Ok(0));
    assert_eq!(mission.owner.pending(peer), Ok(0));
    assert_eq!(mission.owner.pending(other), Ok(1));
    assert_eq!(drain(&mut mission.owner, other, &mission.other_output), 7);
}

#[test]
fn returned_routing_error_retains_its_concrete_source_chain() {
    let mut mission = probe_mission(1);
    let [source, _, other] = mission.ids;
    mission.owner.start(other).unwrap();
    mission
        .owner
        .publish_to(other, &message(Topic::Other, 7))
        .unwrap();
    mission
        .action
        .set(Some(ProbeAction::ReturnRoutingError { target: other }));
    mission
        .owner
        .publish_to(source, &message(Topic::Probe, 0))
        .unwrap();
    let error = mission
        .owner
        .dispatch_one(source)
        .expect_err("typed route error propagates");
    let route = PublishToError::NotSubscribed {
        application_id: other,
    };
    assert_eq!(
        error.operation_error(),
        &MessageDispatchError::Application {
            application_id: source,
            source: CallbackError::Selected(route),
        }
    );
    assert_eq!(error.discarded_deliveries(), 0);
    let callback = error.operation_error().source().unwrap();
    assert_eq!(
        callback.downcast_ref::<CallbackError>(),
        Some(&CallbackError::Selected(route))
    );
    assert_eq!(
        callback.source().unwrap().downcast_ref::<PublishToError>(),
        Some(&route)
    );
    assert_eq!(mission.owner.pending(other), Ok(1));
    assert_eq!(drain(&mut mission.owner, other, &mission.other_output), 7);
}

#[test]
fn capacity_one_callback_can_self_publish_after_input_is_removed() {
    let mut mission = probe_mission(1);
    let source = mission.ids[0];
    mission.action.set(Some(ProbeAction::SelfTwice));
    invoke(&mut mission.owner, source, Topic::Probe, 0);
    assert_eq!(
        mission.results.get(),
        [
            Some(Ok(DeliveryStatus::Delivered)),
            Some(Ok(DeliveryStatus::InboxFull))
        ]
    );
    assert_eq!(mission.owner.pending(source), Ok(1));
    assert_eq!(mission.owner.pending(mission.ids[1]), Ok(0));
    assert_eq!(
        mission.owner.dispatch_one(source),
        Ok(MessageDispatchOutcome::Dispatched)
    );
    assert_eq!(mission.owner.pending(source), Ok(0));
}

#[test]
fn returned_callback_error_clears_self_copies_and_retains_accepted_peer_copy() {
    let mut mission = probe_mission(2);
    let [source, peer, other] = mission.ids;
    mission.owner.start(peer).unwrap();
    mission
        .action
        .set(Some(ProbeAction::SelfPeerThenFail { peer }));
    mission
        .owner
        .publish_to(source, &message(Topic::Probe, 0))
        .unwrap();
    mission
        .owner
        .publish_to(source, &message(Topic::Reply, 9))
        .unwrap();
    let error = mission
        .owner
        .dispatch_one(source)
        .expect_err("cooperative failure requested");
    assert_eq!(
        mission.results.get(),
        [Some(Ok(DeliveryStatus::Delivered)); 2]
    );
    assert_eq!(
        error.operation_error(),
        &MessageDispatchError::Application {
            application_id: source,
            source: CallbackError::Requested,
        }
    );
    assert_eq!(error.discarded_deliveries(), 2);
    assert_eq!(mission.owner.state(source), Ok(ApplicationState::Failed));
    assert_eq!(mission.owner.pending(source), Ok(0));
    assert_eq!(mission.owner.pending(other), Ok(0));
    assert_eq!(drain(&mut mission.owner, peer, &mission.output), 42);
    assert_eq!(mission.owner.state(peer), Ok(ApplicationState::Running));
}
