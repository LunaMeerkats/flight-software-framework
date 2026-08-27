//! Injected elapsed framework time and a manually advanced test clock.
//!
//! This module defines only time observation and explicit manual advancement.
//! It reads no host clock, starts no work, and makes no scheduling, real-time,
//! or cross-clock ordering guarantee.

use std::error::Error;
use std::fmt;
use std::time::Duration;

/// Elapsed time from the origin of one framework clock.
///
/// Ordering is meaningful only for readings governed by the same clock origin.
/// The value carries no clock identity and is not a wall-clock timestamp.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct FrameworkInstant(Duration);

impl FrameworkInstant {
    /// The origin of a framework clock.
    pub const ZERO: Self = Self(Duration::ZERO);

    /// Creates an instant at an explicit elapsed duration from a clock origin.
    #[must_use]
    pub const fn from_elapsed(elapsed: Duration) -> Self {
        Self(elapsed)
    }

    /// Returns elapsed time from the framework clock origin.
    #[must_use]
    pub const fn elapsed(self) -> Duration {
        self.0
    }
}

/// Read access to elapsed framework time.
///
/// Calling [`Clock::now`] observes the current instant without advancing time
/// or performing hidden work.
pub trait Clock {
    /// Returns the current elapsed instant for this clock.
    fn now(&self) -> FrameworkInstant;
}

/// Failure to advance a [`ManualClock`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ManualClockAdvanceError {
    /// Adding the requested duration would exceed the representable range.
    Overflow {
        /// Instant retained by the clock after the rejected advance.
        current: FrameworkInstant,
        /// Duration that could not be added.
        advance: Duration,
    },
}

impl fmt::Display for ManualClockAdvanceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Overflow { current, advance } => write!(
                formatter,
                "advancing manual clock from {:?} by {advance:?} would overflow",
                current.elapsed()
            ),
        }
    }
}

impl Error for ManualClockAdvanceError {}

/// A framework clock advanced only by explicit caller requests.
///
/// New clocks start at [`FrameworkInstant::ZERO`]. Reads and zero-duration
/// advances do not move time. A rejected overflowing advance leaves the
/// current instant unchanged.
#[derive(Debug, Eq, PartialEq)]
pub struct ManualClock {
    current: FrameworkInstant,
}

impl ManualClock {
    /// Creates a manual clock at its zero origin.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            current: FrameworkInstant::ZERO,
        }
    }

    /// Creates a manual clock at an explicit elapsed instant.
    ///
    /// This supports controlled scenario setup; it does not represent a host
    /// wall clock or establish a relationship with another clock instance.
    #[must_use]
    pub const fn from_elapsed(elapsed: Duration) -> Self {
        Self {
            current: FrameworkInstant::from_elapsed(elapsed),
        }
    }

    /// Advances the clock by one nonnegative duration.
    ///
    /// A zero duration is a successful no-op. Successful advances return the
    /// new current instant.
    ///
    /// # Errors
    ///
    /// Returns [`ManualClockAdvanceError::Overflow`] without changing the
    /// clock when the result is not representable by [`Duration`].
    pub fn advance(
        &mut self,
        duration: Duration,
    ) -> Result<FrameworkInstant, ManualClockAdvanceError> {
        let Some(elapsed) = self.current.elapsed().checked_add(duration) else {
            return Err(ManualClockAdvanceError::Overflow {
                current: self.current,
                advance: duration,
            });
        };

        self.current = FrameworkInstant::from_elapsed(elapsed);
        Ok(self.current)
    }
}

impl Default for ManualClock {
    fn default() -> Self {
        Self::new()
    }
}

impl Clock for ManualClock {
    fn now(&self) -> FrameworkInstant {
        self.current
    }
}
