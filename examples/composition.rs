//! Two independent application types composed through public framework APIs.
//!
//! A mission enum stores both concrete application types inline. The host
//! selects every callback and drains one bounded output slot outside callbacks.

#![forbid(unsafe_code)]

use std::cell::Cell;
use std::convert::Infallible;
use std::error::Error;
use std::fmt;
use std::io::{self, Write};

use rust_flight_framework::{
    Application, ApplicationId, ApplicationMessageContext, ApplicationWorkContext, DeliveryStatus,
    Message, MessageCreateError, MessagingApplication, MessagingRuntime, PublishClassification,
    PublishError, PublishReport, Runtime, RuntimeInboxConfig,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Topic {
    Command,
    Output,
}

// These host observations describe callbacks rather than authoritative state.
// A returned callback failure does not call stop. Consult runtime.state for
// the authoritative application state.
#[derive(Default)]
struct ApplicationStatus {
    online: Cell<bool>,
    work_seen: Cell<bool>,
}

struct Relay<'a> {
    status: &'a ApplicationStatus,
}

struct Sink<'a> {
    status: &'a ApplicationStatus,
    output: &'a Cell<Option<u8>>,
}

enum MissionApplication<'a> {
    Relay(Relay<'a>),
    Sink(Sink<'a>),
}

type MissionRuntime<'a> = MessagingRuntime<MissionApplication<'a>, Topic, 1>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum InputError {
    InvalidLength { length: usize },
    InvalidPercent { value: u8 },
}

#[derive(Debug, Eq, PartialEq)]
enum RelayError {
    Input(InputError),
    Message(MessageCreateError),
    Publish(PublishError),
    IncompleteDelivery(PublishReport),
}

#[derive(Debug, Eq, PartialEq)]
enum SinkError {
    Input(InputError),
    OutputFull { rejected: u8 },
}

#[derive(Debug, Eq, PartialEq)]
enum MissionError {
    Relay(RelayError),
    Sink(SinkError),
}

impl Application for Relay<'_> {
    type StartError = Infallible;
    type WorkError = Infallible;
    type StopError = Infallible;
    type RestartError = Infallible;

    fn start(&mut self) -> Result<(), Self::StartError> {
        self.status.online.set(true);
        Ok(())
    }

    fn work(&mut self, _context: ApplicationWorkContext<'_>) -> Result<(), Self::WorkError> {
        self.status.work_seen.set(true);
        Ok(())
    }

    fn stop(&mut self) -> Result<(), Self::StopError> {
        self.status.online.set(false);
        Ok(())
    }

    fn restart(&mut self) -> Result<(), Self::RestartError> {
        self.status.online.set(true);
        Ok(())
    }
}

impl MessagingApplication<Topic, 1> for Relay<'_> {
    type MessageError = RelayError;

    fn handle_message(
        &mut self,
        message: &Message<Topic, 1>,
        context: &mut ApplicationMessageContext<'_, Topic, 1>,
    ) -> Result<(), Self::MessageError> {
        let value = percent(message).map_err(RelayError::Input)?;
        let output = Message::try_new(Topic::Output, &[value]).map_err(RelayError::Message)?;
        let report = context.publish(&output).map_err(RelayError::Publish)?;
        if report.classification() != PublishClassification::Complete {
            return Err(RelayError::IncompleteDelivery(report));
        }
        Ok(())
    }
}

impl Application for Sink<'_> {
    type StartError = Infallible;
    type WorkError = Infallible;
    type StopError = Infallible;
    type RestartError = Infallible;

    fn start(&mut self) -> Result<(), Self::StartError> {
        self.status.online.set(true);
        Ok(())
    }

    fn work(&mut self, _context: ApplicationWorkContext<'_>) -> Result<(), Self::WorkError> {
        self.status.work_seen.set(true);
        Ok(())
    }

    fn stop(&mut self) -> Result<(), Self::StopError> {
        self.status.online.set(false);
        Ok(())
    }

    fn restart(&mut self) -> Result<(), Self::RestartError> {
        // The borrowed host slot and work observation survive restart.
        self.status.online.set(true);
        Ok(())
    }
}

impl MessagingApplication<Topic, 1> for Sink<'_> {
    type MessageError = SinkError;

    fn handle_message(
        &mut self,
        message: &Message<Topic, 1>,
        _context: &mut ApplicationMessageContext<'_, Topic, 1>,
    ) -> Result<(), Self::MessageError> {
        let value = percent(message).map_err(SinkError::Input)?;
        if self.output.get().is_some() {
            return Err(SinkError::OutputFull { rejected: value });
        }
        self.output.set(Some(value));
        Ok(())
    }
}

impl Application for MissionApplication<'_> {
    type StartError = Infallible;
    type WorkError = Infallible;
    type StopError = Infallible;
    type RestartError = Infallible;

    fn start(&mut self) -> Result<(), Self::StartError> {
        match self {
            Self::Relay(application) => application.start(),
            Self::Sink(application) => application.start(),
        }
    }

    fn work(&mut self, context: ApplicationWorkContext<'_>) -> Result<(), Self::WorkError> {
        match self {
            Self::Relay(application) => application.work(context),
            Self::Sink(application) => application.work(context),
        }
    }

    fn stop(&mut self) -> Result<(), Self::StopError> {
        match self {
            Self::Relay(application) => application.stop(),
            Self::Sink(application) => application.stop(),
        }
    }

    fn restart(&mut self) -> Result<(), Self::RestartError> {
        match self {
            Self::Relay(application) => application.restart(),
            Self::Sink(application) => application.restart(),
        }
    }
}

impl MessagingApplication<Topic, 1> for MissionApplication<'_> {
    type MessageError = MissionError;

    fn handle_message(
        &mut self,
        message: &Message<Topic, 1>,
        context: &mut ApplicationMessageContext<'_, Topic, 1>,
    ) -> Result<(), Self::MessageError> {
        match self {
            Self::Relay(application) => application
                .handle_message(message, context)
                .map_err(MissionError::Relay),
            Self::Sink(application) => application
                .handle_message(message, context)
                .map_err(MissionError::Sink),
        }
    }
}

impl fmt::Display for InputError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLength { length } => write!(formatter, "expected one byte, got {length}"),
            Self::InvalidPercent { value } => write!(formatter, "percentage {value} exceeds 100"),
        }
    }
}

impl Error for InputError {}

impl fmt::Display for RelayError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Input(_) => formatter.write_str("relay rejected input"),
            Self::Message(_) => formatter.write_str("relay output construction failed"),
            Self::Publish(_) => formatter.write_str("relay publication failed"),
            Self::IncompleteDelivery(report) => {
                write!(formatter, "incomplete delivery: {report:?}")
            }
        }
    }
}

impl Error for RelayError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Input(error) => Some(error),
            Self::Message(error) => Some(error),
            Self::Publish(error) => Some(error),
            Self::IncompleteDelivery(_) => None,
        }
    }
}

impl fmt::Display for SinkError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Input(_) => formatter.write_str("sink rejected input"),
            Self::OutputFull { rejected } => write!(formatter, "full output rejected {rejected}"),
        }
    }
}

impl Error for SinkError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Input(error) => Some(error),
            Self::OutputFull { .. } => None,
        }
    }
}

impl fmt::Display for MissionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Relay(_) => formatter.write_str("relay callback failed"),
            Self::Sink(_) => formatter.write_str("sink callback failed"),
        }
    }
}

impl Error for MissionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Relay(error) => Some(error),
            Self::Sink(error) => Some(error),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let values = run()?;
    let mut output = io::stdout().lock();
    writeln!(
        output,
        "Experimental host composition; not flight-qualified or safety-certified."
    )?;
    writeln!(
        output,
        "No NASA affiliation, demonstrated TRL, operational suitability, or compatibility."
    )?;
    writeln!(output, "relay -> sink -> host output: {values:?}")?;
    Ok(())
}

fn run() -> Result<[u8; 2], Box<dyn Error>> {
    let statuses = [ApplicationStatus::default(), ApplicationStatus::default()];
    let output = Cell::new(None);
    let (mut runtime, ids) = compose(&statuses, &output)?;
    for id in ids {
        runtime.start(id)?;
        runtime.work(id)?;
    }
    publish(&mut runtime, ids[0], Topic::Command, 42)?;
    runtime.dispatch_one(ids[0])?;
    runtime.dispatch_one(ids[1])?;
    let first = output
        .take()
        .ok_or_else(|| io::Error::other("missing first output"))?;
    for id in ids {
        runtime.stop(id)?;
        runtime.restart(id)?;
        runtime.work(id)?;
    }
    publish(&mut runtime, ids[0], Topic::Command, 7)?;
    runtime.dispatch_one(ids[0])?;
    runtime.dispatch_one(ids[1])?;
    let second = output
        .take()
        .ok_or_else(|| io::Error::other("missing restarted output"))?;
    for id in ids {
        runtime.stop(id)?;
    }
    Ok([first, second])
}

fn compose<'a>(
    statuses: &'a [ApplicationStatus; 2],
    output: &'a Cell<Option<u8>>,
) -> Result<(MissionRuntime<'a>, [ApplicationId; 2]), Box<dyn Error>> {
    let mut runtime = Runtime::new(2)?;
    let mut register = |application| {
        // Setup ends on error; drop the returned value rather than boxing a
        // wrapper containing an application that borrows caller-owned state.
        runtime
            .register(application)
            .map_err(|error| io::Error::other(format!("capacity {} full", error.capacity())))
    };
    let ids = [
        register(MissionApplication::Relay(Relay {
            status: &statuses[0],
        }))?,
        register(MissionApplication::Sink(Sink {
            status: &statuses[1],
            output,
        }))?,
    ];
    let inboxes = [
        RuntimeInboxConfig::new(2, &[Topic::Command]),
        RuntimeInboxConfig::new(1, &[Topic::Output]),
    ];
    let runtime = MessagingRuntime::new(runtime, &inboxes).map_err(|error| error.kind())?;
    Ok((runtime, ids))
}

fn publish(
    runtime: &mut MissionRuntime<'_>,
    destination: ApplicationId,
    topic: Topic,
    value: u8,
) -> Result<(), Box<dyn Error>> {
    let message = Message::try_new(topic, &[value])?;
    let report = runtime.publish(&message)?;
    let [outcome] = report.outcomes() else {
        return Err(io::Error::other("expected exactly one subscriber").into());
    };
    if report.classification() != PublishClassification::Complete
        || outcome.application_id() != destination
        || outcome.status() != DeliveryStatus::Delivered
    {
        return Err(io::Error::other(format!("unexpected publication: {report:?}")).into());
    }
    Ok(())
}

fn percent(message: &Message<Topic, 1>) -> Result<u8, InputError> {
    let [value] = message.payload() else {
        return Err(InputError::InvalidLength {
            length: message.payload().len(),
        });
    };
    if *value > 100 {
        return Err(InputError::InvalidPercent { value: *value });
    }
    Ok(*value)
}

#[cfg(test)]
mod tests {
    use rust_flight_framework::{ApplicationState, MessageDispatchError};

    use super::*;

    #[test]
    fn distinct_applications_forward_lifecycle_work_and_messages() {
        let statuses = [ApplicationStatus::default(), ApplicationStatus::default()];
        let output = Cell::new(None);
        let (mut runtime, ids) = compose(&statuses, &output).unwrap();
        for (id, status) in ids.into_iter().zip(&statuses) {
            runtime.start(id).unwrap();
            assert!(status.online.get());
            assert!(!status.work_seen.get());
            runtime.work(id).unwrap();
            assert!(status.work_seen.get());
        }
        publish(&mut runtime, ids[0], Topic::Command, 42).unwrap();
        runtime.dispatch_one(ids[0]).unwrap();
        runtime.dispatch_one(ids[1]).unwrap();
        for (id, status) in ids.into_iter().zip(&statuses) {
            runtime.stop(id).unwrap();
            assert!(!status.online.get());
            runtime.restart(id).unwrap();
            assert!(status.online.get());
            assert!(status.work_seen.get());
            assert_eq!(runtime.state(id), Ok(ApplicationState::Running));
        }
        assert_eq!(output.take(), Some(42));
        assert_eq!(run().unwrap(), [42, 7]);
    }

    #[test]
    fn relay_failure_preserves_typed_source_and_peer_delivery() {
        let statuses = [ApplicationStatus::default(), ApplicationStatus::default()];
        let output = Cell::new(None);
        let (mut runtime, ids) = compose(&statuses, &output).unwrap();
        for id in ids {
            runtime.start(id).unwrap();
        }
        publish(&mut runtime, ids[1], Topic::Output, 9).unwrap();
        publish(&mut runtime, ids[0], Topic::Command, 42).unwrap();
        publish(&mut runtime, ids[0], Topic::Command, 7).unwrap();
        let failure = runtime.dispatch_one(ids[0]).unwrap_err();
        let operation = failure
            .source()
            .unwrap()
            .downcast_ref::<MessageDispatchError<MissionError>>()
            .unwrap();
        assert_eq!(operation, failure.operation_error());
        let MessageDispatchError::Application {
            application_id,
            source,
        } = operation
        else {
            panic!("expected relay callback failure");
        };
        assert_eq!(*application_id, ids[0]);
        let mission = operation
            .source()
            .unwrap()
            .downcast_ref::<MissionError>()
            .unwrap();
        assert_eq!(mission, source);
        let relay = mission
            .source()
            .unwrap()
            .downcast_ref::<RelayError>()
            .unwrap();
        let RelayError::IncompleteDelivery(report) = relay else {
            panic!("expected full peer");
        };
        assert_eq!(
            report.classification(),
            PublishClassification::WhollyUndelivered
        );
        assert_eq!(report.outcomes().len(), 1);
        assert_eq!(report.outcomes()[0].application_id(), ids[1]);
        assert_eq!(report.outcomes()[0].status(), DeliveryStatus::InboxFull);
        assert_eq!(failure.discarded_deliveries(), 1);
        assert_eq!(runtime.pending(ids[0]), Ok(0));
        assert_eq!(runtime.state(ids[0]), Ok(ApplicationState::Failed));
        assert_eq!(runtime.state(ids[1]), Ok(ApplicationState::Running));
        assert_eq!(runtime.pending(ids[1]), Ok(1));
        runtime.dispatch_one(ids[1]).unwrap();
        assert_eq!(output.take(), Some(9));
    }

    #[test]
    fn sink_failure_preserves_older_output_and_leaves_relay_operable() {
        let statuses = [ApplicationStatus::default(), ApplicationStatus::default()];
        let output = Cell::new(Some(9));
        let (mut runtime, ids) = compose(&statuses, &output).unwrap();
        for id in ids {
            runtime.start(id).unwrap();
        }
        publish(&mut runtime, ids[0], Topic::Command, 42).unwrap();
        runtime.dispatch_one(ids[0]).unwrap();
        let failure = runtime.dispatch_one(ids[1]).unwrap_err();
        let operation = failure
            .source()
            .unwrap()
            .downcast_ref::<MessageDispatchError<MissionError>>()
            .unwrap();
        assert_eq!(operation, failure.operation_error());
        let MessageDispatchError::Application {
            application_id,
            source,
        } = operation
        else {
            panic!("expected sink callback failure");
        };
        assert_eq!(*application_id, ids[1]);
        let mission = operation
            .source()
            .unwrap()
            .downcast_ref::<MissionError>()
            .unwrap();
        assert_eq!(mission, source);
        assert_eq!(
            mission.source().unwrap().downcast_ref::<SinkError>(),
            Some(&SinkError::OutputFull { rejected: 42 })
        );
        assert_eq!(failure.discarded_deliveries(), 0);
        assert_eq!(runtime.state(ids[1]), Ok(ApplicationState::Failed));
        assert_eq!(runtime.pending(ids[1]), Ok(0));
        assert_eq!(output.take(), Some(9));
        assert_eq!(runtime.state(ids[0]), Ok(ApplicationState::Running));
        assert_eq!(runtime.work(ids[0]), Ok(ApplicationState::Running));
        assert!(statuses[0].work_seen.get());
        runtime.stop(ids[0]).unwrap();
        runtime.restart(ids[0]).unwrap();
    }

    #[test]
    fn both_applications_reject_invalid_input_before_output() {
        let inputs: &[(&[u8], InputError)] = &[
            (&[], InputError::InvalidLength { length: 0 }),
            (&[101], InputError::InvalidPercent { value: 101 }),
        ];
        for (topic, selected, peer) in [(Topic::Command, 0, 1), (Topic::Output, 1, 0)] {
            for (payload, expected) in inputs {
                let statuses = [ApplicationStatus::default(), ApplicationStatus::default()];
                let output = Cell::new(None);
                let (mut runtime, ids) = compose(&statuses, &output).unwrap();
                for id in ids {
                    runtime.start(id).unwrap();
                }
                let message = Message::try_new(topic, payload).unwrap();
                let report = runtime.publish(&message).unwrap();
                assert_eq!(report.classification(), PublishClassification::Complete);
                assert_eq!(report.outcomes().len(), 1);
                assert_eq!(report.outcomes()[0].application_id(), ids[selected]);
                let failure = runtime.dispatch_one(ids[selected]).unwrap_err();
                let source = match topic {
                    Topic::Command => MissionError::Relay(RelayError::Input(*expected)),
                    Topic::Output => MissionError::Sink(SinkError::Input(*expected)),
                };
                assert_eq!(
                    failure.operation_error(),
                    &MessageDispatchError::Application {
                        application_id: ids[selected],
                        source,
                    }
                );
                assert_eq!(failure.discarded_deliveries(), 0);
                assert_eq!(runtime.pending(ids[selected]), Ok(0));
                assert_eq!(runtime.pending(ids[peer]), Ok(0));
                assert_eq!(runtime.state(ids[selected]), Ok(ApplicationState::Failed));
                assert_eq!(runtime.state(ids[peer]), Ok(ApplicationState::Running));
                assert_eq!(output.get(), None);
            }
        }
    }
}
