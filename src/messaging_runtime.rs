//! Runtime ownership of lifecycle-aware message availability.
//!
//! This module couples one fresh bounded inbox to every application in a
//! fully composed [`Runtime`]. Only running applications accept deliveries.
//! Stop and returned callback errors clear the selected inbox before returning,
//! while successful stopped-to-running restart exposes the already-empty inbox.

use std::error::Error;
use std::fmt;

use crate::application_messaging::{
    ApplicationMessageContext, MessageDispatchError, MessageDispatchOutcome, MessagingApplication,
};
use crate::lifecycle::{ApplicationId, ApplicationState, LifecycleError};
use crate::messaging::{
    InboxAccessError, Message, MessageBus, MessageBusCreateError, PublishError, PublishReport,
    RuntimeInboxConfig,
};
use crate::runtime::{
    Application, RunningCallbackError, Runtime, RuntimeRestartError, RuntimeStartError,
    RuntimeStopError, RuntimeWorkError,
};

/// The reason a runtime could not take ownership of a message topology.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MessagingRuntimeCreateErrorKind {
    /// The topology did not provide exactly one inbox per application.
    InboxCountMismatch {
        /// Number of application records already owned by the runtime.
        applications: usize,
        /// Number of supplied runtime inbox configurations.
        inboxes: usize,
    },
    /// An application had already left its initial registered state.
    ApplicationNotRegistered {
        /// Runtime-local identity of the record that was not attachable.
        application_id: ApplicationId,
        /// State observed before any message topology was constructed.
        state: ApplicationState,
    },
    /// The fresh bounded message topology was invalid or could not be reserved.
    MessageBus(MessageBusCreateError),
    /// Storage for one lifecycle-state snapshot entry per application could not
    /// be reserved.
    DispatchStateStorageAllocationFailed {
        /// Requested number of application-state entries.
        requested: usize,
    },
}

impl fmt::Display for MessagingRuntimeCreateErrorKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InboxCountMismatch {
                applications,
                inboxes,
            } => write!(
                formatter,
                "runtime owns {applications} applications but received {inboxes} inboxes"
            ),
            Self::ApplicationNotRegistered {
                application_id,
                state,
            } => write!(
                formatter,
                "application {application_id:?} is {state:?}, not Registered"
            ),
            Self::MessageBus(error) => error.fmt(formatter),
            Self::DispatchStateStorageAllocationFailed { requested } => write!(
                formatter,
                "could not reserve dispatch state storage for {requested} applications"
            ),
        }
    }
}

impl Error for MessagingRuntimeCreateErrorKind {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::MessageBus(error) => Some(error),
            Self::InboxCountMismatch { .. }
            | Self::ApplicationNotRegistered { .. }
            | Self::DispatchStateStorageAllocationFailed { .. } => None,
        }
    }
}

/// Failed lifecycle-aware messaging construction with the runtime preserved.
pub struct MessagingRuntimeCreateError<A> {
    kind: MessagingRuntimeCreateErrorKind,
    runtime: Runtime<A>,
}

impl<A> MessagingRuntimeCreateError<A> {
    /// Returns the construction failure without exposing owned applications.
    #[must_use]
    pub const fn kind(&self) -> MessagingRuntimeCreateErrorKind {
        self.kind
    }

    /// Borrows the unchanged runtime that could not be attached.
    #[must_use]
    pub const fn runtime(&self) -> &Runtime<A> {
        &self.runtime
    }

    /// Returns ownership of the unchanged runtime.
    #[must_use]
    pub fn into_runtime(self) -> Runtime<A> {
        self.runtime
    }
}

impl<A> fmt::Debug for MessagingRuntimeCreateError<A> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("MessagingRuntimeCreateError")
            .field("kind", &self.kind)
            .field("runtime", &"<preserved>")
            .finish()
    }
}

impl<A> fmt::Display for MessagingRuntimeCreateError<A> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.kind.fmt(formatter)
    }
}

impl<A> Error for MessagingRuntimeCreateError<A> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.kind)
    }
}

/// A lifecycle or callback error plus deliveries cleared by that operation.
#[derive(Debug, Eq, PartialEq)]
pub struct MessagingOperationError<E> {
    operation_error: E,
    discarded_deliveries: usize,
}

impl<E> MessagingOperationError<E> {
    /// Borrows the original operation-specific runtime error.
    #[must_use]
    pub const fn operation_error(&self) -> &E {
        &self.operation_error
    }

    /// Returns the number of queued delivery records cleared by the operation.
    #[must_use]
    pub const fn discarded_deliveries(&self) -> usize {
        self.discarded_deliveries
    }

    /// Returns ownership of the original operation-specific runtime error.
    #[must_use]
    pub fn into_operation_error(self) -> E {
        self.operation_error
    }
}

impl<E: fmt::Display> fmt::Display for MessagingOperationError<E> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.discarded_deliveries == 0 {
            return self.operation_error.fmt(formatter);
        }
        write!(
            formatter,
            "{}; discarded {} queued deliveries",
            self.operation_error, self.discarded_deliveries
        )
    }
}

impl<E: Error + 'static> Error for MessagingOperationError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.operation_error)
    }
}

/// Successful stop state and its exact queue-clearing result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MessagingStopOutcome {
    state: ApplicationState,
    discarded_deliveries: usize,
}

impl MessagingStopOutcome {
    /// Returns the stopped state committed before queue clearing.
    #[must_use]
    pub const fn state(&self) -> ApplicationState {
        self.state
    }

    /// Returns the number of queued delivery records cleared by this stop.
    #[must_use]
    pub const fn discarded_deliveries(&self) -> usize {
        self.discarded_deliveries
    }
}

/// A caller-driven runtime that owns a lifecycle-synchronized message bus.
///
/// Construction consumes a runtime whose complete application set is still
/// [`ApplicationState::Registered`] and creates one fresh empty inbox per
/// record. Registration is then frozen because neither inner component is
/// exposed mutably. Publication derives availability from runtime state:
/// only `Running` endpoints accept deliveries.
#[derive(Debug)]
pub struct MessagingRuntime<A, Topic, const MAX_PAYLOAD_BYTES: usize> {
    runtime: Runtime<A>,
    message_bus: MessageBus<Topic, MAX_PAYLOAD_BYTES>,
    dispatch_states: Vec<ApplicationState>,
}

impl<A, Topic: Copy + Eq, const MAX_PAYLOAD_BYTES: usize>
    MessagingRuntime<A, Topic, MAX_PAYLOAD_BYTES>
{
    /// Couples a fully composed registered runtime to one fresh inbox per app.
    ///
    /// The supplied configurations are copied in application-registration
    /// order. The caller does not provide identities, so a foreign issuer or
    /// incomplete topology cannot be attached accidentally.
    ///
    /// # Errors
    ///
    /// Returns the owned runtime inside [`MessagingRuntimeCreateError`] when
    /// counts differ, any application is no longer `Registered`, or fresh bus
    /// or dispatch-state storage cannot be reserved. No partial integration is
    /// returned.
    pub fn new(
        runtime: Runtime<A>,
        configurations: &[RuntimeInboxConfig<'_, Topic>],
    ) -> Result<Self, MessagingRuntimeCreateError<A>> {
        let application_count = runtime.len();
        if application_count != configurations.len() {
            return Err(Self::creation_error(
                runtime,
                MessagingRuntimeCreateErrorKind::InboxCountMismatch {
                    applications: application_count,
                    inboxes: configurations.len(),
                },
            ));
        }
        if let Some((application_id, state)) = runtime.first_non_registered() {
            return Err(Self::creation_error(
                runtime,
                MessagingRuntimeCreateErrorKind::ApplicationNotRegistered {
                    application_id,
                    state,
                },
            ));
        }

        let message_bus = match MessageBus::from_runtime_configs(configurations) {
            Ok(message_bus) => message_bus,
            Err(error) => {
                return Err(Self::creation_error(
                    runtime,
                    MessagingRuntimeCreateErrorKind::MessageBus(error),
                ));
            }
        };
        let dispatch_states = match create_dispatch_state_storage(application_count) {
            Ok(dispatch_states) => dispatch_states,
            Err(kind) => return Err(Self::creation_error(runtime, kind)),
        };
        Ok(Self {
            runtime,
            message_bus,
            dispatch_states,
        })
    }

    /// Returns the configured application-record limit.
    #[must_use]
    pub const fn capacity(&self) -> usize {
        self.runtime.capacity()
    }

    /// Returns the current state of one owned application.
    ///
    /// # Errors
    ///
    /// Returns [`LifecycleError::UnknownApplication`] when the identity does
    /// not name an application in this runtime.
    pub fn state(&self, application_id: ApplicationId) -> Result<ApplicationState, LifecycleError> {
        self.runtime.state(application_id)
    }

    /// Publishes to matching endpoints using current runtime state.
    ///
    /// Matching `Registered`, `Stopped`, and `Failed` applications remain in
    /// the ordered report as [`crate::DeliveryStatus::Unavailable`]. Only
    /// `Running` endpoints may enqueue.
    ///
    /// # Errors
    ///
    /// Returns [`PublishError::ReportAllocationFailed`] before any inbox is
    /// modified when the complete ordered report cannot be reserved.
    pub fn publish(
        &mut self,
        message: &Message<Topic, MAX_PAYLOAD_BYTES>,
    ) -> Result<PublishReport, PublishError> {
        let runtime = &self.runtime;
        self.message_bus
            .publish_with_availability(message, |application_id| {
                runtime.state(application_id) == Ok(ApplicationState::Running)
            })
    }

    /// Returns the number of queued deliveries for one application.
    ///
    /// # Errors
    ///
    /// Returns [`InboxAccessError::UnknownApplication`] when the identity does
    /// not name an integrated inbox.
    pub fn pending(&self, application_id: ApplicationId) -> Result<usize, InboxAccessError> {
        self.message_bus.pending(application_id)
    }

    /// Returns the configured logical slot limit for one application inbox.
    ///
    /// # Errors
    ///
    /// Returns [`InboxAccessError::UnknownApplication`] when the identity does
    /// not name an integrated inbox.
    pub fn inbox_capacity(&self, application_id: ApplicationId) -> Result<usize, InboxAccessError> {
        self.message_bus.inbox_capacity(application_id)
    }
}

impl<A: Application, Topic: Copy + Eq, const MAX_PAYLOAD_BYTES: usize>
    MessagingRuntime<A, Topic, MAX_PAYLOAD_BYTES>
{
    /// Starts a registered application and exposes its empty inbox on success.
    ///
    /// # Errors
    ///
    /// Preserves the operation-specific start error. A returned callback error
    /// commits terminal `Failed` and clears the selected inbox before return;
    /// the count is normally zero because registered endpoints are unavailable.
    pub fn start(
        &mut self,
        application_id: ApplicationId,
    ) -> Result<ApplicationState, MessagingOperationError<RuntimeStartError<A::StartError>>> {
        match self.runtime.start(application_id) {
            Ok(state) => Ok(state),
            Err(error @ RuntimeStartError::Application { .. }) => {
                Err(self.cleared_error(application_id, error))
            }
            Err(error @ RuntimeStartError::Lifecycle(_)) => {
                Err(MessagingOperationError::without_clearing(error))
            }
        }
    }

    /// Runs one work callback while preserving lifecycle-derived availability.
    ///
    /// # Errors
    ///
    /// Preserves the operation-specific work error. A returned callback error
    /// commits terminal `Failed` and clears the selected inbox before return.
    /// Lifecycle rejection does not clear or otherwise mutate the inbox.
    pub fn work(
        &mut self,
        application_id: ApplicationId,
    ) -> Result<ApplicationState, MessagingOperationError<RuntimeWorkError<A::WorkError>>> {
        match self.runtime.work(application_id) {
            Ok(state) => Ok(state),
            Err(error @ RuntimeWorkError::Application { .. }) => {
                Err(self.cleared_error(application_id, error))
            }
            Err(error @ RuntimeWorkError::Lifecycle(_)) => {
                Err(MessagingOperationError::without_clearing(error))
            }
        }
    }

    /// Stops a running application and clears its inbox before returning.
    ///
    /// # Errors
    ///
    /// Preserves the operation-specific stop error. A returned callback error
    /// commits terminal `Failed` and clears the selected inbox. Lifecycle
    /// rejection does not clear or otherwise mutate the inbox.
    pub fn stop(
        &mut self,
        application_id: ApplicationId,
    ) -> Result<MessagingStopOutcome, MessagingOperationError<RuntimeStopError<A::StopError>>> {
        match self.runtime.stop(application_id) {
            Ok(state) => Ok(MessagingStopOutcome {
                state,
                discarded_deliveries: self.message_bus.clear_runtime_inbox(application_id),
            }),
            Err(error @ RuntimeStopError::Application { .. }) => {
                Err(self.cleared_error(application_id, error))
            }
            Err(error @ RuntimeStopError::Lifecycle(_)) => {
                Err(MessagingOperationError::without_clearing(error))
            }
        }
    }

    /// Restarts a stopped application and reconnects its already-empty inbox.
    ///
    /// Terminal `Failed` records cannot restart under LC1.
    ///
    /// # Errors
    ///
    /// Preserves the operation-specific restart error. A returned callback
    /// error commits terminal `Failed` and clears the selected inbox before
    /// return; the count is normally zero because stopped endpoints are empty.
    pub fn restart(
        &mut self,
        application_id: ApplicationId,
    ) -> Result<ApplicationState, MessagingOperationError<RuntimeRestartError<A::RestartError>>>
    {
        match self.runtime.restart(application_id) {
            Ok(state) => Ok(state),
            Err(error @ RuntimeRestartError::Application { .. }) => {
                Err(self.cleared_error(application_id, error))
            }
            Err(error @ RuntimeRestartError::Lifecycle(_)) => {
                Err(MessagingOperationError::without_clearing(error))
            }
        }
    }
}

impl<
    A: MessagingApplication<Topic, MAX_PAYLOAD_BYTES>,
    Topic: Copy + Eq,
    const MAX_PAYLOAD_BYTES: usize,
> MessagingRuntime<A, Topic, MAX_PAYLOAD_BYTES>
{
    /// Dispatches at most one oldest message to one running application.
    ///
    /// A running application with an empty inbox returns
    /// [`MessageDispatchOutcome::InboxEmpty`] without invoking application
    /// code. A dispatched message remains outside inbox capacity for the
    /// synchronous callback, whose context can publish through this same bus.
    ///
    /// # Errors
    ///
    /// Identity or running-state rejection occurs before dequeue. A returned
    /// application error commits terminal `Failed`, drops the attempted
    /// in-flight message, and clears the selected application's remaining
    /// queued deliveries. The exact clear count excludes the in-flight item.
    /// Deliveries accepted by peers during the callback are not rolled back.
    pub fn dispatch_one(
        &mut self,
        application_id: ApplicationId,
    ) -> Result<
        MessageDispatchOutcome,
        MessagingOperationError<MessageDispatchError<A::MessageError>>,
    > {
        self.validate_dispatch_state(application_id)?;
        self.runtime.copy_states_into(&mut self.dispatch_states);

        let result = self.runtime.invoke_running(application_id, |application| {
            let Some(message) = self.message_bus.dequeue_runtime_inbox(application_id) else {
                return Ok(MessageDispatchOutcome::InboxEmpty);
            };
            let mut context = ApplicationMessageContext::new(
                application_id,
                &mut self.message_bus,
                &self.dispatch_states,
            );
            application.handle_message(&message, &mut context)?;
            Ok(MessageDispatchOutcome::Dispatched)
        });

        match result {
            Ok(outcome) => Ok(outcome),
            Err(RunningCallbackError::Lifecycle(error)) => Err(
                MessagingOperationError::without_clearing(MessageDispatchError::Lifecycle(error)),
            ),
            Err(RunningCallbackError::Application(source)) => Err(self.cleared_error(
                application_id,
                MessageDispatchError::Application {
                    application_id,
                    source,
                },
            )),
        }
    }
}

impl<A, Topic: Copy + Eq, const MAX_PAYLOAD_BYTES: usize>
    MessagingRuntime<A, Topic, MAX_PAYLOAD_BYTES>
{
    fn creation_error(
        runtime: Runtime<A>,
        kind: MessagingRuntimeCreateErrorKind,
    ) -> MessagingRuntimeCreateError<A> {
        MessagingRuntimeCreateError { kind, runtime }
    }

    fn validate_dispatch_state<E>(
        &self,
        application_id: ApplicationId,
    ) -> Result<(), MessagingOperationError<MessageDispatchError<E>>> {
        match self.runtime.state(application_id) {
            Ok(ApplicationState::Running) => Ok(()),
            Ok(state) => Err(MessagingOperationError::without_clearing(
                MessageDispatchError::Lifecycle(LifecycleError::NotRunning {
                    application_id,
                    state,
                }),
            )),
            Err(error) => Err(MessagingOperationError::without_clearing(
                MessageDispatchError::Lifecycle(error),
            )),
        }
    }
}

impl<A: Application, Topic: Copy + Eq, const MAX_PAYLOAD_BYTES: usize>
    MessagingRuntime<A, Topic, MAX_PAYLOAD_BYTES>
{
    fn cleared_error<E>(
        &mut self,
        application_id: ApplicationId,
        operation_error: E,
    ) -> MessagingOperationError<E> {
        MessagingOperationError {
            operation_error,
            discarded_deliveries: self.message_bus.clear_runtime_inbox(application_id),
        }
    }
}

impl<E> MessagingOperationError<E> {
    const fn without_clearing(operation_error: E) -> Self {
        Self {
            operation_error,
            discarded_deliveries: 0,
        }
    }
}

fn create_dispatch_state_storage(
    application_count: usize,
) -> Result<Vec<ApplicationState>, MessagingRuntimeCreateErrorKind> {
    let mut states = Vec::new();
    states.try_reserve_exact(application_count).map_err(|_| {
        MessagingRuntimeCreateErrorKind::DispatchStateStorageAllocationFailed {
            requested: application_count,
        }
    })?;
    states.resize(application_count, ApplicationState::Registered);
    Ok(states)
}
