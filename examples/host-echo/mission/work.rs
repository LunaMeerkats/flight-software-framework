//! Bounded observations of ordinary work and explicit cooperative injection.
//!
//! The one configuration byte is a sample observation value in 0..=100. It
//! does not alter echo commands or represent an actuator or mission setpoint.
//! Each role retains only its latest copied observation. The host can arm one
//! echo work failure; only a valid echo work callback consumes that request.

use std::cell::Cell;
use std::error::Error;
use std::fmt;

use rust_flight_framework::{ApplicationWorkContext, ConfigurationError, ConfigurationTable};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MissionRole {
    Echo,
    Telemetry,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct WorkObservation {
    pub(crate) revision: u64,
    pub(crate) value: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MissionConfigurationError {
    Length { actual: usize },
    ValueOutOfRange { actual: u8 },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MissionWorkError {
    MissingConfiguration,
    InvalidConfiguration(MissionConfigurationError),
    InjectedEchoFailure,
}

#[derive(Default)]
pub(crate) struct WorkMonitor {
    echo: Cell<Option<WorkObservation>>,
    telemetry: Cell<Option<WorkObservation>>,
    fail_echo: Cell<bool>,
}

/// Creates revision one using the same validator retained for replacement.
pub(crate) fn initial_configuration() -> Result<
    ConfigurationTable<MissionConfigurationError, 1>,
    ConfigurationError<MissionConfigurationError>,
> {
    ConfigurationTable::new(&[10], validate_configuration)
}

impl WorkMonitor {
    /// Consumes at most the latest observation; the next read is empty until
    /// that role completes another configuration observation during work.
    pub(crate) fn take_observation(&self, role: MissionRole) -> Option<WorkObservation> {
        match role {
            MissionRole::Echo => self.echo.take(),
            MissionRole::Telemetry => self.telemetry.take(),
        }
    }

    /// Arms one failure after the next valid echo work observation is copied.
    /// Telemetry work and message callbacks neither consume nor trigger it.
    pub(crate) fn arm_echo_failure(&self) {
        self.fail_echo.set(true);
    }

    fn record(&self, role: MissionRole, observation: WorkObservation) {
        match role {
            MissionRole::Echo => self.echo.set(Some(observation)),
            MissionRole::Telemetry => self.telemetry.set(Some(observation)),
        }
    }
}

pub(super) fn perform_work(
    context: ApplicationWorkContext<'_>,
    role: MissionRole,
    monitor: Option<&WorkMonitor>,
) -> Result<(), MissionWorkError> {
    let configuration = context
        .configuration()
        .ok_or(MissionWorkError::MissingConfiguration)?;
    let value = configuration_value(configuration.bytes())
        .map_err(MissionWorkError::InvalidConfiguration)?;
    if let Some(monitor) = monitor {
        monitor.record(
            role,
            WorkObservation {
                revision: configuration.revision(),
                value,
            },
        );
        if role == MissionRole::Echo && monitor.fail_echo.replace(false) {
            return Err(MissionWorkError::InjectedEchoFailure);
        }
    }
    Ok(())
}

impl fmt::Display for MissionConfigurationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Length { actual } => {
                write!(
                    formatter,
                    "configuration length {actual}; expected one byte"
                )
            }
            Self::ValueOutOfRange { actual } => {
                write!(
                    formatter,
                    "configuration observation value {actual} exceeds 100"
                )
            }
        }
    }
}

impl Error for MissionConfigurationError {}

impl fmt::Display for MissionWorkError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingConfiguration => formatter.write_str("mission configuration is absent"),
            Self::InvalidConfiguration(error) => error.fmt(formatter),
            Self::InjectedEchoFailure => {
                formatter.write_str("injected cooperative echo work failure")
            }
        }
    }
}

impl Error for MissionWorkError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidConfiguration(error) => Some(error),
            Self::MissingConfiguration | Self::InjectedEchoFailure => None,
        }
    }
}

fn validate_configuration(bytes: &[u8]) -> Result<(), MissionConfigurationError> {
    configuration_value(bytes).map(|_| ())
}

fn configuration_value(bytes: &[u8]) -> Result<u8, MissionConfigurationError> {
    match bytes {
        [value] if *value <= 100 => Ok(*value),
        [value] => Err(MissionConfigurationError::ValueOutOfRange { actual: *value }),
        _ => Err(MissionConfigurationError::Length {
            actual: bytes.len(),
        }),
    }
}
