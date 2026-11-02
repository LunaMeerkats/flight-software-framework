//! Public-only session correlation for one application-owned reply.

use std::cell::{Cell, OnceCell};
use std::convert::Infallible;
use std::error::Error;
use std::fmt;
use std::num::NonZeroU8;
use std::rc::Rc;

use rust_flight_framework::{
    Application, ApplicationId, ApplicationMessageContext, ApplicationState,
    ApplicationWorkContext, DeliveryStatus, Message, MessageDispatchError, MessageDispatchOutcome,
    MessagingApplication, MessagingRuntime, PublishToError, Runtime, RuntimeInboxConfig,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct OperationId(NonZeroU8);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct IdExhausted;

impl fmt::Display for IdExhausted {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("session operation IDs are exhausted")
    }
}

impl Error for IdExhausted {}

// The host retains this issuer while any session command or reply survives.
struct OperationIssuer {
    next: Option<NonZeroU8>,
}

impl OperationIssuer {
    fn new() -> Self {
        Self {
            next: NonZeroU8::new(1),
        }
    }

    fn issue(&mut self) -> Result<OperationId, IdExhausted> {
        let id = self.next.ok_or(IdExhausted)?;
        self.next = id.get().checked_add(1).and_then(NonZeroU8::new);
        Ok(OperationId(id))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Topic {
    New,
    Retry,
    Cancel,
    Reply,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ConsumerError {
    InvalidCommand,
    MissingDestinations,
    Routing(PublishToError),
    OutputOccupied,
}

impl fmt::Display for ConsumerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCommand => formatter.write_str("unexpected topic, shape or zero ID"),
            Self::MissingDestinations => formatter.write_str("mission destinations are missing"),
            Self::Routing(error) => error.fmt(formatter),
            Self::OutputOccupied => formatter.write_str("host must drain the output first"),
        }
    }
}

impl Error for ConsumerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Routing(error) => Some(error),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Command {
    New { id: OperationId, value: u8 },
    Retry(OperationId),
    Cancel(OperationId),
}

impl Command {
    fn parse(message: &Message<Topic, 2>) -> Result<Self, ConsumerError> {
        let nonzero = |raw| {
            NonZeroU8::new(raw)
                .map(OperationId)
                .ok_or(ConsumerError::InvalidCommand)
        };
        match (message.topic(), message.payload()) {
            (Topic::New, [raw, value]) => Ok(Self::New {
                id: nonzero(*raw)?,
                value: *value,
            }),
            (Topic::Retry, [raw]) => Ok(Self::Retry(nonzero(*raw)?)),
            (Topic::Cancel, [raw]) => Ok(Self::Cancel(nonzero(*raw)?)),
            _ => Err(ConsumerError::InvalidCommand),
        }
    }

    fn message(self) -> Message<Topic, 2> {
        match self {
            Self::New { id, value } => message(Topic::New, &[id.0.get(), value]),
            Self::Retry(id) => message(Topic::Retry, &[id.0.get()]),
            Self::Cancel(id) => message(Topic::Cancel, &[id.0.get()]),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct PendingOperation {
    id: OperationId,
    reply: Message<Topic, 2>,
    accepted: [bool; 2],
}

type Observation = Rc<Cell<Option<PendingOperation>>>;
type Output = Rc<Cell<Option<(OperationId, u8)>>>;

struct Producer {
    destinations: Rc<OnceCell<[ApplicationId; 2]>>,
    pending: Option<PendingOperation>,
    // This copied test observation never drives command or retry decisions.
    observation: Observation,
}

impl Producer {
    fn handle(
        &mut self,
        input: &Message<Topic, 2>,
        context: &mut ApplicationMessageContext<'_, Topic, 2>,
    ) -> Result<(), ConsumerError> {
        // Validate even busy and idle commands before any state or bus effect.
        match Command::parse(input)? {
            Command::New { id, value } if self.pending.is_none() => {
                self.pending = Some(PendingOperation {
                    id,
                    reply: message(Topic::Reply, &[id.0.get(), value]),
                    accepted: [false; 2],
                });
                self.attempt(context)?;
            }
            Command::Retry(id) if self.matches(id) => self.attempt(context)?,
            Command::Cancel(id) if self.matches(id) => self.pending = None,
            _ => {}
        }
        self.observation.set(self.pending);
        Ok(())
    }

    fn matches(&self, id: OperationId) -> bool {
        self.pending.is_some_and(|operation| operation.id == id)
    }

    fn attempt(
        &mut self,
        context: &mut ApplicationMessageContext<'_, Topic, 2>,
    ) -> Result<(), ConsumerError> {
        let destinations = self
            .destinations
            .get()
            .ok_or(ConsumerError::MissingDestinations)?;
        let Some(operation) = &mut self.pending else {
            return Ok(());
        };
        for (index, destination) in destinations.iter().enumerate() {
            if !operation.accepted[index] {
                let status = context
                    .publish_to(*destination, &operation.reply)
                    .map_err(ConsumerError::Routing)?;
                operation.accepted[index] = status == DeliveryStatus::Delivered;
            }
        }
        if operation.accepted == [true; 2] {
            self.pending = None;
        }
        Ok(())
    }
}

enum Actor {
    Producer(Producer),
    Subscriber(Output),
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

impl MessagingApplication<Topic, 2> for Actor {
    type MessageError = ConsumerError;

    fn handle_message(
        &mut self,
        input: &Message<Topic, 2>,
        context: &mut ApplicationMessageContext<'_, Topic, 2>,
    ) -> Result<(), Self::MessageError> {
        match self {
            Self::Producer(producer) => producer.handle(input, context),
            Self::Subscriber(output) => {
                let (Topic::Reply, [raw, value]) = (input.topic(), input.payload()) else {
                    return Err(ConsumerError::InvalidCommand);
                };
                let id = NonZeroU8::new(*raw).ok_or(ConsumerError::InvalidCommand)?;
                if output.get().is_some() {
                    return Err(ConsumerError::OutputOccupied);
                }
                output.set(Some((OperationId(id), *value)));
                Ok(())
            }
        }
    }
}

type Owner = MessagingRuntime<Actor, Topic, 2>;

struct Mission {
    owner: Owner,
    ids: [ApplicationId; 3],
    issuer: OperationIssuer,
    observation: Observation,
    outputs: [Output; 2],
}

impl Mission {
    fn new() -> Self {
        let destinations = Rc::new(OnceCell::new());
        let observation = Rc::new(Cell::new(None));
        let outputs = std::array::from_fn(|_| Rc::new(Cell::new(None)));
        let actors = [
            Actor::Producer(Producer {
                destinations: Rc::clone(&destinations),
                pending: None,
                observation: Rc::clone(&observation),
            }),
            Actor::Subscriber(Rc::clone(&outputs[0])),
            Actor::Subscriber(Rc::clone(&outputs[1])),
        ];
        let mut runtime = Runtime::new(3).expect("three application slots fit");
        let ids = actors.map(|actor| runtime.register(actor).expect("application slot exists"));
        destinations.set([ids[1], ids[2]]).unwrap();
        let configurations = [
            RuntimeInboxConfig::new(2, &[Topic::New, Topic::Retry, Topic::Cancel]),
            RuntimeInboxConfig::new(2, &[Topic::Reply]),
            RuntimeInboxConfig::new(2, &[Topic::Reply]),
        ];
        let mut owner = MessagingRuntime::new(runtime, &configurations).unwrap();
        for id in ids {
            owner.start(id).unwrap();
        }
        Self {
            owner,
            ids,
            issuer: OperationIssuer::new(),
            observation,
            outputs,
        }
    }

    fn issue(&mut self) -> OperationId {
        self.issuer.issue().expect("test stays within session IDs")
    }

    fn enqueue(&mut self, command: Command) -> DeliveryStatus {
        self.owner
            .publish_to(self.ids[0], &command.message())
            .unwrap()
    }

    fn dispatch(&mut self) {
        assert_eq!(
            self.owner.dispatch_one(self.ids[0]),
            Ok(MessageDispatchOutcome::Dispatched)
        );
    }

    fn invoke(&mut self, command: Command) {
        assert_eq!(self.enqueue(command), DeliveryStatus::Delivered);
        self.dispatch();
    }

    fn drain(&mut self, subscriber: usize) -> (OperationId, u8) {
        assert_eq!(
            self.owner.dispatch_one(self.ids[subscriber + 1]),
            Ok(MessageDispatchOutcome::Dispatched)
        );
        self.outputs[subscriber]
            .take()
            .expect("subscriber staged a reply")
    }

    fn seed_second_inbox(&mut self) -> [(OperationId, u8); 2] {
        let older = [(self.issue(), 3), (self.issue(), 4)];
        for (id, value) in older {
            assert_eq!(
                self.owner
                    .publish_to(self.ids[2], &message(Topic::Reply, &[id.0.get(), value])),
                Ok(DeliveryStatus::Delivered)
            );
        }
        older
    }

    fn assert_pending(&self, id: OperationId, value: u8, accepted: [bool; 2]) {
        assert_eq!(
            self.observation.get(),
            Some(PendingOperation {
                id,
                reply: message(Topic::Reply, &[id.0.get(), value]),
                accepted,
            })
        );
    }

    fn assert_inboxes(&self, expected: [usize; 2]) {
        for (id, count) in self.ids[1..].iter().zip(expected) {
            assert_eq!(self.owner.pending(*id), Ok(count));
        }
    }
}

fn message(topic: Topic, payload: &[u8]) -> Message<Topic, 2> {
    Message::try_new(topic, payload).expect("controlled payload fits two bytes")
}

#[test]
fn delayed_cancel_cannot_erase_newer_operation_and_retry_skips_accepted_peer() {
    let mut mission = Mission::new();
    let first = mission.issue();
    mission.invoke(Command::New {
        id: first,
        value: 40,
    });
    assert_eq!(mission.observation.get(), None);
    assert_eq!(mission.drain(0), (first, 40));
    assert_eq!(mission.drain(1), (first, 40));
    let older = mission.seed_second_inbox();
    let newer = mission.issue();
    assert_eq!(
        mission.enqueue(Command::New {
            id: newer,
            value: 41
        }),
        DeliveryStatus::Delivered
    );
    assert_eq!(
        mission.enqueue(Command::Cancel(first)),
        DeliveryStatus::Delivered
    );
    let retained_cancel = Command::Cancel(newer);
    assert_eq!(mission.enqueue(retained_cancel), DeliveryStatus::InboxFull);
    assert_eq!(mission.observation.get(), None);
    mission.assert_inboxes([0, 2]);
    mission.dispatch();
    mission.assert_pending(newer, 41, [true, false]);
    mission.dispatch();
    mission.assert_pending(newer, 41, [true, false]);
    mission.assert_inboxes([1, 2]);
    assert_eq!(mission.drain(0), (newer, 41));
    assert_eq!(mission.drain(1), older[0]);
    mission.invoke(Command::Retry(first));
    mission.assert_pending(newer, 41, [true, false]);
    mission.assert_inboxes([0, 1]);
    mission.invoke(Command::Retry(newer));
    assert_eq!(mission.observation.get(), None);
    mission.assert_inboxes([0, 2]);
    assert_eq!(mission.drain(1), older[1]);
    assert_eq!(mission.drain(1), (newer, 41));
    mission.invoke(retained_cancel);
    mission.assert_inboxes([0, 0]);
}

#[test]
fn matching_cancellation_preserves_accepted_copy_and_rejected_fifo() {
    let mut mission = Mission::new();
    let older = mission.seed_second_inbox();
    let current = mission.issue();
    let next = mission.issue();
    mission.invoke(Command::New {
        id: current,
        value: 42,
    });
    mission.invoke(Command::Cancel(next));
    mission.assert_pending(current, 42, [true, false]);
    mission.invoke(Command::Cancel(current));
    for _ in 0..2 {
        mission.invoke(Command::Cancel(current));
        mission.invoke(Command::Retry(current));
        assert_eq!(mission.observation.get(), None);
        mission.assert_inboxes([1, 2]);
    }
    assert_eq!(mission.drain(0), (current, 42));
    for record in older {
        assert_eq!(mission.drain(1), record);
    }
    mission.invoke(Command::New {
        id: next,
        value: 43,
    });
    assert_eq!(mission.observation.get(), None);
    assert_eq!(mission.drain(0), (next, 43));
    assert_eq!(mission.drain(1), (next, 43));
}

#[test]
fn busy_new_command_requires_same_id_resubmission_after_pending_completion() {
    let mut mission = Mission::new();
    let older = mission.seed_second_inbox();
    let current = mission.issue();
    let next = mission.issue();
    mission.invoke(Command::New {
        id: current,
        value: 44,
    });
    let retained_request = Command::New {
        id: next,
        value: 45,
    };
    mission.invoke(retained_request);
    mission.invoke(Command::Retry(next));
    mission.invoke(Command::Cancel(next));
    mission.assert_pending(current, 44, [true, false]);
    mission.assert_inboxes([1, 2]);
    assert_eq!(mission.drain(0), (current, 44));
    assert_eq!(mission.drain(1), older[0]);
    mission.invoke(Command::Retry(current));
    assert_eq!(mission.observation.get(), None);
    assert_eq!(mission.drain(1), older[1]);
    assert_eq!(mission.drain(1), (current, 44));
    mission.invoke(retained_request);
    assert_eq!(mission.drain(0), (next, 45));
    assert_eq!(mission.drain(1), (next, 45));
    assert_eq!(mission.issue().0.get(), next.0.get() + 1);
}

#[test]
fn producer_restart_retains_operation_and_host_issuer_but_clears_commands() {
    let mut mission = Mission::new();
    let [producer, _, second] = mission.ids;
    mission.owner.stop(second).unwrap();
    let current = mission.issue();
    mission.invoke(Command::New {
        id: current,
        value: 46,
    });
    assert_eq!(
        mission.enqueue(Command::Cancel(current)),
        DeliveryStatus::Delivered
    );
    assert_eq!(
        mission.enqueue(Command::Retry(current)),
        DeliveryStatus::Delivered
    );
    assert_eq!(
        mission.owner.stop(producer).unwrap().discarded_deliveries(),
        2
    );
    mission.assert_pending(current, 46, [true, false]);
    let newer = mission.issue();
    assert_eq!(newer.0.get(), current.0.get() + 1);
    assert_eq!(
        mission.enqueue(Command::Cancel(current)),
        DeliveryStatus::Unavailable
    );
    mission.owner.restart(producer).unwrap();
    assert_eq!(
        mission.owner.dispatch_one(producer),
        Ok(MessageDispatchOutcome::InboxEmpty)
    );
    mission.invoke(Command::Cancel(newer));
    mission.assert_pending(current, 46, [true, false]);
    mission.owner.restart(second).unwrap();
    mission.invoke(Command::Retry(current));
    assert_eq!(mission.observation.get(), None);
    assert_eq!(mission.drain(0), (current, 46));
    assert_eq!(mission.drain(1), (current, 46));
    mission.assert_inboxes([0, 0]);
    assert_eq!(mission.issue().0.get(), newer.0.get() + 1);
    assert_eq!(mission.owner.state(producer), Ok(ApplicationState::Running));
}

#[test]
fn session_issuer_uses_entire_nonzero_domain_then_repeatedly_rejects_exhaustion() {
    let mut mission = Mission::new();
    mission.owner.stop(mission.ids[2]).unwrap();
    let mut last = None;
    for expected in 1..=u8::MAX {
        let id = mission.issue();
        assert_eq!(id.0.get(), expected);
        last = Some(id);
    }
    let id = last.expect("the nonzero domain is nonempty");
    mission.invoke(Command::New { id, value: 49 });
    mission.assert_pending(id, 49, [true, false]);
    assert_eq!(mission.issuer.next, None);
    for _ in 0..3 {
        assert_eq!(mission.issuer.issue(), Err(IdExhausted));
        assert_eq!(mission.issuer.next, None);
    }
    mission.invoke(Command::Retry(id));
    mission.assert_pending(id, 49, [true, false]);
    mission.assert_inboxes([1, 0]);
    mission.invoke(Command::Cancel(id));
    assert_eq!(mission.observation.get(), None);
    assert_eq!(mission.drain(0), (id, 49));
    assert_eq!(mission.issuer.issue(), Err(IdExhausted));
}

#[test]
fn idle_repeated_and_completed_control_commands_leave_copies_intact() {
    let mut mission = Mission::new();
    let id = mission.issue();
    for _ in 0..2 {
        mission.invoke(Command::Cancel(id));
        mission.invoke(Command::Retry(id));
        assert_eq!(mission.observation.get(), None);
        mission.assert_inboxes([0, 0]);
    }
    mission.invoke(Command::New { id, value: 47 });
    assert_eq!(mission.observation.get(), None);
    for _ in 0..2 {
        mission.invoke(Command::Cancel(id));
        mission.invoke(Command::Retry(id));
        mission.assert_inboxes([1, 1]);
    }
    assert_eq!(mission.drain(0), (id, 47));
    assert_eq!(mission.drain(1), (id, 47));
}

#[test]
fn malformed_commands_are_rejected_before_reply_or_cancellation_effects() {
    let invalid: [(Topic, &[u8]); 10] = [
        (Topic::New, &[]),
        (Topic::New, &[1]),
        (Topic::New, &[0, 8]),
        (Topic::Retry, &[]),
        (Topic::Retry, &[1, 8]),
        (Topic::Retry, &[0]),
        (Topic::Cancel, &[]),
        (Topic::Cancel, &[1, 8]),
        (Topic::Cancel, &[0]),
        (Topic::Reply, &[1, 8]),
    ];
    for (topic, payload) in invalid {
        assert_eq!(
            Command::parse(&message(topic, payload)),
            Err(ConsumerError::InvalidCommand)
        );
        if topic == Topic::Reply {
            continue;
        }
        let mut mission = Mission::new();
        let [producer, _, second] = mission.ids;
        mission.owner.stop(second).unwrap();
        let id = mission.issue();
        mission.invoke(Command::New { id, value: 48 });
        let before = mission.observation.get();
        assert_eq!(
            mission.owner.publish_to(producer, &message(topic, payload)),
            Ok(DeliveryStatus::Delivered)
        );
        assert_eq!(
            mission.enqueue(Command::Cancel(id)),
            DeliveryStatus::Delivered
        );
        let error = mission.owner.dispatch_one(producer).unwrap_err();
        assert_eq!(
            error.operation_error(),
            &MessageDispatchError::Application {
                application_id: producer,
                source: ConsumerError::InvalidCommand,
            }
        );
        assert_eq!(error.discarded_deliveries(), 1);
        assert_eq!(mission.observation.get(), before);
        mission.assert_inboxes([1, 0]);
        assert_eq!(mission.drain(0), (id, 48));
        assert_eq!(mission.owner.state(producer), Ok(ApplicationState::Failed));
        assert_eq!(mission.issue().0.get(), 2);
    }
}
