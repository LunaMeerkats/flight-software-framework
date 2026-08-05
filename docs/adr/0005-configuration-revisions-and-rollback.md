# ADR-0005: Configuration revisions and consume-once rollback

- Status: Accepted; not implemented
- Date: 2026-08-05
- Scope: One in-memory validated v0.1 runtime configuration

## Context

RFF-REQ-006 needs validation before activation, rejection without mutation,
observable version behavior, and bounded rollback history. NASA cFE active and
inactive table images inform the problem boundary but do not prescribe this
project's revision or rollback rules. (`SRC-NASA-CFE`)

## Decision

- The runtime owns immutable configuration snapshots.
- A revision identifies one successfully accepted snapshot; it is not a schema
  version, timestamp, content hash, or cFE table version.
- The initial accepted configuration has revision 1. Each successful replacement
  receives the next unused `u64` revision from a monotonic high-water mark.
- Validation completes before mutation. A rejected candidate changes neither
  active content, active revision, rollback content, nor the high-water mark.
- Successful replacement moves the former active snapshot into the sole rollback
  slot, discarding any older retained snapshot, then activates the candidate.
- Equal content may receive a new revision; v0.1 defines no semantic equality or
  hashing requirement.
- Rollback restores the retained snapshot with its original revision and consumes
  the slot. The active revision may therefore decrease, but the internal
  high-water mark never decreases or reuses a revision.
- After `1 -> 2 -> rollback to 1`, the next successful replacement is revision 3.
- A second rollback without an intervening replacement returns a typed
  `NoRollbackAvailable` outcome and changes nothing.
- Revision exhaustion returns an explicit error before mutation.
- Replacement and rollback occur only at caller-driven safe points between
  application work calls. Application restart retains the current configuration,
  and an application error does not automatically trigger rollback.
- Revisions and rollback state are process-local. Persistence, schema migration,
  history enumeration, redo, and automatic rollback are outside v0.1.

## Alternatives considered

- Caller-supplied versions: deferred because they require collision, trust, and
  ordering policy.
- Content hashes: deferred because they require canonical encoding and a hash
  selection.
- Assigning a new revision to restored content: rejected because the approved
  contract uses revision as snapshot identity; chronological activation remains
  observable through ordered events and the high-water mark.
- Swap/toggle rollback: rejected because it creates indefinite undo/redo behavior
  despite one storage slot.
- Multi-entry or unbounded history: rejected as unnecessary and inconsistent
  with bounded-resource direction.

## Required verification and revisit conditions

Tests must cover initial revision, successive replacement, rejection without any
mutation or revision consumption, replacement of older history, consume-once
rollback, a failed second rollback, non-reuse after revision decrease, equal-
content activation, safe-point visibility, and checked exhaustion.

Revisit when configuration requires persistent identity, schema migration,
concurrent writers, compare-and-swap, more than one recovery point, or durable
audit history.
