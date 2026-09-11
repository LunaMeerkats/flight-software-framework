//! Private two-application mission and explicit command ingress.
//!
//! Integration tests load this same module through a relative path; this is
//! not a library API. Construction leaves both applications Registered. The
//! host selects every lifecycle operation, work callback, and dispatch.

use std::error::Error;
use std::fmt;

use rust_flight_framework::{
    ApplicationId, ConfigurationError, Message, MessageCreateError, MessagingRuntime,
    MessagingRuntimeCreateErrorKind, PublishError, PublishReport, Runtime, RuntimeCreateError,
    RuntimeInboxConfig,
};

// Explicit child paths keep example and attributed test loading identical.
#[path = "mission/applications.rs"]
pub(crate) mod applications;
#[path = "mission/codec.rs"]
pub(crate) mod codec;
#[path = "mission/work.rs"]
pub(crate) mod work;

use applications::{EchoApplication, MissionApplication, OutputMailbox, TelemetryApplication};
use codec::{ValidationError, decode_command};
use work::{MissionConfigurationError, WorkMonitor, initial_configuration};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MissionTopic {
    EchoCommand,
    EchoTelemetry,
}

pub(crate) type EchoRuntime<'a> =
    MessagingRuntime<MissionApplication<'a>, MissionTopic, 1, MissionConfigurationError, 1>;

pub(crate) struct ComposedMission<'a> {
    pub(crate) runtime: EchoRuntime<'a>,
    pub(crate) echo_id: ApplicationId,
    pub(crate) telemetry_id: ApplicationId,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum SetupError {
    Configuration(ConfigurationError<MissionConfigurationError>),
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

/// Constructs the fixed topology without starting either application.
/// An optional borrowed monitor enables bounded observations and injection.
/// Setup errors preserve their concrete reason; no partial mission escapes.
pub(crate) fn compose<'a>(
    mailbox: &'a OutputMailbox,
    monitor: Option<&'a WorkMonitor>,
) -> Result<ComposedMission<'a>, SetupError> {
    let configuration = initial_configuration().map_err(SetupError::Configuration)?;
    let mut runtime = Runtime::with_configuration(2, configuration)
        .map_err(|error| SetupError::Runtime(error.kind()))?;
    let echo_id = runtime
        .register(MissionApplication::Echo(EchoApplication::new(monitor)))
        .map_err(|error| SetupError::RegistrationCapacity {
            capacity: error.capacity(),
        })?;
    let telemetry_id = runtime
        .register(MissionApplication::Telemetry(TelemetryApplication::new(
            mailbox, monitor,
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
            Self::Configuration(error) => error.fmt(formatter),
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
            Self::Configuration(error) => Some(error),
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
