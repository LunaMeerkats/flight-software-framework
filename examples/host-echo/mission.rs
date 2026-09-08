//! Private two-application mission and explicit command ingress.
//!
//! Integration tests load this same module through a relative path. It is
//! neither a library API nor the full v0.1 sample. Construction leaves both
//! applications Registered; the host selects every lifecycle and dispatch.

use std::error::Error;
use std::fmt;

use rust_flight_framework::{
    ApplicationId, Message, MessageCreateError, MessagingRuntime, MessagingRuntimeCreateErrorKind,
    PublishError, PublishReport, Runtime, RuntimeCreateError, RuntimeInboxConfig,
};

// Explicit child paths keep example and attributed test loading identical.
#[path = "mission/applications.rs"]
pub(crate) mod applications;
#[path = "mission/codec.rs"]
pub(crate) mod codec;

use applications::{EchoApplication, MissionApplication, OutputMailbox, TelemetryApplication};
use codec::{ValidationError, decode_command};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MissionTopic {
    EchoCommand,
    EchoTelemetry,
}

pub(crate) type EchoRuntime<'a> = MessagingRuntime<MissionApplication<'a>, MissionTopic, 1>;

pub(crate) struct ComposedMission<'a> {
    pub(crate) runtime: EchoRuntime<'a>,
    pub(crate) echo_id: ApplicationId,
    pub(crate) telemetry_id: ApplicationId,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SetupError {
    Runtime(RuntimeCreateError),
    RegistrationCapacity { capacity: usize },
    Messaging(MessagingRuntimeCreateErrorKind),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum IngestError {
    Validation(ValidationError),
    Message(MessageCreateError),
    Publish(PublishError),
}

/// Constructs the fixed ADR-0019 topology without starting any application.
/// Setup errors preserve their concrete reason; no partial mission escapes.
pub(crate) fn compose(mailbox: &OutputMailbox) -> Result<ComposedMission<'_>, SetupError> {
    let mut runtime = Runtime::new(2).map_err(SetupError::Runtime)?;
    let echo_id = runtime
        .register(MissionApplication::Echo(EchoApplication))
        .map_err(|error| SetupError::RegistrationCapacity {
            capacity: error.capacity(),
        })?;
    let telemetry_id = runtime
        .register(MissionApplication::Telemetry(TelemetryApplication::new(
            mailbox,
        )))
        .map_err(|error| SetupError::RegistrationCapacity {
            capacity: error.capacity(),
        })?;
    let inboxes = [
        RuntimeInboxConfig::new(1, &[MissionTopic::EchoCommand]),
        RuntimeInboxConfig::new(1, &[MissionTopic::EchoTelemetry]),
    ];
    let runtime = MessagingRuntime::new(runtime, &inboxes)
        .map_err(|error| SetupError::Messaging(error.kind()))?;
    Ok(ComposedMission {
        runtime,
        echo_id,
        telemetry_id,
    })
}

/// Validates then publishes once, returning the complete original report.
/// Acceptance is queued delivery only; this never dispatches or drains.
pub(crate) fn ingest(
    runtime: &mut EchoRuntime<'_>,
    record: &[u8],
) -> Result<PublishReport, IngestError> {
    let command = decode_command(record).map_err(IngestError::Validation)?;
    let message = Message::try_new(MissionTopic::EchoCommand, &[command.value()])
        .map_err(IngestError::Message)?;
    runtime.publish(&message).map_err(IngestError::Publish)
}

impl fmt::Display for SetupError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Runtime(error) => error.fmt(formatter),
            Self::RegistrationCapacity { capacity } => {
                write!(
                    formatter,
                    "mission registration exceeded capacity {capacity}"
                )
            }
            Self::Messaging(error) => error.fmt(formatter),
        }
    }
}

impl Error for SetupError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Runtime(error) => Some(error),
            Self::Messaging(error) => Some(error),
            Self::RegistrationCapacity { .. } => None,
        }
    }
}

impl fmt::Display for IngestError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Validation(error) => error.fmt(formatter),
            Self::Message(error) => error.fmt(formatter),
            Self::Publish(error) => error.fmt(formatter),
        }
    }
}

impl Error for IngestError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Validation(error) => Some(error),
            Self::Message(error) => Some(error),
            Self::Publish(error) => Some(error),
        }
    }
}
