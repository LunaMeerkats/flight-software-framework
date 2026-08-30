//! Bounded immutable configuration snapshots and consume-once rollback.
//!
//! One retained function validates opaque bytes before initial acceptance or
//! replacement. The table retains an active snapshot, at most one rollback
//! snapshot, and a monotonic revision high-water mark. Runtime ownership and
//! application access remain separate integration work.

use std::error::Error;
use std::fmt;

/// One accepted immutable configuration value and its table-local revision.
///
/// A revision identifies an acceptance, not a schema, hash, or timestamp. It
/// carries no table identity; equal revisions from different tables can alias.
/// Cloned snapshots are caller-owned and outside the table's retention bound.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfigurationSnapshot<const MAX_BYTES: usize> {
    bytes: [u8; MAX_BYTES],
    length: usize,
    revision: u64,
}

/// A rejected configuration operation; retained table state is unchanged.
#[derive(Debug, Eq, PartialEq)]
pub enum ConfigurationError<E> {
    /// The candidate exceeded the inline byte bound; validation was not called.
    TooLarge {
        /// Number of bytes supplied by the caller.
        length: usize,
        /// Maximum bytes in each snapshot.
        maximum: usize,
    },
    /// The retained mission validator rejected the candidate.
    Rejected(E),
    /// Every representable nonzero revision has already been assigned.
    RevisionExhausted,
    /// No accepted previous snapshot remains to restore.
    NoRollbackAvailable,
}

/// A standalone bounded configuration value with one rollback slot.
///
/// The same function validates initial and replacement bytes. It must implement
/// the mission's validation policy without relying on this table for callback
/// termination, purity, or panic containment. No wire format is prescribed.
///
/// At most two snapshots are retained, each with `MAX_BYTES` inline storage.
/// Replacement can also hold one bounded candidate. Input buffers, validator
/// effects/errors, and caller-owned clones are outside that retention bound.
/// No heap allocation is performed by the table, but large const bounds can
/// exhaust host stack space. A zero bound permits only empty accepted content.
/// These logical storage bounds do not measure compiler-generated temporaries.
///
/// Mutation requires exclusive access; readers receive immutable views. The
/// table does not own applications or establish runtime work safe points.
#[derive(Debug)]
pub struct ConfigurationTable<E, const MAX_BYTES: usize> {
    active: ConfigurationSnapshot<MAX_BYTES>,
    rollback: Option<ConfigurationSnapshot<MAX_BYTES>>,
    highest_revision: u64,
    validate: fn(&[u8]) -> Result<(), E>,
}

impl<const MAX_BYTES: usize> ConfigurationSnapshot<MAX_BYTES> {
    /// Borrows accepted content without exposing unused inline storage.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes[..self.length]
    }

    /// Returns the original acceptance revision, starting at one per table.
    #[must_use]
    pub const fn revision(&self) -> u64 {
        self.revision
    }
}

impl<E, const MAX_BYTES: usize> ConfigurationTable<E, MAX_BYTES> {
    /// Validates and copies the initial value, accepting it at revision one.
    ///
    /// No rollback snapshot exists initially. The function is retained for
    /// every later replacement, including equal content.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigurationError::TooLarge`] before calling the validator
    /// for oversized input, or [`ConfigurationError::Rejected`] with the exact
    /// validation error. No table is created on rejection.
    pub fn new(
        initial: &[u8],
        validate: fn(&[u8]) -> Result<(), E>,
    ) -> Result<Self, ConfigurationError<E>> {
        let bytes = Self::validated_bytes(initial, validate)?;
        let active = ConfigurationSnapshot {
            bytes,
            length: initial.len(),
            revision: 1,
        };
        Ok(Self {
            active,
            rollback: None,
            highest_revision: 1,
            validate,
        })
    }

    /// Borrows the active immutable snapshot.
    #[must_use]
    pub const fn active(&self) -> &ConfigurationSnapshot<MAX_BYTES> {
        &self.active
    }

    /// Accepts a validated replacement and returns its newly assigned revision.
    ///
    /// The former active snapshot replaces the sole rollback slot, discarding
    /// older history. Equal content still receives a fresh revision. Validation
    /// precedes the exhaustion check; neither failure consumes a revision.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigurationError::TooLarge`],
    /// [`ConfigurationError::Rejected`], or
    /// [`ConfigurationError::RevisionExhausted`], in that precedence order.
    /// All retained content, revisions, and rollback availability stay intact.
    pub fn replace(&mut self, candidate: &[u8]) -> Result<u64, ConfigurationError<E>> {
        // The candidate stays private until both validation and revision
        // assignment succeed, so no rejected operation can evict history.
        let bytes = Self::validated_bytes(candidate, self.validate)?;
        let revision = self
            .highest_revision
            .checked_add(1)
            .ok_or(ConfigurationError::RevisionExhausted)?;
        let accepted = ConfigurationSnapshot {
            bytes,
            length: candidate.len(),
            revision,
        };
        self.rollback = Some(std::mem::replace(&mut self.active, accepted));
        self.highest_revision = revision;
        Ok(revision)
    }

    /// Restores the previous snapshot and consumes its rollback slot.
    ///
    /// Returns that snapshot's original revision without revalidation. The
    /// displaced active value is discarded; no redo slot is created. The
    /// revision high-water mark never decreases, even after exhaustion.
    ///
    /// # Errors
    ///
    /// Returns only [`ConfigurationError::NoRollbackAvailable`] when no
    /// previous snapshot remains, leaving all state unchanged.
    pub fn rollback(&mut self) -> Result<u64, ConfigurationError<E>> {
        let previous = self
            .rollback
            .take()
            .ok_or(ConfigurationError::NoRollbackAvailable)?;
        self.active = previous;
        Ok(self.active.revision)
    }

    fn validated_bytes(
        candidate: &[u8],
        validate: fn(&[u8]) -> Result<(), E>,
    ) -> Result<[u8; MAX_BYTES], ConfigurationError<E>> {
        if candidate.len() > MAX_BYTES {
            return Err(ConfigurationError::TooLarge {
                length: candidate.len(),
                maximum: MAX_BYTES,
            });
        }
        let mut bytes = [0; MAX_BYTES];
        bytes[..candidate.len()].copy_from_slice(candidate);
        validate(&bytes[..candidate.len()]).map_err(ConfigurationError::Rejected)?;
        Ok(bytes)
    }
}

impl<E: fmt::Display> fmt::Display for ConfigurationError<E> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooLarge { length, maximum } => write!(
                formatter,
                "configuration length {length} exceeds maximum {maximum} bytes"
            ),
            Self::Rejected(error) => {
                write!(formatter, "configuration validation rejected: {error}")
            }
            Self::RevisionExhausted => formatter.write_str("configuration revisions exhausted"),
            Self::NoRollbackAvailable => formatter.write_str("no configuration rollback available"),
        }
    }
}

impl<E: Error + 'static> Error for ConfigurationError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Rejected(error) => Some(error),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ConfigurationError, ConfigurationTable};

    #[test]
    fn oversize_replacement_and_rollback_do_not_invoke_validator() {
        let mut table = ConfigurationTable::<(), 1>::new(&[1], |_| Ok(())).unwrap();
        table.replace(&[2]).unwrap();
        table.validate = |_| panic!("validation must not run on this path");

        assert_eq!(
            table.replace(&[3, 4]),
            Err(ConfigurationError::TooLarge {
                length: 2,
                maximum: 1,
            })
        );
        assert_eq!(table.active.bytes(), &[2]);
        assert_eq!(table.highest_revision, 2);
        assert_eq!(table.rollback(), Ok(1));
        assert_eq!(table.active.bytes(), &[1]);
        assert_eq!(
            table.rollback(),
            Err(ConfigurationError::NoRollbackAvailable)
        );
        assert_eq!(table.highest_revision, 2);
    }

    #[test]
    fn final_revision_and_exhaustion_preserve_snapshots_and_high_water() {
        let mut table = ConfigurationTable::<(), 1>::new(&[1], |_| Ok(())).unwrap();
        // Reach the boundary without exposing arbitrary revision assignment.
        table.highest_revision = u64::MAX - 1;
        table.active.revision = u64::MAX - 1;
        assert_eq!(table.replace(&[2]), Ok(u64::MAX));
        let active = table.active.clone();
        let rollback = table.rollback.clone();

        assert_eq!(
            table.replace(&[3]),
            Err(ConfigurationError::RevisionExhausted)
        );
        assert_eq!(table.active, active);
        assert_eq!(table.rollback, rollback);
        assert_eq!(table.highest_revision, u64::MAX);

        assert_eq!(table.rollback(), Ok(u64::MAX - 1));
        assert_eq!(table.active.bytes(), &[1]);
        assert_eq!(
            table.replace(&[4]),
            Err(ConfigurationError::RevisionExhausted)
        );
        assert_eq!(table.active.revision(), u64::MAX - 1);
        assert_eq!(table.active.bytes(), &[1]);
        assert!(table.rollback.is_none());
        assert_eq!(table.highest_revision, u64::MAX);
        assert_eq!(
            table.rollback(),
            Err(ConfigurationError::NoRollbackAvailable)
        );
    }

    #[test]
    fn validation_precedes_exhaustion_without_changing_retained_state() {
        let mut table = ConfigurationTable::<u8, 1>::new(&[1], |bytes| match bytes {
            [0] => Err(7),
            _ => Ok(()),
        })
        .unwrap();
        table.highest_revision = u64::MAX - 1;
        table.replace(&[2]).unwrap();
        let active = table.active.clone();
        let rollback = table.rollback.clone();

        assert_eq!(
            table.replace(&[0, 0]),
            Err(ConfigurationError::TooLarge {
                length: 2,
                maximum: 1,
            })
        );
        assert_eq!(table.replace(&[0]), Err(ConfigurationError::Rejected(7)));
        assert_eq!(table.active, active);
        assert_eq!(table.rollback, rollback);
        assert_eq!(table.highest_revision, u64::MAX);
        assert_eq!(table.rollback(), Ok(1));
    }
}
