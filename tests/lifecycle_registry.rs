use rust_flight_framework::{
    ApplicationState, LifecycleError, LifecycleOperation, LifecycleRegistry, RegistrationError,
    RegistryCreateError,
};

#[test]
fn oversized_registry_capacity_returns_exact_reservation_error() {
    assert_eq!(
        LifecycleRegistry::new(usize::MAX)
            .expect_err("an impossible record capacity must be rejected"),
        RegistryCreateError::CapacityAllocationFailed {
            requested: usize::MAX,
        }
    );
}

#[test]
fn configured_capacity_bounds_registration_without_mutating_existing_records() {
    assert_eq!(
        LifecycleRegistry::new(0).expect_err("zero capacity is invalid"),
        RegistryCreateError::ZeroCapacity
    );

    let mut registry = LifecycleRegistry::new(2).expect("small capacity can be reserved");
    let first = registry.register().expect("first record fits");
    let second = registry.register().expect("second record fits");

    assert_ne!(first, second);
    assert_eq!(registry.capacity(), 2);
    assert_eq!(registry.len(), 2);
    assert_eq!(
        registry.register(),
        Err(RegistrationError::RegistryFull { capacity: 2 })
    );
    assert_eq!(registry.len(), 2);
    assert_eq!(registry.state(first), Ok(ApplicationState::Registered));
    assert_eq!(registry.state(second), Ok(ApplicationState::Registered));
}

#[test]
fn approved_lc1_sequence_preserves_identity_and_registry_bound() {
    let mut registry = LifecycleRegistry::new(1).expect("test capacity is valid");
    let application_id = registry.register().expect("test record fits");

    assert_eq!(
        registry.start(application_id),
        Ok(ApplicationState::Running)
    );
    assert_eq!(registry.stop(application_id), Ok(ApplicationState::Stopped));
    assert_eq!(
        registry.restart(application_id),
        Ok(ApplicationState::Running)
    );

    assert_eq!(
        registry.state(application_id),
        Ok(ApplicationState::Running)
    );
    assert_eq!(registry.len(), 1);
    assert_eq!(registry.capacity(), 1);
}

#[test]
fn invalid_transition_is_typed_and_leaves_state_unchanged() {
    let mut registry = LifecycleRegistry::new(1).expect("test capacity is valid");
    let application_id = registry.register().expect("test record fits");

    assert_eq!(
        registry.restart(application_id),
        Err(LifecycleError::InvalidTransition {
            application_id,
            state: ApplicationState::Registered,
            operation: LifecycleOperation::Restart,
        })
    );
    assert_eq!(
        registry.state(application_id),
        Ok(ApplicationState::Registered)
    );
}

#[test]
fn two_lifecycle_records_transition_independently() {
    let mut registry = LifecycleRegistry::new(2).expect("test capacity is valid");
    let first = registry.register().expect("first record fits");
    let second = registry.register().expect("second record fits");

    registry.start(first).expect("first starts");
    registry.start(second).expect("second starts");
    registry.stop(first).expect("first stops");
    registry.restart(first).expect("first restarts");

    assert_eq!(registry.state(first), Ok(ApplicationState::Running));
    assert_eq!(registry.state(second), Ok(ApplicationState::Running));

    assert_eq!(
        registry.restart(second),
        Err(LifecycleError::InvalidTransition {
            application_id: second,
            state: ApplicationState::Running,
            operation: LifecycleOperation::Restart,
        })
    );
    assert_eq!(registry.state(first), Ok(ApplicationState::Running));
    assert_eq!(registry.state(second), Ok(ApplicationState::Running));
}
