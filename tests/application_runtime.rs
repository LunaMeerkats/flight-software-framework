use std::cell::Cell;
use std::error::Error;
use std::fmt;
use std::rc::Rc;

use rust_flight_framework::{
    Application, ApplicationState, LifecycleError, LifecycleOperation, Runtime, RuntimeCreateError,
    RuntimeStartError, RuntimeStopError,
};

#[derive(Debug)]
struct HealthyApplication {
    start_calls: Rc<Cell<usize>>,
    stop_calls: Rc<Cell<usize>>,
}

impl HealthyApplication {
    fn start(&mut self) -> Result<(), MissionStartError> {
        self.start_calls.set(self.start_calls.get() + 1);
        Ok(())
    }

    fn stop(&mut self) -> Result<(), MissionStopError> {
        self.stop_calls.set(self.stop_calls.get() + 1);
        Ok(())
    }
}

#[derive(Debug)]
struct FaultingApplication {
    start_calls: Rc<Cell<usize>>,
    stop_calls: Rc<Cell<usize>>,
    error_code: u16,
}

impl FaultingApplication {
    fn start(&mut self) -> Result<(), MissionStartError> {
        self.start_calls.set(self.start_calls.get() + 1);
        Err(MissionStartError::Rejected(StartRejected {
            code: self.error_code,
        }))
    }

    fn stop(&mut self) -> Result<(), MissionStopError> {
        self.stop_calls.set(self.stop_calls.get() + 1);
        Ok(())
    }
}

#[derive(Debug)]
struct StopFaultingApplication {
    start_calls: Rc<Cell<usize>>,
    stop_calls: Rc<Cell<usize>>,
    error_code: u16,
}

impl StopFaultingApplication {
    fn start(&mut self) -> Result<(), MissionStartError> {
        self.start_calls.set(self.start_calls.get() + 1);
        Ok(())
    }

    fn stop(&mut self) -> Result<(), MissionStopError> {
        self.stop_calls.set(self.stop_calls.get() + 1);
        Err(MissionStopError::Rejected(StopRejected {
            code: self.error_code,
        }))
    }
}

#[derive(Debug)]
enum MissionApplication {
    Healthy(HealthyApplication),
    Faulting(FaultingApplication),
    StopFaulting(StopFaultingApplication),
}

impl Application for MissionApplication {
    type StartError = MissionStartError;
    type StopError = MissionStopError;

    fn start(&mut self) -> Result<(), Self::StartError> {
        match self {
            Self::Healthy(application) => application.start(),
            Self::Faulting(application) => application.start(),
            Self::StopFaulting(application) => application.start(),
        }
    }

    fn stop(&mut self) -> Result<(), Self::StopError> {
        match self {
            Self::Healthy(application) => application.stop(),
            Self::Faulting(application) => application.stop(),
            Self::StopFaulting(application) => application.stop(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum MissionStartError {
    Rejected(StartRejected),
}

impl fmt::Display for MissionStartError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rejected(error) => error.fmt(formatter),
        }
    }
}

impl Error for MissionStartError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Rejected(error) => Some(error),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct StartRejected {
    code: u16,
}

impl fmt::Display for StartRejected {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "start rejected with code {}", self.code)
    }
}

impl Error for StartRejected {}

#[derive(Debug, Eq, PartialEq)]
enum MissionStopError {
    Rejected(StopRejected),
}

impl fmt::Display for MissionStopError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rejected(error) => error.fmt(formatter),
        }
    }
}

impl Error for MissionStopError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Rejected(error) => Some(error),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct StopRejected {
    code: u16,
}

impl fmt::Display for StopRejected {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "stop rejected with code {}", self.code)
    }
}

impl Error for StopRejected {}

#[test]
fn returned_start_error_fails_only_the_selected_application() {
    let faulting_calls = Rc::new(Cell::new(0));
    let faulting_stop_calls = Rc::new(Cell::new(0));
    let healthy_calls = Rc::new(Cell::new(0));
    let healthy_stop_calls = Rc::new(Cell::new(0));
    let mut runtime = Runtime::new(2).expect("two runtime records can be reserved");

    let faulting_id = runtime
        .register(MissionApplication::Faulting(FaultingApplication {
            start_calls: Rc::clone(&faulting_calls),
            stop_calls: Rc::clone(&faulting_stop_calls),
            error_code: 17,
        }))
        .expect("faulting application fits");
    let healthy_id = runtime
        .register(MissionApplication::Healthy(HealthyApplication {
            start_calls: Rc::clone(&healthy_calls),
            stop_calls: Rc::clone(&healthy_stop_calls),
        }))
        .expect("healthy application fits");
    assert_ne!(faulting_id, healthy_id);
    assert_eq!(runtime.len(), 2);
    assert_eq!(runtime.capacity(), 2);

    let returned_error = runtime
        .start(faulting_id)
        .expect_err("faulting application returns its concrete error");
    assert_eq!(
        returned_error,
        RuntimeStartError::Application {
            application_id: faulting_id,
            source: MissionStartError::Rejected(StartRejected { code: 17 }),
        }
    );
    assert_eq!(
        Error::source(&returned_error).and_then(|source| source.downcast_ref()),
        Some(&MissionStartError::Rejected(StartRejected { code: 17 }))
    );
    assert_eq!(runtime.state(faulting_id), Ok(ApplicationState::Failed));
    assert_eq!(faulting_calls.get(), 1);
    assert_eq!(faulting_stop_calls.get(), 0);

    assert_eq!(runtime.start(healthy_id), Ok(ApplicationState::Running));
    assert_eq!(runtime.state(healthy_id), Ok(ApplicationState::Running));
    assert_eq!(healthy_calls.get(), 1);

    assert_eq!(
        runtime.start(faulting_id),
        Err(RuntimeStartError::Lifecycle(
            LifecycleError::InvalidTransition {
                application_id: faulting_id,
                state: ApplicationState::Failed,
                operation: LifecycleOperation::Start,
            }
        ))
    );
    assert_eq!(faulting_calls.get(), 1);

    assert_eq!(
        runtime.stop(faulting_id),
        Err(RuntimeStopError::Lifecycle(
            LifecycleError::InvalidTransition {
                application_id: faulting_id,
                state: ApplicationState::Failed,
                operation: LifecycleOperation::Stop,
            }
        ))
    );
    assert_eq!(faulting_stop_calls.get(), 0);

    assert_eq!(
        runtime.start(healthy_id),
        Err(RuntimeStartError::Lifecycle(
            LifecycleError::InvalidTransition {
                application_id: healthy_id,
                state: ApplicationState::Running,
                operation: LifecycleOperation::Start,
            }
        ))
    );
    assert_eq!(healthy_calls.get(), 1);
    assert_eq!(healthy_stop_calls.get(), 0);
}

#[test]
fn valid_stop_commits_stopped_and_rejected_stops_suppress_the_callback() {
    let start_calls = Rc::new(Cell::new(0));
    let stop_calls = Rc::new(Cell::new(0));
    let mut runtime = Runtime::new(1).expect("one runtime record can be reserved");
    let application_id = runtime
        .register(MissionApplication::Healthy(HealthyApplication {
            start_calls: Rc::clone(&start_calls),
            stop_calls: Rc::clone(&stop_calls),
        }))
        .expect("healthy application fits");

    assert_eq!(
        runtime.stop(application_id),
        Err(RuntimeStopError::Lifecycle(
            LifecycleError::InvalidTransition {
                application_id,
                state: ApplicationState::Registered,
                operation: LifecycleOperation::Stop,
            }
        ))
    );
    assert_eq!(
        runtime.state(application_id),
        Ok(ApplicationState::Registered)
    );
    assert_eq!(stop_calls.get(), 0);

    assert_eq!(runtime.start(application_id), Ok(ApplicationState::Running));
    assert_eq!(runtime.stop(application_id), Ok(ApplicationState::Stopped));
    assert_eq!(runtime.state(application_id), Ok(ApplicationState::Stopped));
    assert_eq!(start_calls.get(), 1);
    assert_eq!(stop_calls.get(), 1);

    assert_eq!(
        runtime.start(application_id),
        Err(RuntimeStartError::Lifecycle(
            LifecycleError::InvalidTransition {
                application_id,
                state: ApplicationState::Stopped,
                operation: LifecycleOperation::Start,
            }
        ))
    );
    assert_eq!(runtime.state(application_id), Ok(ApplicationState::Stopped));
    assert_eq!(start_calls.get(), 1);

    assert_eq!(
        runtime.stop(application_id),
        Err(RuntimeStopError::Lifecycle(
            LifecycleError::InvalidTransition {
                application_id,
                state: ApplicationState::Stopped,
                operation: LifecycleOperation::Stop,
            }
        ))
    );
    assert_eq!(runtime.state(application_id), Ok(ApplicationState::Stopped));
    assert_eq!(stop_calls.get(), 1);
    assert_eq!(runtime.len(), 1);
    assert_eq!(runtime.capacity(), 1);
}

#[test]
fn returned_stop_error_fails_only_the_selected_application() {
    let faulting_start_calls = Rc::new(Cell::new(0));
    let faulting_stop_calls = Rc::new(Cell::new(0));
    let peer_start_calls = Rc::new(Cell::new(0));
    let peer_stop_calls = Rc::new(Cell::new(0));
    let mut runtime = Runtime::new(2).expect("two runtime records can be reserved");

    let faulting_id = runtime
        .register(MissionApplication::StopFaulting(StopFaultingApplication {
            start_calls: Rc::clone(&faulting_start_calls),
            stop_calls: Rc::clone(&faulting_stop_calls),
            error_code: 29,
        }))
        .expect("stop-faulting application fits");
    let peer_id = runtime
        .register(MissionApplication::Healthy(HealthyApplication {
            start_calls: Rc::clone(&peer_start_calls),
            stop_calls: Rc::clone(&peer_stop_calls),
        }))
        .expect("healthy peer fits");

    assert_eq!(runtime.start(faulting_id), Ok(ApplicationState::Running));
    assert_eq!(runtime.start(peer_id), Ok(ApplicationState::Running));

    let returned_error = runtime
        .stop(faulting_id)
        .expect_err("stop-faulting application returns its concrete error");
    assert_eq!(
        returned_error,
        RuntimeStopError::Application {
            application_id: faulting_id,
            source: MissionStopError::Rejected(StopRejected { code: 29 }),
        }
    );
    assert_eq!(
        Error::source(&returned_error).and_then(|source| source.downcast_ref()),
        Some(&MissionStopError::Rejected(StopRejected { code: 29 }))
    );
    assert_eq!(runtime.state(faulting_id), Ok(ApplicationState::Failed));
    assert_eq!(faulting_start_calls.get(), 1);
    assert_eq!(faulting_stop_calls.get(), 1);

    assert_eq!(
        runtime.stop(faulting_id),
        Err(RuntimeStopError::Lifecycle(
            LifecycleError::InvalidTransition {
                application_id: faulting_id,
                state: ApplicationState::Failed,
                operation: LifecycleOperation::Stop,
            }
        ))
    );
    assert_eq!(faulting_stop_calls.get(), 1);

    assert_eq!(runtime.stop(peer_id), Ok(ApplicationState::Stopped));
    assert_eq!(runtime.state(peer_id), Ok(ApplicationState::Stopped));
    assert_eq!(peer_start_calls.get(), 1);
    assert_eq!(peer_stop_calls.get(), 1);
    assert_eq!(runtime.len(), 2);
    assert_eq!(runtime.capacity(), 2);
}

#[test]
fn runtime_capacity_rejection_preserves_application_ownership() {
    assert_eq!(
        Runtime::<MissionApplication>::new(0).expect_err("zero runtime capacity is invalid"),
        RuntimeCreateError::ZeroCapacity
    );

    let retained_calls = Rc::new(Cell::new(0));
    let rejected_calls = Rc::new(Cell::new(0));
    let mut runtime = Runtime::new(1).expect("one runtime record can be reserved");
    assert!(runtime.is_empty());

    let retained_id = runtime
        .register(MissionApplication::Healthy(HealthyApplication {
            start_calls: Rc::clone(&retained_calls),
            stop_calls: Rc::new(Cell::new(0)),
        }))
        .expect("first application fits");
    let rejection = runtime
        .register(MissionApplication::Faulting(FaultingApplication {
            start_calls: Rc::clone(&rejected_calls),
            stop_calls: Rc::new(Cell::new(0)),
            error_code: 23,
        }))
        .expect_err("second application exceeds the record limit");

    assert_eq!(rejection.capacity(), 1);
    match rejection.application() {
        MissionApplication::Faulting(application) => {
            assert!(Rc::ptr_eq(&application.start_calls, &rejected_calls));
            assert_eq!(application.error_code, 23);
        }
        MissionApplication::Healthy(_) | MissionApplication::StopFaulting(_) => {
            panic!("the rejected application changed variant")
        }
    }

    let rejected_application = rejection.into_application();
    assert!(matches!(
        rejected_application,
        MissionApplication::Faulting(FaultingApplication { error_code: 23, .. })
    ));
    assert_eq!(runtime.capacity(), 1);
    assert_eq!(runtime.len(), 1);
    assert_eq!(runtime.state(retained_id), Ok(ApplicationState::Registered));
    assert_eq!(retained_calls.get(), 0);
    assert_eq!(rejected_calls.get(), 0);
}

#[test]
fn unknown_identity_is_rejected_before_application_code_runs() {
    let target_calls = Rc::new(Cell::new(0));
    let target_stop_calls = Rc::new(Cell::new(0));
    let mut target_runtime = Runtime::new(1).expect("one target record can be reserved");
    let target_id = target_runtime
        .register(MissionApplication::Healthy(HealthyApplication {
            start_calls: Rc::clone(&target_calls),
            stop_calls: Rc::clone(&target_stop_calls),
        }))
        .expect("target application fits");

    let mut issuing_runtime = Runtime::new(2).expect("two issuing records can be reserved");
    issuing_runtime
        .register(MissionApplication::Healthy(HealthyApplication {
            start_calls: Rc::new(Cell::new(0)),
            stop_calls: Rc::new(Cell::new(0)),
        }))
        .expect("first issuing application fits");
    let out_of_range_id = issuing_runtime
        .register(MissionApplication::Healthy(HealthyApplication {
            start_calls: Rc::new(Cell::new(0)),
            stop_calls: Rc::new(Cell::new(0)),
        }))
        .expect("second issuing application fits");

    assert_eq!(
        target_runtime.start(out_of_range_id),
        Err(RuntimeStartError::Lifecycle(
            LifecycleError::UnknownApplication {
                application_id: out_of_range_id,
            }
        ))
    );
    assert_eq!(target_calls.get(), 0);
    assert_eq!(
        target_runtime.stop(out_of_range_id),
        Err(RuntimeStopError::Lifecycle(
            LifecycleError::UnknownApplication {
                application_id: out_of_range_id,
            }
        ))
    );
    assert_eq!(target_stop_calls.get(), 0);
    assert_eq!(
        target_runtime.state(target_id),
        Ok(ApplicationState::Registered)
    );
}
