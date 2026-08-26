//! Bounded structured events and FIFO queueing.
//!
//! This module defines the first standalone event path for RFF-REQ-005. It
//! stores only structured metadata, reserves a positive record bound during
//! construction, and reports saturation directly instead of recursively
//! emitting another event. Clock generation and runtime integration remain
//! outside this slice.

use std::collections::VecDeque;
use std::error::Error;
use std::fmt;
use std::time::Duration;

use crate::lifecycle::ApplicationId;

/// The producer represented by one structured event.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EventSource {
    /// Framework behavior not attributed to one application.
    Framework,
    /// Behavior attributed to one runtime-local application identity.
    ///
    /// A standalone queue cannot validate which runtime issued this identity.
    Application(ApplicationId),
}

/// The local importance category carried by one event.
///
/// These categories do not imply a response, priority, filtering policy, or
/// compatibility with another event system.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EventSeverity {
    /// Expected operation or useful state information.
    Informational,
    /// A condition that deserves attention but is not reported as an error.
    Warning,
    /// A defined operation or behavior reported an error.
    Error,
}

/// Elapsed framework time captured for one event.
///
/// The duration is measured from the origin selected by a framework clock. No
/// clock exists in this slice, so event producers supply this value explicitly.
/// It is not a wall-clock timestamp, and the event queue does not enforce
/// monotonicity.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct EventTimestamp(Duration);

impl EventTimestamp {
    /// Creates a timestamp from elapsed time since the framework clock origin.
    #[must_use]
    pub const fn from_elapsed(elapsed: Duration) -> Self {
        Self(elapsed)
    }

    /// Returns elapsed time since the framework clock origin.
    #[must_use]
    pub const fn elapsed(self) -> Duration {
        self.0
    }
}

/// One machine-inspectable event record.
///
/// The identifier type is selected by the mission and is not a protocol, cFS,
/// or wire identifier. [`EventQueue`] requires it to be copied into each
/// bounded record.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Event<EventId> {
    source: EventSource,
    severity: EventSeverity,
    identifier: EventId,
    timestamp: EventTimestamp,
}

impl<EventId> Event<EventId> {
    /// Creates an event from its complete structured metadata.
    #[must_use]
    pub const fn new(
        source: EventSource,
        severity: EventSeverity,
        identifier: EventId,
        timestamp: EventTimestamp,
    ) -> Self {
        Self {
            source,
            severity,
            identifier,
            timestamp,
        }
    }

    /// Returns the producer represented by this event.
    #[must_use]
    pub const fn source(&self) -> EventSource {
        self.source
    }

    /// Returns the local importance category.
    #[must_use]
    pub const fn severity(&self) -> EventSeverity {
        self.severity
    }

    /// Borrows the mission-defined identifier.
    #[must_use]
    pub const fn identifier(&self) -> &EventId {
        &self.identifier
    }

    /// Returns the explicit framework timestamp.
    #[must_use]
    pub const fn timestamp(&self) -> EventTimestamp {
        self.timestamp
    }
}

/// Failure to construct bounded event storage.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EventQueueCreateError {
    /// The configured event-record capacity was zero.
    ZeroCapacity,
    /// Storage for the configured number of event records could not be
    /// reserved.
    CapacityAllocationFailed {
        /// Requested number of event records.
        requested: usize,
    },
}

impl fmt::Display for EventQueueCreateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroCapacity => formatter.write_str("event capacity must be positive"),
            Self::CapacityAllocationFailed { requested } => write!(
                formatter,
                "could not reserve event storage for {requested} records"
            ),
        }
    }
}

impl Error for EventQueueCreateError {}

/// Caller-visible result of one event emission attempt.
#[must_use = "event emission can reject the newest event when the queue is full"]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EventEmitOutcome {
    /// The event was appended to the queue.
    Recorded,
    /// The queue retained its older records and rejected this newest event.
    QueueFull {
        /// Configured logical event-record capacity.
        capacity: usize,
    },
}

/// A positive-capacity FIFO queue for structured event records.
///
/// Construction reserves storage for the configured number of records. The
/// logical capacity is enforced directly, independent of allocator capacity.
/// A full queue retains every older event and rejects the newest event without
/// retry, spill, overwrite, hidden work, or recursive diagnostics.
#[derive(Debug)]
pub struct EventQueue<EventId: Copy> {
    events: VecDeque<Event<EventId>>,
    max_events: usize,
}

impl<EventId: Copy> EventQueue<EventId> {
    /// Creates an empty event queue with a positive fixed record capacity.
    ///
    /// # Errors
    ///
    /// Returns an error when capacity is zero or storage for the complete
    /// configured bound cannot be reserved without panicking.
    pub fn new(max_events: usize) -> Result<Self, EventQueueCreateError> {
        if max_events == 0 {
            return Err(EventQueueCreateError::ZeroCapacity);
        }

        let mut events = VecDeque::new();
        events.try_reserve_exact(max_events).map_err(|_| {
            EventQueueCreateError::CapacityAllocationFailed {
                requested: max_events,
            }
        })?;
        Ok(Self { events, max_events })
    }

    /// Returns the configured logical event-record limit.
    #[must_use]
    pub const fn capacity(&self) -> usize {
        self.max_events
    }

    /// Returns the number of queued event records.
    #[must_use]
    pub fn pending(&self) -> usize {
        self.events.len()
    }

    /// Returns whether the event queue contains no records.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Attempts to append one event without waiting for consumer progress.
    ///
    /// The event is borrowed, so the caller retains it after either outcome.
    /// At the exact configured limit, this returns
    /// [`EventEmitOutcome::QueueFull`] without changing the queue.
    pub fn emit(&mut self, event: &Event<EventId>) -> EventEmitOutcome {
        if self.events.len() >= self.max_events {
            return EventEmitOutcome::QueueFull {
                capacity: self.max_events,
            };
        }

        self.events.push_back(*event);
        EventEmitOutcome::Recorded
    }

    /// Removes and returns the oldest accepted event, if any.
    ///
    /// A removed event becomes caller-owned and no longer counts against the
    /// queue's configured storage bound.
    pub fn dequeue(&mut self) -> Option<Event<EventId>> {
        self.events.pop_front()
    }
}
