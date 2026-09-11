//! Independently defined echo processing and bounded host-output capture.
//!
//! The mailbox outlives its borrowing application. Serial deposit preserves
//! older output on full; a returned error uses the runtime's terminal policy.
//! Only the host drains or performs physical I/O, outside message callbacks.

use std::cell::Cell;
use std::convert::Infallible;
use std::error::Error;
use std::fmt;

use rust_flight_framework::{
    Application, ApplicationMessageContext, ApplicationWorkContext, Message, MessageCreateError,
    MessagingApplication, PublishClassification, PublishError, PublishReport,
};

use super::MissionTopic;
use super::codec::{ValidatedPercent, ValidationError, decode_internal};
use super::work::{MissionRole, MissionWorkError, WorkMonitor, perform_work};

#[derive(Default)]
pub(crate) struct OutputMailbox {
    record: Cell<Option<ValidatedPercent>>,
}

pub(crate) struct EchoApplication<'a> {
    work_monitor: Option<&'a WorkMonitor>,
}

pub(crate) struct TelemetryApplication<'a> {
    mailbox: &'a OutputMailbox,
    work_monitor: Option<&'a WorkMonitor>,
}

pub(crate) enum MissionApplication<'a> {
    Echo(EchoApplication<'a>),
    Telemetry(TelemetryApplication<'a>),
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum MissionMessageError {
    Validation(ValidationError),
    Message(MessageCreateError),
    Publish(PublishError),
    TelemetryDelivery(PublishReport),
    OutputFull { rejected: u8 },
}

impl OutputMailbox {
    /// Consumes at most one record and encodes exact host bytes; empty is None.
    /// The slot can be reused while the borrowing runtime remains alive.
    pub(crate) fn drain(&self) -> Option<[u8; 2]> {
        self.record.take().map(ValidatedPercent::encode_telemetry)
    }

    fn deposit(&self, value: ValidatedPercent) -> Result<(), MissionMessageError> {
        if self.record.get().is_some() {
            return Err(MissionMessageError::OutputFull {
                rejected: value.value(),
            });
        }
        self.record.set(Some(value));
        Ok(())
    }
}

impl<'a> EchoApplication<'a> {
    pub(crate) const fn new(work_monitor: Option<&'a WorkMonitor>) -> Self {
        Self { work_monitor }
    }

    fn handle_message(
        &mut self,
        message: &Message<MissionTopic, 1>,
        context: &mut ApplicationMessageContext<'_, MissionTopic, 1>,
    ) -> Result<(), MissionMessageError> {
        let command = decode_internal(message, MissionTopic::EchoCommand)
            .map_err(MissionMessageError::Validation)?;
        let response = echo_percent(command);
        let telemetry = Message::try_new(MissionTopic::EchoTelemetry, &[response.value()])
            .map_err(MissionMessageError::Message)?;
        let report = context
            .publish(&telemetry)
            .map_err(MissionMessageError::Publish)?;
        if report.classification() != PublishClassification::Complete {
            return Err(MissionMessageError::TelemetryDelivery(report));
        }
        Ok(())
    }
}

impl<'a> TelemetryApplication<'a> {
    pub(crate) const fn new(
        mailbox: &'a OutputMailbox,
        work_monitor: Option<&'a WorkMonitor>,
    ) -> Self {
        Self {
            mailbox,
            work_monitor,
        }
    }

    fn handle_message(
        &mut self,
        message: &Message<MissionTopic, 1>,
    ) -> Result<(), MissionMessageError> {
        let value = decode_internal(message, MissionTopic::EchoTelemetry)
            .map_err(MissionMessageError::Validation)?;
        self.mailbox.deposit(value)
    }
}

impl Application for MissionApplication<'_> {
    type StartError = Infallible;
    type WorkError = MissionWorkError;
    type StopError = Infallible;
    type RestartError = Infallible;

    fn start(&mut self) -> Result<(), Self::StartError> {
        Ok(())
    }

    fn work(&mut self, context: ApplicationWorkContext<'_>) -> Result<(), Self::WorkError> {
        match self {
            Self::Echo(application) => {
                perform_work(context, MissionRole::Echo, application.work_monitor)
            }
            Self::Telemetry(application) => {
                perform_work(context, MissionRole::Telemetry, application.work_monitor)
            }
        }
    }

    fn stop(&mut self) -> Result<(), Self::StopError> {
        // Host output belongs to the caller and is retained across lifecycle.
        Ok(())
    }

    fn restart(&mut self) -> Result<(), Self::RestartError> {
        Ok(())
    }
}

impl MessagingApplication<MissionTopic, 1> for MissionApplication<'_> {
    type MessageError = MissionMessageError;

    fn handle_message(
        &mut self,
        message: &Message<MissionTopic, 1>,
        context: &mut ApplicationMessageContext<'_, MissionTopic, 1>,
    ) -> Result<(), Self::MessageError> {
        match self {
            Self::Echo(application) => application.handle_message(message, context),
            Self::Telemetry(application) => application.handle_message(message),
        }
    }
}

impl fmt::Display for MissionMessageError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Validation(error) => error.fmt(formatter),
            Self::Message(error) => error.fmt(formatter),
            Self::Publish(error) => error.fmt(formatter),
            Self::TelemetryDelivery(report) => {
                write!(formatter, "telemetry publication incomplete: {report:?}")
            }
            Self::OutputFull { rejected } => {
                write!(formatter, "full host output rejected percentage {rejected}")
            }
        }
    }
}

impl Error for MissionMessageError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Validation(error) => Some(error),
            Self::Message(error) => Some(error),
            Self::Publish(error) => Some(error),
            Self::TelemetryDelivery(_) | Self::OutputFull { .. } => None,
        }
    }
}

/// Pure mission business logic accepts only a validated percentage.
fn echo_percent(command: ValidatedPercent) -> ValidatedPercent {
    command
}
