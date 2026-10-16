//! Finite public-API evidence for standalone event-queue histories.
//!
//! The frozen domain is in EVENT_QUEUE_SEQUENCE_REVIEW.md. An append-only
//! accepted-record ledger and consumed ordinal predict observations without
//! reproducing the queue's storage or relying on timestamp order.

use std::time::Duration;

use rust_flight_framework::{
    ApplicationId, Event, EventEmitOutcome, EventQueue, EventSeverity, EventSource, EventTimestamp,
    LifecycleRegistry,
};

const MAX_SEQUENCE_LENGTH: usize = 8;
const EXPECTED_TRACES: usize = 29_523;
const EXPECTED_OPERATIONS: usize = 221_436;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MissionEventId {
    Alpha,
    Beta,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Record {
    A,
    B,
}

#[derive(Clone, Copy, Debug)]
enum Operation {
    Emit(Record),
    Dequeue,
}

const OPERATIONS: [Operation; 3] = [
    Operation::Emit(Record::A),
    Operation::Emit(Record::B),
    Operation::Dequeue,
];

struct Reference {
    accepted: Vec<Record>,
    consumed: usize,
}

struct SequenceComparison {
    queue: EventQueue<MissionEventId>,
    reference: Reference,
    capacity: usize,
    application_id: ApplicationId,
}

#[test]
fn all_bounded_event_sequences_match_reference_history() {
    let mut registry = LifecycleRegistry::new(1).expect("test registry capacity is positive");
    let application_id = registry.register().expect("one application fits");
    let mut trace_count = 0;
    let mut operation_count = 0;
    for capacity in [1, 2, 3] {
        let mut sequences_at_length = 1;
        for length in 0..=MAX_SEQUENCE_LENGTH {
            for ordinal in 0..sequences_at_length {
                let sequence = decode_sequence(ordinal, length);
                let mut comparison = SequenceComparison::new(capacity, application_id);
                comparison.assert_state(&sequence, 0);
                for (position, operation) in sequence.iter().copied().enumerate() {
                    comparison.perform(operation, &sequence, position + 1);
                }
                comparison.drain_remaining(&sequence);
                trace_count += 1;
                operation_count += sequence.len();
            }
            sequences_at_length *= OPERATIONS.len();
        }
    }
    assert_eq!(trace_count, EXPECTED_TRACES);
    assert_eq!(operation_count, EXPECTED_OPERATIONS);
}

impl Record {
    fn event(self, application_id: ApplicationId) -> Event<MissionEventId> {
        let (source, severity, identifier, milliseconds) = match self {
            Self::A => (
                EventSource::Framework,
                EventSeverity::Informational,
                MissionEventId::Alpha,
                20,
            ),
            Self::B => (
                EventSource::Application(application_id),
                EventSeverity::Error,
                MissionEventId::Beta,
                10,
            ),
        };
        Event::new(
            source,
            severity,
            identifier,
            EventTimestamp::from_elapsed(Duration::from_millis(milliseconds)),
        )
    }
}

impl Reference {
    fn pending(&self) -> usize {
        self.accepted.len() - self.consumed
    }

    fn emit(&mut self, record: Record, capacity: usize) -> EventEmitOutcome {
        if self.pending() == capacity {
            EventEmitOutcome::QueueFull { capacity }
        } else {
            self.accepted.push(record);
            EventEmitOutcome::Recorded
        }
    }

    fn dequeue(&mut self) -> Option<Record> {
        let record = self.accepted.get(self.consumed).copied()?;
        self.consumed += 1;
        Some(record)
    }
}

impl SequenceComparison {
    fn new(capacity: usize, application_id: ApplicationId) -> Self {
        Self {
            queue: EventQueue::new(capacity).expect("frozen positive capacity can be reserved"),
            reference: Reference {
                accepted: Vec::new(),
                consumed: 0,
            },
            capacity,
            application_id,
        }
    }

    fn perform(&mut self, operation: Operation, sequence: &[Operation], position: usize) {
        match operation {
            Operation::Emit(record) => {
                let expected = self.reference.emit(record, self.capacity);
                let actual = self.queue.emit(&record.event(self.application_id));
                assert_eq!(
                    actual, expected,
                    "capacity {}, trace {sequence:?}, step {position}",
                    self.capacity
                );
            }
            Operation::Dequeue => {
                let expected = self
                    .reference
                    .dequeue()
                    .map(|record| record.event(self.application_id));
                assert_eq!(
                    self.queue.dequeue(),
                    expected,
                    "capacity {}, trace {sequence:?}, step {position}",
                    self.capacity
                );
            }
        }
        self.assert_state(sequence, position);
    }

    fn assert_state(&self, sequence: &[Operation], position: usize) {
        assert_eq!(
            self.queue.capacity(),
            self.capacity,
            "capacity {}, trace {sequence:?}, step {position}",
            self.capacity
        );
        assert_eq!(
            self.queue.pending(),
            self.reference.pending(),
            "capacity {}, trace {sequence:?}, step {position}",
            self.capacity
        );
        assert_eq!(
            self.queue.is_empty(),
            self.reference.pending() == 0,
            "capacity {}, trace {sequence:?}, step {position}",
            self.capacity
        );
    }

    fn drain_remaining(&mut self, sequence: &[Operation]) {
        // Terminal observations expose the retained suffix without changing
        // any generated history. Two further reads check stable exhaustion.
        let remaining = self.reference.pending();
        for offset in 0..remaining + 2 {
            self.perform(Operation::Dequeue, sequence, sequence.len() + offset + 1);
        }
    }
}

fn decode_sequence(mut ordinal: usize, length: usize) -> Vec<Operation> {
    let mut sequence = Vec::with_capacity(length);
    for _ in 0..length {
        sequence.push(OPERATIONS[ordinal % OPERATIONS.len()]);
        ordinal /= OPERATIONS.len();
    }
    sequence
}
