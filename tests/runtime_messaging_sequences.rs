//! Finite sequence evidence for runtime-owned availability and inbox clearing.
//!
//! The frozen domain is recorded in RUNTIME_MESSAGE_SEQUENCE_REVIEW.md.
//! Accepted histories and discard watermarks predict retained deliveries;
//! passive final dispatch observes them without generating dispatch sequences.

use std::cell::RefCell;
use std::convert::Infallible;
use std::rc::Rc;

use rust_flight_framework::{
    Application, ApplicationId, ApplicationMessageContext, ApplicationState,
    ApplicationWorkContext, DeliveryStatus, LifecycleError, LifecycleOperation, Message,
    MessageDispatchError, MessageDispatchOutcome, MessagingApplication, MessagingRuntime,
    PublishClassification, Runtime, RuntimeInboxConfig, RuntimeRestartError, RuntimeStopError,
};

const CAPACITIES: [usize; 2] = [1, 2];
const MAX_SEQUENCE_LENGTH: usize = 6;
const EXPECTED_TRACES: usize = 19_531;
const EXPECTED_OPERATIONS: usize = 112_305;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Topic {
    Command,
}

#[derive(Clone, Copy, Debug)]
enum Operation {
    Publish,
    Stop(usize),
    Restart(usize),
}

const OPERATIONS: [Operation; 5] = [
    Operation::Publish,
    Operation::Stop(0),
    Operation::Stop(1),
    Operation::Restart(0),
    Operation::Restart(1),
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum State {
    Active,
    Paused,
}

#[derive(Clone, Copy, Debug)]
enum Destination {
    Accepted,
    Full,
    Offline,
}

struct Reference {
    states: [State; 2],
    accepted: [Vec<u8>; 2],
    discarded_through: [usize; 2],
    stops: [usize; 2],
    restarts: [usize; 2],
}

#[derive(Debug, Default)]
struct Observations {
    starts: usize,
    stops: usize,
    restarts: usize,
    received: Vec<(Topic, Vec<u8>)>,
}

#[derive(Debug)]
struct PassiveApplication(Rc<RefCell<Observations>>);

struct SequenceComparison {
    application_ids: [ApplicationId; 2],
    owner: MessagingRuntime<PassiveApplication, Topic, 1>,
    observations: [Rc<RefCell<Observations>>; 2],
    reference: Reference,
}

#[test]
fn all_bounded_publication_stop_restart_sequences_match_reference_history() {
    let mut trace_count = 0;
    let mut operation_count = 0;
    let mut sequences_at_length = 1;
    for length in 0..=MAX_SEQUENCE_LENGTH {
        for ordinal in 0..sequences_at_length {
            let sequence = decode_sequence(ordinal, length);
            let mut comparison = SequenceComparison::new();
            comparison.assert_state(&sequence);
            for (position, operation) in sequence.iter().copied().enumerate() {
                match operation {
                    Operation::Publish => {
                        let payload = u8::try_from(position).expect("six positions fit");
                        comparison.publish(payload, &sequence);
                    }
                    Operation::Stop(inbox) => comparison.stop(inbox, &sequence),
                    Operation::Restart(inbox) => comparison.restart(inbox, &sequence),
                }
                comparison.assert_state(&sequence);
            }
            comparison.observe_remaining(&sequence);
            trace_count += 1;
            operation_count += sequence.len();
        }
        sequences_at_length *= OPERATIONS.len();
    }
    assert_eq!(trace_count, EXPECTED_TRACES);
    assert_eq!(operation_count, EXPECTED_OPERATIONS);
}

impl Reference {
    fn new() -> Self {
        Self {
            states: [State::Active; 2],
            accepted: std::array::from_fn(|_| Vec::new()),
            discarded_through: [0; 2],
            stops: [0; 2],
            restarts: [0; 2],
        }
    }

    fn outstanding(&self, inbox: usize) -> usize {
        self.accepted[inbox].len() - self.discarded_through[inbox]
    }

    fn publish(&mut self, payload: u8) -> [Destination; 2] {
        // Both configured routes remain present even while unavailable.
        [0, 1].map(|inbox| {
            if self.states[inbox] == State::Paused {
                return Destination::Offline;
            }
            if self.outstanding(inbox) == CAPACITIES[inbox] {
                return Destination::Full;
            }
            self.accepted[inbox].push(payload);
            Destination::Accepted
        })
    }

    fn stop(&mut self, inbox: usize) -> Option<usize> {
        if self.states[inbox] == State::Paused {
            return None;
        }
        let discarded = self.outstanding(inbox);
        self.discarded_through[inbox] = self.accepted[inbox].len();
        self.states[inbox] = State::Paused;
        self.stops[inbox] += 1;
        Some(discarded)
    }

    fn restart(&mut self, inbox: usize) -> bool {
        if self.states[inbox] == State::Active {
            return false;
        }
        self.states[inbox] = State::Active;
        self.restarts[inbox] += 1;
        true
    }
}

impl SequenceComparison {
    fn new() -> Self {
        let observations = std::array::from_fn(|_| Rc::new(RefCell::new(Observations::default())));
        let mut runtime = Runtime::new(2).expect("two records fit");
        let application_ids = observations.each_ref().map(|observer| {
            runtime
                .register(PassiveApplication(Rc::clone(observer)))
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
            reference: Reference::new(),
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
            "payload {payload}, trace {sequence:?}"
        );
        assert_eq!(
            report.classification(),
            classification,
            "trace {sequence:?}"
        );
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
                let error = actual.expect_err("Paused stop is rejected");
                let expected_error =
                    RuntimeStopError::Lifecycle(LifecycleError::InvalidTransition {
                        application_id,
                        state: ApplicationState::Stopped,
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
        let error = actual.expect_err("Active restart is rejected");
        let expected_error = RuntimeRestartError::Lifecycle(LifecycleError::InvalidTransition {
            application_id,
            state: ApplicationState::Running,
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
                "trace {sequence:?}"
            );
            assert!(observed.received.is_empty(), "trace {sequence:?}");
            if self.reference.states[inbox] == State::Paused {
                assert_eq!(outstanding, 0, "inbox {inbox}, trace {sequence:?}");
            }
            self.assert_lifecycle(inbox, sequence);
        }
    }

    fn assert_lifecycle(&self, inbox: usize, sequence: &[Operation]) {
        let observed = self.observations[inbox].borrow();
        assert_eq!(
            self.owner.state(self.application_ids[inbox]),
            Ok(public_state(self.reference.states[inbox])),
            "inbox {inbox}, trace {sequence:?}"
        );
        assert_eq!(
            (observed.starts, observed.stops, observed.restarts),
            (
                1,
                self.reference.stops[inbox],
                self.reference.restarts[inbox]
            ),
            "inbox {inbox}, trace {sequence:?}"
        );
    }

    fn observe_remaining(&mut self, sequence: &[Operation]) {
        for inbox in 0..CAPACITIES.len() {
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
                let error = final_attempt.expect_err("Paused dispatch is rejected");
                let expected_error = MessageDispatchError::Lifecycle(LifecycleError::NotRunning {
                    application_id,
                    state: ApplicationState::Stopped,
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
        for inbox in 0..CAPACITIES.len() {
            self.assert_lifecycle(inbox, sequence);
        }
    }
}

impl Application for PassiveApplication {
    type StartError = Infallible;
    type WorkError = Infallible;
    type StopError = Infallible;
    type RestartError = Infallible;

    fn start(&mut self) -> Result<(), Self::StartError> {
        self.0.borrow_mut().starts += 1;
        Ok(())
    }

    fn work(&mut self, _context: ApplicationWorkContext<'_>) -> Result<(), Self::WorkError> {
        Ok(())
    }

    fn stop(&mut self) -> Result<(), Self::StopError> {
        self.0.borrow_mut().stops += 1;
        Ok(())
    }

    fn restart(&mut self) -> Result<(), Self::RestartError> {
        self.0.borrow_mut().restarts += 1;
        Ok(())
    }
}

impl MessagingApplication<Topic, 1> for PassiveApplication {
    type MessageError = Infallible;

    fn handle_message(
        &mut self,
        message: &Message<Topic, 1>,
        _context: &mut ApplicationMessageContext<'_, Topic, 1>,
    ) -> Result<(), Self::MessageError> {
        self.0
            .borrow_mut()
            .received
            .push((*message.topic(), message.payload().to_vec()));
        Ok(())
    }
}

fn public_state(state: State) -> ApplicationState {
    match state {
        State::Active => ApplicationState::Running,
        State::Paused => ApplicationState::Stopped,
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
