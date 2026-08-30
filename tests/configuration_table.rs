//! Public-API evidence for bounded, validated configuration replacement.

use std::error::Error;
use std::fmt;

use rust_flight_framework::{ConfigurationError, ConfigurationTable};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MissionValidationError {
    InvalidLength(usize),
    UnknownMode(u8),
    InvalidPeriod(u8),
}

impl fmt::Display for MissionValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLength(length) => write!(formatter, "expected two bytes, got {length}"),
            Self::UnknownMode(mode) => write!(formatter, "unknown mission mode {mode}"),
            Self::InvalidPeriod(period) => write!(formatter, "invalid sample period {period}"),
        }
    }
}

impl Error for MissionValidationError {}

// The fixture defines a mode byte followed by a period in simulated seconds.
// Parsing belongs to the mission; the table only retains validated bytes.
fn validate_mission(bytes: &[u8]) -> Result<(), MissionValidationError> {
    let [mode, period] = bytes else {
        return Err(MissionValidationError::InvalidLength(bytes.len()));
    };
    if *mode > 1 {
        return Err(MissionValidationError::UnknownMode(*mode));
    }
    if !(1..=10).contains(period) {
        return Err(MissionValidationError::InvalidPeriod(*period));
    }
    Ok(())
}

#[test]
fn initial_configuration_is_validated_and_has_no_rollback_history() {
    let mut table = ConfigurationTable::<_, 2>::new(&[0, 1], validate_mission)
        .expect("valid initial mission configuration fits the byte bound");

    assert_eq!(table.active().bytes(), &[0, 1]);
    assert_eq!(table.active().revision(), 1);
    assert_eq!(
        table.rollback(),
        Err(ConfigurationError::NoRollbackAvailable)
    );
    assert_eq!(table.active().bytes(), &[0, 1]);
    assert_eq!(table.active().revision(), 1);

    assert_eq!(
        ConfigurationTable::<_, 2>::new(&[0, 0], validate_mission)
            .expect_err("the initial configuration must pass mission validation"),
        ConfigurationError::Rejected(MissionValidationError::InvalidPeriod(0))
    );
}

#[test]
fn oversized_initial_configuration_is_rejected_before_validation() {
    let result = ConfigurationTable::<MissionValidationError, 2>::new(&[0, 1, 2], |_| {
        panic!("the validator must not see an oversized initial configuration")
    });

    assert_eq!(
        result.expect_err("three bytes exceed a two-byte bound"),
        ConfigurationError::TooLarge {
            length: 3,
            maximum: 2,
        }
    );
}

#[test]
fn oversized_replacement_is_rejected_before_validation() {
    let mut table = ConfigurationTable::<MissionValidationError, 2>::new(&[0, 1], |bytes| {
        assert_eq!(bytes, &[0, 1], "only the initial bytes reach validation");
        Ok(())
    })
    .expect("the initial bytes fit");
    let before = table.active().clone();

    assert_eq!(
        table.replace(&[1, 2, 3]),
        Err(ConfigurationError::TooLarge {
            length: 3,
            maximum: 2,
        })
    );
    assert_eq!(table.active(), &before);
    assert_eq!(
        table.rollback(),
        Err(ConfigurationError::NoRollbackAvailable)
    );
}

#[test]
fn exact_bound_is_accepted_and_shorter_snapshots_expose_only_used_bytes() {
    let mut table = ConfigurationTable::<MissionValidationError, 4>::new(&[0, 1, 2, 3], |_| Ok(()))
        .expect("exactly four bytes fit");

    assert_eq!(table.active().bytes(), &[0, 1, 2, 3]);
    assert_eq!(table.replace(&[4]), Ok(2));
    assert_eq!(table.active().bytes(), &[4]);
    assert_eq!(table.rollback(), Ok(1));
    assert_eq!(table.active().bytes(), &[0, 1, 2, 3]);
}

#[test]
fn zero_capacity_accepts_valid_empty_content_and_rejects_nonempty_content() {
    let mut table = ConfigurationTable::<MissionValidationError, 0>::new(&[], |bytes| {
        assert!(bytes.is_empty(), "nonempty bytes must fail the bound first");
        Ok(())
    })
    .expect("an empty configuration can fit a zero-byte bound");

    assert_eq!(table.active().bytes(), &[]);
    assert_eq!(table.active().revision(), 1);
    assert_eq!(table.replace(&[]), Ok(2));
    assert_eq!(
        table.replace(&[0]),
        Err(ConfigurationError::TooLarge {
            length: 1,
            maximum: 0,
        })
    );
    assert_eq!(table.active().bytes(), &[]);
    assert_eq!(table.active().revision(), 2);
    assert_eq!(table.rollback(), Ok(1));
}

#[test]
fn initial_and_replacement_bytes_are_owned_copies() {
    let mut initial = [0, 1];
    let mut table = ConfigurationTable::<_, 2>::new(&initial, validate_mission)
        .expect("initial configuration is valid");
    initial.fill(255);

    assert_eq!(initial, [255, 255]);
    assert_eq!(table.active().bytes(), &[0, 1]);

    let mut replacement = [1, 2];
    assert_eq!(table.replace(&replacement), Ok(2));
    replacement.fill(255);

    assert_eq!(replacement, [255, 255]);
    assert_eq!(table.active().bytes(), &[1, 2]);
    assert_eq!(table.rollback(), Ok(1));
    assert_eq!(table.active().bytes(), &[0, 1]);
}

#[test]
fn cloned_snapshot_retains_its_bytes_and_revision_after_replacement() {
    let mut table = ConfigurationTable::<_, 2>::new(&[0, 1], validate_mission)
        .expect("initial configuration is valid");
    let initial = table.active().clone();

    assert_eq!(table.replace(&[1, 2]), Ok(2));
    assert_eq!(initial.bytes(), &[0, 1]);
    assert_eq!(initial.revision(), 1);
    assert_ne!(table.active(), &initial);
    assert_eq!(table.rollback(), Ok(1));
    assert_eq!(table.active(), &initial);
}

#[test]
fn rejected_replacements_preserve_active_history_and_revision_allocation() {
    let mut table = ConfigurationTable::<_, 2>::new(&[0, 1], validate_mission)
        .expect("initial configuration is valid");
    assert_eq!(table.replace(&[1, 2]), Ok(2));
    let before = table.active().clone();
    let invalid: &[(&[u8], ConfigurationError<MissionValidationError>)] = &[
        (
            &[2, 2],
            ConfigurationError::Rejected(MissionValidationError::UnknownMode(2)),
        ),
        (
            &[1, 0],
            ConfigurationError::Rejected(MissionValidationError::InvalidPeriod(0)),
        ),
        (
            &[1],
            ConfigurationError::Rejected(MissionValidationError::InvalidLength(1)),
        ),
        (
            &[0, 1, 2],
            ConfigurationError::TooLarge {
                length: 3,
                maximum: 2,
            },
        ),
    ];

    for (bytes, expected_error) in invalid {
        let actual = table.replace(bytes).expect_err("candidate is invalid");
        assert_eq!(&actual, expected_error);
        assert_eq!(table.active(), &before);
    }

    assert_eq!(table.rollback(), Ok(1));
    assert_eq!(table.active().bytes(), &[0, 1]);
    assert_eq!(table.replace(&[1, 3]), Ok(3));
    assert_eq!(table.active().bytes(), &[1, 3]);
}

#[test]
fn equal_content_replacements_still_allocate_distinct_revisions() {
    let mut table = ConfigurationTable::<_, 2>::new(&[0, 1], validate_mission)
        .expect("initial configuration is valid");

    for revision in 2..=4 {
        assert_eq!(table.replace(&[0, 1]), Ok(revision));
        assert_eq!(table.active().bytes(), &[0, 1]);
        assert_eq!(table.active().revision(), revision);
    }

    assert_eq!(table.rollback(), Ok(3));
    assert_eq!(table.active().bytes(), &[0, 1]);
    assert_eq!(table.active().revision(), 3);
    assert_eq!(table.replace(&[0, 1]), Ok(5));
}

#[test]
fn rollback_is_consumed_once_and_does_not_reuse_issued_revisions() {
    let mut table = ConfigurationTable::<_, 2>::new(&[0, 1], validate_mission)
        .expect("initial configuration is valid");
    assert_eq!(table.replace(&[1, 2]), Ok(2));

    assert_eq!(table.rollback(), Ok(1));
    assert_eq!(table.active().bytes(), &[0, 1]);
    assert_eq!(table.active().revision(), 1);
    assert_eq!(
        table.rollback(),
        Err(ConfigurationError::NoRollbackAvailable)
    );
    assert_eq!(table.active().bytes(), &[0, 1]);
    assert_eq!(table.active().revision(), 1);

    assert_eq!(table.replace(&[1, 3]), Ok(3));
    assert_eq!(table.active().bytes(), &[1, 3]);
    assert_eq!(table.active().revision(), 3);
    assert_eq!(table.rollback(), Ok(1));
    assert_eq!(table.active().bytes(), &[0, 1]);
}

#[test]
fn each_replacement_retains_only_the_immediately_previous_active_snapshot() {
    let mut table = ConfigurationTable::<_, 2>::new(&[0, 1], validate_mission)
        .expect("initial configuration is valid");
    assert_eq!(table.replace(&[1, 2]), Ok(2));
    assert_eq!(table.replace(&[0, 3]), Ok(3));

    assert_eq!(table.rollback(), Ok(2));
    assert_eq!(table.active().bytes(), &[1, 2]);
    assert_eq!(table.active().revision(), 2);
    assert_eq!(
        table.rollback(),
        Err(ConfigurationError::NoRollbackAvailable)
    );
    assert_eq!(table.active().bytes(), &[1, 2]);

    assert_eq!(table.replace(&[1, 4]), Ok(4));
    assert_eq!(table.rollback(), Ok(2));
    assert_eq!(table.active().bytes(), &[1, 2]);
}

#[test]
fn rejected_error_preserves_concrete_mission_error_as_its_source() {
    let mut table = ConfigurationTable::<_, 2>::new(&[0, 1], validate_mission)
        .expect("initial configuration is valid");
    let error = table
        .replace(&[7, 1])
        .expect_err("an unknown mission mode is rejected");

    assert_eq!(
        error,
        ConfigurationError::Rejected(MissionValidationError::UnknownMode(7))
    );
    assert_eq!(
        error
            .source()
            .and_then(|source| source.downcast_ref::<MissionValidationError>()),
        Some(&MissionValidationError::UnknownMode(7))
    );
    assert!(!error.to_string().is_empty());

    for structural in [
        ConfigurationError::<MissionValidationError>::TooLarge {
            length: 3,
            maximum: 2,
        },
        ConfigurationError::RevisionExhausted,
        ConfigurationError::NoRollbackAvailable,
    ] {
        assert!(structural.source().is_none());
        assert!(!structural.to_string().is_empty());
    }
}
