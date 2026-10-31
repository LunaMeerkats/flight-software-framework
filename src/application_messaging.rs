//! Application-facing bounded message handling.
//!
//! This module defines the callback and publish-only context used when
//! [`crate::MessagingRuntime`] dispatches one runtime-owned inbox delivery.
//! It exposes no dequeue, nested dispatch, lifecycle mutation, or raw bus
//! access.

use std::error::Error;
use std::fmt;

use crate::lifecycle::{ApplicationId, ApplicationState, LifecycleError};
use crate::messaging::{
    DeliveryStatus, Message, MessageBus, PublishError, PublishReport, PublishToError,
};
use crate::runtime::Application;

/// Application behavior for one caller-selected queued message.
///
/// This remains separate from [`Application::work`], which represents work not
/// caused by an inbox delivery. The runtime invokes this callback synchronously
/// only for a running application after removing exactly one oldest message.
pub trait MessagingApplication<Topic, const MAX_PAYLOAD_BYTES: usize>: Application {
    /// The concrete error returned by this application's message callback.
    type MessageError: Error + 'static;

    /// Handles one message with publish-only access to the integrated bus.
    ///
    /// The borrowed message is the sole framework-owned in-flight delivery for
    /// this serial callback. The context cannot receive another message or
    /// change lifecycle state.
    ///
    /// # Errors
    ///
    /// Returns an application-defined error when cooperative message handling
    /// fails. The messaging runtime preserves that error, moves the selected
    /// application to terminal [`ApplicationState::Failed`], and clears its
    /// remaining queued deliveries.
    fn handle_message(
        &mut self,
        message: &Message<Topic, MAX_PAYLOAD_BYTES>,
        context: &mut ApplicationMessageContext<'_, Topic, MAX_PAYLOAD_BYTES>,
    ) -> Result<(), Self::MessageError>;
}

/// Publish-only framework access for one application message callback.
///
/// Publication uses lifecycle states captured immediately before the serial
/// callback. The context exposes neither the snapshot nor the underlying bus.
pub struct ApplicationMessageContext<'a, Topic, const MAX_PAYLOAD_BYTES: usize> {
    application_id: ApplicationId,
    message_bus: &'a mut MessageBus<Topic, MAX_PAYLOAD_BYTES>,
    dispatch_states: &'a [ApplicationState],
}

impl<'a, Topic, const MAX_PAYLOAD_BYTES: usize>
    ApplicationMessageContext<'a, Topic, MAX_PAYLOAD_BYTES>
{
    pub(crate) const fn new(
        application_id: ApplicationId,
        message_bus: &'a mut MessageBus<Topic, MAX_PAYLOAD_BYTES>,
        dispatch_states: &'a [ApplicationState],
    ) -> Self {
        Self {
            application_id,
            message_bus,
            dispatch_states,
        }
    }

    /// Returns the runtime-local identity of the application being dispatched.
    #[must_use]
    pub const fn application_id(&self) -> ApplicationId {
        self.application_id
    }
}

impl<Topic: Copy + Eq, const MAX_PAYLOAD_BYTES: usize>
    ApplicationMessageContext<'_, Topic, MAX_PAYLOAD_BYTES>
{
    /// Publishes immediately through the same lifecycle-owned bounded bus.
    ///
    /// Delivery availability is fixed for this synchronous callback because
    /// the context cannot perform lifecycle operations. Publication reports
    /// self-delivery, peer delivery, saturation, and unavailable destinations
    /// through the ordinary ordered result.
    ///
    /// # Errors
    ///
    /// Returns [`PublishError::ReportAllocationFailed`] before changing any
    /// inbox when the complete result cannot be reserved.
    pub fn publish(
        &mut self,
        message: &Message<Topic, MAX_PAYLOAD_BYTES>,
    ) -> Result<PublishReport, PublishError> {
        let dispatch_states = self.dispatch_states;
        self.message_bus
            .publish_with_availability(message, |application_id| {
                dispatch_states.get(application_id.index()) == Some(&ApplicationState::Running)
            })
    }

    /// Attempts publication to one subscriber without touching peer inboxes.
    ///
    /// Membership and subscription checks precede availability and capacity.
    /// Availability uses the same pre-callback lifecycle snapshot as
    /// [`Self::publish`]. This method allocates no report or retry history;
    /// the caller retains the borrowed message after every outcome.
    ///
    /// [`DeliveryStatus::Delivered`] means enqueued, not processed. A later
    /// returned callback error clears the callback owner's inbox,
    /// including accepted self-publications, while retaining accepted peer
    /// copies. Repeated calls can enqueue duplicates.
    ///
    /// # Errors
    ///
    /// Returns [`PublishToError::UnknownApplication`] for an identity outside
    /// this bus or [`PublishToError::NotSubscribed`] for a known destination
    /// without the topic. Both errors leave every inbox unchanged. Identities
    /// remain caller-scoped; same-position foreign identities can alias.
    pub fn publish_to(
        &mut self,
        application_id: ApplicationId,
        message: &Message<Topic, MAX_PAYLOAD_BYTES>,
    ) -> Result<DeliveryStatus, PublishToError> {
        let dispatch_states = self.dispatch_states;
        self.message_bus
            .publish_to_with_availability(application_id, message, |application_id| {
                dispatch_states.get(application_id.index()) == Some(&ApplicationState::Running)
            })
    }
}

/// Successful result of one caller-selected dispatch attempt.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MessageDispatchOutcome {
    /// The running application's inbox was empty; no callback ran.
    InboxEmpty,
    /// Exactly one oldest message was presented and the callback succeeded.
    Dispatched,
}

/// Failure before or during one application message callback.
#[derive(Debug, Eq, PartialEq)]
pub enum MessageDispatchError<E> {
    /// Identity or running-state validation rejected the request.
    Lifecycle(LifecycleError),
    /// The selected application returned its concrete message error.
    Application {
        /// Runtime-local identity selected for dispatch.
        application_id: ApplicationId,
        /// Original concrete error returned by [`MessagingApplication`].
        source: E,
    },
}

impl<E: fmt::Display> fmt::Display for MessageDispatchError<E> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Lifecycle(error) => error.fmt(formatter),
            Self::Application {
                application_id,
                source,
            } => write!(
                formatter,
                "application {application_id:?} returned a message error: {source}"
            ),
        }
    }
}

impl<E: Error + 'static> Error for MessageDispatchError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Lifecycle(error) => Some(error),
            Self::Application { source, .. } => Some(source),
        }
    }
}
