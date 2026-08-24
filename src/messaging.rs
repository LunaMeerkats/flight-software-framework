//! Bounded in-process messages and serial fan-out.
//!
//! This module implements the first routing core from ADR-0004. It copies an
//! immutable mission topology, pre-reserves each inbox, and processes matching
//! destinations in application-registration order. Standalone [`MessageBus`]
//! publication models configured endpoints as available.
//! [`crate::MessagingRuntime`] owns a freshly configured bus and supplies
//! lifecycle-derived availability and clearing; application dispatch and
//! service contexts remain outside this module.

use std::collections::VecDeque;
use std::error::Error;
use std::fmt;

use crate::ApplicationId;

/// An in-process message with a compile-time maximum payload length.
///
/// The topic type is selected by the mission and is not a protocol or wire
/// identifier. Payload storage is inline, so every queued message consumes
/// space for `MAX_PAYLOAD_BYTES` even when its logical payload is shorter.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Message<Topic, const MAX_PAYLOAD_BYTES: usize> {
    topic: Topic,
    payload_length: usize,
    payload: [u8; MAX_PAYLOAD_BYTES],
}

impl<Topic, const MAX_PAYLOAD_BYTES: usize> Message<Topic, MAX_PAYLOAD_BYTES> {
    /// Creates a message by copying a payload into fixed inline storage.
    ///
    /// Empty payloads are valid, including when `MAX_PAYLOAD_BYTES` is zero.
    ///
    /// # Errors
    ///
    /// Returns [`MessageCreateError::PayloadTooLong`] when `payload` exceeds
    /// the compile-time maximum. The topic and payload are otherwise retained
    /// exactly.
    pub fn try_new(topic: Topic, payload: &[u8]) -> Result<Self, MessageCreateError> {
        if payload.len() > MAX_PAYLOAD_BYTES {
            return Err(MessageCreateError::PayloadTooLong {
                length: payload.len(),
                maximum: MAX_PAYLOAD_BYTES,
            });
        }

        let mut stored_payload = [0; MAX_PAYLOAD_BYTES];
        stored_payload[..payload.len()].copy_from_slice(payload);
        Ok(Self {
            topic,
            payload_length: payload.len(),
            payload: stored_payload,
        })
    }

    /// Borrows the mission-defined topic value.
    #[must_use]
    pub const fn topic(&self) -> &Topic {
        &self.topic
    }

    /// Returns the logical payload without unused inline storage.
    #[must_use]
    pub fn payload(&self) -> &[u8] {
        &self.payload[..self.payload_length]
    }

    /// Returns the compile-time payload limit in bytes.
    #[must_use]
    pub const fn payload_capacity(&self) -> usize {
        MAX_PAYLOAD_BYTES
    }
}

/// Failure to construct an inline message.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MessageCreateError {
    /// The supplied payload exceeded the message type's inline limit.
    PayloadTooLong {
        /// Supplied payload length in bytes.
        length: usize,
        /// Maximum payload length accepted by the message type.
        maximum: usize,
    },
}

impl fmt::Display for MessageCreateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PayloadTooLong { length, maximum } => write!(
                formatter,
                "payload length {length} exceeds the {maximum}-byte message limit"
            ),
        }
    }
}

impl Error for MessageCreateError {}

/// Immutable mission configuration for one application inbox.
///
/// Configurations passed to [`MessageBus::new`] must use identities from one
/// issuer in registration order. This slice cannot distinguish a same-slot
/// identity issued by a different registry or runtime.
#[derive(Clone, Copy, Debug)]
pub struct ApplicationInboxConfig<'a, Topic> {
    application_id: ApplicationId,
    capacity: usize,
    topics: &'a [Topic],
}

impl<'a, Topic> ApplicationInboxConfig<'a, Topic> {
    /// Describes one inbox and its complete immutable topic set.
    ///
    /// A zero-capacity inbox and repeated topic are rejected atomically when
    /// the configured bus topology is constructed. An empty topic set is valid.
    #[must_use]
    pub const fn new(application_id: ApplicationId, capacity: usize, topics: &'a [Topic]) -> Self {
        Self {
            application_id,
            capacity,
            topics,
        }
    }
}

/// Immutable inbox configuration in application-registration order.
///
/// [`crate::MessagingRuntime`] assigns each entry to the application at the
/// same registration position. Callers therefore cannot pair its owned bus
/// with identities from another runtime or omit an inbox for a registered
/// application. Mission composition remains responsible for placing each
/// application's intended capacity and topics at the correct position.
#[derive(Clone, Copy, Debug)]
pub struct RuntimeInboxConfig<'a, Topic> {
    capacity: usize,
    topics: &'a [Topic],
}

impl<'a, Topic> RuntimeInboxConfig<'a, Topic> {
    /// Describes one runtime-owned inbox and its immutable topic set.
    ///
    /// A zero-capacity inbox and repeated topic are rejected atomically during
    /// [`crate::MessagingRuntime`] construction. An empty topic set is valid.
    #[must_use]
    pub const fn new(capacity: usize, topics: &'a [Topic]) -> Self {
        Self { capacity, topics }
    }
}

/// Failure to construct the configured bounded message topology.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MessageBusCreateError {
    /// No application inbox was configured.
    NoInboxes,
    /// An identity was not supplied at its registration-order position.
    ApplicationOutOfOrder {
        /// Identity found in the configuration.
        application_id: ApplicationId,
        /// Zero-based registration position required at this location.
        expected_index: usize,
    },
    /// An application inbox had no delivery slots.
    ZeroInboxCapacity {
        /// Identity whose configured capacity was zero.
        application_id: ApplicationId,
    },
    /// One application listed the same topic more than once.
    DuplicateTopic {
        /// Identity whose immutable topic set contained the duplicate.
        application_id: ApplicationId,
        /// Position of the first equal topic in the application's topic set.
        first_index: usize,
        /// Position of the repeated equal topic.
        duplicate_index: usize,
    },
    /// Storage for the configured endpoint table could not be reserved.
    EndpointStorageAllocationFailed {
        /// Requested number of application endpoints.
        requested: usize,
    },
    /// Storage for one immutable topic set could not be reserved.
    TopicStorageAllocationFailed {
        /// Identity whose topic storage could not be reserved.
        application_id: ApplicationId,
        /// Requested number of topics.
        requested: usize,
    },
    /// Storage for one bounded inbox could not be reserved.
    InboxStorageAllocationFailed {
        /// Identity whose inbox storage could not be reserved.
        application_id: ApplicationId,
        /// Requested number of delivery slots.
        requested: usize,
    },
}

impl fmt::Display for MessageBusCreateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoInboxes => formatter.write_str("at least one application inbox is required"),
            Self::ApplicationOutOfOrder {
                application_id,
                expected_index,
            } => write!(
                formatter,
                "application {application_id:?} is not registration position {expected_index}"
            ),
            Self::ZeroInboxCapacity { application_id } => {
                write!(
                    formatter,
                    "application {application_id:?} has no inbox slots"
                )
            }
            Self::DuplicateTopic {
                application_id,
                first_index,
                duplicate_index,
            } => write!(
                formatter,
                "application {application_id:?} repeats topic {first_index} at {duplicate_index}"
            ),
            Self::EndpointStorageAllocationFailed { requested } => write!(
                formatter,
                "could not reserve messaging storage for {requested} applications"
            ),
            Self::TopicStorageAllocationFailed {
                application_id,
                requested,
            } => write!(
                formatter,
                "could not reserve {requested} topics for application {application_id:?}"
            ),
            Self::InboxStorageAllocationFailed {
                application_id,
                requested,
            } => write!(
                formatter,
                "could not reserve {requested} inbox slots for application {application_id:?}"
            ),
        }
    }
}

impl Error for MessageBusCreateError {}

/// A per-destination result from one serial publication.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DestinationOutcome {
    application_id: ApplicationId,
    status: DeliveryStatus,
}

impl DestinationOutcome {
    /// Returns the destination processed at this stable route position.
    #[must_use]
    pub const fn application_id(&self) -> ApplicationId {
        self.application_id
    }

    /// Returns whether this destination accepted or rejected the delivery.
    #[must_use]
    pub const fn status(&self) -> DeliveryStatus {
        self.status
    }
}

/// The result of attempting delivery to one matching endpoint.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeliveryStatus {
    /// The message was appended to the destination inbox.
    Delivered,
    /// The logical inbox limit was already full; its older entries were kept.
    InboxFull,
    /// The application was known but its lifecycle state was not `Running`.
    Unavailable,
}

/// Publisher-visible summary derived from ordered destination outcomes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PublishClassification {
    /// The published topic had no configured destination.
    NoSubscribers,
    /// Every configured destination accepted the message.
    Complete,
    /// At least one destination accepted and at least one rejected the message.
    Partial,
    /// Configured destinations existed, but none accepted the message.
    WhollyUndelivered,
}

/// Ordered publisher-visible evidence for one serial publication.
///
/// The caller owns this report and may retain it independently of bounded
/// framework inbox storage.
#[derive(Debug, Eq, PartialEq)]
pub struct PublishReport {
    outcomes: Vec<DestinationOutcome>,
}

impl PublishReport {
    /// Classifies delivery from the ordered outcomes without cached state.
    #[must_use]
    pub fn classification(&self) -> PublishClassification {
        if self.outcomes.is_empty() {
            return PublishClassification::NoSubscribers;
        }

        let delivered = self
            .outcomes
            .iter()
            .filter(|outcome| outcome.status == DeliveryStatus::Delivered)
            .count();
        match delivered {
            0 => PublishClassification::WhollyUndelivered,
            count if count == self.outcomes.len() => PublishClassification::Complete,
            _ => PublishClassification::Partial,
        }
    }

    /// Returns destination results in application-registration order.
    #[must_use]
    pub fn outcomes(&self) -> &[DestinationOutcome] {
        &self.outcomes
    }
}

/// Failure before a publication modifies any inbox.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PublishError {
    /// The complete bounded result vector could not be reserved.
    ReportAllocationFailed {
        /// Number of matching destinations whose results required storage.
        destinations: usize,
    },
}

impl fmt::Display for PublishError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReportAllocationFailed { destinations } => write!(
                formatter,
                "could not reserve a publication report for {destinations} destinations"
            ),
        }
    }
}

impl Error for PublishError {}

/// Failure to inspect or consume an inbox outside the configured topology.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InboxAccessError {
    /// The identity does not name an inbox in this message bus.
    UnknownApplication {
        /// Identity supplied by the caller.
        application_id: ApplicationId,
    },
}

impl fmt::Display for InboxAccessError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownApplication { application_id } => {
                write!(
                    formatter,
                    "application {application_id:?} has no message inbox"
                )
            }
        }
    }
}

impl Error for InboxAccessError {}

/// A finite serial routing core with immutable subscriptions.
///
/// Each configuration position must match the same registration position in
/// one [`crate::Runtime`] or [`crate::LifecycleRegistry`]. The existing opaque
/// identity representation cannot detect a same-position key from another
/// issuer. Direct construction and [`MessageBus::publish`] treat all configured
/// endpoints as available.
///
/// Construction also cannot prove that the identity issuer has no additional
/// application records. [`crate::MessagingRuntime`] instead constructs and owns
/// a fresh bus with exactly one inbox per registered application, and derives
/// availability from runtime lifecycle state.
///
/// Queue and topic storage are reserved during construction. Logical inbox
/// limits are enforced independently of allocator capacity. Messages and
/// matching destination results are processed in endpoint registration order.
#[derive(Debug)]
pub struct MessageBus<Topic, const MAX_PAYLOAD_BYTES: usize> {
    endpoints: Vec<ApplicationEndpoint<Topic, MAX_PAYLOAD_BYTES>>,
}

impl<Topic: Copy + Eq, const MAX_PAYLOAD_BYTES: usize> MessageBus<Topic, MAX_PAYLOAD_BYTES> {
    /// Copies and validates the caller-supplied immutable endpoint topology.
    ///
    /// # Errors
    ///
    /// Returns a typed configuration error without producing a partial bus for
    /// an empty topology, an out-of-order identity, a zero inbox capacity, a
    /// repeated topic, or a storage reservation failure.
    pub fn new(
        configurations: &[ApplicationInboxConfig<'_, Topic>],
    ) -> Result<Self, MessageBusCreateError> {
        validate_topology(configurations)?;

        let mut endpoints = Vec::new();
        endpoints
            .try_reserve_exact(configurations.len())
            .map_err(|_| MessageBusCreateError::EndpointStorageAllocationFailed {
                requested: configurations.len(),
            })?;
        for configuration in configurations {
            endpoints.push(create_endpoint(
                configuration.application_id,
                configuration.capacity,
                configuration.topics,
            )?);
        }

        Ok(Self { endpoints })
    }

    /// Publishes without waiting for inbox capacity or subscriber progress.
    ///
    /// Publication does not retry or spill. Matching endpoints are processed
    /// in application-registration order. A full inbox keeps its older entries
    /// and rejects this delivery while unaffected endpoints continue.
    ///
    /// # Errors
    ///
    /// Returns [`PublishError::ReportAllocationFailed`] before any inbox is
    /// modified when storage for all matching destination outcomes cannot be
    /// reserved.
    pub fn publish(
        &mut self,
        message: &Message<Topic, MAX_PAYLOAD_BYTES>,
    ) -> Result<PublishReport, PublishError> {
        self.publish_with_availability(message, |_| true)
    }

    /// Returns the configured logical slot limit for one inbox.
    ///
    /// # Errors
    ///
    /// Returns [`InboxAccessError::UnknownApplication`] when the identity does
    /// not name a configured endpoint.
    pub fn inbox_capacity(&self, application_id: ApplicationId) -> Result<usize, InboxAccessError> {
        Ok(self.endpoint(application_id)?.capacity)
    }

    /// Returns the number of queued messages in one inbox.
    ///
    /// # Errors
    ///
    /// Returns [`InboxAccessError::UnknownApplication`] when the identity does
    /// not name a configured endpoint.
    pub fn pending(&self, application_id: ApplicationId) -> Result<usize, InboxAccessError> {
        Ok(self.endpoint(application_id)?.messages.len())
    }

    /// Removes and returns the oldest queued message for one application.
    ///
    /// This provisional access proves FIFO behavior but does not yet represent
    /// runtime-owned application dispatch or its one in-flight delivery bound.
    ///
    /// # Errors
    ///
    /// Returns [`InboxAccessError::UnknownApplication`] when the identity does
    /// not name a configured endpoint.
    pub fn dequeue(
        &mut self,
        application_id: ApplicationId,
    ) -> Result<Option<Message<Topic, MAX_PAYLOAD_BYTES>>, InboxAccessError> {
        Ok(self.endpoint_mut(application_id)?.messages.pop_front())
    }
}

impl<Topic: Copy + Eq, const MAX_PAYLOAD_BYTES: usize> MessageBus<Topic, MAX_PAYLOAD_BYTES> {
    pub(crate) fn from_runtime_configs(
        configurations: &[RuntimeInboxConfig<'_, Topic>],
    ) -> Result<Self, MessageBusCreateError> {
        validate_runtime_topology(configurations)?;

        let mut endpoints = Vec::new();
        endpoints
            .try_reserve_exact(configurations.len())
            .map_err(|_| MessageBusCreateError::EndpointStorageAllocationFailed {
                requested: configurations.len(),
            })?;
        for (index, configuration) in configurations.iter().enumerate() {
            endpoints.push(create_endpoint(
                ApplicationId::from_index(index),
                configuration.capacity,
                configuration.topics,
            )?);
        }

        Ok(Self { endpoints })
    }

    pub(crate) fn publish_with_availability(
        &mut self,
        message: &Message<Topic, MAX_PAYLOAD_BYTES>,
        mut is_available: impl FnMut(ApplicationId) -> bool,
    ) -> Result<PublishReport, PublishError> {
        let destination_count = self
            .endpoints
            .iter()
            .filter(|endpoint| endpoint.subscribes_to(message.topic()))
            .count();
        let mut outcomes = Vec::new();
        outcomes.try_reserve_exact(destination_count).map_err(|_| {
            PublishError::ReportAllocationFailed {
                destinations: destination_count,
            }
        })?;

        for endpoint in &mut self.endpoints {
            if endpoint.subscribes_to(message.topic()) {
                outcomes.push(DestinationOutcome {
                    application_id: endpoint.application_id,
                    status: endpoint.enqueue(*message, is_available(endpoint.application_id)),
                });
            }
        }
        Ok(PublishReport { outcomes })
    }

    pub(crate) fn clear_runtime_inbox(&mut self, application_id: ApplicationId) -> usize {
        // MessagingRuntime construction fixes this vector to the runtime's
        // complete registration order, and runtime operations validate the ID.
        let endpoint = &mut self.endpoints[application_id.index()];
        debug_assert_eq!(endpoint.application_id, application_id);
        let discarded = endpoint.messages.len();
        endpoint.messages.clear();
        discarded
    }

    fn endpoint(
        &self,
        application_id: ApplicationId,
    ) -> Result<&ApplicationEndpoint<Topic, MAX_PAYLOAD_BYTES>, InboxAccessError> {
        self.endpoints
            .get(application_id.index())
            .ok_or(InboxAccessError::UnknownApplication { application_id })
    }

    fn endpoint_mut(
        &mut self,
        application_id: ApplicationId,
    ) -> Result<&mut ApplicationEndpoint<Topic, MAX_PAYLOAD_BYTES>, InboxAccessError> {
        self.endpoints
            .get_mut(application_id.index())
            .ok_or(InboxAccessError::UnknownApplication { application_id })
    }
}

#[derive(Debug)]
struct ApplicationEndpoint<Topic, const MAX_PAYLOAD_BYTES: usize> {
    application_id: ApplicationId,
    capacity: usize,
    topics: Vec<Topic>,
    messages: VecDeque<Message<Topic, MAX_PAYLOAD_BYTES>>,
}

impl<Topic: Eq, const MAX_PAYLOAD_BYTES: usize> ApplicationEndpoint<Topic, MAX_PAYLOAD_BYTES> {
    fn subscribes_to(&self, topic: &Topic) -> bool {
        self.topics.contains(topic)
    }

    fn enqueue(
        &mut self,
        message: Message<Topic, MAX_PAYLOAD_BYTES>,
        is_available: bool,
    ) -> DeliveryStatus {
        if !is_available {
            return DeliveryStatus::Unavailable;
        }
        if self.messages.len() >= self.capacity {
            return DeliveryStatus::InboxFull;
        }

        self.messages.push_back(message);
        DeliveryStatus::Delivered
    }
}

fn validate_topology<Topic: Eq>(
    configurations: &[ApplicationInboxConfig<'_, Topic>],
) -> Result<(), MessageBusCreateError> {
    if configurations.is_empty() {
        return Err(MessageBusCreateError::NoInboxes);
    }

    for (expected_index, configuration) in configurations.iter().enumerate() {
        if configuration.application_id.index() != expected_index {
            return Err(MessageBusCreateError::ApplicationOutOfOrder {
                application_id: configuration.application_id,
                expected_index,
            });
        }
        validate_endpoint(
            configuration.application_id,
            configuration.capacity,
            configuration.topics,
        )?;
    }
    Ok(())
}

fn validate_runtime_topology<Topic: Eq>(
    configurations: &[RuntimeInboxConfig<'_, Topic>],
) -> Result<(), MessageBusCreateError> {
    if configurations.is_empty() {
        return Err(MessageBusCreateError::NoInboxes);
    }

    for (index, configuration) in configurations.iter().enumerate() {
        validate_endpoint(
            ApplicationId::from_index(index),
            configuration.capacity,
            configuration.topics,
        )?;
    }
    Ok(())
}

fn validate_endpoint<Topic: Eq>(
    application_id: ApplicationId,
    capacity: usize,
    topics: &[Topic],
) -> Result<(), MessageBusCreateError> {
    if capacity == 0 {
        return Err(MessageBusCreateError::ZeroInboxCapacity { application_id });
    }
    validate_unique_topics(application_id, topics)
}

fn validate_unique_topics<Topic: Eq>(
    application_id: ApplicationId,
    topics: &[Topic],
) -> Result<(), MessageBusCreateError> {
    for duplicate_index in 1..topics.len() {
        if let Some(first_index) = topics[..duplicate_index]
            .iter()
            .position(|topic| topic == &topics[duplicate_index])
        {
            return Err(MessageBusCreateError::DuplicateTopic {
                application_id,
                first_index,
                duplicate_index,
            });
        }
    }
    Ok(())
}

fn create_endpoint<Topic: Copy, const MAX_PAYLOAD_BYTES: usize>(
    application_id: ApplicationId,
    capacity: usize,
    configured_topics: &[Topic],
) -> Result<ApplicationEndpoint<Topic, MAX_PAYLOAD_BYTES>, MessageBusCreateError> {
    let mut topics = Vec::new();
    topics
        .try_reserve_exact(configured_topics.len())
        .map_err(|_| MessageBusCreateError::TopicStorageAllocationFailed {
            application_id,
            requested: configured_topics.len(),
        })?;
    topics.extend_from_slice(configured_topics);

    let mut messages = VecDeque::new();
    messages.try_reserve_exact(capacity).map_err(|_| {
        MessageBusCreateError::InboxStorageAllocationFailed {
            application_id,
            requested: capacity,
        }
    })?;
    Ok(ApplicationEndpoint {
        application_id,
        capacity,
        topics,
        messages,
    })
}
