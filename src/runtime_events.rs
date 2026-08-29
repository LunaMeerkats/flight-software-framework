//! Runtime integration for cooperative application failure events.
//!
//! This module composes the existing synchronous work, injected-clock, and
//! bounded-event boundaries. It adds one opt-in work operation and does not
//! attach a clock or event queue permanently to the runtime.

use std::error::Error;
use std::fmt;

use crate::clock::Clock;
use crate::events::{
    Event, EventEmitOutcome, EventQueue, EventSeverity, EventSource, EventTimestamp,
};
use crate::lifecycle::{ApplicationId, ApplicationState};
use crate::runtime::{Application, Runtime, RuntimeWorkError};

/// One runtime-generated failure event and its bounded queue outcome.
///
/// The event remains available after either outcome. When the queue was full,
/// a caller can therefore inspect or explicitly retry the exact event with its
/// original failure timestamp. Such a retry is never automatic.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FailureEventAttempt<EventId: Copy> {
    event: Event<EventId>,
    outcome: EventEmitOutcome,
}

impl<EventId: Copy> FailureEventAttempt<EventId> {
    /// Borrows the exact event supplied to the bounded queue.
    #[must_use]
    pub const fn event(&self) -> &Event<EventId> {
        &self.event
    }

    /// Returns the bounded queue's result for the event attempt.
    pub const fn outcome(&self) -> EventEmitOutcome {
        self.outcome
    }

    /// Returns the event and queue outcome as caller-owned values.
    #[must_use = "the event and emission outcome may require explicit caller handling"]
    pub fn into_parts(self) -> (Event<EventId>, EventEmitOutcome) {
        (self.event, self.outcome)
    }
}

/// Failure from work that also reports cooperative application errors as
/// events.
///
/// [`RuntimeWorkEventError::failure_event_attempt`] is `Some` exactly when
/// [`RuntimeWorkEventError::work_error`] is
/// [`RuntimeWorkError::Application`]. Lifecycle rejection occurs before
/// application code, reads no clock, and has no failure-event attempt.
#[derive(Debug, Eq, PartialEq)]
pub struct RuntimeWorkEventError<E, EventId: Copy> {
    work_error: RuntimeWorkError<E>,
    failure_event_attempt: Option<FailureEventAttempt<EventId>>,
}

impl<E, EventId: Copy> RuntimeWorkEventError<E, EventId> {
    /// Borrows the complete error returned by the ordinary runtime work path.
    #[must_use]
    pub const fn work_error(&self) -> &RuntimeWorkError<E> {
        &self.work_error
    }

    /// Borrows the application failure-event attempt, when one occurred.
    #[must_use]
    pub const fn failure_event_attempt(&self) -> Option<&FailureEventAttempt<EventId>> {
        self.failure_event_attempt.as_ref()
    }

    /// Returns the complete work error without retaining event-attempt context.
    #[must_use]
    pub fn into_work_error(self) -> RuntimeWorkError<E> {
        self.work_error
    }

    /// Returns the complete work error and optional event attempt.
    #[must_use]
    pub fn into_parts(self) -> (RuntimeWorkError<E>, Option<FailureEventAttempt<EventId>>) {
        (self.work_error, self.failure_event_attempt)
    }
}

impl<E: fmt::Display, EventId: Copy> fmt::Display for RuntimeWorkEventError<E, EventId> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.work_error, formatter)?;
        if let Some(attempt) = self.failure_event_attempt
            && let EventEmitOutcome::QueueFull { capacity } = attempt.outcome()
        {
            write!(
                formatter,
                "; failure event was rejected at queue capacity {capacity}"
            )?;
        }
        Ok(())
    }
}

impl<E, EventId> Error for RuntimeWorkEventError<E, EventId>
where
    E: Error + 'static,
    EventId: Copy + fmt::Debug,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.work_error)
    }
}

impl<A: Application> Runtime<A> {
    /// Invokes one work callback and reports a returned application error once.
    ///
    /// This delegates lifecycle and callback behavior to [`Runtime::work`]. On
    /// success or lifecycle rejection, it neither reads `clock` nor attempts an
    /// event. After an application returns an error and the runtime commits the
    /// selected record to [`ApplicationState::Failed`], it captures one clock
    /// reading, constructs one application-sourced [`EventSeverity::Error`]
    /// event, and attempts to append it once to `event_queue`.
    ///
    /// The caller supplies the mission-defined event identifier and retains the
    /// complete work error plus the exact event attempt. Queue saturation never
    /// replaces the work error and never retries implicitly.
    ///
    /// # Errors
    ///
    /// Returns [`RuntimeWorkEventError`] for the same lifecycle or application
    /// failures as [`Runtime::work`]. Only a returned application error carries
    /// a [`FailureEventAttempt`].
    pub fn work_with_failure_event<EventId, C>(
        &mut self,
        application_id: ApplicationId,
        failure_event_identifier: EventId,
        clock: &C,
        event_queue: &mut EventQueue<EventId>,
    ) -> Result<ApplicationState, RuntimeWorkEventError<A::WorkError, EventId>>
    where
        EventId: Copy,
        C: Clock + ?Sized,
    {
        let work_error = match self.work(application_id) {
            Ok(state) => return Ok(state),
            Err(error) => error,
        };

        let failure_event_attempt = match &work_error {
            RuntimeWorkError::Lifecycle(_) => None,
            RuntimeWorkError::Application { application_id, .. } => {
                let event = Event::new(
                    EventSource::Application(*application_id),
                    EventSeverity::Error,
                    failure_event_identifier,
                    EventTimestamp::from_clock(clock),
                );
                let outcome = event_queue.emit(&event);
                Some(FailureEventAttempt { event, outcome })
            }
        };

        Err(RuntimeWorkEventError {
            work_error,
            failure_event_attempt,
        })
    }
}
