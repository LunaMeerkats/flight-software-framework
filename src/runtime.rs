//! Owned application lifecycle execution.
//!
//! This module adds synchronous start, stop, and in-place restart boundaries
//! over the LC1 lifecycle vocabulary. It deliberately omits work dispatch,
//! factories, threads, executors, panic containment, and recovery policy.

use std::error::Error;
use std::fmt;

use crate::lifecycle::{
    ApplicationId, ApplicationState, LifecycleError, LifecycleOperation, next_state,
};

/// The lifecycle behavior currently required by the owned runtime.
///
/// The runtime invokes these methods synchronously and starts no hidden work.
/// Work, service context, and recovery behaviors are deliberately absent until
/// later slices can define and verify them.
pub trait Application {
    /// The concrete error returned by this application's start operation.
    type StartError: Error + 'static;

    /// Attempts to start the application.
    ///
    /// A returned error is retained in [`RuntimeStartError`] and moves the
    /// runtime record to terminal [`ApplicationState::Failed`]. Panics and
    /// non-returning calls are outside this cooperative failure boundary.
    fn start(&mut self) -> Result<(), Self::StartError>;

    /// The concrete error returned by this application's stop operation.
    type StopError: Error + 'static;

    /// Attempts to stop the application.
    ///
    /// A returned error is retained in [`RuntimeStopError`] and moves the
    /// runtime record to terminal [`ApplicationState::Failed`]. Success records
    /// only that this cooperative callback returned successfully; the runtime
    /// does not independently prove cleanup of application-owned resources.
    fn stop(&mut self) -> Result<(), Self::StopError>;

    /// The concrete error returned by this application's restart operation.
    type RestartError: Error + 'static;

    /// Attempts to restart the stopped application in place.
    ///
    /// The callback receives the same application value retained across its
    /// successful start and stop. A returned error is retained in
    /// [`RuntimeRestartError`] and moves the runtime record to terminal
    /// [`ApplicationState::Failed`]. The runtime does not reconstruct or reset
    /// application-owned state.
    fn restart(&mut self) -> Result<(), Self::RestartError>;
}

/// Failure to construct bounded owned runtime storage.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeCreateError {
    /// The configured capacity was zero.
    ZeroCapacity,
    /// Storage for the configured number of owned application records could not
    /// be reserved.
    CapacityAllocationFailed {
        /// Requested number of owned application records.
        requested: usize,
    },
}

impl fmt::Display for RuntimeCreateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroCapacity => formatter.write_str("application capacity must be positive"),
            Self::CapacityAllocationFailed { requested } => write!(
                formatter,
                "could not reserve runtime storage for {requested} applications"
            ),
        }
    }
}

impl Error for RuntimeCreateError {}

/// A full runtime rejected an application while preserving caller ownership.
pub struct RuntimeRegistrationError<A> {
    capacity: usize,
    application: A,
}

impl<A> RuntimeRegistrationError<A> {
    /// Returns the configured application limit that was already full.
    #[must_use]
    pub const fn capacity(&self) -> usize {
        self.capacity
    }

    /// Borrows the application value that was not registered.
    #[must_use]
    pub const fn application(&self) -> &A {
        &self.application
    }

    /// Returns ownership of the application value that was not registered.
    #[must_use]
    pub fn into_application(self) -> A {
        self.application
    }
}

impl<A> fmt::Debug for RuntimeRegistrationError<A> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RuntimeRegistrationError")
            .field("capacity", &self.capacity)
            .field("application", &"<preserved>")
            .finish()
    }
}

impl<A> fmt::Display for RuntimeRegistrationError<A> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "application runtime is full at capacity {}",
            self.capacity
        )
    }
}

impl<A> Error for RuntimeRegistrationError<A> {}

/// A runtime start request rejected by lifecycle or failed by the application.
#[derive(Debug, Eq, PartialEq)]
pub enum RuntimeStartError<E> {
    /// Lifecycle validation rejected the request before application code ran.
    Lifecycle(LifecycleError),
    /// Application start returned a cooperative failure.
    Application {
        /// Runtime-local identity of the application that returned the error.
        application_id: ApplicationId,
        /// The original concrete error returned by [`Application::start`].
        source: E,
    },
}

impl<E: fmt::Display> fmt::Display for RuntimeStartError<E> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Lifecycle(error) => error.fmt(formatter),
            Self::Application {
                application_id,
                source,
            } => write!(
                formatter,
                "application {application_id:?} returned a start error: {source}"
            ),
        }
    }
}

impl<E: Error + 'static> Error for RuntimeStartError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Lifecycle(error) => Some(error),
            Self::Application { source, .. } => Some(source),
        }
    }
}

/// A runtime stop request rejected by lifecycle or failed by the application.
#[derive(Debug, Eq, PartialEq)]
pub enum RuntimeStopError<E> {
    /// Lifecycle validation rejected the request before application code ran.
    Lifecycle(LifecycleError),
    /// Application stop returned a cooperative failure.
    Application {
        /// Runtime-local identity of the application that returned the error.
        application_id: ApplicationId,
        /// The original concrete error returned by [`Application::stop`].
        source: E,
    },
}

impl<E: fmt::Display> fmt::Display for RuntimeStopError<E> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Lifecycle(error) => error.fmt(formatter),
            Self::Application {
                application_id,
                source,
            } => write!(
                formatter,
                "application {application_id:?} returned a stop error: {source}"
            ),
        }
    }
}

impl<E: Error + 'static> Error for RuntimeStopError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Lifecycle(error) => Some(error),
            Self::Application { source, .. } => Some(source),
        }
    }
}

/// A runtime restart request rejected by lifecycle or failed by the application.
#[derive(Debug, Eq, PartialEq)]
pub enum RuntimeRestartError<E> {
    /// Lifecycle validation rejected the request before application code ran.
    Lifecycle(LifecycleError),
    /// Application restart returned a cooperative failure.
    Application {
        /// Runtime-local identity of the application that returned the error.
        application_id: ApplicationId,
        /// The original concrete error returned by [`Application::restart`].
        source: E,
    },
}

impl<E: fmt::Display> fmt::Display for RuntimeRestartError<E> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Lifecycle(error) => error.fmt(formatter),
            Self::Application {
                application_id,
                source,
            } => write!(
                formatter,
                "application {application_id:?} returned a restart error: {source}"
            ),
        }
    }
}

impl<E: Error + 'static> Error for RuntimeRestartError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Lifecycle(error) => Some(error),
            Self::Application { source, .. } => Some(source),
        }
    }
}

#[derive(Debug)]
struct RuntimeRecord<A> {
    state: ApplicationState,
    application: A,
}

/// A finite-capacity, caller-driven owner of application values.
///
/// This pre-v0.1 slice supports registration, state inspection, and synchronous
/// start, stop, and in-place restart. The concrete application representation
/// is selected by the mission. Different application types can be composed
/// explicitly in an enum without requiring trait-object allocation.
///
/// Construction reserves storage for the configured record count. That bounds
/// the number of runtime records, not memory allocated inside application or
/// error values.
#[derive(Debug)]
pub struct Runtime<A> {
    records: Vec<RuntimeRecord<A>>,
    max_applications: usize,
}

impl<A> Runtime<A> {
    /// Creates owned runtime storage with a positive fixed record capacity.
    ///
    /// # Errors
    ///
    /// Returns an error when capacity is zero or record storage cannot be
    /// reserved without panicking.
    pub fn new(max_applications: usize) -> Result<Self, RuntimeCreateError> {
        if max_applications == 0 {
            return Err(RuntimeCreateError::ZeroCapacity);
        }

        let mut records = Vec::new();
        records.try_reserve_exact(max_applications).map_err(|_| {
            RuntimeCreateError::CapacityAllocationFailed {
                requested: max_applications,
            }
        })?;

        Ok(Self {
            records,
            max_applications,
        })
    }

    /// Returns the configured owned application limit.
    #[must_use]
    pub const fn capacity(&self) -> usize {
        self.max_applications
    }

    /// Returns the number of applications owned by this runtime.
    #[must_use]
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Returns whether this runtime owns no applications.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Transfers an application into the runtime in
    /// [`ApplicationState::Registered`].
    ///
    /// # Errors
    ///
    /// A full runtime returns the rejected value inside
    /// [`RuntimeRegistrationError`] without changing existing records.
    pub fn register(
        &mut self,
        application: A,
    ) -> Result<ApplicationId, RuntimeRegistrationError<A>> {
        if self.records.len() >= self.max_applications {
            return Err(RuntimeRegistrationError {
                capacity: self.max_applications,
                application,
            });
        }

        let application_id = ApplicationId::from_index(self.records.len());
        self.records.push(RuntimeRecord {
            state: ApplicationState::Registered,
            application,
        });
        Ok(application_id)
    }

    /// Returns the current state of an owned application.
    ///
    /// # Errors
    ///
    /// Returns [`LifecycleError::UnknownApplication`] when the identity does
    /// not name a record in this runtime.
    pub fn state(&self, application_id: ApplicationId) -> Result<ApplicationState, LifecycleError> {
        let record = self.record(application_id)?;
        Ok(record.state)
    }

    fn record(&self, application_id: ApplicationId) -> Result<&RuntimeRecord<A>, LifecycleError> {
        self.records
            .get(application_id.index())
            .ok_or(LifecycleError::UnknownApplication { application_id })
    }

    fn record_mut(
        &mut self,
        application_id: ApplicationId,
    ) -> Result<&mut RuntimeRecord<A>, LifecycleError> {
        self.records
            .get_mut(application_id.index())
            .ok_or(LifecycleError::UnknownApplication { application_id })
    }
}

impl<A: Application> Runtime<A> {
    /// Invokes application start after validating `Registered -> Running`.
    ///
    /// Success commits [`ApplicationState::Running`]. A returned application
    /// error is preserved in [`RuntimeStartError::Application`] and commits
    /// terminal [`ApplicationState::Failed`].
    ///
    /// # Errors
    ///
    /// Invalid or unknown lifecycle requests return
    /// [`RuntimeStartError::Lifecycle`] before application code is invoked.
    /// Cooperative application failure returns [`RuntimeStartError::Application`].
    pub fn start(
        &mut self,
        application_id: ApplicationId,
    ) -> Result<ApplicationState, RuntimeStartError<A::StartError>> {
        let record = self
            .record_mut(application_id)
            .map_err(RuntimeStartError::Lifecycle)?;
        let current = record.state;

        if next_state(current, LifecycleOperation::Start) != Some(ApplicationState::Running) {
            return Err(RuntimeStartError::Lifecycle(
                LifecycleError::InvalidTransition {
                    application_id,
                    state: current,
                    operation: LifecycleOperation::Start,
                },
            ));
        }

        match record.application.start() {
            Ok(()) => {
                record.state = ApplicationState::Running;
                Ok(ApplicationState::Running)
            }
            Err(source) => {
                record.state = ApplicationState::Failed;
                Err(RuntimeStartError::Application {
                    application_id,
                    source,
                })
            }
        }
    }

    /// Invokes application stop after validating `Running -> Stopped`.
    ///
    /// Success commits [`ApplicationState::Stopped`]. A returned application
    /// error is preserved in [`RuntimeStopError::Application`] and commits
    /// terminal [`ApplicationState::Failed`].
    ///
    /// # Errors
    ///
    /// Invalid or unknown lifecycle requests return
    /// [`RuntimeStopError::Lifecycle`] before application code is invoked.
    /// Cooperative application failure returns [`RuntimeStopError::Application`].
    pub fn stop(
        &mut self,
        application_id: ApplicationId,
    ) -> Result<ApplicationState, RuntimeStopError<A::StopError>> {
        let record = self
            .record_mut(application_id)
            .map_err(RuntimeStopError::Lifecycle)?;
        let current = record.state;

        if next_state(current, LifecycleOperation::Stop) != Some(ApplicationState::Stopped) {
            return Err(RuntimeStopError::Lifecycle(
                LifecycleError::InvalidTransition {
                    application_id,
                    state: current,
                    operation: LifecycleOperation::Stop,
                },
            ));
        }

        match record.application.stop() {
            Ok(()) => {
                record.state = ApplicationState::Stopped;
                Ok(ApplicationState::Stopped)
            }
            Err(source) => {
                record.state = ApplicationState::Failed;
                Err(RuntimeStopError::Application {
                    application_id,
                    source,
                })
            }
        }
    }

    /// Invokes application restart after validating `Stopped -> Running`.
    ///
    /// The callback mutably borrows the same application value retained across
    /// start and stop. Success commits [`ApplicationState::Running`]. A returned
    /// application error is preserved in [`RuntimeRestartError::Application`]
    /// and commits terminal [`ApplicationState::Failed`].
    ///
    /// # Errors
    ///
    /// Invalid or unknown lifecycle requests return
    /// [`RuntimeRestartError::Lifecycle`] before application code is invoked.
    /// Cooperative application failure returns
    /// [`RuntimeRestartError::Application`].
    pub fn restart(
        &mut self,
        application_id: ApplicationId,
    ) -> Result<ApplicationState, RuntimeRestartError<A::RestartError>> {
        let record = self
            .record_mut(application_id)
            .map_err(RuntimeRestartError::Lifecycle)?;
        let current = record.state;

        if next_state(current, LifecycleOperation::Restart) != Some(ApplicationState::Running) {
            return Err(RuntimeRestartError::Lifecycle(
                LifecycleError::InvalidTransition {
                    application_id,
                    state: current,
                    operation: LifecycleOperation::Restart,
                },
            ));
        }

        match record.application.restart() {
            Ok(()) => {
                record.state = ApplicationState::Running;
                Ok(ApplicationState::Running)
            }
            Err(source) => {
                record.state = ApplicationState::Failed;
                Err(RuntimeRestartError::Application {
                    application_id,
                    source,
                })
            }
        }
    }
}
