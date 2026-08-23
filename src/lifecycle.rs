//! Bounded application identity and lifecycle-state supervision.
//!
//! This module implements the standalone logical LC1 transition model. The
//! owned [`crate::Runtime`] uses the same state and error vocabulary for its
//! synchronous lifecycle execution boundary.

use std::error::Error;
use std::fmt;

/// An opaque slot key allocated by one lifecycle registry or runtime.
///
/// Callers must scope the key to the registry or runtime that issued it. This
/// initial representation does not encode registry origin: equal-position keys
/// from separate issuers can compare equal and can address the corresponding
/// slot. It is not a persistent mission identifier or wire-format value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ApplicationId(usize);

impl ApplicationId {
    pub(crate) const fn from_index(index: usize) -> Self {
        Self(index)
    }

    pub(crate) const fn index(self) -> usize {
        self.0
    }
}

/// A stable LC1 application lifecycle state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ApplicationState {
    /// Registered but never successfully started.
    Registered,
    /// Eligible for caller-driven application work.
    Running,
    /// Cleanly stopped and eligible for an in-place restart.
    Stopped,
    /// An application lifecycle or work operation returned an error.
    ///
    /// This state is terminal in LC1. The standalone registry cannot enter it,
    /// while [`crate::Runtime`] enters it when application start, work, stop,
    /// or restart returns an error.
    Failed,
}

/// An LC1 lifecycle operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LifecycleOperation {
    /// Start a registered application for the first time.
    Start,
    /// Stop a running application.
    Stop,
    /// Restart a stopped application in place.
    Restart,
}

/// Failure to construct a bounded registry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RegistryCreateError {
    /// The configured capacity was zero.
    ZeroCapacity,
    /// Storage for the configured number of records could not be reserved.
    CapacityAllocationFailed {
        /// Requested number of application records.
        requested: usize,
    },
}

impl fmt::Display for RegistryCreateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroCapacity => formatter.write_str("application capacity must be positive"),
            Self::CapacityAllocationFailed { requested } => write!(
                formatter,
                "could not reserve lifecycle storage for {requested} applications"
            ),
        }
    }
}

impl Error for RegistryCreateError {}

/// Failure to register another logical application record.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RegistrationError {
    /// The registry already contains its configured maximum number of records.
    RegistryFull {
        /// Configured maximum number of application records.
        capacity: usize,
    },
}

impl fmt::Display for RegistrationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RegistryFull { capacity } => {
                write!(
                    formatter,
                    "application registry is full at capacity {capacity}"
                )
            }
        }
    }
}

impl Error for RegistrationError {}

/// A lifecycle or running-only work request rejected by the logical registry or
/// owned runtime.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LifecycleError {
    /// The identity's slot does not name a record in this registry or runtime.
    ///
    /// This does not detect a key issued by another registry or runtime when
    /// that key's slot exists in the addressed owner.
    UnknownApplication {
        /// Requested runtime-local identity.
        application_id: ApplicationId,
    },
    /// Application work requires a record in [`ApplicationState::Running`].
    NotRunning {
        /// Requested runtime-local identity.
        application_id: ApplicationId,
        /// Current state, which remains unchanged because application work was
        /// not invoked.
        state: ApplicationState,
    },
    /// The operation is not valid in the record's current state.
    InvalidTransition {
        /// Requested runtime-local identity.
        application_id: ApplicationId,
        /// Current state, which remains unchanged.
        state: ApplicationState,
        /// Rejected operation.
        operation: LifecycleOperation,
    },
}

impl fmt::Display for LifecycleError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownApplication { application_id } => {
                write!(
                    formatter,
                    "application {application_id:?} is not registered"
                )
            }
            Self::NotRunning {
                application_id,
                state,
            } => write!(
                formatter,
                "application {application_id:?} cannot perform work while {state:?}"
            ),
            Self::InvalidTransition {
                application_id,
                state,
                operation,
            } => write!(
                formatter,
                "application {application_id:?} cannot perform {operation:?} while {state:?}"
            ),
        }
    }
}

impl Error for LifecycleError {}

#[derive(Debug)]
struct ApplicationRecord {
    state: ApplicationState,
}

/// A bounded registry of runtime-local identities and LC1 lifecycle records.
///
/// Construction reserves storage for the full configured record count. The
/// registry provides no removal operation, so identities remain stable and are
/// never reused during its lifetime.
#[derive(Debug)]
pub struct LifecycleRegistry {
    records: Vec<ApplicationRecord>,
    max_applications: usize,
}

impl LifecycleRegistry {
    /// Creates a registry with a positive, fixed application capacity.
    ///
    /// # Errors
    ///
    /// Returns an error when the capacity is zero or cannot be reserved without
    /// panicking.
    pub fn new(max_applications: usize) -> Result<Self, RegistryCreateError> {
        if max_applications == 0 {
            return Err(RegistryCreateError::ZeroCapacity);
        }

        let mut records = Vec::new();
        records.try_reserve_exact(max_applications).map_err(|_| {
            RegistryCreateError::CapacityAllocationFailed {
                requested: max_applications,
            }
        })?;

        Ok(Self {
            records,
            max_applications,
        })
    }

    /// Returns the configured logical record limit.
    #[must_use]
    pub const fn capacity(&self) -> usize {
        self.max_applications
    }

    /// Returns the number of registered logical applications.
    #[must_use]
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Returns whether no logical applications are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Allocates one runtime-local identity in [`ApplicationState::Registered`].
    ///
    /// # Errors
    ///
    /// Returns [`RegistrationError::RegistryFull`] without mutation when the
    /// configured logical limit has been reached.
    pub fn register(&mut self) -> Result<ApplicationId, RegistrationError> {
        if self.records.len() >= self.max_applications {
            return Err(RegistrationError::RegistryFull {
                capacity: self.max_applications,
            });
        }

        let application_id = ApplicationId(self.records.len());
        self.records.push(ApplicationRecord {
            state: ApplicationState::Registered,
        });
        Ok(application_id)
    }

    /// Returns the current state of a registered logical application.
    ///
    /// # Errors
    ///
    /// Returns [`LifecycleError::UnknownApplication`] when the identity does
    /// not name a record in this registry.
    pub fn state(&self, application_id: ApplicationId) -> Result<ApplicationState, LifecycleError> {
        let record = self.record(application_id)?;
        Ok(record.state)
    }

    /// Applies `Registered -> Running`.
    ///
    /// # Errors
    ///
    /// Returns a typed error without mutation for an unknown identity or any
    /// state other than [`ApplicationState::Registered`].
    pub fn start(
        &mut self,
        application_id: ApplicationId,
    ) -> Result<ApplicationState, LifecycleError> {
        self.apply(application_id, LifecycleOperation::Start)
    }

    /// Applies `Running -> Stopped`.
    ///
    /// # Errors
    ///
    /// Returns a typed error without mutation for an unknown identity or any
    /// state other than [`ApplicationState::Running`].
    pub fn stop(
        &mut self,
        application_id: ApplicationId,
    ) -> Result<ApplicationState, LifecycleError> {
        self.apply(application_id, LifecycleOperation::Stop)
    }

    /// Applies the LC1 in-place transition `Stopped -> Running`.
    ///
    /// This changes only the logical lifecycle record. Application-object
    /// retention is not implemented by this registry slice.
    ///
    /// # Errors
    ///
    /// Returns a typed error without mutation for an unknown identity or any
    /// state other than [`ApplicationState::Stopped`].
    pub fn restart(
        &mut self,
        application_id: ApplicationId,
    ) -> Result<ApplicationState, LifecycleError> {
        self.apply(application_id, LifecycleOperation::Restart)
    }

    fn apply(
        &mut self,
        application_id: ApplicationId,
        operation: LifecycleOperation,
    ) -> Result<ApplicationState, LifecycleError> {
        let record = self.record_mut(application_id)?;
        let current = record.state;
        let Some(next) = next_state(current, operation) else {
            return Err(LifecycleError::InvalidTransition {
                application_id,
                state: current,
                operation,
            });
        };

        record.state = next;
        Ok(next)
    }

    fn record(&self, application_id: ApplicationId) -> Result<&ApplicationRecord, LifecycleError> {
        self.records
            .get(application_id.0)
            .ok_or(LifecycleError::UnknownApplication { application_id })
    }

    fn record_mut(
        &mut self,
        application_id: ApplicationId,
    ) -> Result<&mut ApplicationRecord, LifecycleError> {
        self.records
            .get_mut(application_id.0)
            .ok_or(LifecycleError::UnknownApplication { application_id })
    }
}

pub(crate) const fn next_state(
    state: ApplicationState,
    operation: LifecycleOperation,
) -> Option<ApplicationState> {
    match (state, operation) {
        (ApplicationState::Registered, LifecycleOperation::Start) => {
            Some(ApplicationState::Running)
        }
        (ApplicationState::Registered, LifecycleOperation::Stop)
        | (ApplicationState::Registered, LifecycleOperation::Restart) => None,
        (ApplicationState::Running, LifecycleOperation::Start)
        | (ApplicationState::Running, LifecycleOperation::Restart) => None,
        (ApplicationState::Running, LifecycleOperation::Stop) => Some(ApplicationState::Stopped),
        (ApplicationState::Stopped, LifecycleOperation::Start)
        | (ApplicationState::Stopped, LifecycleOperation::Stop) => None,
        (ApplicationState::Stopped, LifecycleOperation::Restart) => Some(ApplicationState::Running),
        (ApplicationState::Failed, LifecycleOperation::Start)
        | (ApplicationState::Failed, LifecycleOperation::Stop)
        | (ApplicationState::Failed, LifecycleOperation::Restart) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const STATES: [ApplicationState; 4] = [
        ApplicationState::Registered,
        ApplicationState::Running,
        ApplicationState::Stopped,
        ApplicationState::Failed,
    ];
    const OPERATIONS: [LifecycleOperation; 3] = [
        LifecycleOperation::Start,
        LifecycleOperation::Stop,
        LifecycleOperation::Restart,
    ];

    #[test]
    fn registry_enforces_every_state_operation_pair() {
        let expected = [
            Some(ApplicationState::Running),
            None,
            None,
            None,
            Some(ApplicationState::Stopped),
            None,
            None,
            None,
            Some(ApplicationState::Running),
            None,
            None,
            None,
        ];

        for ((state, operation), expected_state) in STATES
            .into_iter()
            .flat_map(|state| {
                OPERATIONS
                    .into_iter()
                    .map(move |operation| (state, operation))
            })
            .zip(expected)
        {
            let mut registry = LifecycleRegistry::new(1).expect("test capacity is valid");
            let application_id = registry.register().expect("test record fits");
            registry.records[application_id.0].state = state;

            let result = match operation {
                LifecycleOperation::Start => registry.start(application_id),
                LifecycleOperation::Stop => registry.stop(application_id),
                LifecycleOperation::Restart => registry.restart(application_id),
            };

            match expected_state {
                Some(next) => {
                    assert_eq!(result, Ok(next));
                    assert_eq!(registry.state(application_id), Ok(next));
                }
                None => {
                    assert_eq!(
                        result,
                        Err(LifecycleError::InvalidTransition {
                            application_id,
                            state,
                            operation,
                        })
                    );
                    assert_eq!(registry.state(application_id), Ok(state));
                }
            }
        }
    }

    #[test]
    fn unknown_identity_is_rejected_without_mutating_existing_records() {
        let mut registry = LifecycleRegistry::new(1).expect("test capacity is valid");
        let known_id = registry.register().expect("test record fits");
        let unknown_id = ApplicationId(1);

        assert_eq!(
            registry.start(unknown_id),
            Err(LifecycleError::UnknownApplication {
                application_id: unknown_id,
            })
        );
        assert_eq!(registry.state(known_id), Ok(ApplicationState::Registered));
    }
}
