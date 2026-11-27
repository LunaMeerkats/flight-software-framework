//! Injected elapsed framework time, host observation, and manual test time.
//!
//! Host time progresses independently; manual time advances only by explicit
//! request. Neither clock starts work or establishes scheduling, real-time,
//! or cross-clock ordering guarantees.

use std::error::Error;
use std::fmt;
use std::time::{Duration, Instant};

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
/// Calling [`Clock::now`] observes time without requesting an advance or
/// running application work. A host clock may progress independently between
/// observations; a manual clock retains time until explicitly advanced.
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

/// Host elapsed time measured from one private [`Instant`] origin.
///
/// Construction samples the host clock once. Each observation samples it once
/// more and maps that reading to elapsed time from the retained origin. A
/// reading earlier than the origin saturates to [`FrameworkInstant::ZERO`].
/// Readings are not clamped against earlier observations, so platform clock
/// bugs can produce regressing values, including zero. This adapter inherits
/// the host clock's resolution and platform behavior; it promises neither a
/// steady rate nor specified behavior across system suspend.
///
/// Use one instance for related deadlines and observations. Values from
/// separately constructed clocks have unrelated origins. They are not UTC
/// timestamps. The adapter starts no threads, waits, callbacks, or schedules,
/// and makes no real-time guarantee. Use [`ManualClock`] for controlled replay.
#[derive(Debug)]
pub struct HostClock {
    origin: Instant,
}

impl HostClock {
    /// Samples the host clock once to establish this clock's private origin.
    ///
    /// The first subsequent observation can already exceed zero.
    #[must_use]
    pub fn new() -> Self {
        Self {
            origin: Instant::now(),
        }
    }

    fn observed_at(&self, observed_at: Instant) -> FrameworkInstant {
        FrameworkInstant::from_elapsed(observed_at.saturating_duration_since(self.origin))
    }
}

impl Default for HostClock {
    fn default() -> Self {
        Self::new()
    }
}

impl Clock for HostClock {
    fn now(&self) -> FrameworkInstant {
        self.observed_at(Instant::now())
    }
}

#[cfg(test)]
mod tests {
    use super::{Clock, FrameworkInstant, HostClock};
    use std::time::{Duration, Instant};

    #[test]
    fn host_mapping_preserves_elapsed_seconds_and_nanoseconds() {
        let origin = Instant::now();
        let clock = HostClock { origin };
        let elapsed = Duration::new(2, 123);
        let observed = origin.checked_add(elapsed).unwrap();

        assert_eq!(
            clock.observed_at(observed),
            FrameworkInstant::from_elapsed(elapsed)
        );
    }

    #[test]
    fn host_mapping_at_origin_is_zero() {
        let origin = Instant::now();
        let clock = HostClock { origin };

        assert_eq!(clock.observed_at(origin), FrameworkInstant::ZERO);
    }

    #[test]
    fn host_mapping_before_origin_saturates_without_changing_origin() {
        let before_origin = Instant::now();
        let origin = before_origin.checked_add(Duration::from_secs(1)).unwrap();
        let clock = HostClock { origin };

        assert_eq!(clock.observed_at(before_origin), FrameworkInstant::ZERO);
        assert_eq!(clock.origin, origin);
        assert_eq!(clock.observed_at(origin), FrameworkInstant::ZERO);
    }

    #[test]
    fn host_mapping_does_not_clamp_regressing_observations() {
        let origin = Instant::now();
        let clock = HostClock { origin };
        let later = origin.checked_add(Duration::from_nanos(9)).unwrap();
        let earlier = origin.checked_add(Duration::from_nanos(3)).unwrap();

        assert_eq!(clock.observed_at(later).elapsed(), Duration::from_nanos(9));
        assert_eq!(
            clock.observed_at(earlier).elapsed(),
            Duration::from_nanos(3)
        );
    }

    #[test]
    fn host_observation_uses_retained_origin_between_native_samples() {
        let origin = Instant::now().checked_sub(Duration::from_secs(1)).unwrap();
        let clock = HostClock { origin };
        let before = Instant::now();
        let observed = clock.now();
        let after = Instant::now();

        // Bracket the actual Clock entry point without waiting for time to
        // advance. The synthetic past origin exposes a reset-origin defect.
        assert!(observed >= clock.observed_at(before));
        assert!(observed <= clock.observed_at(after));
        assert_eq!(clock.origin, origin);
    }
}
