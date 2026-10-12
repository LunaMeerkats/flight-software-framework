//! Finite sequence evidence for returned message errors and peer publication.
//!
//! The frozen domain is recorded in RUNTIME_MESSAGE_FAILURE_SEQUENCE_REVIEW.md.
//! Independent accepted histories and retired cursors predict callback inputs,
//! immediate publication and selected clearing, including in-flight exclusion.

use std::cell::RefCell;
use std::convert::Infallible;
use std::error::Error;
use std::fmt;
use std::rc::Rc;

use rust_flight_framework::{
    Application, ApplicationId, ApplicationMessageContext, ApplicationState,
    ApplicationWorkContext, DeliveryStatus, LifecycleError, Message, MessageDispatchError,
    MessageDispatchOutcome, MessagingApplication, MessagingOperationError, MessagingRuntime,
    PublishClassification, PublishReport, Runtime, RuntimeInboxConfig,
};

const CAPACITIES: [usize; 2] = [1, 2];
const MAX_SEQUENCE_LENGTH: usize = 6;
const EXPECTED_TRACES: usize = 2_186;
const EXPECTED_OPERATIONS: usize = 12_030;
const REPLY_TAG: u8 = 0x80;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Topic {
    Command,
}

#[derive(Clone, Copy, Debug)]
enum Operation {
    Publish,
    DispatchFault,
    DispatchPeer,
}

const OPERATIONS: [Operation; 3] = [
    Operation::Publish,
    Operation::DispatchFault,
    Operation::DispatchPeer,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum State {
    Active,
    Terminal,
}

#[derive(Clone, Copy, Debug)]
enum Destination {
    Accepted,
    Full,
    Offline,
}

#[derive(Clone, Copy, Debug)]
enum DispatchResult {
    Empty,
    Succeeded,
    Failed { payload: u8, discarded: usize },
    Rejected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct MessageFailure {
    application_position: usize,
    received_payload: u8,
}

struct Publication {
    payload: u8,
    destinations: [Destination; 2],
}

struct Reference {
    faulting: usize,
    states: [State; 2],
    accepted: [Vec<u8>; 2],
    retired_through: [usize; 2],
    received: [Vec<u8>; 2],
    publications: [Vec<Publication>; 2],
}

#[derive(Debug)]
struct PublicationObservation {
    application_id: ApplicationId,
    topic: Topic,
    payload: Vec<u8>,
    report: PublishReport,
}

#[derive(Debug, Default)]
struct Observations {
    starts: usize,
    works: usize,
    stops: usize,
    restarts: usize,
    received: Vec<(ApplicationId, Topic, Vec<u8>)>,
    publications: Vec<PublicationObservation>,
}

#[derive(Debug)]
struct FaultApplication {
    fault_position: Option<usize>,
    observations: Rc<RefCell<Observations>>,
}

struct SequenceComparison {
    application_ids: [ApplicationId; 2],
    owner: MessagingRuntime<FaultApplication, Topic, 1>,
    observations: [Rc<RefCell<Observations>>; 2],
    reference: Reference,
}

#[test]
fn all_bounded_message_failure_sequences_match_reference_history() {
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
            retired_through: [0; 2],
            received: std::array::from_fn(|_| Vec::new()),
            publications: std::array::from_fn(|_| Vec::new()),
        }
    }

    fn outstanding(&self, inbox: usize) -> usize {
        self.accepted[inbox].len() - self.retired_through[inbox]
    }

    fn publish(&mut self, payload: u8) -> Publication {
        let destinations = [0, 1].map(|inbox| {
            if self.states[inbox] == State::Terminal {
                return Destination::Offline;
            }
            if self.outstanding(inbox) == CAPACITIES[inbox] {
                return Destination::Full;
            }
            self.accepted[inbox].push(payload);
            Destination::Accepted
        });
        Publication {
            payload,
            destinations,
        }
    }

    fn dispatch(&mut self, inbox: usize) -> DispatchResult {
        if self.states[inbox] == State::Terminal {
            return DispatchResult::Rejected;
        }
        if self.outstanding(inbox) == 0 {
            return DispatchResult::Empty;
        }
        let payload = self.accepted[inbox][self.retired_through[inbox]];
        self.retired_through[inbox] += 1;
        self.received[inbox].push(payload);
        if inbox != self.faulting {
            return DispatchResult::Succeeded;
        }
        // Accepted callback effects precede terminal commitment and cleanup.
        let publication = self.publish(REPLY_TAG | payload);
        self.publications[inbox].push(publication);
        let discarded = self.outstanding(inbox);
        self.retired_through[inbox] = self.accepted[inbox].len();
        self.states[inbox] = State::Terminal;
        DispatchResult::Failed { payload, discarded }
    }
}

impl SequenceComparison {
    fn new(faulting: usize) -> Self {
        let observations = std::array::from_fn(|_| Rc::new(RefCell::new(Observations::default())));
        let mut runtime = Runtime::new(2).expect("two records fit");
        let application_ids = [0, 1].map(|inbox| {
            runtime
                .register(FaultApplication {
                    fault_position: (inbox == faulting).then_some(inbox),
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
                let expected = self.reference.publish(payload);
                let message = Message::try_new(Topic::Command, &[payload]).expect("one byte fits");
                let report = self
                    .owner
                    .publish(&message)
                    .expect("report allocation succeeds");
                self.assert_report(&report, &expected, sequence);
            }
            Operation::DispatchFault => self.dispatch(faulting, sequence),
            Operation::DispatchPeer => self.dispatch(1 - faulting, sequence),
        }
    }

    fn dispatch(&mut self, inbox: usize, sequence: &[Operation]) {
        let expected = self.reference.dispatch(inbox);
        let actual = self.owner.dispatch_one(self.application_ids[inbox]);
        match expected {
            DispatchResult::Empty => {
                assert_eq!(
                    actual,
                    Ok(MessageDispatchOutcome::InboxEmpty),
                    "fault {}, trace {sequence:?}",
                    self.reference.faulting
                );
            }
            DispatchResult::Succeeded => {
                assert_eq!(
                    actual,
                    Ok(MessageDispatchOutcome::Dispatched),
                    "fault {}, trace {sequence:?}",
                    self.reference.faulting
                );
            }
            DispatchResult::Failed { payload, discarded } => self.assert_failure(
                actual.expect_err("selected callback returns its concrete error"),
                inbox,
                payload,
                discarded,
                sequence,
            ),
            DispatchResult::Rejected => self.assert_rejection(
                actual.expect_err("terminal dispatch is rejected"),
                inbox,
                sequence,
            ),
        }
    }

    fn assert_failure(
        &self,
        error: MessagingOperationError<MessageDispatchError<MessageFailure>>,
        inbox: usize,
        payload: u8,
        discarded: usize,
        sequence: &[Operation],
    ) {
        let source = MessageFailure {
            application_position: inbox,
            received_payload: payload,
        };
        let expected = MessageDispatchError::Application {
            application_id: self.application_ids[inbox],
            source,
        };
        assert_eq!(error.operation_error(), &expected, "trace {sequence:?}");
        assert_eq!(
            error.discarded_deliveries(),
            discarded,
            "trace {sequence:?}"
        );
        let dispatch_error = Error::source(&error)
            .and_then(|error| error.downcast_ref::<MessageDispatchError<MessageFailure>>())
            .expect("messaging error retains concrete dispatch error");
        assert_eq!(dispatch_error, &expected, "trace {sequence:?}");
        let concrete = Error::source(dispatch_error)
            .and_then(|error| error.downcast_ref::<MessageFailure>())
            .expect("dispatch error retains actual callback error");
        assert_eq!(concrete, &source, "trace {sequence:?}");
        assert!(Error::source(concrete).is_none(), "trace {sequence:?}");
    }

    fn assert_rejection(
        &self,
        error: MessagingOperationError<MessageDispatchError<MessageFailure>>,
        inbox: usize,
        sequence: &[Operation],
    ) {
        let lifecycle_error = LifecycleError::NotRunning {
            application_id: self.application_ids[inbox],
            state: ApplicationState::Failed,
        };
        assert_eq!(
            error.operation_error(),
            &MessageDispatchError::Lifecycle(lifecycle_error),
            "trace {sequence:?}"
        );
        assert_eq!(error.discarded_deliveries(), 0, "trace {sequence:?}");
        let dispatch_error = Error::source(&error)
            .and_then(|error| error.downcast_ref::<MessageDispatchError<MessageFailure>>())
            .expect("messaging error retains lifecycle rejection");
        assert_eq!(
            Error::source(dispatch_error).and_then(|error| error.downcast_ref()),
            Some(&lifecycle_error),
            "trace {sequence:?}"
        );
    }

    fn assert_report(
        &self,
        report: &PublishReport,
        expected: &Publication,
        sequence: &[Operation],
    ) {
        let actual: Vec<_> = report
            .outcomes()
            .iter()
            .map(|outcome| (outcome.application_id(), outcome.status()))
            .collect();
        let outcomes = [0, 1].map(|inbox| {
            let status = match expected.destinations[inbox] {
                Destination::Accepted => DeliveryStatus::Delivered,
                Destination::Full => DeliveryStatus::InboxFull,
                Destination::Offline => DeliveryStatus::Unavailable,
            };
            (self.application_ids[inbox], status)
        });
        let accepted = expected
            .destinations
            .iter()
            .filter(|destination| matches!(destination, Destination::Accepted))
            .count();
        let classification = match accepted {
            0 => PublishClassification::WhollyUndelivered,
            2 => PublishClassification::Complete,
            _ => PublishClassification::Partial,
        };
        assert_eq!(
            actual, outcomes,
            "fault {}, publication {}, trace {sequence:?}",
            self.reference.faulting, expected.payload
        );
        assert_eq!(
            report.classification(),
            classification,
            "trace {sequence:?}"
        );
    }

    fn assert_state(&self, sequence: &[Operation]) {
        for (inbox, capacity) in CAPACITIES.into_iter().enumerate() {
            let application_id = self.application_ids[inbox];
            let outstanding = self.reference.outstanding(inbox);
            let state = match self.reference.states[inbox] {
                State::Active => ApplicationState::Running,
                State::Terminal => ApplicationState::Failed,
            };
            assert!(outstanding <= capacity, "inbox {inbox}, trace {sequence:?}");
            assert_eq!(self.owner.inbox_capacity(application_id), Ok(capacity));
            assert_eq!(
                self.owner.pending(application_id),
                Ok(outstanding),
                "fault {}, inbox {inbox}, trace {sequence:?}",
                self.reference.faulting
            );
            assert_eq!(
                self.owner.state(application_id),
                Ok(state),
                "inbox {inbox}, trace {sequence:?}"
            );
            if self.reference.states[inbox] == State::Terminal {
                assert_eq!(outstanding, 0, "trace {sequence:?}");
            }
            self.assert_observations(inbox, sequence);
        }
    }

    fn assert_observations(&self, inbox: usize, sequence: &[Operation]) {
        let application_id = self.application_ids[inbox];
        let observed = self.observations[inbox].borrow();
        assert_eq!(
            (
                observed.starts,
                observed.works,
                observed.stops,
                observed.restarts
            ),
            (1, 0, 0, 0),
            "inbox {inbox}, trace {sequence:?}"
        );
        let received: Vec<_> = self.reference.received[inbox]
            .iter()
            .map(|&byte| (application_id, Topic::Command, vec![byte]))
            .collect();
        assert_eq!(
            observed.received, received,
            "fault {}, inbox {inbox}, trace {sequence:?}",
            self.reference.faulting
        );
        assert_eq!(
            observed.publications.len(),
            self.reference.publications[inbox].len(),
            "trace {sequence:?}"
        );
        for (actual, expected) in observed
            .publications
            .iter()
            .zip(&self.reference.publications[inbox])
        {
            assert_eq!(actual.application_id, application_id, "trace {sequence:?}");
            assert_eq!(actual.topic, Topic::Command, "trace {sequence:?}");
            assert_eq!(actual.payload, [expected.payload], "trace {sequence:?}");
            self.assert_report(&actual.report, expected, sequence);
        }
    }

    fn observe_remaining(&mut self, sequence: &[Operation]) {
        // Only the passive peer is drained; an unfailed prefix stays unfailed.
        let peer = 1 - self.reference.faulting;
        let remaining = self.reference.outstanding(peer);
        for _ in 0..remaining {
            self.dispatch(peer, sequence);
            self.assert_state(sequence);
        }
        self.dispatch(peer, sequence);
        self.assert_state(sequence);
        if self.reference.states[self.reference.faulting] == State::Terminal {
            self.dispatch(self.reference.faulting, sequence);
            self.assert_state(sequence);
        }
    }
}

impl Application for FaultApplication {
    type StartError = Infallible;
    type WorkError = Infallible;
    type StopError = Infallible;
    type RestartError = Infallible;

    fn start(&mut self) -> Result<(), Self::StartError> {
        self.observations.borrow_mut().starts += 1;
        Ok(())
    }

    fn work(&mut self, _context: ApplicationWorkContext<'_>) -> Result<(), Self::WorkError> {
        self.observations.borrow_mut().works += 1;
        Ok(())
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
    type MessageError = MessageFailure;

    fn handle_message(
        &mut self,
        message: &Message<Topic, 1>,
        context: &mut ApplicationMessageContext<'_, Topic, 1>,
    ) -> Result<(), Self::MessageError> {
        let application_id = context.application_id();
        self.observations.borrow_mut().received.push((
            application_id,
            *message.topic(),
            message.payload().to_vec(),
        ));
        let Some(application_position) = self.fault_position else {
            return Ok(());
        };
        let received_payload = message.payload()[0];
        let reply = Message::try_new(Topic::Command, &[REPLY_TAG | received_payload])
            .expect("one byte reply fits");
        let report = context.publish(&reply).expect("report allocation succeeds");
        self.observations
            .borrow_mut()
            .publications
            .push(PublicationObservation {
                application_id,
                topic: *reply.topic(),
                payload: reply.payload().to_vec(),
                report,
            });
        Err(MessageFailure {
            application_position,
            received_payload,
        })
    }
}

impl fmt::Display for MessageFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "requested message failure at position {} for payload {}",
            self.application_position, self.received_payload
        )
    }
}

impl Error for MessageFailure {}

fn decode_sequence(mut ordinal: usize, length: usize) -> Vec<Operation> {
    (0..length)
        .map(|_| {
            let operation = OPERATIONS[ordinal % OPERATIONS.len()];
            ordinal /= OPERATIONS.len();
            operation
        })
        .collect()
}
