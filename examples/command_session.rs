//! A finite serial command session composed only from public framework APIs.
//!
//! Host slots gate dispatch, and host code drains them explicitly. Command
//! results describe local effects. Reply output separately observes
//! subscribers.

#![forbid(unsafe_code)]

use std::cell::Cell;
use std::convert::Infallible;
use std::error::Error;
use std::fmt;
use std::io::{self, Cursor, Write};
use std::num::NonZeroU8;

use rust_flight_framework::{
    Application, ApplicationId, ApplicationMessageContext, ApplicationWorkContext, DeliveryStatus,
    Message, MessageCreateError, MessageDispatchError, MessageDispatchOutcome,
    MessagingApplication, MessagingOperationError, MessagingRuntime, PublishToError, Runtime,
    RuntimeInboxConfig,
};

const OUTPUT_LINE_CAPACITY: usize = 128;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct OperationId(NonZeroU8);

struct Issuer(Option<NonZeroU8>);

impl Issuer {
    fn new() -> Self {
        Self(NonZeroU8::new(1))
    }

    fn issue(&mut self) -> Result<OperationId, SessionError> {
        let next = self.0.ok_or(SessionError::IdsExhausted)?;
        self.0 = next.get().checked_add(1).and_then(NonZeroU8::new);
        Ok(OperationId(next))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Topic {
    Command,
    Reply,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SessionError {
    InvalidFrame,
    IdsExhausted,
    ResultOccupied,
    OutputOccupied,
    RegistrationFull {
        capacity: usize,
    },
    MissingSubscription {
        application_id: ApplicationId,
        topic: Topic,
    },
    Message(MessageCreateError),
    Routing(PublishToError),
}

impl fmt::Display for SessionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFrame => formatter.write_str("invalid command or reply frame"),
            Self::IdsExhausted => formatter.write_str("session operation IDs exhausted"),
            Self::ResultOccupied => formatter.write_str("drain the command result before dispatch"),
            Self::OutputOccupied => {
                formatter.write_str("drain the subscriber output before dispatch")
            }
            Self::RegistrationFull { capacity } => {
                write!(formatter, "runtime capacity {capacity} full")
            }
            Self::MissingSubscription {
                application_id,
                topic,
            } => write!(
                formatter,
                "application {application_id:?} must subscribe to {topic:?}"
            ),
            Self::Message(error) => error.fmt(formatter),
            Self::Routing(error) => error.fmt(formatter),
        }
    }
}

impl Error for SessionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Message(error) => Some(error),
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
    // This example grammar is local framing, not a transport protocol.
    fn parse(frame: &[u8]) -> Result<Self, SessionError> {
        let id = |raw| {
            NonZeroU8::new(raw)
                .map(OperationId)
                .ok_or(SessionError::InvalidFrame)
        };
        match frame {
            [1, raw, value] => Ok(Self::New {
                id: id(*raw)?,
                value: *value,
            }),
            [2, raw] => Ok(Self::Retry(id(*raw)?)),
            [3, raw] => Ok(Self::Cancel(id(*raw)?)),
            _ => Err(SessionError::InvalidFrame),
        }
    }

    fn message(self) -> Result<Message<Topic, 3>, MessageCreateError> {
        match self {
            Self::New { id, value } => Message::try_new(Topic::Command, &[1, id.0.get(), value]),
            Self::Retry(id) => Message::try_new(Topic::Command, &[2, id.0.get()]),
            Self::Cancel(id) => Message::try_new(Topic::Command, &[3, id.0.get()]),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Reply {
    id: OperationId,
    value: u8,
}

impl Reply {
    fn parse(input: &Message<Topic, 3>) -> Result<Self, SessionError> {
        let (Topic::Reply, [raw, value]) = (input.topic(), input.payload()) else {
            return Err(SessionError::InvalidFrame);
        };
        let id = NonZeroU8::new(*raw)
            .map(OperationId)
            .ok_or(SessionError::InvalidFrame)?;
        Ok(Self { id, value: *value })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CommandEffect {
    ReplyAccepted([bool; 2]),
    Busy(OperationId),
    Cancelled,
    NoMatch,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct CommandResult {
    command: Command,
    effect: CommandEffect,
}

#[derive(Default)]
struct HostSlots {
    result: Cell<Option<CommandResult>>,
    outputs: [Cell<Option<Reply>>; 2],
}

#[derive(Debug, Eq, PartialEq)]
enum LineWrite {
    Pending,
    Accepted,
}

// Keep the selected slot occupied and the writer exclusive until completion.
struct SlotOutput<'a, T, W> {
    slot: &'a Cell<Option<T>>,
    writer: &'a mut W,
    line: Cursor<[u8; OUTPUT_LINE_CAPACITY]>,
    written: usize,
}

#[derive(Clone, Copy)]
struct Pending {
    reply: Reply,
    accepted: [bool; 2],
}

struct Producer<'a> {
    destinations: [ApplicationId; 2],
    pending: Option<Pending>,
    result: &'a Cell<Option<CommandResult>>,
}

enum Actor<'a> {
    Producer(Producer<'a>),
    Subscriber(&'a Cell<Option<Reply>>),
}

type Owner<'a> = MessagingRuntime<Actor<'a>, Topic, 3>;
type DispatchError = MessagingOperationError<MessageDispatchError<SessionError>>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum HostTurn {
    SlotOccupied,
    Dispatch(MessageDispatchOutcome),
}

#[derive(Clone, Copy)]
enum Subscriber {
    First,
    Second,
}

struct Session<'a> {
    owner: Owner<'a>,
    producer: ApplicationId,
    subscribers: [ApplicationId; 2],
    issuer: Issuer,
    slots: &'a HostSlots,
}

impl Producer<'_> {
    fn handle(
        &mut self,
        input: &Message<Topic, 3>,
        context: &mut ApplicationMessageContext<'_, Topic, 3>,
    ) -> Result<(), SessionError> {
        if self.result.get().is_some() {
            return Err(SessionError::ResultOccupied);
        }
        if input.topic() != &Topic::Command {
            return Err(SessionError::InvalidFrame);
        }
        let command = Command::parse(input.payload())?;
        let effect = match command {
            Command::New { id, value } => match self.pending {
                Some(pending) => CommandEffect::Busy(pending.reply.id),
                None => {
                    self.pending = Some(Pending {
                        reply: Reply { id, value },
                        accepted: [false; 2],
                    });
                    self.attempt(context)?
                }
            },
            Command::Retry(id) if self.matches(id) => self.attempt(context)?,
            Command::Cancel(id) if self.matches(id) => {
                self.pending = None;
                CommandEffect::Cancelled
            }
            _ => CommandEffect::NoMatch,
        };
        self.result.set(Some(CommandResult { command, effect }));
        Ok(())
    }

    fn matches(&self, id: OperationId) -> bool {
        self.pending.is_some_and(|pending| pending.reply.id == id)
    }

    fn attempt(
        &mut self,
        context: &mut ApplicationMessageContext<'_, Topic, 3>,
    ) -> Result<CommandEffect, SessionError> {
        let Some(pending) = &mut self.pending else {
            return Ok(CommandEffect::NoMatch);
        };
        let reply = Message::try_new(
            Topic::Reply,
            &[pending.reply.id.0.get(), pending.reply.value],
        )
        .map_err(SessionError::Message)?;
        for (index, destination) in self.destinations.iter().enumerate() {
            if !pending.accepted[index] {
                pending.accepted[index] = context
                    .publish_to(*destination, &reply)
                    .map_err(SessionError::Routing)?
                    == DeliveryStatus::Delivered;
            }
        }
        let accepted = pending.accepted;
        if accepted == [true; 2] {
            self.pending = None;
        }
        Ok(CommandEffect::ReplyAccepted(accepted))
    }
}

impl Application for Actor<'_> {
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

impl MessagingApplication<Topic, 3> for Actor<'_> {
    type MessageError = SessionError;

    fn handle_message(
        &mut self,
        input: &Message<Topic, 3>,
        context: &mut ApplicationMessageContext<'_, Topic, 3>,
    ) -> Result<(), Self::MessageError> {
        match self {
            Self::Producer(producer) => producer.handle(input, context),
            Self::Subscriber(output) => {
                let reply = Reply::parse(input)?;
                if output.get().is_some() {
                    return Err(SessionError::OutputOccupied);
                }
                output.set(Some(reply));
                Ok(())
            }
        }
    }
}

impl<'a> Session<'a> {
    fn new(slots: &'a HostSlots) -> Result<Self, Box<dyn Error>> {
        let mut runtime = Runtime::new(3)?;
        // This one-shot demo drops rejected setup ownership after retaining
        // its diagnostic. A recovering host can keep the original wrapper.
        let register = |runtime: &mut Runtime<Actor<'a>>, actor| {
            runtime
                .register(actor)
                .map_err(|error| SessionError::RegistrationFull {
                    capacity: error.capacity(),
                })
        };
        let subscribers = [
            register(&mut runtime, Actor::Subscriber(&slots.outputs[0]))?,
            register(&mut runtime, Actor::Subscriber(&slots.outputs[1]))?,
        ];
        let producer = register(
            &mut runtime,
            Actor::Producer(Producer {
                destinations: subscribers,
                pending: None,
                result: &slots.result,
            }),
        )?;
        let inboxes = [
            RuntimeInboxConfig::new(1, &[Topic::Reply]),
            RuntimeInboxConfig::new(1, &[Topic::Reply]),
            RuntimeInboxConfig::new(2, &[Topic::Command]),
        ];
        let mut owner = MessagingRuntime::new(runtime, &inboxes).map_err(|error| error.kind())?;
        start_session(&mut owner, producer, subscribers)?;
        Ok(Self {
            owner,
            producer,
            subscribers,
            issuer: Issuer::new(),
            slots,
        })
    }

    fn new_command(&mut self, value: u8) -> Result<Command, SessionError> {
        Ok(Command::New {
            id: self.issuer.issue()?,
            value,
        })
    }

    fn ingress(&mut self, frame: &[u8]) -> Result<DeliveryStatus, SessionError> {
        Command::parse(frame)?;
        let input = Message::try_new(Topic::Command, frame).map_err(SessionError::Message)?;
        self.owner
            .publish_to(self.producer, &input)
            .map_err(SessionError::Routing)
    }

    fn enqueue(&mut self, command: Command) -> Result<DeliveryStatus, SessionError> {
        self.ingress(command.message().map_err(SessionError::Message)?.payload())
    }

    fn admit_pending(
        &mut self,
        pending: &mut Option<Command>,
    ) -> Result<Option<DeliveryStatus>, SessionError> {
        let Some(command) = *pending else {
            return Ok(None);
        };
        let admission = self.enqueue(command)?;
        if admission == DeliveryStatus::Delivered {
            *pending = None;
        }
        Ok(Some(admission))
    }

    fn producer_turn(&mut self) -> Result<HostTurn, DispatchError> {
        if self.slots.result.get().is_some() {
            return Ok(HostTurn::SlotOccupied);
        }
        self.owner
            .dispatch_one(self.producer)
            .map(HostTurn::Dispatch)
    }

    fn subscriber_turn(&mut self, subscriber: Subscriber) -> Result<HostTurn, DispatchError> {
        let index = subscriber as usize;
        if self.slots.outputs[index].get().is_some() {
            return Ok(HostTurn::SlotOccupied);
        }
        self.owner
            .dispatch_one(self.subscribers[index])
            .map(HostTurn::Dispatch)
    }
}

impl<'a, T: Copy + fmt::Debug, W: Write> SlotOutput<'a, T, W> {
    fn new(
        slot: &'a Cell<Option<T>>,
        writer: &'a mut W,
        label: impl fmt::Display,
    ) -> io::Result<Self> {
        let record = slot
            .get()
            .ok_or_else(|| io::Error::other(format!("missing {label} output")))?;
        let mut line = Cursor::new([0; OUTPUT_LINE_CAPACITY]);
        writeln!(line, "{label}: {record:?}")?;
        Ok(Self {
            slot,
            writer,
            line,
            written: 0,
        })
    }

    fn write_turn(&mut self) -> io::Result<LineWrite> {
        let length = self.line.position() as usize;
        if self.written == length {
            return Ok(LineWrite::Accepted);
        }
        let remainder = &self.line.get_ref()[self.written..length];
        let accepted = self.writer.write(remainder)?;
        if accepted == 0 {
            return Err(io::ErrorKind::WriteZero.into());
        }
        if accepted > remainder.len() {
            return Err(io::ErrorKind::InvalidData.into());
        }
        self.written += accepted;
        if self.written < length {
            return Ok(LineWrite::Pending);
        }

        self.slot.set(None);
        Ok(LineWrite::Accepted)
    }

    fn finish(mut self) -> io::Result<()> {
        // Each pending turn accepts at least one byte of the fixed-size line.
        while self.write_turn()? == LineWrite::Pending {}
        Ok(())
    }
}

fn start_session(
    owner: &mut Owner<'_>,
    producer: ApplicationId,
    [first, second]: [ApplicationId; 2],
) -> Result<(), Box<dyn Error>> {
    // A structurally valid positional topology can still bind the wrong role.
    for (application_id, topic) in [
        (producer, Topic::Command),
        (first, Topic::Reply),
        (second, Topic::Reply),
    ] {
        if !owner.subscribes_to(application_id, &topic)? {
            return Err(SessionError::MissingSubscription {
                application_id,
                topic,
            }
            .into());
        }
    }
    for application_id in [first, second, producer] {
        owner.start(application_id)?;
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    run_session(&mut io::stdout().lock())
}

fn run_session(output: &mut impl Write) -> Result<(), Box<dyn Error>> {
    writeln!(
        output,
        "Experimental host session; not flight-qualified or safety-certified."
    )?;
    writeln!(
        output,
        "No NASA affiliation, demonstrated TRL, operational suitability or compatibility."
    )?;
    let slots = HostSlots::default();
    let mut session = Session::new(&slots)?;
    let older = session.new_command(3)?;
    send(&mut session, older, output)?;
    observe_reply(&mut session, Subscriber::First, output)?;
    let active = session.new_command(40)?;
    admit(&mut session, active, output)?;
    writeln!(output, "producer turn: {:?}", session.producer_turn()?)?;
    let busy = session.new_command(7)?;
    admit(&mut session, busy, output)?;
    writeln!(output, "producer gate: {:?}", session.producer_turn()?)?;
    observe_result(&slots, output)?;
    writeln!(output, "producer turn: {:?}", session.producer_turn()?)?;
    observe_result(&slots, output)?;
    observe_reply(&mut session, Subscriber::First, output)?;
    observe_reply(&mut session, Subscriber::Second, output)?;
    let Command::New { id, .. } = active else {
        return Err(SessionError::InvalidFrame.into());
    };
    send(&mut session, Command::Retry(id), output)?;
    observe_reply(&mut session, Subscriber::Second, output)?;
    // Busy did not allocate another ID; the host resubmits its saved request.
    send(&mut session, busy, output)?;
    observe_reply(&mut session, Subscriber::First, output)?;
    observe_reply(&mut session, Subscriber::Second, output)?;
    recover_ingress(&mut session, id, output)
}

fn recover_ingress(
    session: &mut Session<'_>,
    stale: OperationId,
    output: &mut impl Write,
) -> Result<(), Box<dyn Error>> {
    // Stale controls fill the FIFO without creating more subscriber replies.
    admit(session, Command::Retry(stale), output)?;
    writeln!(output, "producer turn: {:?}", session.producer_turn()?)?;
    admit(session, Command::Cancel(stale), output)?;
    admit(session, Command::Retry(stale), output)?;

    let command = session.new_command(9)?;
    let mut pending = Some(command);
    let Some(status @ DeliveryStatus::InboxFull) = session.admit_pending(&mut pending)? else {
        return Err(io::Error::other("expected full command inbox").into());
    };
    writeln!(output, "retained ingress {command:?}: {status:?}")?;
    writeln!(output, "producer gate: {:?}", session.producer_turn()?)?;
    observe_result(session.slots, output)?;
    writeln!(output, "producer turn: {:?}", session.producer_turn()?)?;

    let Some(status @ DeliveryStatus::Delivered) = session.admit_pending(&mut pending)? else {
        return Err(io::Error::other("expected room for the retained command").into());
    };
    writeln!(output, "retained ingress {command:?}: {status:?}")?;
    observe_result(session.slots, output)?;

    for _ in 0..2 {
        writeln!(output, "producer turn: {:?}", session.producer_turn()?)?;
        observe_result(session.slots, output)?;
    }
    observe_reply(session, Subscriber::First, output)?;
    observe_reply(session, Subscriber::Second, output)?;
    Ok(())
}

fn send(
    session: &mut Session<'_>,
    command: Command,
    output: &mut impl Write,
) -> Result<(), Box<dyn Error>> {
    admit(session, command, output)?;
    writeln!(output, "producer turn: {:?}", session.producer_turn()?)?;
    observe_result(session.slots, output)
}

fn admit(
    session: &mut Session<'_>,
    command: Command,
    output: &mut impl Write,
) -> Result<(), Box<dyn Error>> {
    let admission = session.enqueue(command)?;
    writeln!(output, "ingress {command:?}: {admission:?}")?;
    if admission != DeliveryStatus::Delivered {
        return Err(io::Error::other("scenario must retain and resubmit rejected request").into());
    }
    Ok(())
}

fn observe_result(slots: &HostSlots, output: &mut impl Write) -> Result<(), Box<dyn Error>> {
    SlotOutput::new(&slots.result, output, "result")?.finish()?;
    Ok(())
}

fn observe_reply(
    session: &mut Session<'_>,
    subscriber: Subscriber,
    output: &mut impl Write,
) -> Result<(), Box<dyn Error>> {
    session.subscriber_turn(subscriber)?;
    let index = subscriber as usize;
    SlotOutput::new(
        &session.slots.outputs[index],
        output,
        format_args!("subscriber {index}"),
    )?
    .finish()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use rust_flight_framework::ApplicationState;

    use super::*;

    const FIRST_RESULT_LINE: &str = concat!(
        "result: CommandResult { command: New { id: OperationId(1), value: 3 }, ",
        "effect: ReplyAccepted([true, true]) }"
    );

    #[derive(Default)]
    struct ScriptedWriter {
        calls: usize,
        accepted: Vec<u8>,
    }

    impl Write for ScriptedWriter {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            self.calls += 1;
            let count = match self.calls {
                1 => 7,
                2 => return Err(io::ErrorKind::WouldBlock.into()),
                3 => return Ok(0),
                4 => return Ok(bytes.len() + 1),
                _ => bytes.len(),
            };
            self.accepted.extend_from_slice(&bytes[..count]);
            Ok(count)
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    fn id(command: Command) -> OperationId {
        let Command::New { id, .. } = command else {
            panic!("expected a new operation")
        };
        id
    }

    fn invoke(session: &mut Session<'_>, command: Command) -> CommandEffect {
        assert_eq!(session.enqueue(command), Ok(DeliveryStatus::Delivered));
        assert_eq!(
            session.producer_turn(),
            Ok(HostTurn::Dispatch(MessageDispatchOutcome::Dispatched))
        );
        let result = session.slots.result.take().unwrap();
        assert_eq!(result.command, command);
        result.effect
    }

    fn drain(session: &mut Session<'_>, subscriber: Subscriber) -> Reply {
        assert_eq!(
            session.subscriber_turn(subscriber),
            Ok(HostTurn::Dispatch(MessageDispatchOutcome::Dispatched))
        );
        session.slots.outputs[subscriber as usize].take().unwrap()
    }

    #[test]
    fn finite_session_prints_exact_results_and_separate_subscriber_observations() {
        let mut output = Vec::new();
        run_session(&mut output).unwrap();
        let actual = String::from_utf8(output).unwrap();
        let expected: Vec<_> = expected_transcript()
            .into_iter()
            .chain(expected_ingress_recovery())
            .collect();
        assert_eq!(actual.lines().collect::<Vec<_>>(), expected);
    }

    #[test]
    fn result_write_retry_preserves_the_slot_and_resumes_at_the_accepted_prefix() {
        let slots = HostSlots::default();
        let mut session = Session::new(&slots).unwrap();
        let first = session.new_command(3).unwrap();
        let queued = Command::Retry(id(first));
        assert_eq!(session.enqueue(first), Ok(DeliveryStatus::Delivered));
        let dispatched = HostTurn::Dispatch(MessageDispatchOutcome::Dispatched);
        assert_eq!(session.producer_turn(), Ok(dispatched));
        let first_result = CommandResult {
            command: first,
            effect: CommandEffect::ReplyAccepted([true, true]),
        };
        assert_eq!(slots.result.get(), Some(first_result));
        assert_eq!(session.enqueue(queued), Ok(DeliveryStatus::Delivered));

        let mut writer = ScriptedWriter::default();
        let mut output = SlotOutput::new(&slots.result, &mut writer, "result").unwrap();
        assert_eq!(output.write_turn().unwrap(), LineWrite::Pending);
        assert_eq!(session.producer_turn(), Ok(HostTurn::SlotOccupied));
        for kind in [
            io::ErrorKind::WouldBlock,
            io::ErrorKind::WriteZero,
            io::ErrorKind::InvalidData,
        ] {
            assert_eq!(output.write_turn().unwrap_err().kind(), kind);
            assert_eq!(output.writer.accepted, b"result:");
            assert_eq!(slots.result.get(), Some(first_result));
            assert_eq!(session.producer_turn(), Ok(HostTurn::SlotOccupied));
            assert_eq!(session.owner.pending(session.producer), Ok(1));
        }

        assert_eq!(output.write_turn().unwrap(), LineWrite::Accepted);
        assert_eq!(
            output.writer.accepted,
            format!("{FIRST_RESULT_LINE}\n").as_bytes()
        );
        assert_eq!(slots.result.get(), None);
        assert_eq!(session.producer_turn(), Ok(dispatched));
        assert_eq!(session.owner.pending(session.producer), Ok(0));
        let queued_result = CommandResult {
            command: queued,
            effect: CommandEffect::NoMatch,
        };
        assert_eq!(output.write_turn().unwrap(), LineWrite::Accepted);
        assert_eq!(output.writer.calls, 5);
        assert_eq!(slots.result.get(), Some(queued_result));
    }

    #[test]
    fn subscriber_write_retry_preserves_independent_gates_and_line_order() {
        let slots = HostSlots::default();
        let mut session = Session::new(&slots).unwrap();
        let [first, second] = [Subscriber::First, Subscriber::Second];
        let dispatched = HostTurn::Dispatch(MessageDispatchOutcome::Dispatched);
        let older = session.new_command(3).unwrap();
        let newer = session.new_command(4).unwrap();
        let older_reply = Reply {
            id: id(older),
            value: 3,
        };
        let accepted = CommandEffect::ReplyAccepted([true, true]);
        assert_eq!(invoke(&mut session, older), accepted);
        assert_eq!(session.subscriber_turn(first), Ok(dispatched));

        let mut writer = ScriptedWriter::default();
        let mut output = SlotOutput::new(&slots.outputs[0], &mut writer, "subscriber 0").unwrap();
        assert_eq!(output.write_turn().unwrap(), LineWrite::Pending);
        assert_eq!(
            output.write_turn().unwrap_err().kind(),
            io::ErrorKind::WouldBlock
        );
        assert_eq!(session.subscriber_turn(second), Ok(dispatched));
        assert_eq!(invoke(&mut session, newer), accepted);
        assert_eq!(session.subscriber_turn(first), Ok(HostTurn::SlotOccupied));
        assert_eq!(session.owner.pending(session.subscribers[0]), Ok(1));
        assert_eq!(
            slots.outputs.each_ref().map(Cell::get),
            [Some(older_reply); 2]
        );
        assert_eq!(output.writer.accepted, b"subscri");
        for kind in [io::ErrorKind::WriteZero, io::ErrorKind::InvalidData] {
            assert_eq!(output.write_turn().unwrap_err().kind(), kind);
        }
        assert_eq!(output.write_turn().unwrap(), LineWrite::Accepted);
        assert_eq!(
            slots.outputs.each_ref().map(Cell::get),
            [None, Some(older_reply)]
        );
        assert_eq!(session.subscriber_turn(first), Ok(dispatched));
        assert_eq!(session.subscriber_turn(second), Ok(HostTurn::SlotOccupied));

        observe_reply(&mut session, second, &mut writer).unwrap();
        assert_eq!(
            writer.accepted,
            b"subscriber 0: Reply { id: OperationId(1), value: 3 }\n\
              subscriber 1: Reply { id: OperationId(1), value: 3 }\n"
        );
        assert_eq!(writer.calls, 6);
        assert_eq!(
            slots.outputs[0].get(),
            Some(Reply {
                id: id(newer),
                value: 4
            })
        );
        assert_eq!(slots.outputs[1].get(), None);
        assert_eq!(session.owner.pending(session.subscribers[1]), Ok(1));
    }

    #[test]
    fn misbound_inboxes_are_reported_before_any_application_starts() {
        let command = RuntimeInboxConfig::new(2, &[Topic::Command]);
        let reply = RuntimeInboxConfig::new(1, &[Topic::Reply]);
        assert_setup_rejects_inboxes([command, reply, reply], Topic::Command);
        assert_setup_rejects_inboxes([reply, command, command], Topic::Reply);
    }

    fn assert_setup_rejects_inboxes(inboxes: [RuntimeInboxConfig<'_, Topic>; 3], missing: Topic) {
        let slots = HostSlots::default();
        let mut runtime = Runtime::new(3).unwrap();
        let subscribers = [
            runtime
                .register(Actor::Subscriber(&slots.outputs[0]))
                .unwrap(),
            runtime
                .register(Actor::Subscriber(&slots.outputs[1]))
                .unwrap(),
        ];
        let producer = runtime
            .register(Actor::Producer(Producer {
                destinations: subscribers,
                pending: None,
                result: &slots.result,
            }))
            .unwrap();
        let mut owner = Owner::new(runtime, &inboxes).unwrap();
        let error = start_session(&mut owner, producer, subscribers).unwrap_err();
        let application_id = match missing {
            Topic::Command => producer,
            Topic::Reply => subscribers[1],
        };
        assert_eq!(
            error.downcast_ref::<SessionError>(),
            Some(&SessionError::MissingSubscription {
                application_id,
                topic: missing,
            })
        );
        for application_id in [subscribers[0], subscribers[1], producer] {
            assert_eq!(
                owner.state(application_id),
                Ok(ApplicationState::Registered)
            );
            assert_eq!(owner.pending(application_id), Ok(0));
        }
    }

    fn expected_transcript() -> [&'static str; 24] {
        [
            "Experimental host session; not flight-qualified or safety-certified.",
            "No NASA affiliation, demonstrated TRL, operational suitability or compatibility.",
            "ingress New { id: OperationId(1), value: 3 }: Delivered",
            "producer turn: Dispatch(Dispatched)",
            FIRST_RESULT_LINE,
            "subscriber 0: Reply { id: OperationId(1), value: 3 }",
            "ingress New { id: OperationId(2), value: 40 }: Delivered",
            "producer turn: Dispatch(Dispatched)",
            "ingress New { id: OperationId(3), value: 7 }: Delivered",
            "producer gate: SlotOccupied",
            concat!(
                "result: CommandResult { command: New { id: OperationId(2), value: 40 }, ",
                "effect: ReplyAccepted([true, false]) }"
            ),
            "producer turn: Dispatch(Dispatched)",
            concat!(
                "result: CommandResult { command: New { id: OperationId(3), value: 7 }, ",
                "effect: Busy(OperationId(2)) }"
            ),
            "subscriber 0: Reply { id: OperationId(2), value: 40 }",
            "subscriber 1: Reply { id: OperationId(1), value: 3 }",
            "ingress Retry(OperationId(2)): Delivered",
            "producer turn: Dispatch(Dispatched)",
            concat!(
                "result: CommandResult { command: Retry(OperationId(2)), ",
                "effect: ReplyAccepted([true, true]) }"
            ),
            "subscriber 1: Reply { id: OperationId(2), value: 40 }",
            "ingress New { id: OperationId(3), value: 7 }: Delivered",
            "producer turn: Dispatch(Dispatched)",
            concat!(
                "result: CommandResult { command: New { id: OperationId(3), value: 7 }, ",
                "effect: ReplyAccepted([true, true]) }"
            ),
            "subscriber 0: Reply { id: OperationId(3), value: 7 }",
            "subscriber 1: Reply { id: OperationId(3), value: 7 }",
        ]
    }

    fn expected_ingress_recovery() -> [&'static str; 16] {
        [
            "ingress Retry(OperationId(2)): Delivered",
            "producer turn: Dispatch(Dispatched)",
            "ingress Cancel(OperationId(2)): Delivered",
            "ingress Retry(OperationId(2)): Delivered",
            "retained ingress New { id: OperationId(4), value: 9 }: InboxFull",
            "producer gate: SlotOccupied",
            "result: CommandResult { command: Retry(OperationId(2)), effect: NoMatch }",
            "producer turn: Dispatch(Dispatched)",
            "retained ingress New { id: OperationId(4), value: 9 }: Delivered",
            "result: CommandResult { command: Cancel(OperationId(2)), effect: NoMatch }",
            "producer turn: Dispatch(Dispatched)",
            "result: CommandResult { command: Retry(OperationId(2)), effect: NoMatch }",
            "producer turn: Dispatch(Dispatched)",
            concat!(
                "result: CommandResult { command: New { id: OperationId(4), value: 9 }, ",
                "effect: ReplyAccepted([true, true]) }"
            ),
            "subscriber 0: Reply { id: OperationId(4), value: 9 }",
            "subscriber 1: Reply { id: OperationId(4), value: 9 }",
        ]
    }

    #[test]
    fn pending_ingress_survives_stop_and_clears_only_after_admission() {
        let slots = HostSlots::default();
        let mut session = Session::new(&slots).unwrap();
        let command = session.new_command(9).unwrap();
        let mut pending = Some(command);

        session.owner.stop(session.producer).unwrap();
        assert_eq!(
            session.admit_pending(&mut pending),
            Ok(Some(DeliveryStatus::Unavailable))
        );
        assert_eq!(pending, Some(command));
        assert_eq!(session.owner.pending(session.producer), Ok(0));

        session.owner.restart(session.producer).unwrap();
        assert_eq!(
            session.admit_pending(&mut pending),
            Ok(Some(DeliveryStatus::Delivered))
        );
        assert_eq!(pending, None);
        assert_eq!(session.admit_pending(&mut pending), Ok(None));
        assert_eq!(session.owner.pending(session.producer), Ok(1));
        assert_eq!(slots.result.get(), None);
        assert_eq!(
            session.producer_turn(),
            Ok(HostTurn::Dispatch(MessageDispatchOutcome::Dispatched))
        );
        assert_eq!(slots.result.take().unwrap().command, command);
        assert_eq!(session.owner.pending(session.producer), Ok(0));
    }

    #[test]
    fn occupied_result_preserves_command_fifo_and_rejected_host_request() {
        let slots = HostSlots::default();
        let mut session = Session::new(&slots).unwrap();
        let first = session.new_command(10).unwrap();
        let saved = session.new_command(20).unwrap();
        assert_eq!(session.enqueue(first), Ok(DeliveryStatus::Delivered));
        session.producer_turn().unwrap();
        let result = slots.result.get();
        let cancel = Command::Cancel(id(first));
        let retry = Command::Retry(id(first));
        assert_eq!(session.enqueue(cancel), Ok(DeliveryStatus::Delivered));
        assert_eq!(session.enqueue(retry), Ok(DeliveryStatus::Delivered));
        assert_eq!(session.enqueue(saved), Ok(DeliveryStatus::InboxFull));
        assert_eq!(session.producer_turn(), Ok(HostTurn::SlotOccupied));
        assert_eq!(slots.result.take(), result);
        assert_eq!(session.owner.pending(session.producer), Ok(2));
        for command in [cancel, retry] {
            session.producer_turn().unwrap();
            assert_eq!(
                slots.result.take(),
                Some(CommandResult {
                    command,
                    effect: CommandEffect::NoMatch
                })
            );
        }
        assert_eq!(session.enqueue(saved), Ok(DeliveryStatus::Delivered));
        session.producer_turn().unwrap();
        assert_eq!(
            slots.result.take(),
            Some(CommandResult {
                command: saved,
                effect: CommandEffect::ReplyAccepted([false; 2]),
            })
        );
        assert_eq!(
            session.owner.state(session.producer),
            Ok(ApplicationState::Running)
        );
    }

    #[test]
    fn occupied_output_preserves_older_reply_and_gates_next_subscriber_dispatch() {
        let slots = HostSlots::default();
        let mut session = Session::new(&slots).unwrap();
        let first = session.new_command(10).unwrap();
        assert_eq!(
            invoke(&mut session, first),
            CommandEffect::ReplyAccepted([true; 2])
        );
        session.subscriber_turn(Subscriber::First).unwrap();
        let second = session.new_command(20).unwrap();
        assert_eq!(
            invoke(&mut session, second),
            CommandEffect::ReplyAccepted([true, false])
        );
        for _ in 0..2 {
            assert_eq!(
                session.subscriber_turn(Subscriber::First),
                Ok(HostTurn::SlotOccupied)
            );
            assert_eq!(session.owner.pending(session.subscribers[0]), Ok(1));
        }
        assert_eq!(
            slots.outputs[0].take(),
            Some(Reply {
                id: id(first),
                value: 10
            })
        );
        assert_eq!(
            drain(&mut session, Subscriber::First),
            Reply {
                id: id(second),
                value: 20
            }
        );
        assert_eq!(
            drain(&mut session, Subscriber::Second),
            Reply {
                id: id(first),
                value: 10
            }
        );
        assert_eq!(
            invoke(&mut session, Command::Retry(id(second))),
            CommandEffect::ReplyAccepted([true; 2])
        );
        assert_eq!(session.owner.pending(session.subscribers[0]), Ok(0));
        assert_eq!(
            drain(&mut session, Subscriber::Second),
            Reply {
                id: id(second),
                value: 20
            }
        );
    }

    #[test]
    fn malformed_ingress_is_rejected_before_queue_or_result_effects() {
        let slots = HostSlots::default();
        let mut session = Session::new(&slots).unwrap();
        let valid = session.new_command(5).unwrap();
        session.enqueue(valid).unwrap();
        for frame in [
            &[][..],
            &[1, 0, 5],
            &[1, 1],
            &[2, 0],
            &[3, 1, 0],
            &[4, 1],
            &[1, 1, 2, 3],
        ] {
            assert_eq!(session.ingress(frame), Err(SessionError::InvalidFrame));
            assert_eq!(session.owner.pending(session.producer), Ok(1));
            assert_eq!(slots.result.get(), None);
            assert_eq!(
                session.owner.state(session.producer),
                Ok(ApplicationState::Running)
            );
        }
        session.producer_turn().unwrap();
        assert_eq!(slots.result.take().unwrap().command, valid);
    }

    #[test]
    fn malformed_callback_bypass_retains_peer_effect_and_reports_exact_cleanup() {
        let slots = HostSlots::default();
        let mut session = Session::new(&slots).unwrap();
        let accepted = session.new_command(5).unwrap();
        invoke(&mut session, accepted);
        let malformed = Message::try_new(Topic::Command, &[2, 0]).unwrap();
        session
            .owner
            .publish_to(session.producer, &malformed)
            .unwrap();
        session.enqueue(Command::Cancel(id(accepted))).unwrap();
        let error = session.producer_turn().unwrap_err();
        assert_eq!(
            error.operation_error(),
            &MessageDispatchError::Application {
                application_id: session.producer,
                source: SessionError::InvalidFrame,
            }
        );
        assert_eq!(error.discarded_deliveries(), 1);
        assert_eq!(slots.result.get(), None);
        assert_eq!(session.owner.pending(session.producer), Ok(0));
        assert_eq!(
            session.owner.state(session.producer),
            Ok(ApplicationState::Failed)
        );
        assert_eq!(
            drain(&mut session, Subscriber::Second),
            Reply {
                id: id(accepted),
                value: 5
            }
        );
    }

    #[test]
    fn direct_producer_bypass_preserves_result_before_terminal_cleanup() {
        let slots = HostSlots::default();
        let mut session = Session::new(&slots).unwrap();
        let first = session.new_command(5).unwrap();
        session.enqueue(first).unwrap();
        session.producer_turn().unwrap();
        let result = slots.result.get();
        let second = session.new_command(7).unwrap();
        session.enqueue(second).unwrap();
        session.enqueue(Command::Cancel(id(second))).unwrap();
        let error = session.owner.dispatch_one(session.producer).unwrap_err();
        assert_eq!(
            error.operation_error(),
            &MessageDispatchError::Application {
                application_id: session.producer,
                source: SessionError::ResultOccupied,
            }
        );
        assert_eq!(error.discarded_deliveries(), 1);
        assert_eq!(slots.result.get(), result);
        assert_eq!(
            session.owner.state(session.producer),
            Ok(ApplicationState::Failed)
        );
        for subscriber in [Subscriber::First, Subscriber::Second] {
            assert_eq!(
                drain(&mut session, subscriber),
                Reply {
                    id: id(first),
                    value: 5
                }
            );
        }
    }

    #[test]
    fn direct_subscriber_bypass_preserves_old_output_and_peer_reply() {
        let slots = HostSlots::default();
        let mut session = Session::new(&slots).unwrap();
        let first = session.new_command(5).unwrap();
        invoke(&mut session, first);
        session.subscriber_turn(Subscriber::First).unwrap();
        let second = session.new_command(7).unwrap();
        invoke(&mut session, second);
        let error = session
            .owner
            .dispatch_one(session.subscribers[0])
            .unwrap_err();
        assert_eq!(
            error.operation_error(),
            &MessageDispatchError::Application {
                application_id: session.subscribers[0],
                source: SessionError::OutputOccupied,
            }
        );
        assert_eq!(error.discarded_deliveries(), 0);
        assert_eq!(
            session.owner.state(session.subscribers[0]),
            Ok(ApplicationState::Failed)
        );
        assert_eq!(
            slots.outputs[0].take(),
            Some(Reply {
                id: id(first),
                value: 5
            })
        );
        assert_eq!(
            drain(&mut session, Subscriber::Second),
            Reply {
                id: id(first),
                value: 5
            }
        );
    }

    #[test]
    fn matching_cancel_preserves_accepted_copy_and_stale_control_preserves_new_operation() {
        let slots = HostSlots::default();
        let mut session = Session::new(&slots).unwrap();
        let older = session.new_command(3).unwrap();
        invoke(&mut session, older);
        drain(&mut session, Subscriber::First);
        let cancelled = session.new_command(40).unwrap();
        assert_eq!(
            invoke(&mut session, cancelled),
            CommandEffect::ReplyAccepted([true, false])
        );
        assert_eq!(
            invoke(&mut session, Command::Cancel(id(cancelled))),
            CommandEffect::Cancelled
        );
        let newer = session.new_command(7).unwrap();
        assert_eq!(
            invoke(&mut session, newer),
            CommandEffect::ReplyAccepted([false; 2])
        );
        for command in [
            Command::Cancel(id(cancelled)),
            Command::Retry(id(cancelled)),
        ] {
            assert_eq!(invoke(&mut session, command), CommandEffect::NoMatch);
        }
        assert_eq!(
            drain(&mut session, Subscriber::First),
            Reply {
                id: id(cancelled),
                value: 40
            }
        );
        assert_eq!(
            drain(&mut session, Subscriber::Second),
            Reply {
                id: id(older),
                value: 3
            }
        );
        assert_eq!(
            invoke(&mut session, Command::Retry(id(newer))),
            CommandEffect::ReplyAccepted([true; 2])
        );
        for subscriber in [Subscriber::First, Subscriber::Second] {
            assert_eq!(
                drain(&mut session, subscriber),
                Reply {
                    id: id(newer),
                    value: 7
                }
            );
        }
    }

    #[test]
    fn restart_retains_result_operation_and_issuer_while_clearing_queued_command() {
        let slots = HostSlots::default();
        let mut session = Session::new(&slots).unwrap();
        let older = session.new_command(3).unwrap();
        invoke(&mut session, older);
        drain(&mut session, Subscriber::First);
        let pending = session.new_command(40).unwrap();
        session.enqueue(pending).unwrap();
        session.producer_turn().unwrap();
        session.enqueue(Command::Cancel(id(pending))).unwrap();
        let result = slots.result.get();
        assert_eq!(
            session
                .owner
                .stop(session.producer)
                .unwrap()
                .discarded_deliveries(),
            1
        );
        session.owner.restart(session.producer).unwrap();
        assert_eq!(session.producer_turn(), Ok(HostTurn::SlotOccupied));
        assert_eq!(slots.result.take(), result);
        assert_eq!(
            session.producer_turn(),
            Ok(HostTurn::Dispatch(MessageDispatchOutcome::InboxEmpty))
        );
        assert_eq!(slots.result.get(), None);
        assert_eq!(id(session.new_command(7).unwrap()).0.get(), 3);
        drain(&mut session, Subscriber::Second);
        assert_eq!(
            invoke(&mut session, Command::Retry(id(pending))),
            CommandEffect::ReplyAccepted([true; 2])
        );
        assert_eq!(
            drain(&mut session, Subscriber::Second),
            Reply {
                id: id(pending),
                value: 40
            }
        );
        assert_eq!(
            drain(&mut session, Subscriber::First),
            Reply {
                id: id(pending),
                value: 40
            }
        );
    }

    #[test]
    fn session_issuer_uses_all_nonzero_ids_and_rejects_repeated_exhaustion() {
        let mut issuer = Issuer::new();
        for expected in 1..=u8::MAX {
            assert_eq!(issuer.issue().unwrap().0.get(), expected);
        }
        for _ in 0..2 {
            assert_eq!(issuer.issue(), Err(SessionError::IdsExhausted));
        }
    }
}
