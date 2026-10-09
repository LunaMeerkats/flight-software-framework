//! Finite public-API sequence evidence for standalone available-endpoint
//! publication and dequeue.
//!
//! The frozen alphabet and bounds are recorded in MESSAGE_SEQUENCE_REVIEW.md.
//! The oracle keeps accepted-delivery history and consumption positions rather
//! than a second queue implementation. It does not consult production routing.

use rust_flight_framework::{
    ApplicationId, ApplicationInboxConfig, DeliveryStatus, LifecycleRegistry, Message, MessageBus,
    PublishClassification,
};

const CAPACITIES: [usize; 3] = [1, 2, 2];
const MAX_SEQUENCE_LENGTH: usize = 6;
const EXPECTED_TRACES: usize = 55_987;
const EXPECTED_OPERATIONS: usize = 324_726;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Topic {
    Command,
    Telemetry,
    Unrouted,
}

#[derive(Clone, Copy, Debug)]
enum Operation {
    Publish(Topic),
    Dequeue(usize),
}

const OPERATIONS: [Operation; 6] = [
    Operation::Publish(Topic::Command),
    Operation::Publish(Topic::Telemetry),
    Operation::Publish(Topic::Unrouted),
    Operation::Dequeue(0),
    Operation::Dequeue(1),
    Operation::Dequeue(2),
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Record {
    topic: Topic,
    payload: u8,
}

#[derive(Clone, Copy, Debug)]
enum Publication {
    NoRoute,
    AllAccepted,
    SomeAccepted,
    NoneAccepted,
}

#[derive(Default)]
struct Reference {
    accepted: [Vec<Record>; 3],
    consumed: [usize; 3],
}

struct SequenceComparison {
    application_ids: [ApplicationId; 3],
    bus: MessageBus<Topic, 1>,
    reference: Reference,
}

#[test]
fn all_bounded_publication_dequeue_sequences_match_reference_history() {
    let mut trace_count = 0;
    let mut operation_count = 0;
    let mut sequences_at_length = 1;
    for length in 0..=MAX_SEQUENCE_LENGTH {
        for ordinal in 0..sequences_at_length {
            let sequence = decode_sequence(ordinal, length);
            let mut comparison = SequenceComparison::new();
            comparison.assert_occupancy(&sequence);
            for (position, operation) in sequence.iter().copied().enumerate() {
                match operation {
                    Operation::Publish(topic) => {
                        let payload =
                            u8::try_from(position).expect("six positions fit in one byte");
                        comparison.publish(Record { topic, payload }, &sequence);
                    }
                    Operation::Dequeue(inbox) => comparison.dequeue(inbox, &sequence),
                }
                comparison.assert_occupancy(&sequence);
            }
            comparison.drain_remaining(&sequence);
            trace_count += 1;
            operation_count += sequence.len();
        }
        sequences_at_length *= OPERATIONS.len();
    }
    assert_eq!(trace_count, EXPECTED_TRACES);
    assert_eq!(operation_count, EXPECTED_OPERATIONS);
}

impl Reference {
    fn outstanding(&self, inbox: usize) -> usize {
        self.accepted[inbox].len() - self.consumed[inbox]
    }

    fn publish(&mut self, record: Record) -> (Vec<(usize, bool)>, Publication) {
        let destinations: &[usize] = match record.topic {
            Topic::Command => &[0, 2],
            Topic::Telemetry => &[1, 2],
            Topic::Unrouted => &[],
        };
        let mut outcomes = Vec::new();
        let mut accepted_count = 0;
        for &inbox in destinations {
            let accepted = self.outstanding(inbox) < CAPACITIES[inbox];
            if accepted {
                self.accepted[inbox].push(record);
                accepted_count += 1;
            }
            outcomes.push((inbox, accepted));
        }
        let publication = match (destinations.len(), accepted_count) {
            (0, _) => Publication::NoRoute,
            (_, 0) => Publication::NoneAccepted,
            (total, accepted) if total == accepted => Publication::AllAccepted,
            _ => Publication::SomeAccepted,
        };
        (outcomes, publication)
    }

    fn consume(&mut self, inbox: usize) -> Option<Record> {
        let record = self.accepted[inbox].get(self.consumed[inbox]).copied();
        if record.is_some() {
            self.consumed[inbox] += 1;
        }
        record
    }
}

impl SequenceComparison {
    fn new() -> Self {
        let mut registry = LifecycleRegistry::new(3).expect("three records fit");
        let application_ids = std::array::from_fn(|_| registry.register().expect("record fits"));
        let configurations = [
            ApplicationInboxConfig::new(application_ids[0], 1, &[Topic::Command]),
            ApplicationInboxConfig::new(application_ids[1], 2, &[Topic::Telemetry]),
            ApplicationInboxConfig::new(application_ids[2], 2, &[Topic::Telemetry, Topic::Command]),
        ];
        let bus = MessageBus::new(&configurations).expect("frozen topology is valid");
        Self {
            application_ids,
            bus,
            reference: Reference::default(),
        }
    }

    fn publish(&mut self, record: Record, sequence: &[Operation]) {
        let (expected_outcomes, expected_publication) = self.reference.publish(record);
        let message = Message::try_new(record.topic, &[record.payload]).expect("one byte fits");
        let report = self
            .bus
            .publish(&message)
            .expect("report allocation succeeds");
        let actual_outcomes: Vec<_> = report
            .outcomes()
            .iter()
            .map(|outcome| (outcome.application_id(), outcome.status()))
            .collect();
        let expected_outcomes: Vec<_> = expected_outcomes
            .into_iter()
            .map(|(inbox, accepted)| {
                let status = if accepted {
                    DeliveryStatus::Delivered
                } else {
                    DeliveryStatus::InboxFull
                };
                (self.application_ids[inbox], status)
            })
            .collect();
        let expected_classification = match expected_publication {
            Publication::NoRoute => PublishClassification::NoSubscribers,
            Publication::AllAccepted => PublishClassification::Complete,
            Publication::SomeAccepted => PublishClassification::Partial,
            Publication::NoneAccepted => PublishClassification::WhollyUndelivered,
        };
        assert_eq!(
            actual_outcomes, expected_outcomes,
            "publication {record:?}, trace {sequence:?}"
        );
        assert_eq!(
            report.classification(),
            expected_classification,
            "publication {record:?}, trace {sequence:?}"
        );
    }

    fn dequeue(&mut self, inbox: usize, sequence: &[Operation]) {
        let expected = self.reference.consume(inbox);
        let actual = self
            .bus
            .dequeue(self.application_ids[inbox])
            .expect("configured identity is known")
            .map(|message| (*message.topic(), message.payload().to_vec()));
        let expected = expected.map(|record| (record.topic, vec![record.payload]));
        assert_eq!(actual, expected, "inbox {inbox}, trace {sequence:?}");
    }

    fn assert_occupancy(&self, sequence: &[Operation]) {
        for (inbox, capacity) in CAPACITIES.into_iter().enumerate() {
            let application_id = self.application_ids[inbox];
            let outstanding = self.reference.outstanding(inbox);
            assert!(outstanding <= capacity, "inbox {inbox}, trace {sequence:?}");
            assert_eq!(
                self.bus.inbox_capacity(application_id),
                Ok(capacity),
                "inbox {inbox}, trace {sequence:?}"
            );
            assert_eq!(
                self.bus.pending(application_id),
                Ok(outstanding),
                "inbox {inbox}, trace {sequence:?}"
            );
        }
    }

    fn drain_remaining(&mut self, sequence: &[Operation]) {
        for inbox in 0..CAPACITIES.len() {
            while self.reference.outstanding(inbox) > 0 {
                self.dequeue(inbox, sequence);
                self.assert_occupancy(sequence);
            }
            self.dequeue(inbox, sequence);
            assert_eq!(
                self.bus.pending(self.application_ids[inbox]),
                Ok(0),
                "drained inbox {inbox}, trace {sequence:?}"
            );
        }
        self.assert_occupancy(sequence);
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
