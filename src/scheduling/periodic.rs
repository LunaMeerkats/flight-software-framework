//! One host-owned recurring release with observation-based re-arming.

use std::error::Error;
use std::fmt;
use std::time::Duration;

use crate::clock::{Clock, FrameworkInstant};

use super::ScheduledWork;

/// Failure to construct a [`PeriodicWork`] cursor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PeriodicWorkCreateError {
    /// A zero interval would permit repeated release at the same instant.
    ZeroInterval,
}

impl fmt::Display for PeriodicWorkCreateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroInterval => write!(formatter, "periodic work interval must be positive"),
        }
    }
}

impl Error for PeriodicWorkCreateError {}

/// The release decision made by one [`PeriodicWork::poll`] call.
///
/// Releasing an item does not invoke application work or establish that the
/// host will invoke it. The host chooses execution, error and disposal policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PeriodicWorkOutcome {
    /// No further release is representable; the clock was not read.
    Exhausted,
    /// The retained release is later than the single observed clock reading.
    Waiting {
        /// Next release retained without change.
        next: ScheduledWork,
        /// Single clock reading used for this decision.
        observed_at: FrameworkInstant,
    },
    /// One due or overdue item was consumed before returning to the host.
    Released {
        /// Item released once by this call.
        scheduled_work: ScheduledWork,
        /// Single clock reading used for this decision and re-arming.
        observed_at: FrameworkInstant,
        /// Next release, or `None` when observed-time addition overflowed.
        next_scheduled_at: Option<FrameworkInstant>,
    },
}

/// One recurring work release controlled by explicit host polling.
///
/// The cursor retains at most one [`ScheduledWork`] and one positive
/// [`Duration`] inline. Construction and polling allocate no agenda or queue,
/// and retain no missed-release count or history. This describes logical
/// storage, not a bound on process memory or a real-time guarantee.
///
/// Due polling releases one item and re-arms from the observed instant plus
/// the interval. Late polling therefore coalesces missed intervals instead of
/// accumulating a catch-up backlog. This is delay from clock observation,
/// not fixed phase or delay from application callback completion.
///
/// The cursor owns no runtime and does not validate application lifecycle or
/// invoke work. Release is consumed regardless of later host action. The host
/// must pair the runtime-local application ID and clock origin correctly,
/// choose polling order and decide when to discard the cursor. A clock moving
/// backward leaves a retained future release waiting; no synchronization,
/// fairness, callback completion or deadline guarantee is provided.
#[derive(Debug)]
pub struct PeriodicWork {
    next: Option<ScheduledWork>,
    interval: Duration,
}

impl PeriodicWork {
    /// Creates a cursor for one application and a positive release interval.
    ///
    /// The first release may be earlier than the first clock observation.
    /// No clock is read and no application or clock origin is authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`PeriodicWorkCreateError::ZeroInterval`] when `interval` is
    /// zero. A positive interval is retained without allocation.
    pub const fn new(
        first: ScheduledWork,
        interval: Duration,
    ) -> Result<Self, PeriodicWorkCreateError> {
        if interval.is_zero() {
            return Err(PeriodicWorkCreateError::ZeroInterval);
        }

        Ok(Self {
            next: Some(first),
            interval,
        })
    }

    /// Returns the retained next release, or `None` after exhaustion.
    ///
    /// This observes cursor state without reading the clock or consuming work.
    #[must_use]
    pub const fn next_scheduled_work(&self) -> Option<ScheduledWork> {
        self.next
    }

    /// Returns the positive duration added to each due poll's observed time.
    #[must_use]
    pub const fn interval(&self) -> Duration {
        self.interval
    }

    /// Releases at most one due work item after one injected clock reading.
    ///
    /// An exhausted cursor returns [`PeriodicWorkOutcome::Exhausted`] without
    /// reading the clock. A future release returns
    /// [`PeriodicWorkOutcome::Waiting`] without mutation. At or after the
    /// release instant, the item is consumed and the next release is set to
    /// the observed instant plus the interval, using checked arithmetic.
    ///
    /// Overflow still releases the valid due item, with
    /// `next_scheduled_at: None`, and exhausts the cursor. Otherwise the next
    /// release is strictly later than the observation. No application callback
    /// is invoked; subsequent host failure does not restore the consumed item
    /// or disable future releases. The host chooses retry and disposal policy.
    pub fn poll<C: Clock + ?Sized>(&mut self, clock: &C) -> PeriodicWorkOutcome {
        let Some(scheduled_work) = self.next else {
            return PeriodicWorkOutcome::Exhausted;
        };

        let observed_at = clock.now();
        if observed_at < scheduled_work.scheduled_at() {
            return PeriodicWorkOutcome::Waiting {
                next: scheduled_work,
                observed_at,
            };
        }

        let next_scheduled_at = observed_at
            .elapsed()
            .checked_add(self.interval)
            .map(FrameworkInstant::from_elapsed);
        self.next = next_scheduled_at
            .map(|scheduled_at| ScheduledWork::new(scheduled_work.application_id(), scheduled_at));

        PeriodicWorkOutcome::Released {
            scheduled_work,
            observed_at,
            next_scheduled_at,
        }
    }
}
