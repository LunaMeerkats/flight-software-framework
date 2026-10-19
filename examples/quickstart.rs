//! A caller-driven, public-API-only host consumer with one bounded inbox.
//!
//! Each supplied byte slice is one complete command. This example uses no
//! transport, stream framing, automatic dispatch, or physical output delivery.

#![forbid(unsafe_code)]

use std::cell::Cell;
use std::convert::Infallible;
use std::error::Error;
use std::fmt;
use std::io::{self, Write};

use rust_flight_framework::{
    Application, ApplicationId, ApplicationMessageContext, ApplicationState,
    ApplicationWorkContext, DeliveryStatus, Message, MessagingApplication, MessagingRuntime,
    PublishClassification, Runtime, RuntimeInboxConfig,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CommandTopic {
    SetPercent,
}

struct PercentApplication<'a> {
    output: &'a Cell<Option<u8>>,
}

type PercentRuntime<'a> = MessagingRuntime<PercentApplication<'a>, CommandTopic, 1>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CommandError {
    InvalidLength { length: usize },
    InvalidPercent { value: u8 },
    OutputFull { rejected: u8 },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct RegistrationFull {
    capacity: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Publication {
    classification: PublishClassification,
    status: DeliveryStatus,
}

#[derive(Debug, Eq, PartialEq)]
struct QuickstartReport {
    publications: [Publication; 3],
    output: [u8; 2],
    states: [ApplicationState; 5],
}

impl Application for PercentApplication<'_> {
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

impl MessagingApplication<CommandTopic, 1> for PercentApplication<'_> {
    type MessageError = CommandError;

    fn handle_message(
        &mut self,
        message: &Message<CommandTopic, 1>,
        _context: &mut ApplicationMessageContext<'_, CommandTopic, 1>,
    ) -> Result<(), Self::MessageError> {
        let [value] = message.payload() else {
            return Err(CommandError::InvalidLength {
                length: message.payload().len(),
            });
        };
        if *value > 100 {
            return Err(CommandError::InvalidPercent { value: *value });
        }
        if self.output.get().is_some() {
            return Err(CommandError::OutputFull { rejected: *value });
        }
        self.output.set(Some(*value));
        Ok(())
    }
}

impl fmt::Display for CommandError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLength { length } => {
                write!(formatter, "command length {length} must be one byte")
            }
            Self::InvalidPercent { value } => {
                write!(formatter, "percentage {value} exceeds 100")
            }
            Self::OutputFull { rejected } => {
                write!(formatter, "full host output rejected percentage {rejected}")
            }
        }
    }
}

impl Error for CommandError {}

impl fmt::Display for RegistrationFull {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "application capacity {} is full", self.capacity)
    }
}

impl Error for RegistrationFull {}

fn main() -> Result<(), Box<dyn Error>> {
    let report = run()?;
    let mut output = io::stdout().lock();
    writeln!(
        output,
        "Experimental host quickstart; not flight-qualified or safety-certified."
    )?;
    writeln!(
        output,
        "No NASA affiliation, demonstrated TRL, operational suitability, or compatibility."
    )?;
    for (value, publication) in [42, 43, 7].into_iter().zip(report.publications) {
        writeln!(
            output,
            "publish {value}: {:?} / {:?}",
            publication.classification, publication.status
        )?;
    }
    writeln!(output, "explicit dispatch output: {:?}", report.output)?;
    writeln!(output, "lifecycle: {:?}", report.states)?;
    Ok(())
}

fn run() -> Result<QuickstartReport, Box<dyn Error>> {
    let output = Cell::new(None);
    let (mut runtime, application_id) = compose(&output)?;
    let registered = runtime.state(application_id)?;
    let started = runtime.start(application_id)?;
    let accepted = publish(&mut runtime, application_id, 42)?;
    let saturated = publish(&mut runtime, application_id, 43)?;
    runtime.dispatch_one(application_id)?;
    let first = output
        .take()
        .ok_or_else(|| io::Error::other("no first output"))?;
    let stopped = runtime.stop(application_id)?.state();
    let restarted = runtime.restart(application_id)?;
    let after_restart = publish(&mut runtime, application_id, 7)?;
    runtime.dispatch_one(application_id)?;
    let second = output
        .take()
        .ok_or_else(|| io::Error::other("no restarted output"))?;
    let final_state = runtime.stop(application_id)?.state();
    Ok(QuickstartReport {
        publications: [accepted, saturated, after_restart],
        output: [first, second],
        states: [registered, started, stopped, restarted, final_state],
    })
}

fn compose(
    output: &Cell<Option<u8>>,
) -> Result<(PercentRuntime<'_>, ApplicationId), Box<dyn Error>> {
    let mut runtime = Runtime::new(1)?;
    // Setup errors end this demo. Retain the diagnostic kind while dropping
    // the returned application/runtime; a long-lived host may recover them.
    let application_id = runtime
        .register(PercentApplication { output })
        .map_err(|error| RegistrationFull {
            capacity: error.capacity(),
        })?;
    // Inbox positions match registration order; attach before starting.
    let inboxes = [RuntimeInboxConfig::new(1, &[CommandTopic::SetPercent])];
    let runtime = MessagingRuntime::new(runtime, &inboxes).map_err(|error| error.kind())?;
    Ok((runtime, application_id))
}

fn publish(
    runtime: &mut PercentRuntime<'_>,
    application_id: ApplicationId,
    value: u8,
) -> Result<Publication, Box<dyn Error>> {
    let message = Message::try_new(CommandTopic::SetPercent, &[value])?;
    let report = runtime.publish(&message)?;
    let [destination] = report.outcomes() else {
        return Err(io::Error::other("expected exactly one command subscriber").into());
    };
    if destination.application_id() != application_id {
        return Err(io::Error::other("command addressed an unexpected application").into());
    }
    Ok(Publication {
        classification: report.classification(),
        status: destination.status(),
    })
}

#[cfg(test)]
mod tests {
    use rust_flight_framework::{MessageDispatchError, MessageDispatchOutcome};

    use super::*;

    #[test]
    fn scenario_retains_older_input_and_reuses_restarted_inbox() {
        let report = run().unwrap();
        let delivered = Publication {
            classification: PublishClassification::Complete,
            status: DeliveryStatus::Delivered,
        };
        assert_eq!(
            report.publications,
            [
                delivered,
                Publication {
                    classification: PublishClassification::WhollyUndelivered,
                    status: DeliveryStatus::InboxFull,
                },
                delivered,
            ]
        );
        assert_eq!(report.output, [42, 7]);
        assert_eq!(
            report.states,
            [
                ApplicationState::Registered,
                ApplicationState::Running,
                ApplicationState::Stopped,
                ApplicationState::Running,
                ApplicationState::Stopped,
            ]
        );
    }

    #[test]
    fn malformed_commands_fail_before_output_and_leave_terminal_state() {
        let cases: &[(&[u8], CommandError)] = &[
            (&[], CommandError::InvalidLength { length: 0 }),
            (&[101], CommandError::InvalidPercent { value: 101 }),
        ];
        for (payload, expected) in cases {
            let output = Cell::new(None);
            let (mut runtime, application_id) = compose(&output).unwrap();
            runtime.start(application_id).unwrap();
            let message = Message::try_new(CommandTopic::SetPercent, payload).unwrap();
            runtime.publish(&message).unwrap();
            let error = runtime.dispatch_one(application_id).unwrap_err();
            assert_eq!(
                error.operation_error(),
                &MessageDispatchError::Application {
                    application_id,
                    source: *expected,
                }
            );
            assert_eq!(error.discarded_deliveries(), 0);
            assert_eq!(output.get(), None);
            assert_eq!(runtime.state(application_id), Ok(ApplicationState::Failed));
            assert!(runtime.restart(application_id).is_err());
        }
    }

    #[test]
    fn output_saturation_retains_older_value_after_callback_failure() {
        let output = Cell::new(None);
        let (mut runtime, application_id) = compose(&output).unwrap();
        runtime.start(application_id).unwrap();
        publish(&mut runtime, application_id, 42).unwrap();
        assert_eq!(
            runtime.dispatch_one(application_id),
            Ok(MessageDispatchOutcome::Dispatched)
        );
        publish(&mut runtime, application_id, 7).unwrap();
        let error = runtime.dispatch_one(application_id).unwrap_err();
        assert_eq!(
            error.operation_error(),
            &MessageDispatchError::Application {
                application_id,
                source: CommandError::OutputFull { rejected: 7 },
            }
        );
        assert_eq!(error.discarded_deliveries(), 0);
        assert_eq!(runtime.pending(application_id), Ok(0));
        assert_eq!(output.take(), Some(42));
        assert_eq!(output.take(), None);
        assert_eq!(runtime.state(application_id), Ok(ApplicationState::Failed));
        assert!(runtime.dispatch_one(application_id).is_err());
    }
}
