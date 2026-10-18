//! Finite messaging-owned failure-event histories through the public API.
//!
//! MESSAGING_EVENT_SEQUENCE_REVIEW.md freezes the domain. Separate accepted
//! message/event histories and cursors predict retained records independently
//! of production storage. Failure history predicts lifecycle and work effects.

use std::cell::{Cell, RefCell};
use std::convert::Infallible;
use std::error::Error;
use std::fmt;
use std::rc::Rc;
use std::time::Duration;

use rust_flight_framework::{
    Application, ApplicationId, ApplicationMessageContext, ApplicationState,
    ApplicationWorkContext, Clock, DeliveryStatus, Event, EventEmitOutcome, EventQueue,
    EventSeverity, EventSource, EventTimestamp, FrameworkInstant, LifecycleError, ManualClock,
    Message, MessageDispatchError, MessageDispatchOutcome, MessagingApplication, MessagingRuntime,
    PublishClassification, Runtime, RuntimeInboxConfig, RuntimeWorkError,
};

const INBOX_CAPACITIES: [usize; 2] = [1, 2];
const MAX_SEQUENCE_LENGTH: usize = 5;
const EXPECTED_TRACES: usize = 78_432;
const EXPECTED_OPERATIONS: usize = 379_092;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Topic {
    Command,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MissionEventId {
    OlderDiagnostic,
    AlphaFailure,
    BetaFailure,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Record {
    Older,
    Failure { application: usize, seconds: u64 },
}

#[derive(Clone, Copy, Debug)]
enum Operation {
    Publish,
    FaultWork,
    PeerWork,
    PeerDispatch,
    Advance,
    EventDequeue,
    Retry,
}

const OPERATIONS: [Operation; 7] = [
    Operation::Publish,
    Operation::FaultWork,
    Operation::PeerWork,
    Operation::PeerDispatch,
    Operation::Advance,
    Operation::EventDequeue,
    Operation::Retry,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct WorkFailure(u16);

#[derive(Default)]
struct Observations {
    works: usize,
    messages: Vec<u8>,
}

struct TestApplication {
    observations: Rc<RefCell<Observations>>,
    failure: Option<WorkFailure>,
}

struct CountingClock {
    manual: ManualClock,
    reads: Cell<usize>,
}

struct Reference {
    faulting: usize,
    failed_at: Option<u64>,
    works: [usize; 2],
    accepted_messages: [Vec<u8>; 2],
    consumed_messages: [usize; 2],
    discarded_through: [usize; 2],
    seconds: u64,
    retry: Option<Record>,
    accepted_events: Vec<Record>,
    consumed_events: usize,
}

#[derive(Default)]
struct Supplemental {
    peer_works: usize,
    message_dispatches: usize,
    final_dispatch_checks: usize,
    event_dequeues: usize,
}

struct SequenceComparison {
    owner: MessagingRuntime<TestApplication, Topic, 1>,
    ids: [ApplicationId; 2],
    observations: [Rc<RefCell<Observations>>; 2],
    clock: CountingClock,
    events: EventQueue<MissionEventId>,
    retry: Option<Event<MissionEventId>>,
    reference: Reference,
    event_capacity: usize,
}

#[test]
fn all_bounded_messaging_event_sequences_match_reference_histories() {
    let mut traces = 0;
    let mut operations = 0;
    let mut supplemental = Supplemental::default();
    for faulting in 0..2 {
        for event_capacity in [1, 2] {
            let mut sequences_at_length = 1;
            for length in 0..=MAX_SEQUENCE_LENGTH {
                for ordinal in 0..sequences_at_length {
                    let sequence = decode_sequence(ordinal, length);
                    let mut comparison = SequenceComparison::new(faulting, event_capacity);
                    comparison.assert_state(&sequence, 0);
                    for (position, operation) in sequence.iter().copied().enumerate() {
                        comparison.perform(operation, &sequence, position + 1);
                    }
                    comparison.observe_remaining(&sequence, &mut supplemental);
                    traces += 1;
                    operations += length;
                }
                sequences_at_length *= OPERATIONS.len();
            }
        }
    }
    assert_eq!(traces, EXPECTED_TRACES);
    assert_eq!(operations, EXPECTED_OPERATIONS);
    assert_eq!(supplemental.peer_works, EXPECTED_TRACES);
    assert_eq!(supplemental.message_dispatches, 61_384);
    assert_eq!(supplemental.final_dispatch_checks, 156_864);
    assert_eq!(supplemental.event_dequeues, 217_988);
}

impl Application for TestApplication {
    type StartError = Infallible;
    type WorkError = WorkFailure;
    type StopError = Infallible;
    type RestartError = Infallible;

    fn start(&mut self) -> Result<(), Self::StartError> {
        Ok(())
    }

    fn work(&mut self, _context: ApplicationWorkContext<'_>) -> Result<(), Self::WorkError> {
        self.observations.borrow_mut().works += 1;
        self.failure.map_or(Ok(()), Err)
    }

    fn stop(&mut self) -> Result<(), Self::StopError> {
        Ok(())
    }

    fn restart(&mut self) -> Result<(), Self::RestartError> {
        Ok(())
    }
}

impl MessagingApplication<Topic, 1> for TestApplication {
    type MessageError = Infallible;

    fn handle_message(
        &mut self,
        message: &Message<Topic, 1>,
        _context: &mut ApplicationMessageContext<'_, Topic, 1>,
    ) -> Result<(), Self::MessageError> {
        assert_eq!(*message.topic(), Topic::Command);
        self.observations
            .borrow_mut()
            .messages
            .extend_from_slice(message.payload());
        Ok(())
    }
}

impl Clock for CountingClock {
    fn now(&self) -> FrameworkInstant {
        self.reads.set(self.reads.get() + 1);
        self.manual.now()
    }
}

impl fmt::Display for WorkFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "requested work failure {}", self.0)
    }
}

impl Error for WorkFailure {}

impl Record {
    fn event(self, ids: &[ApplicationId; 2]) -> Event<MissionEventId> {
        let (source, severity, identifier, seconds) = match self {
            Self::Older => (
                EventSource::Framework,
                EventSeverity::Warning,
                MissionEventId::OlderDiagnostic,
                99,
            ),
            Self::Failure {
                application,
                seconds,
            } => (
                EventSource::Application(ids[application]),
                EventSeverity::Error,
                failure_identifier(application),
                seconds,
            ),
        };
        Event::new(
            source,
            severity,
            identifier,
            EventTimestamp::from_elapsed(Duration::from_secs(seconds)),
        )
    }
}

impl Reference {
    fn running(&self, application: usize) -> bool {
        application != self.faulting || self.failed_at.is_none()
    }

    fn pending_messages(&self, application: usize) -> usize {
        self.accepted_messages[application].len()
            - self.consumed_messages[application].max(self.discarded_through[application])
    }

    fn pending_events(&self) -> usize {
        self.accepted_events.len() - self.consumed_events
    }

    fn emit(&mut self, record: Record, capacity: usize) -> EventEmitOutcome {
        if self.pending_events() == capacity {
            EventEmitOutcome::QueueFull { capacity }
        } else {
            self.accepted_events.push(record);
            EventEmitOutcome::Recorded
        }
    }

    fn dequeue_event(&mut self) -> Option<Record> {
        let record = self.accepted_events.get(self.consumed_events).copied()?;
        self.consumed_events += 1;
        Some(record)
    }
}

impl SequenceComparison {
    fn new(faulting: usize, event_capacity: usize) -> Self {
        let observations = std::array::from_fn(|_| Rc::new(RefCell::new(Observations::default())));
        let mut runtime = Runtime::new(2).expect("two application records fit");
        let ids = std::array::from_fn(|application| {
            runtime
                .register(TestApplication {
                    observations: Rc::clone(&observations[application]),
                    failure: (application == faulting).then_some(work_failure(application)),
                })
                .expect("test application fits")
        });
        let inboxes =
            INBOX_CAPACITIES.map(|capacity| RuntimeInboxConfig::new(capacity, &[Topic::Command]));
        let mut owner = MessagingRuntime::new(runtime, &inboxes).expect("test topology is valid");
        for id in ids {
            owner.start(id).expect("test application starts");
        }
        let mut events = EventQueue::new(event_capacity).expect("positive test capacity fits");
        assert_eq!(
            events.emit(&Record::Older.event(&ids)),
            EventEmitOutcome::Recorded
        );
        Self {
            owner,
            ids,
            observations,
            clock: CountingClock {
                manual: ManualClock::new(),
                reads: Cell::new(0),
            },
            events,
            retry: None,
            reference: Reference {
                faulting,
                failed_at: None,
                works: [0; 2],
                accepted_messages: std::array::from_fn(|_| Vec::new()),
                consumed_messages: [0; 2],
                discarded_through: [0; 2],
                seconds: 0,
                retry: None,
                accepted_events: vec![Record::Older],
                consumed_events: 0,
            },
            event_capacity,
        }
    }

    fn perform(&mut self, operation: Operation, sequence: &[Operation], position: usize) {
        let context = self.context(sequence, position);
        match operation {
            Operation::Publish => {
                self.publish(
                    u8::try_from(position).expect("five positions fit"),
                    &context,
                );
            }
            Operation::FaultWork => self.work(self.reference.faulting, &context),
            Operation::PeerWork => self.work(1 - self.reference.faulting, &context),
            Operation::PeerDispatch => self.dispatch(1 - self.reference.faulting, &context),
            Operation::Advance => {
                self.reference.seconds += 1;
                assert_eq!(
                    self.clock.manual.advance(Duration::from_secs(1)),
                    Ok(FrameworkInstant::from_elapsed(Duration::from_secs(
                        self.reference.seconds
                    ))),
                    "{context}"
                );
            }
            Operation::EventDequeue => {
                let expected = self
                    .reference
                    .dequeue_event()
                    .map(|record| record.event(&self.ids));
                assert_eq!(self.events.dequeue(), expected, "{context}");
            }
            Operation::Retry => self.retry(&context),
        }
        self.assert_state(sequence, position);
    }

    fn publish(&mut self, payload: u8, context: &str) {
        let expected = [0, 1].map(|application| {
            let status = if !self.reference.running(application) {
                DeliveryStatus::Unavailable
            } else if self.reference.pending_messages(application) == INBOX_CAPACITIES[application]
            {
                DeliveryStatus::InboxFull
            } else {
                self.reference.accepted_messages[application].push(payload);
                DeliveryStatus::Delivered
            };
            (self.ids[application], status)
        });
        let message = Message::try_new(Topic::Command, &[payload]).expect("one byte fits");
        let actual = self
            .owner
            .publish(&message)
            .expect("report allocation succeeds");
        let outcomes: Vec<_> = actual
            .outcomes()
            .iter()
            .map(|outcome| (outcome.application_id(), outcome.status()))
            .collect();
        assert_eq!(outcomes, expected, "{context}");
        let delivered = expected
            .iter()
            .filter(|(_, status)| *status == DeliveryStatus::Delivered)
            .count();
        let classification = match delivered {
            0 => PublishClassification::WhollyUndelivered,
            2 => PublishClassification::Complete,
            _ => PublishClassification::Partial,
        };
        assert_eq!(actual.classification(), classification, "{context}");
    }

    fn work(&mut self, application: usize, context: &str) {
        let id = self.ids[application];
        let result = self.owner.work_with_failure_event(
            id,
            failure_identifier(application),
            &self.clock,
            &mut self.events,
        );
        if application != self.reference.faulting {
            self.reference.works[application] += 1;
            assert_eq!(result, Ok(ApplicationState::Running), "{context}");
            return;
        }
        let error = result.expect_err(context);
        let operation_error = error.operation_error();
        if self.reference.failed_at.is_some() {
            assert_eq!(
                operation_error.work_error(),
                &RuntimeWorkError::Lifecycle(LifecycleError::NotRunning {
                    application_id: id,
                    state: ApplicationState::Failed,
                }),
                "{context}"
            );
            assert_eq!(error.discarded_deliveries(), 0, "{context}");
            assert_eq!(operation_error.failure_event_attempt(), None, "{context}");
            return;
        }
        self.reference.works[application] += 1;
        self.reference.failed_at = Some(self.reference.seconds);
        assert_eq!(
            operation_error.work_error(),
            &RuntimeWorkError::Application {
                application_id: id,
                source: work_failure(application),
            },
            "{context}"
        );
        assert_eq!(
            error.discarded_deliveries(),
            self.reference.pending_messages(application),
            "{context}"
        );
        self.reference.discarded_through[application] =
            self.reference.accepted_messages[application].len();
        let record = Record::Failure {
            application,
            seconds: self.reference.seconds,
        };
        let expected = self.reference.emit(record, self.event_capacity);
        let attempt = operation_error.failure_event_attempt().expect(context);
        assert_eq!(attempt.event(), &record.event(&self.ids), "{context}");
        assert_eq!(attempt.outcome(), expected, "{context}");
        if matches!(expected, EventEmitOutcome::QueueFull { .. }) {
            self.reference.retry = Some(record);
            self.retry = Some(*attempt.event());
        }
    }

    fn dispatch(&mut self, application: usize, context: &str) {
        let actual = self.owner.dispatch_one(self.ids[application]);
        if !self.reference.running(application) {
            let error = actual.expect_err(context);
            assert_eq!(
                error.operation_error(),
                &MessageDispatchError::Lifecycle(LifecycleError::NotRunning {
                    application_id: self.ids[application],
                    state: ApplicationState::Failed,
                }),
                "{context}"
            );
            assert_eq!(error.discarded_deliveries(), 0, "{context}");
        } else if self.reference.pending_messages(application) == 0 {
            assert_eq!(actual, Ok(MessageDispatchOutcome::InboxEmpty), "{context}");
        } else {
            self.reference.consumed_messages[application] += 1;
            assert_eq!(actual, Ok(MessageDispatchOutcome::Dispatched), "{context}");
        }
    }

    fn retry(&mut self, context: &str) {
        // The symbolic slot and returned copy model one explicit caller policy.
        // Retry cannot restore lifecycle, inbox deliveries or failure time.
        let Some(record) = self.reference.retry else {
            assert_eq!(self.retry, None, "{context}");
            return;
        };
        let event = self.retry.expect(context);
        assert_eq!(event, record.event(&self.ids), "{context}");
        let expected = self.reference.emit(record, self.event_capacity);
        assert_eq!(self.events.emit(&event), expected, "{context}");
        if expected == EventEmitOutcome::Recorded {
            self.reference.retry = None;
            self.retry = None;
        }
    }

    fn assert_state(&self, sequence: &[Operation], position: usize) {
        let context = self.context(sequence, position);
        for (application, capacity) in INBOX_CAPACITIES.into_iter().enumerate() {
            let expected = if self.reference.running(application) {
                ApplicationState::Running
            } else {
                ApplicationState::Failed
            };
            assert_eq!(
                self.owner.state(self.ids[application]),
                Ok(expected),
                "{context}"
            );
            assert_eq!(
                self.owner.inbox_capacity(self.ids[application]),
                Ok(capacity),
                "{context}"
            );
            assert_eq!(
                self.owner.pending(self.ids[application]),
                Ok(self.reference.pending_messages(application)),
                "{context}"
            );
            let observed = self.observations[application].borrow();
            assert_eq!(
                observed.works, self.reference.works[application],
                "{context}"
            );
            assert_eq!(
                observed.messages,
                self.reference.accepted_messages[application]
                    [..self.reference.consumed_messages[application]],
                "{context}"
            );
        }
        assert_eq!(
            self.clock.reads.get(),
            usize::from(self.reference.failed_at.is_some()),
            "{context}"
        );
        assert_eq!(self.events.capacity(), self.event_capacity, "{context}");
        assert_eq!(
            self.events.pending(),
            self.reference.pending_events(),
            "{context}"
        );
        assert_eq!(
            self.events.is_empty(),
            self.reference.pending_events() == 0,
            "{context}"
        );
        let expected_retry = self.reference.retry.map(|record| record.event(&self.ids));
        assert_eq!(self.retry, expected_retry, "{context}");
    }

    fn observe_remaining(&mut self, sequence: &[Operation], counts: &mut Supplemental) {
        // Supplemental probes expose every final FIFO suffix and peer progress.
        // Their counts remain separate from generated actions.
        let position = sequence.len() + 1;
        self.perform(Operation::PeerWork, sequence, position);
        counts.peer_works += 1;
        for application in 0..2 {
            let remaining = self.reference.pending_messages(application);
            for _ in 0..remaining {
                self.dispatch(application, &self.context(sequence, position));
                self.assert_state(sequence, position);
                counts.message_dispatches += 1;
            }
            self.dispatch(application, &self.context(sequence, position));
            self.assert_state(sequence, position);
            counts.final_dispatch_checks += 1;
        }
        let remaining = self.reference.pending_events();
        for offset in 0..remaining + 2 {
            self.perform(Operation::EventDequeue, sequence, position + offset + 1);
            counts.event_dequeues += 1;
        }
    }

    fn context(&self, sequence: &[Operation], position: usize) -> String {
        format!(
            "fault {}, event capacity {}, trace {sequence:?}, step {position}",
            self.reference.faulting, self.event_capacity,
        )
    }
}

fn work_failure(application: usize) -> WorkFailure {
    WorkFailure([11, 22][application])
}

fn failure_identifier(application: usize) -> MissionEventId {
    [MissionEventId::AlphaFailure, MissionEventId::BetaFailure][application]
}

fn decode_sequence(mut ordinal: usize, length: usize) -> Vec<Operation> {
    let mut sequence = Vec::with_capacity(length);
    for _ in 0..length {
        sequence.push(OPERATIONS[ordinal % OPERATIONS.len()]);
        ordinal /= OPERATIONS.len();
    }
    sequence
}
