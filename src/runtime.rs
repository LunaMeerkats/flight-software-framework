//! Start-only owned application execution.
//!
//! This module adds one synchronous execution boundary over the LC1 lifecycle
//! vocabulary. It deliberately omits work dispatch, stop/restart callbacks,
//! factories, threads, executors, panic containment, and recovery policy.

use std::error::Error;
use std::fmt;

use crate::lifecycle::{
    ApplicationId, ApplicationState, LifecycleError, LifecycleOperation, next_state,
};

/// The start behavior required by the first owned runtime slice.
///
/// The runtime invokes this method synchronously and starts no hidden work.
/// Work, stop, restart, service context, and recovery behaviors are deliberately
/// absent until later slices can define and verify them.
pub trait Application {
    /// The concrete error returned by this application's start operation.
    type StartError: Error + 'static;

    /// Attempts to start the application.
    ///
    /// A returned error is retained in [`RuntimeStartError`] and moves the
    /// runtime record to terminal [`ApplicationState::Failed`]. Panics and
    /// non-returning calls are outside this cooperative failure boundary.
    fn start(&mut self) -> Result<(), Self::StartError>;
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

#[derive(Debug)]
struct RuntimeRecord<A> {
    state: ApplicationState,
    application: A,
}

/// A finite-capacity, caller-driven owner of application values.
///
/// This pre-v0.1 slice supports registration, state inspection, and synchronous
/// start only. The concrete application representation is selected by the
/// mission. Different application types can be composed explicitly in an enum
/// without requiring trait-object allocation.
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
}
