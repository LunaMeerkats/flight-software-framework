//! Finite sequence evidence for returned ordinary-work failure and peer queues.
//!
//! The frozen domain is recorded in RUNTIME_WORK_FAILURE_SEQUENCE_REVIEW.md.
//! Independent lifecycle states, accepted histories and discard watermarks
//! predict outcomes; final passive dispatch observes retained FIFO records.

use std::cell::RefCell;
use std::convert::Infallible;
use std::error::Error;
use std::fmt;
use std::rc::Rc;

use rust_flight_framework::{
    Application, ApplicationId, ApplicationMessageContext, ApplicationState,
    ApplicationWorkContext, DeliveryStatus, LifecycleError, LifecycleOperation, Message,
    MessageDispatchError, MessageDispatchOutcome, MessagingApplication, MessagingRuntime,
    PublishClassification, Runtime, RuntimeInboxConfig, RuntimeRestartError, RuntimeStopError,
    RuntimeWorkError,
};

const CAPACITIES: [usize; 2] = [1, 2];
const MAX_SEQUENCE_LENGTH: usize = 6;
const EXPECTED_TRACES: usize = 39_062;
const EXPECTED_OPERATIONS: usize = 224_610;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Topic {
    Command,
}

#[derive(Clone, Copy, Debug)]
enum Operation {
    Publish,
    WorkFault,
    WorkPeer,
    StopFault,
    RestartFault,
}

const OPERATIONS: [Operation; 5] = [
    Operation::Publish,
    Operation::WorkFault,
    Operation::WorkPeer,
    Operation::StopFault,
    Operation::RestartFault,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum State {
    Active,
    Paused,
    Terminal,
}

#[derive(Clone, Copy, Debug)]
enum Destination {
    Accepted,
    Full,
    Offline,
}

#[derive(Clone, Copy, Debug)]
enum WorkResult {
    Succeeded,
    Failed { discarded: usize },
    Rejected(State),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct WorkFailure {
    application_position: usize,
}

struct Reference {
    faulting: usize,
    states: [State; 2],
    accepted: [Vec<u8>; 2],
    discarded_through: [usize; 2],
    works: [usize; 2],
    stops: [usize; 2],
    restarts: [usize; 2],
}

#[derive(Debug, Default)]
struct Observations {
    starts: usize,
    works: usize,
    stops: usize,
    restarts: usize,
    received: Vec<(Topic, Vec<u8>)>,
}

#[derive(Debug)]
struct FaultApplication {
    work_failure: Option<WorkFailure>,
    observations: Rc<RefCell<Observations>>,
}

struct SequenceComparison {
    application_ids: [ApplicationId; 2],
    owner: MessagingRuntime<FaultApplication, Topic, 1>,
    observations: [Rc<RefCell<Observations>>; 2],
    reference: Reference,
}

#[test]
fn all_bounded_work_failure_sequences_match_reference_history() {
    let mut trace_count = 0;
    let mut operation_count = 0;
    for faulting in 0..CAPACITIES.len() {
        let mut sequences_at_length = 1;
        for length in 0..=MAX_SEQUENCE_LENGTH {
            for ordinal in 0..sequences_at_length {
                let sequence = decode_sequence(ordinal, length);
                let mut comparison = SequenceComparison::new(faulting);
                comparison.assert_state(&sequence);
                for (position, operation) in sequence.iter().copied().enumerate() {
                    comparison.execute(operation, position, &sequence);
                    comparison.assert_state(&sequence);
                }
                comparison.observe_remaining(&sequence);
                trace_count += 1;
                operation_count += sequence.len();
            }
            sequences_at_length *= OPERATIONS.len();
        }
    }
    assert_eq!(trace_count, EXPECTED_TRACES);
    assert_eq!(operation_count, EXPECTED_OPERATIONS);
}

impl Reference {
    fn new(faulting: usize) -> Self {
        Self {
            faulting,
            states: [State::Active; 2],
            accepted: std::array::from_fn(|_| Vec::new()),
            discarded_through: [0; 2],
            works: [0; 2],
            stops: [0; 2],
            restarts: [0; 2],
        }
    }

    fn outstanding(&self, inbox: usize) -> usize {
        self.accepted[inbox].len() - self.discarded_through[inbox]
    }

    fn publish(&mut self, payload: u8) -> [Destination; 2] {
        // Configured routes remain present when an endpoint becomes terminal.
        [0, 1].map(|inbox| {
            if self.states[inbox] != State::Active {
                return Destination::Offline;
            }
            if self.outstanding(inbox) == CAPACITIES[inbox] {
                return Destination::Full;
            }
            self.accepted[inbox].push(payload);
            Destination::Accepted
        })
    }

    fn work(&mut self, inbox: usize) -> WorkResult {
        if self.states[inbox] != State::Active {
            return WorkResult::Rejected(self.states[inbox]);
        }
        self.works[inbox] += 1;
        if inbox != self.faulting {
            return WorkResult::Succeeded;
        }
        let discarded = self.outstanding(inbox);
        self.discarded_through[inbox] = self.accepted[inbox].len();
        self.states[inbox] = State::Terminal;
        WorkResult::Failed { discarded }
    }

    fn stop(&mut self, inbox: usize) -> Option<usize> {
        if self.states[inbox] != State::Active {
            return None;
        }
        let discarded = self.outstanding(inbox);
        self.discarded_through[inbox] = self.accepted[inbox].len();
        self.states[inbox] = State::Paused;
        self.stops[inbox] += 1;
        Some(discarded)
    }

    fn restart(&mut self, inbox: usize) -> bool {
        if self.states[inbox] != State::Paused {
            return false;
        }
        self.states[inbox] = State::Active;
        self.restarts[inbox] += 1;
        true
    }
}

impl SequenceComparison {
    fn new(faulting: usize) -> Self {
        let observations = std::array::from_fn(|_| Rc::new(RefCell::new(Observations::default())));
        let mut runtime = Runtime::new(2).expect("two records fit");
        let application_ids = [0, 1].map(|inbox| {
            let work_failure = (inbox == faulting).then_some(WorkFailure {
                application_position: inbox,
            });
            runtime
                .register(FaultApplication {
                    work_failure,
                    observations: Rc::clone(&observations[inbox]),
                })
                .expect("application fits")
        });
        let configurations =
            CAPACITIES.map(|capacity| RuntimeInboxConfig::new(capacity, &[Topic::Command]));
        let mut owner = MessagingRuntime::new(runtime, &configurations).expect("topology is valid");
        for application_id in application_ids {
            assert_eq!(owner.start(application_id), Ok(ApplicationState::Running));
        }
        Self {
            application_ids,
            owner,
            observations,
            reference: Reference::new(faulting),
        }
    }

    fn execute(&mut self, operation: Operation, position: usize, sequence: &[Operation]) {
        let faulting = self.reference.faulting;
        match operation {
            Operation::Publish => {
                let payload = u8::try_from(position).expect("six positions fit");
                self.publish(payload, sequence);
            }
            Operation::WorkFault => self.work(faulting, sequence),
            Operation::WorkPeer => self.work(1 - faulting, sequence),
            Operation::StopFault => self.stop(faulting, sequence),
            Operation::RestartFault => self.restart(faulting, sequence),
        }
    }

    fn publish(&mut self, payload: u8, sequence: &[Operation]) {
        let expected = self.reference.publish(payload);
        let message = Message::try_new(Topic::Command, &[payload]).expect("one byte fits");
        let report = self
            .owner
            .publish(&message)
            .expect("report allocation succeeds");
        let actual: Vec<_> = report
            .outcomes()
            .iter()
            .map(|outcome| (outcome.application_id(), outcome.status()))
            .collect();
        let expected_outcomes = [0, 1].map(|inbox| {
            let status = match expected[inbox] {
                Destination::Accepted => DeliveryStatus::Delivered,
                Destination::Full => DeliveryStatus::InboxFull,
                Destination::Offline => DeliveryStatus::Unavailable,
            };
            (self.application_ids[inbox], status)
        });
        let accepted = expected
            .iter()
            .filter(|destination| matches!(destination, Destination::Accepted))
            .count();
        let classification = match accepted {
            0 => PublishClassification::WhollyUndelivered,
            2 => PublishClassification::Complete,
            _ => PublishClassification::Partial,
        };
        assert_eq!(
            actual, expected_outcomes,
            "fault {}, payload {payload}, trace {sequence:?}",
            self.reference.faulting
        );
        assert_eq!(
            report.classification(),
            classification,
            "trace {sequence:?}"
        );
    }

    fn work(&mut self, inbox: usize, sequence: &[Operation]) {
        let application_id = self.application_ids[inbox];
        let expected = self.reference.work(inbox);
        let actual = self.owner.work(application_id);
        match expected {
            WorkResult::Succeeded => {
                assert_eq!(actual, Ok(ApplicationState::Running), "trace {sequence:?}");
            }
            WorkResult::Failed { discarded } => {
                let error = actual.expect_err("selected ordinary work returns its error");
                let source = WorkFailure {
                    application_position: inbox,
                };
                let expected_error = RuntimeWorkError::Application {
                    application_id,
                    source,
                };
                assert_eq!(
                    error.operation_error(),
                    &expected_error,
                    "trace {sequence:?}"
                );
                assert_eq!(
                    error.discarded_deliveries(),
                    discarded,
                    "trace {sequence:?}"
                );
                let runtime_error = Error::source(&error)
                    .and_then(|error| error.downcast_ref::<RuntimeWorkError<WorkFailure>>())
                    .expect("messaging error retains concrete work error");
                assert_eq!(runtime_error, &expected_error, "trace {sequence:?}");
                assert_eq!(
                    Error::source(runtime_error).and_then(|error| error.downcast_ref()),
                    Some(&source),
                    "trace {sequence:?}"
                );
            }
            WorkResult::Rejected(state) => {
                let error = actual.expect_err("unavailable ordinary work is rejected");
                let expected_error = RuntimeWorkError::Lifecycle(LifecycleError::NotRunning {
                    application_id,
                    state: public_state(state),
                });
                assert_eq!(
                    error.operation_error(),
                    &expected_error,
                    "trace {sequence:?}"
                );
                assert_eq!(error.discarded_deliveries(), 0, "trace {sequence:?}");
            }
        }
    }

    fn stop(&mut self, inbox: usize, sequence: &[Operation]) {
        let application_id = self.application_ids[inbox];
        let expected = self.reference.stop(inbox);
        let actual = self.owner.stop(application_id);
        match expected {
            Some(discarded) => {
                let outcome = actual.expect("Active stop succeeds");
                assert_eq!(
                    outcome.state(),
                    ApplicationState::Stopped,
                    "trace {sequence:?}"
                );
                assert_eq!(
                    outcome.discarded_deliveries(),
                    discarded,
                    "trace {sequence:?}"
                );
            }
            None => {
                let error = actual.expect_err("unavailable stop is rejected");
                let expected_error =
                    RuntimeStopError::Lifecycle(LifecycleError::InvalidTransition {
                        application_id,
                        state: public_state(self.reference.states[inbox]),
                        operation: LifecycleOperation::Stop,
                    });
                assert_eq!(
                    error.operation_error(),
                    &expected_error,
                    "trace {sequence:?}"
                );
                assert_eq!(error.discarded_deliveries(), 0, "trace {sequence:?}");
            }
        }
    }

    fn restart(&mut self, inbox: usize, sequence: &[Operation]) {
        let application_id = self.application_ids[inbox];
        let expected = self.reference.restart(inbox);
        let actual = self.owner.restart(application_id);
        if expected {
            assert_eq!(actual, Ok(ApplicationState::Running), "trace {sequence:?}");
            return;
        }
        let error = actual.expect_err("Active or Terminal restart is rejected");
        let expected_error = RuntimeRestartError::Lifecycle(LifecycleError::InvalidTransition {
            application_id,
            state: public_state(self.reference.states[inbox]),
            operation: LifecycleOperation::Restart,
        });
        assert_eq!(
            error.operation_error(),
            &expected_error,
            "trace {sequence:?}"
        );
        assert_eq!(error.discarded_deliveries(), 0, "trace {sequence:?}");
    }

    fn assert_state(&self, sequence: &[Operation]) {
        for (inbox, capacity) in CAPACITIES.into_iter().enumerate() {
            let application_id = self.application_ids[inbox];
            let outstanding = self.reference.outstanding(inbox);
            let observed = self.observations[inbox].borrow();
            assert!(outstanding <= capacity, "inbox {inbox}, trace {sequence:?}");
            assert_eq!(self.owner.inbox_capacity(application_id), Ok(capacity));
            assert_eq!(
                self.owner.pending(application_id),
                Ok(outstanding),
                "fault {}, inbox {inbox}, trace {sequence:?}",
                self.reference.faulting
            );
            assert!(observed.received.is_empty(), "trace {sequence:?}");
            if self.reference.states[inbox] != State::Active {
                assert_eq!(outstanding, 0, "inbox {inbox}, trace {sequence:?}");
            }
            self.assert_callbacks_and_lifecycle(inbox, sequence);
        }
    }

    fn assert_callbacks_and_lifecycle(&self, inbox: usize, sequence: &[Operation]) {
        let observed = self.observations[inbox].borrow();
        assert_eq!(
            self.owner.state(self.application_ids[inbox]),
            Ok(public_state(self.reference.states[inbox])),
            "fault {}, inbox {inbox}, trace {sequence:?}",
            self.reference.faulting
        );
        assert_eq!(
            (
                observed.starts,
                observed.works,
                observed.stops,
                observed.restarts
            ),
            (
                1,
                self.reference.works[inbox],
                self.reference.stops[inbox],
                self.reference.restarts[inbox]
            ),
            "fault {}, inbox {inbox}, trace {sequence:?}",
            self.reference.faulting
        );
    }

    fn observe_remaining(&mut self, sequence: &[Operation]) {
        // Each prefix observes healthy work, even when failure is last.
        self.work(1 - self.reference.faulting, sequence);
        self.assert_state(sequence);
        for inbox in 0..CAPACITIES.len() {
            self.observe_inbox(inbox, sequence);
        }
        for inbox in 0..CAPACITIES.len() {
            self.assert_callbacks_and_lifecycle(inbox, sequence);
        }
    }

    fn observe_inbox(&mut self, inbox: usize, sequence: &[Operation]) {
        let application_id = self.application_ids[inbox];
        let suffix = &self.reference.accepted[inbox][self.reference.discarded_through[inbox]..];
        let expected: Vec<_> = suffix
            .iter()
            .map(|&byte| (Topic::Command, vec![byte]))
            .collect();
        for consumed in 0..suffix.len() {
            assert_eq!(
                self.owner.dispatch_one(application_id),
                Ok(MessageDispatchOutcome::Dispatched),
                "inbox {inbox}, trace {sequence:?}"
            );
            assert_eq!(
                self.observations[inbox].borrow().received,
                expected[..=consumed],
                "inbox {inbox}, trace {sequence:?}"
            );
            assert_eq!(
                self.owner.pending(application_id),
                Ok(suffix.len() - consumed - 1)
            );
        }
        let final_attempt = self.owner.dispatch_one(application_id);
        if self.reference.states[inbox] == State::Active {
            assert_eq!(
                final_attempt,
                Ok(MessageDispatchOutcome::InboxEmpty),
                "trace {sequence:?}"
            );
        } else {
            let error = final_attempt.expect_err("unavailable dispatch is rejected");
            let expected_error = MessageDispatchError::Lifecycle(LifecycleError::NotRunning {
                application_id,
                state: public_state(self.reference.states[inbox]),
            });
            assert_eq!(
                error.operation_error(),
                &expected_error,
                "trace {sequence:?}"
            );
            assert_eq!(error.discarded_deliveries(), 0, "trace {sequence:?}");
        }
        assert_eq!(
            self.observations[inbox].borrow().received,
            expected,
            "trace {sequence:?}"
        );
        assert_eq!(
            self.owner.pending(application_id),
            Ok(0),
            "trace {sequence:?}"
        );
    }
}

impl Application for FaultApplication {
    type StartError = Infallible;
    type WorkError = WorkFailure;
    type StopError = Infallible;
    type RestartError = Infallible;

    fn start(&mut self) -> Result<(), Self::StartError> {
        self.observations.borrow_mut().starts += 1;
        Ok(())
    }

    fn work(&mut self, _context: ApplicationWorkContext<'_>) -> Result<(), Self::WorkError> {
        self.observations.borrow_mut().works += 1;
        match self.work_failure {
            Some(error) => Err(error),
            None => Ok(()),
        }
    }

    fn stop(&mut self) -> Result<(), Self::StopError> {
        self.observations.borrow_mut().stops += 1;
        Ok(())
    }

    fn restart(&mut self) -> Result<(), Self::RestartError> {
        self.observations.borrow_mut().restarts += 1;
        Ok(())
    }
}

impl MessagingApplication<Topic, 1> for FaultApplication {
    type MessageError = Infallible;

    fn handle_message(
        &mut self,
        message: &Message<Topic, 1>,
        _context: &mut ApplicationMessageContext<'_, Topic, 1>,
    ) -> Result<(), Self::MessageError> {
        self.observations
            .borrow_mut()
            .received
            .push((*message.topic(), message.payload().to_vec()));
        Ok(())
    }
}

impl fmt::Display for WorkFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "requested work failure at position {}",
            self.application_position
        )
    }
}

impl Error for WorkFailure {}

fn public_state(state: State) -> ApplicationState {
    match state {
        State::Active => ApplicationState::Running,
        State::Paused => ApplicationState::Stopped,
        State::Terminal => ApplicationState::Failed,
    }
}

fn decode_sequence(mut ordinal: usize, length: usize) -> Vec<Operation> {
    (0..length)
        .map(|_| {
            let operation = OPERATIONS[ordinal % OPERATIONS.len()];
            ordinal /= OPERATIONS.len();
            operation
        })
        .collect()
}
