//! Fixed mission startup and reverse cleanup through public runtime APIs.
//!
//! Only successful starts enter the cleanup prefix. Every stop result remains
//! owned by the host, including secondary errors after a startup failure.

#![forbid(unsafe_code)]

use std::cell::RefCell;
use std::convert::Infallible;
use std::error::Error;
use std::fmt;
use std::io::{self, Write};

use rust_flight_framework::{
    Application, ApplicationId, ApplicationState, ApplicationWorkContext, Runtime,
    RuntimeStartError, RuntimeStopError,
};

const APPLICATIONS: usize = 3;
const CALLBACKS: usize = APPLICATIONS * 2;
const ROLES: [Role; APPLICATIONS] = [Role::Power, Role::Navigation, Role::Payload];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Role {
    Power,
    Navigation,
    Payload,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Operation {
    Start,
    Stop,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Callback {
    role: Role,
    operation: Operation,
}

// Deliberately non-Copy and non-Clone: callbacks transfer these owned values.
#[derive(Debug, Eq, PartialEq)]
enum FailureCause {
    Injected { marker: [u8; 2] },
    ObservationFull,
}

#[derive(Debug, Eq, PartialEq)]
struct CallbackFailure {
    callback: Callback,
    cause: FailureCause,
}

#[derive(Default)]
struct CallbackTrace {
    entries: [Option<Callback>; CALLBACKS],
    used: usize,
}

struct MissionApplication<'a> {
    role: Role,
    trace: &'a RefCell<CallbackTrace>,
    start_failure: Option<CallbackFailure>,
    stop_failure: Option<CallbackFailure>,
    start_effect: bool,
}

// This private token belongs to this fixed mission and its issuing runtime.
// It carries no origin validation and is intentionally neither Copy nor Clone.
#[derive(Debug)]
struct StartedMission {
    ids: [ApplicationId; APPLICATIONS],
    started: usize,
}

#[derive(Debug)]
struct StopAttempt {
    application_id: ApplicationId,
    result: Result<ApplicationState, RuntimeStopError<CallbackFailure>>,
}

#[derive(Debug)]
struct CleanupReport {
    attempts: [Option<StopAttempt>; APPLICATIONS],
}

#[derive(Debug)]
struct StartupFailure {
    start_error: RuntimeStartError<CallbackFailure>,
    cleanup: CleanupReport,
}

// The multi-error report stays inline rather than boxing a large Result error.
#[derive(Debug)]
enum StartupOutcome {
    Started(StartedMission),
    Failed(StartupFailure),
}

type MissionRuntime<'a> = Runtime<MissionApplication<'a>>;

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = io::stdout().lock();
    demonstrate(&mut output)
}

fn demonstrate(output: &mut impl Write) -> Result<(), Box<dyn Error>> {
    writeln!(
        output,
        "Experimental host startup and cleanup; callbacks must return."
    )?;
    for failed_start in [None, Some(Role::Payload)] {
        let trace = RefCell::new(CallbackTrace::default());
        let stop_failures = match failed_start {
            None => std::array::from_fn(|_| None),
            Some(_) => [
                Some(injected(Role::Power, Operation::Stop)),
                Some(injected(Role::Navigation, Operation::Stop)),
                None,
            ],
        };
        let (mut runtime, ids) = compose(&trace, failed_start, stop_failures)?;
        match start_mission(&mut runtime, ids) {
            StartupOutcome::Started(started) => {
                writeln!(output, "startup: all three Running; shutdown is explicit")?;
                let cleanup = shutdown(&mut runtime, started);
                present_cleanup(output, &runtime, cleanup)?;
            }
            StartupOutcome::Failed(failure) => {
                writeln!(output, "startup failed: {}", failure.start_error)?;
                present_cleanup(output, &runtime, failure.cleanup)?;
            }
        }
        let states = ids.map(|id| runtime.state(id));
        writeln!(output, "states: {states:?}")?;
    }
    Ok(())
}

fn present_cleanup(
    output: &mut impl Write,
    runtime: &MissionRuntime<'_>,
    report: CleanupReport,
) -> Result<(), Box<dyn Error>> {
    for attempt in report.attempts.into_iter().flatten() {
        let role = runtime.application(attempt.application_id)?.role;
        match attempt.result {
            Ok(state) => writeln!(output, "cleanup {role:?}: {state:?}")?,
            Err(error) => writeln!(output, "cleanup {role:?}: {error}")?,
        }
    }
    Ok(())
}

impl Application for MissionApplication<'_> {
    type StartError = CallbackFailure;
    type WorkError = Infallible;
    type StopError = CallbackFailure;
    type RestartError = Infallible;

    fn start(&mut self) -> Result<(), Self::StartError> {
        self.trace
            .borrow_mut()
            .record(self.role, Operation::Start)?;
        // A callback can change its own state before returning an error.
        // Partial-start cleanup cannot undo this failed application's effect.
        self.start_effect = true;
        self.start_failure.take().map_or(Ok(()), Err)
    }

    fn work(&mut self, _context: ApplicationWorkContext<'_>) -> Result<(), Self::WorkError> {
        Ok(())
    }

    fn stop(&mut self) -> Result<(), Self::StopError> {
        self.trace.borrow_mut().record(self.role, Operation::Stop)?;
        if let Some(error) = self.stop_failure.take() {
            return Err(error);
        }
        self.start_effect = false;
        Ok(())
    }

    fn restart(&mut self) -> Result<(), Self::RestartError> {
        self.start_effect = true;
        Ok(())
    }
}

fn start_mission(
    runtime: &mut MissionRuntime<'_>,
    ids: [ApplicationId; APPLICATIONS],
) -> StartupOutcome {
    for (started, id) in ids.into_iter().enumerate() {
        if let Err(start_error) = runtime.start(id) {
            let prefix = StartedMission { ids, started };
            return StartupOutcome::Failed(StartupFailure {
                start_error,
                cleanup: shutdown(runtime, prefix),
            });
        }
    }
    StartupOutcome::Started(StartedMission {
        ids,
        started: APPLICATIONS,
    })
}

fn shutdown(runtime: &mut MissionRuntime<'_>, started: StartedMission) -> CleanupReport {
    let mut attempts = std::array::from_fn(|_| None);
    for (slot, id) in started.ids[..started.started].iter().rev().enumerate() {
        attempts[slot] = Some(StopAttempt {
            application_id: *id,
            result: runtime.stop(*id),
        });
    }
    CleanupReport { attempts }
}

fn compose<'a>(
    trace: &'a RefCell<CallbackTrace>,
    start_failure: Option<Role>,
    stop_failures: [Option<CallbackFailure>; APPLICATIONS],
) -> Result<(MissionRuntime<'a>, [ApplicationId; APPLICATIONS]), Box<dyn Error>> {
    let mut runtime = Runtime::new(APPLICATIONS)?;
    let mut register = |role, stop_failure| {
        let application = MissionApplication {
            role,
            trace,
            start_failure: (Some(role) == start_failure).then(|| injected(role, Operation::Start)),
            stop_failure,
            start_effect: false,
        };
        // Setup errors consume only the rejected value; no application has
        // started. Avoid boxing an ownership wrapper borrowing this trace.
        runtime.register(application).map_err(|error| {
            io::Error::other(format!("application capacity {} full", error.capacity()))
        })
    };
    let [power, navigation, payload] = ROLES;
    let [power_failure, navigation_failure, payload_failure] = stop_failures;
    let ids = [
        register(power, power_failure)?,
        register(navigation, navigation_failure)?,
        register(payload, payload_failure)?,
    ];
    Ok((runtime, ids))
}
fn injected(role: Role, operation: Operation) -> CallbackFailure {
    CallbackFailure {
        callback: Callback { role, operation },
        cause: FailureCause::Injected {
            marker: [role as u8, operation as u8],
        },
    }
}

impl CallbackTrace {
    fn record(&mut self, role: Role, operation: Operation) -> Result<(), CallbackFailure> {
        let callback = Callback { role, operation };
        let Some(slot) = self.entries.get_mut(self.used) else {
            return Err(CallbackFailure {
                callback,
                cause: FailureCause::ObservationFull,
            });
        };
        *slot = Some(callback);
        self.used += 1;
        Ok(())
    }
}

impl fmt::Display for FailureCause {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Injected { marker } => write!(formatter, "injected marker {marker:?}"),
            Self::ObservationFull => formatter.write_str("callback observation capacity exhausted"),
        }
    }
}

impl Error for FailureCause {}

impl fmt::Display for CallbackFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{:?} {:?}: {}",
            self.callback.role, self.callback.operation, self.cause
        )
    }
}

impl Error for CallbackFailure {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.cause)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_flight_framework::{LifecycleError, LifecycleOperation};

    fn no_failures() -> [Option<CallbackFailure>; APPLICATIONS] {
        std::array::from_fn(|_| None)
    }

    fn observed(trace: &RefCell<CallbackTrace>) -> Vec<Callback> {
        trace.borrow().entries.iter().flatten().copied().collect()
    }

    fn call(role: Role, operation: Operation) -> Callback {
        Callback { role, operation }
    }

    fn attempts(report: &CleanupReport) -> Vec<&StopAttempt> {
        report.attempts.iter().flatten().collect()
    }

    fn assert_callback_source(error: &(dyn Error + 'static), expected: CallbackFailure) {
        let source = error
            .source()
            .unwrap()
            .downcast_ref::<CallbackFailure>()
            .unwrap();
        assert_eq!(source, &expected);
        assert_eq!(
            source.source().unwrap().downcast_ref::<FailureCause>(),
            Some(&expected.cause)
        );
    }

    #[test]
    fn complete_start_waits_for_explicit_reverse_shutdown() {
        let trace = RefCell::new(CallbackTrace::default());
        let (mut runtime, ids) = compose(&trace, None, no_failures()).unwrap();
        let StartupOutcome::Started(started) = start_mission(&mut runtime, ids) else {
            panic!("all callbacks succeed");
        };
        assert_eq!(
            observed(&trace),
            ROLES.map(|role| call(role, Operation::Start))
        );
        for id in ids {
            assert_eq!(runtime.state(id), Ok(ApplicationState::Running));
            assert!(runtime.application(id).unwrap().start_effect);
        }
        let report = shutdown(&mut runtime, started);
        let attempts = attempts(&report);
        assert_eq!(attempts.len(), APPLICATIONS);
        for (attempt, id) in attempts.into_iter().zip(ids.into_iter().rev()) {
            assert_eq!(attempt.application_id, id);
            assert_eq!(attempt.result, Ok(ApplicationState::Stopped));
            assert!(!runtime.application(id).unwrap().start_effect);
        }
        assert_eq!(
            observed(&trace),
            [
                call(Role::Power, Operation::Start),
                call(Role::Navigation, Operation::Start),
                call(Role::Payload, Operation::Start),
                call(Role::Payload, Operation::Stop),
                call(Role::Navigation, Operation::Stop),
                call(Role::Power, Operation::Stop),
            ]
        );
    }

    #[test]
    fn each_start_failure_cleans_only_successful_prefix_and_keeps_failed_effect() {
        for (failed, role) in ROLES.into_iter().enumerate() {
            let trace = RefCell::new(CallbackTrace::default());
            let (mut runtime, ids) = compose(&trace, Some(role), no_failures()).unwrap();
            let StartupOutcome::Failed(failure) = start_mission(&mut runtime, ids) else {
                panic!("selected startup fails");
            };
            assert_eq!(
                failure.start_error,
                RuntimeStartError::Application {
                    application_id: ids[failed],
                    source: injected(role, Operation::Start),
                }
            );
            let expected = ROLES[..=failed]
                .iter()
                .map(|role| call(*role, Operation::Start))
                .chain(
                    ROLES[..failed]
                        .iter()
                        .rev()
                        .map(|role| call(*role, Operation::Stop)),
                )
                .collect::<Vec<_>>();
            assert_eq!(observed(&trace), expected);
            let cleanup = attempts(&failure.cleanup);
            assert_eq!(cleanup.len(), failed);
            for (attempt, id) in cleanup.into_iter().zip(ids[..failed].iter().rev()) {
                assert_eq!(attempt.application_id, *id);
                assert_eq!(attempt.result, Ok(ApplicationState::Stopped));
            }
            for (index, id) in ids.into_iter().enumerate() {
                let state = match index.cmp(&failed) {
                    std::cmp::Ordering::Less => ApplicationState::Stopped,
                    std::cmp::Ordering::Equal => ApplicationState::Failed,
                    std::cmp::Ordering::Greater => ApplicationState::Registered,
                };
                assert_eq!(runtime.state(id), Ok(state));
                let application = runtime.application(id).unwrap();
                assert_eq!(application.start_effect, index == failed);
                assert!(application.start_failure.is_none());
            }
        }
    }

    #[test]
    fn partial_start_retains_primary_and_multiple_owned_cleanup_sources() {
        let trace = RefCell::new(CallbackTrace::default());
        let stop_failures = [
            Some(injected(Role::Power, Operation::Stop)),
            Some(injected(Role::Navigation, Operation::Stop)),
            None,
        ];
        let (mut runtime, ids) = compose(&trace, Some(Role::Payload), stop_failures).unwrap();
        let StartupOutcome::Failed(failure) = start_mission(&mut runtime, ids) else {
            panic!("payload startup fails");
        };
        assert_callback_source(
            &failure.start_error,
            injected(Role::Payload, Operation::Start),
        );
        let RuntimeStartError::Application {
            application_id,
            source,
        } = failure.start_error
        else {
            panic!("application error retains owned source");
        };
        assert_eq!(application_id, ids[2]);
        assert_eq!(source, injected(Role::Payload, Operation::Start));
        let mut retained = Vec::new();
        for (attempt, index) in failure.cleanup.attempts.into_iter().flatten().zip([1, 0]) {
            assert_eq!(attempt.application_id, ids[index]);
            let error = attempt.result.unwrap_err();
            assert_callback_source(&error, injected(ROLES[index], Operation::Stop));
            let RuntimeStopError::Application {
                application_id,
                source,
            } = error
            else {
                panic!("secondary source remains owned");
            };
            assert_eq!(application_id, ids[index]);
            retained.push(source);
        }
        assert_eq!(retained.len(), 2);
        for id in ids {
            let application = runtime.application(id).unwrap();
            assert!(application.start_failure.is_none());
            assert!(application.stop_failure.is_none());
            assert!(application.start_effect);
            assert_eq!(runtime.state(id), Ok(ApplicationState::Failed));
        }
        drop(runtime);
        assert_eq!(
            source.source().unwrap().downcast_ref::<FailureCause>(),
            Some(&source.cause)
        );
        assert_eq!(
            retained,
            [
                injected(Role::Navigation, Operation::Stop),
                injected(Role::Power, Operation::Stop),
            ]
        );
        assert_eq!(observed(&trace).len(), 5);
    }

    #[test]
    fn cleanup_continues_after_error_to_an_earlier_successful_stop() {
        let trace = RefCell::new(CallbackTrace::default());
        let stop_failures = [
            None,
            Some(injected(Role::Navigation, Operation::Stop)),
            None,
        ];
        let (mut runtime, ids) = compose(&trace, Some(Role::Payload), stop_failures).unwrap();
        let StartupOutcome::Failed(failure) = start_mission(&mut runtime, ids) else {
            panic!("payload startup fails");
        };
        let cleanup = attempts(&failure.cleanup);
        assert_eq!(cleanup.len(), 2);
        assert_eq!(cleanup[0].application_id, ids[1]);
        assert_eq!(
            cleanup[0].result,
            Err(RuntimeStopError::Application {
                application_id: ids[1],
                source: injected(Role::Navigation, Operation::Stop),
            })
        );
        assert_eq!(cleanup[1].application_id, ids[0]);
        assert_eq!(cleanup[1].result, Ok(ApplicationState::Stopped));
        assert_eq!(runtime.state(ids[0]), Ok(ApplicationState::Stopped));
        assert_eq!(
            observed(&trace).last(),
            Some(&call(Role::Power, Operation::Stop))
        );
    }

    #[test]
    fn lifecycle_rejection_never_adds_the_rejected_application_to_cleanup() {
        let trace = RefCell::new(CallbackTrace::default());
        let (mut runtime, ids) = compose(&trace, None, no_failures()).unwrap();
        runtime.start(ids[1]).unwrap();
        let StartupOutcome::Failed(failure) = start_mission(&mut runtime, ids) else {
            panic!("already running navigation rejects start");
        };
        assert_eq!(
            failure.start_error,
            RuntimeStartError::Lifecycle(LifecycleError::InvalidTransition {
                application_id: ids[1],
                state: ApplicationState::Running,
                operation: LifecycleOperation::Start,
            })
        );
        let cleanup = attempts(&failure.cleanup);
        assert_eq!(cleanup.len(), 1);
        assert_eq!(cleanup[0].application_id, ids[0]);
        assert_eq!(cleanup[0].result, Ok(ApplicationState::Stopped));
        assert_eq!(runtime.state(ids[1]), Ok(ApplicationState::Running));
        assert_eq!(runtime.state(ids[2]), Ok(ApplicationState::Registered));
        assert_eq!(
            observed(&trace),
            [
                call(Role::Navigation, Operation::Start),
                call(Role::Power, Operation::Start),
                call(Role::Power, Operation::Stop),
            ]
        );
    }

    #[test]
    fn explicit_shutdown_keeps_all_errors_and_finishes_the_reverse_sequence() {
        let trace = RefCell::new(CallbackTrace::default());
        let stop_failures = ROLES.map(|role| Some(injected(role, Operation::Stop)));
        let (mut runtime, ids) = compose(&trace, None, stop_failures).unwrap();
        let StartupOutcome::Started(started) = start_mission(&mut runtime, ids) else {
            panic!("all start callbacks succeed");
        };
        let report = shutdown(&mut runtime, started);
        let cleanup = attempts(&report);
        assert_eq!(cleanup.len(), APPLICATIONS);
        for (attempt, index) in cleanup.into_iter().zip((0..APPLICATIONS).rev()) {
            assert_eq!(attempt.application_id, ids[index]);
            assert_eq!(
                attempt.result,
                Err(RuntimeStopError::Application {
                    application_id: ids[index],
                    source: injected(ROLES[index], Operation::Stop),
                })
            );
            assert_eq!(runtime.state(ids[index]), Ok(ApplicationState::Failed));
        }
        assert_eq!(
            &observed(&trace)[APPLICATIONS..],
            &[
                call(Role::Payload, Operation::Stop),
                call(Role::Navigation, Operation::Stop),
                call(Role::Power, Operation::Stop),
            ]
        );
    }

    #[test]
    fn demonstration_reports_deterministic_success_and_secondary_failures() {
        let mut output = Vec::new();
        demonstrate(&mut output).unwrap();
        let expected = concat!(
            "Experimental host startup and cleanup; callbacks must return.\n",
            "startup: all three Running; shutdown is explicit\n",
            "cleanup Payload: Stopped\n",
            "cleanup Navigation: Stopped\n",
            "cleanup Power: Stopped\n",
            "states: [Ok(Stopped), Ok(Stopped), Ok(Stopped)]\n",
            "startup failed: application ApplicationId(2) returned a start error: ",
            "Payload Start: injected marker [2, 0]\n",
            "cleanup Navigation: application ApplicationId(1) returned a stop error: ",
            "Navigation Stop: injected marker [1, 1]\n",
            "cleanup Power: application ApplicationId(0) returned a stop error: ",
            "Power Stop: injected marker [0, 1]\n",
            "states: [Ok(Failed), Ok(Failed), Ok(Failed)]\n",
        );
        assert_eq!(output, expected.as_bytes());
    }
}
