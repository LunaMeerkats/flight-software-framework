//! Finite public-API evidence for configuration history composition.
//!
//! The frozen alphabet and bounds are in CONFIGURATION_SEQUENCE_REVIEW.md.
//! An append-only acceptance history and movement log predict observations;
//! they do not duplicate the table's active/rollback/high-water storage.

use std::error::Error;
use std::fmt;

use rust_flight_framework::{ConfigurationError, ConfigurationTable};

const MAX_BYTES: usize = 2;
const MAX_SEQUENCE_LENGTH: usize = 6;
const EXPECTED_TRACES: usize = 19_531;
const EXPECTED_OPERATIONS: usize = 112_305;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Content {
    Short,
    ExactBound,
}

#[derive(Clone, Copy, Debug)]
enum Operation {
    Replace(Content),
    Reject,
    Oversize,
    Rollback,
}

const OPERATIONS: [Operation; 5] = [
    Operation::Replace(Content::Short),
    Operation::Replace(Content::ExactBound),
    Operation::Reject,
    Operation::Oversize,
    Operation::Rollback,
];

#[derive(Debug, Eq, PartialEq)]
struct ValidationError;

struct Acceptance {
    content: Content,
    predecessor: Option<usize>,
}

enum Movement {
    Accept(usize),
    Restore(usize),
}

struct Reference {
    accepted: Vec<Acceptance>,
    movements: Vec<Movement>,
}

struct SequenceComparison {
    table: ConfigurationTable<ValidationError, MAX_BYTES>,
    reference: Reference,
}

#[test]
fn all_bounded_configuration_sequences_match_reference_history() {
    let mut trace_count = 0;
    let mut operation_count = 0;
    let mut sequences_at_length = 1;
    for length in 0..=MAX_SEQUENCE_LENGTH {
        for ordinal in 0..sequences_at_length {
            let sequence = decode_sequence(ordinal, length);
            let mut comparison = SequenceComparison::new();
            comparison.assert_active(&sequence, 0);
            for (position, operation) in sequence.iter().copied().enumerate() {
                comparison.perform(operation, &sequence, position + 1);
            }
            comparison.probe_retained_history(&sequence);
            trace_count += 1;
            operation_count += sequence.len();
        }
        sequences_at_length *= OPERATIONS.len();
    }
    assert_eq!(trace_count, EXPECTED_TRACES);
    assert_eq!(operation_count, EXPECTED_OPERATIONS);
}

impl Content {
    fn bytes(self) -> &'static [u8] {
        match self {
            Self::Short => &[1],
            Self::ExactBound => &[2, 3],
        }
    }
}

impl Reference {
    fn new() -> Self {
        Self {
            accepted: vec![Acceptance {
                content: Content::Short,
                predecessor: None,
            }],
            movements: vec![Movement::Accept(0)],
        }
    }

    fn current_index(&self) -> usize {
        match self.movements.last().expect("initial acceptance exists") {
            Movement::Accept(index) | Movement::Restore(index) => *index,
        }
    }

    fn perform(
        &mut self,
        operation: Operation,
    ) -> Result<u64, ConfigurationError<ValidationError>> {
        match operation {
            Operation::Replace(content) => {
                let index = self.accepted.len();
                self.accepted.push(Acceptance {
                    content,
                    predecessor: Some(self.current_index()),
                });
                self.movements.push(Movement::Accept(index));
                Ok(revision(index))
            }
            Operation::Reject => Err(ConfigurationError::Rejected(ValidationError)),
            Operation::Oversize => Err(ConfigurationError::TooLarge {
                length: 3,
                maximum: MAX_BYTES,
            }),
            Operation::Rollback => self.restore_predecessor(),
        }
    }

    fn restore_predecessor(&mut self) -> Result<u64, ConfigurationError<ValidationError>> {
        let predecessor = match self.movements.last().expect("initial acceptance exists") {
            Movement::Accept(index) => self.accepted[*index].predecessor,
            Movement::Restore(_) => None,
        };
        let index = predecessor.ok_or(ConfigurationError::NoRollbackAvailable)?;
        self.movements.push(Movement::Restore(index));
        Ok(revision(index))
    }
}

impl SequenceComparison {
    fn new() -> Self {
        let table = ConfigurationTable::new(Content::Short.bytes(), validate)
            .expect("the initial frozen value is accepted");
        Self {
            table,
            reference: Reference::new(),
        }
    }

    fn perform(&mut self, operation: Operation, sequence: &[Operation], position: usize) {
        let expected = self.reference.perform(operation);
        let actual = match operation {
            Operation::Replace(content) => self.table.replace(content.bytes()),
            Operation::Reject => self.table.replace(&[0]),
            Operation::Oversize => self.table.replace(&[0, 0, 0]),
            Operation::Rollback => self.table.rollback(),
        };
        assert_eq!(actual, expected, "sequence {sequence:?}, step {position}");
        if let Err(error) = &actual {
            assert_error_source(error, sequence, position);
        }
        self.assert_active(sequence, position);
    }

    fn assert_active(&self, sequence: &[Operation], position: usize) {
        let index = self.reference.current_index();
        let expected = &self.reference.accepted[index];
        assert_eq!(
            self.table.active().revision(),
            revision(index),
            "sequence {sequence:?}, step {position}"
        );
        assert_eq!(
            self.table.active().bytes(),
            expected.content.bytes(),
            "sequence {sequence:?}, step {position}"
        );
    }

    fn probe_retained_history(&mut self, sequence: &[Operation]) {
        // Terminal prefixes have no generated extension. These bounded probes
        // expose their rollback slot and next unused acceptance revision.
        for (offset, operation) in [
            Operation::Rollback,
            Operation::Replace(Content::Short),
            Operation::Rollback,
        ]
        .into_iter()
        .enumerate()
        {
            self.perform(operation, sequence, sequence.len() + offset + 1);
        }
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("the frozen mission rejects empty content or a first zero byte")
    }
}

impl Error for ValidationError {}

fn validate(bytes: &[u8]) -> Result<(), ValidationError> {
    match bytes.first() {
        Some(first) if *first != 0 => Ok(()),
        _ => Err(ValidationError),
    }
}

fn assert_error_source(
    error: &ConfigurationError<ValidationError>,
    sequence: &[Operation],
    position: usize,
) {
    match error {
        ConfigurationError::Rejected(expected) => {
            assert_eq!(
                Error::source(error).and_then(|source| source.downcast_ref::<ValidationError>()),
                Some(expected),
                "sequence {sequence:?}, step {position}"
            );
            assert!(Error::source(expected).is_none());
        }
        _ => assert!(
            Error::source(error).is_none(),
            "sequence {sequence:?}, step {position}"
        ),
    }
}

fn revision(index: usize) -> u64 {
    u64::try_from(index + 1).expect("the bounded acceptance history fits in u64")
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
