//! Finite caller-driven scheduling of owned application work.
//!
//! A schedule is an immutable, one-shot agenda ordered by elapsed framework
//! instants. Each runtime call observes an injected clock and attempts at most
//! one due item. This module adds no periodic generation, automatic draining,
//! wall-clock access, concurrency, or execution-time guarantee.

use std::error::Error;
use std::fmt;

use crate::clock::{Clock, FrameworkInstant};
use crate::lifecycle::{ApplicationId, ApplicationState};
use crate::runtime::{Application, Runtime, RuntimeWorkError};

/// One application work item released at an elapsed framework instant.
///
/// `scheduled_at` is the earliest instant at which the item may be attempted.
/// It is not a wall-clock value or a guarantee that work completes by a
/// deadline. Application and clock origins remain caller-scoped.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ScheduledWork {
    application_id: ApplicationId,
    scheduled_at: FrameworkInstant,
}

impl ScheduledWork {
    /// Creates one scheduled, one-shot work item.
    #[must_use]
    pub const fn new(application_id: ApplicationId, scheduled_at: FrameworkInstant) -> Self {
        Self {
            application_id,
            scheduled_at,
        }
    }

    /// Returns the runtime-local application selected for work.
    #[must_use]
    pub const fn application_id(self) -> ApplicationId {
        self.application_id
    }

    /// Returns the earliest elapsed instant at which work may be attempted.
    #[must_use]
    pub const fn scheduled_at(self) -> FrameworkInstant {
        self.scheduled_at
    }
}

/// Failure to construct a finite [`WorkSchedule`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkScheduleCreateError {
    /// One item was earlier than its predecessor.
    ScheduledInstantsOutOfOrder {
        /// Zero-based index of the descending item.
        index: usize,
        /// Scheduled instant of the preceding item.
        previous: FrameworkInstant,
        /// Scheduled instant of the rejected item.
        scheduled_at: FrameworkInstant,
    },
    /// Storage for the complete configured agenda could not be reserved.
    StorageAllocationFailed {
        /// Requested number of scheduled work items.
        requested: usize,
    },
}

impl fmt::Display for WorkScheduleCreateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ScheduledInstantsOutOfOrder {
                index,
                previous,
                scheduled_at,
            } => write!(
                formatter,
                "scheduled work item {index} at {:?} precedes {:?}",
                scheduled_at.elapsed(),
                previous.elapsed()
            ),
            Self::StorageAllocationFailed { requested } => write!(
                formatter,
                "could not reserve schedule storage for {requested} work items"
            ),
        }
    }
}

impl Error for WorkScheduleCreateError {}

/// One caller-visible result when no scheduled work error was returned.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScheduledWorkOutcome {
    /// Every configured one-shot item has already been attempted.
    Complete,
    /// The next item is not yet due, so no state or schedule changed.
    Waiting {
        /// Next item retained by the schedule.
        next: ScheduledWork,
        /// Single clock reading used for this decision.
        observed_at: FrameworkInstant,
    },
    /// One due or overdue application work callback returned successfully.
    Completed {
        /// Item consumed and attempted by this call.
        scheduled_work: ScheduledWork,
        /// Single clock reading used for this attempt.
        observed_at: FrameworkInstant,
        /// Lifecycle state returned by the successful runtime work operation.
        state: ApplicationState,
    },
}

/// A due scheduled item whose existing runtime work operation returned an
/// error.
#[derive(Debug, Eq, PartialEq)]
pub struct ScheduledWorkError<E> {
    scheduled_work: ScheduledWork,
    observed_at: FrameworkInstant,
    work_error: RuntimeWorkError<E>,
}

impl<E> ScheduledWorkError<E> {
    /// Returns the item consumed by the failed work attempt.
    #[must_use]
    pub const fn scheduled_work(&self) -> ScheduledWork {
        self.scheduled_work
    }

    /// Returns the clock reading used to determine that the item was due.
    #[must_use]
    pub const fn observed_at(&self) -> FrameworkInstant {
        self.observed_at
    }

    /// Borrows the exact error returned by the existing runtime work boundary.
    #[must_use]
    pub const fn work_error(&self) -> &RuntimeWorkError<E> {
        &self.work_error
    }

    /// Returns ownership of the exact runtime work error.
    #[must_use]
    pub fn into_work_error(self) -> RuntimeWorkError<E> {
        self.work_error
    }
}

impl<E: fmt::Display> fmt::Display for ScheduledWorkError<E> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "scheduled work for {:?} at {:?} failed when observed at {:?}: {}",
            self.scheduled_work.application_id(),
            self.scheduled_work.scheduled_at().elapsed(),
            self.observed_at.elapsed(),
            self.work_error
        )
    }
}

impl<E: Error + 'static> Error for ScheduledWorkError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.work_error)
    }
}

/// A fixed finite agenda of one-shot application work items.
///
/// Construction copies items in caller order and requires nondecreasing
/// scheduled instants. Equal-time items therefore retain configuration order.
/// Dispatch advances only a private cursor; the agenda has no operation that
/// adds work or grows storage.
#[derive(Debug)]
pub struct WorkSchedule {
    scheduled_work: Vec<ScheduledWork>,
    next_index: usize,
}

impl WorkSchedule {
    /// Copies a complete finite agenda after validating scheduled-time order.
    ///
    /// An empty agenda is valid and immediately complete.
    ///
    /// # Errors
    ///
    /// Returns [`WorkScheduleCreateError::ScheduledInstantsOutOfOrder`] before
    /// allocation when an item precedes its predecessor. Returns
    /// [`WorkScheduleCreateError::StorageAllocationFailed`] when storage for
    /// the complete item count cannot be reserved without panicking.
    pub fn new(scheduled_work: &[ScheduledWork]) -> Result<Self, WorkScheduleCreateError> {
        validate_order(scheduled_work)?;

        let mut owned_work = Vec::new();
        owned_work
            .try_reserve_exact(scheduled_work.len())
            .map_err(|_| WorkScheduleCreateError::StorageAllocationFailed {
                requested: scheduled_work.len(),
            })?;
        owned_work.extend_from_slice(scheduled_work);

        Ok(Self {
            scheduled_work: owned_work,
            next_index: 0,
        })
    }

    /// Returns the number of configured items not yet attempted.
    #[must_use]
    pub fn remaining(&self) -> usize {
        self.scheduled_work.len() - self.next_index
    }

    /// Returns whether every configured item has been attempted.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.remaining() == 0
    }

    fn next(&self) -> Option<ScheduledWork> {
        self.scheduled_work.get(self.next_index).copied()
    }

    fn consume_next(&mut self) {
        self.next_index += 1;
    }
}

impl<A: Application, E, const MAX_CONFIGURATION_BYTES: usize>
    Runtime<A, E, MAX_CONFIGURATION_BYTES>
{
    /// Attempts at most the next due work item under an injected clock.
    ///
    /// An empty schedule returns [`ScheduledWorkOutcome::Complete`] without
    /// reading the clock. A future item returns
    /// [`ScheduledWorkOutcome::Waiting`] after one clock read and changes
    /// neither runtime nor schedule. A due or overdue item is consumed, then
    /// delegated to [`Runtime::work`].
    ///
    /// Consumption is final after either work success or error. This prevents
    /// one stopped or failed application from blocking later due work; retry
    /// requires a separately configured item.
    ///
    /// # Errors
    ///
    /// Returns [`ScheduledWorkError`] with the consumed item, observed instant,
    /// and exact [`RuntimeWorkError`] when lifecycle validation rejects work or
    /// the application callback returns an error.
    pub fn run_next_scheduled_work<C: Clock + ?Sized>(
        &mut self,
        schedule: &mut WorkSchedule,
        clock: &C,
    ) -> Result<ScheduledWorkOutcome, ScheduledWorkError<A::WorkError>> {
        let Some(scheduled_work) = schedule.next() else {
            return Ok(ScheduledWorkOutcome::Complete);
        };

        let observed_at = clock.now();
        if observed_at < scheduled_work.scheduled_at() {
            return Ok(ScheduledWorkOutcome::Waiting {
                next: scheduled_work,
                observed_at,
            });
        }

        schedule.consume_next();
        self.work(scheduled_work.application_id())
            .map(|state| ScheduledWorkOutcome::Completed {
                scheduled_work,
                observed_at,
                state,
            })
            .map_err(|work_error| ScheduledWorkError {
                scheduled_work,
                observed_at,
                work_error,
            })
    }
}

fn validate_order(scheduled_work: &[ScheduledWork]) -> Result<(), WorkScheduleCreateError> {
    for (offset, pair) in scheduled_work.windows(2).enumerate() {
        let previous = pair[0].scheduled_at();
        let scheduled_at = pair[1].scheduled_at();
        if scheduled_at < previous {
            return Err(WorkScheduleCreateError::ScheduledInstantsOutOfOrder {
                index: offset + 1,
                previous,
                scheduled_at,
            });
        }
    }

    Ok(())
}
