//! Mission-local record validation. No input is buffered or framed here.
//!
//! A percentage can reach business logic only through checked construction.
//! External and internal records have explicit, different validation order.

use std::error::Error;
use std::fmt;

use rust_flight_framework::Message;

use super::MissionTopic;

const COMMAND_IDENTIFIER: u8 = 0x01;
const TELEMETRY_IDENTIFIER: u8 = 0x81;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ValidatedPercent(u8);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ValidationError {
    HostLength {
        actual: usize,
    },
    UnknownCommand {
        actual: u8,
    },
    PercentOutOfRange {
        actual: u8,
    },
    UnexpectedTopic {
        expected: MissionTopic,
        actual: MissionTopic,
    },
    PayloadLength {
        actual: usize,
    },
}

/// Validates one caller-framed record in length, identifier, then value order.
/// Oversized input is rejected by length without copying or scanning bytes.
pub(crate) fn decode_command(record: &[u8]) -> Result<ValidatedPercent, ValidationError> {
    if record.len() != 2 {
        return Err(ValidationError::HostLength {
            actual: record.len(),
        });
    }
    if record[0] != COMMAND_IDENTIFIER {
        return Err(ValidationError::UnknownCommand { actual: record[0] });
    }
    ValidatedPercent::new(record[1])
}

/// Revalidates topic, exact used payload length, then percentage range.
pub(crate) fn decode_internal(
    message: &Message<MissionTopic, 1>,
    expected: MissionTopic,
) -> Result<ValidatedPercent, ValidationError> {
    if *message.topic() != expected {
        return Err(ValidationError::UnexpectedTopic {
            expected,
            actual: *message.topic(),
        });
    }
    match message.payload() {
        [value] => ValidatedPercent::new(*value),
        payload => Err(ValidationError::PayloadLength {
            actual: payload.len(),
        }),
    }
}

impl ValidatedPercent {
    fn new(value: u8) -> Result<Self, ValidationError> {
        if value > 100 {
            return Err(ValidationError::PercentOutOfRange { actual: value });
        }
        Ok(Self(value))
    }

    pub(crate) const fn value(self) -> u8 {
        self.0
    }

    pub(crate) const fn encode_telemetry(self) -> [u8; 2] {
        [TELEMETRY_IDENTIFIER, self.0]
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HostLength { actual } => {
                write!(
                    formatter,
                    "command record length {actual}; expected two bytes"
                )
            }
            Self::UnknownCommand { actual } => {
                write!(formatter, "unknown command identifier {actual:#04x}")
            }
            Self::PercentOutOfRange { actual } => {
                write!(formatter, "percentage {actual} exceeds 100")
            }
            Self::UnexpectedTopic { expected, actual } => {
                write!(formatter, "topic {actual:?}; expected {expected:?}")
            }
            Self::PayloadLength { actual } => {
                write!(
                    formatter,
                    "internal payload length {actual}; expected one byte"
                )
            }
        }
    }
}

impl Error for ValidationError {}
