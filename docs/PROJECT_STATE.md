# Project state

Last updated: **2026-08-28**

## Current milestone

Stage 1 and the first source-quality checkpoint are complete. Stage 2 has
verified bounded application messaging for RFF-REQ-003 and completed its first
structured-event and injected-time slices. The event queue proves fixed fields,
a positive record bound, emission-order FIFO, and explicit reject-newest
saturation. The manual clock proves explicit nondecreasing advancement,
non-mutating overflow, repeatable instant traces, and an object-safe clock
capture seam for event timestamps.

No scheduled work or runtime/host-owned event path exists, so RFF-REQ-004 and
RFF-REQ-005 remain partial. RFF-REQ-008 also remains partial because returned
application errors do not emit structured events. Caller-driven scheduling is
the next Stage 2 responsibility.

## Verified baseline

- Lifecycle/work implementation commit
  `43ef56b92da18302eee0c0a0a1f071a29aae0ade` contains caller-driven work and
  complete RFF-REQ-002 integration evidence.
- Source-quality commit `c8f2f7af9232aa370a3ea9ded211eac1581dd375`
  adds stable rustfmt and Clippy configuration and denies
  `clippy::too_many_lines` at 60 for all targets.
- Messaging-core commit `6c6e0b0b31e7338414f28f330024c2bfbd3eec40`,
  ownership commit `65bd4fa458bb6f82fe73af291f90e90ee582e3d5`, and
  dispatch commit `150b924e6391c9adcc14f23bf21138011b747313` contain the
  routing, lifecycle availability, clearing, self-publication, and one-message
  dispatch evidence that verifies RFF-REQ-003.
- Event implementation commit `cc0e453431588a1f39e147249af701c21c1e1b1d`
  adds structured `Event` records, `EventTimestamp`, the bounded `EventQueue`,
  ADR-0013, and four focused public tests.
- Manual-time implementation commit
  `0ec684f6d50505aea67acef27e54d7175f0a11ee` adds `FrameworkInstant`, the
  injected `Clock` boundary, `ManualClock`, checked advancement, ADR-0014, and
  five focused public tests. RFF-REQ-004 and RFF-REQ-005 are partially verified.
- Repository content is available under `MIT OR Apache-2.0` with the confirmed
  notice `Copyright 2026 Daniel Smith`. Both canonical licence files, Cargo
  metadata, predicted package inventory, and generated package archive have
  been verified while `publish = false` remains in force.
- Formatting, all-target checking, warnings-denied Clippy, all 47 tests, and
  warnings-denied all-feature documentation generation pass with rustc/cargo
  1.98.0, rustfmt 1.9.0-stable, and Clippy 0.1.98.
- Fifteen handwritten Rust files have no physical line over 100 columns and no
  comment-only line over the 80-column review default. The three existing
  reasoned chronological test expectations remain fulfilled; production,
  messaging, event, and clock code need none.

The active toolchain changed from the 1.96.1 policy-establishment evidence. The
whole-tree re-audit passes; this records evidence rather than a minimum supported
Rust version or toolchain pin.

## Current architecture

- ADR-0001 selects a provisional caller-driven, serial host runtime.
- ADR-0003 defines four stable LC1 states and stop-gated restart with
  runtime-local identity.
- ADR-0006 through ADR-0009 select finite-capacity static application ownership
  and distinct synchronous start, work, stop, and in-place restart callbacks
  with concrete returned errors.
- ADR-0004 defines bounded application inbox behavior. ADR-0010 implements its
  routing core, ADR-0011 its lifecycle owner, and ADR-0012 its one-message
  application dispatch boundary.
- ADR-0013 defines the standalone bounded structured-event queue and explicit
  elapsed event timestamp field.
- ADR-0014 defines general elapsed framework instants, an object-safe injected
  clock read, and checked manual advancement.
- ADR-0005 defines configuration revision and rollback behavior; that service
  is not implemented.

`MessagingRuntime<A, Topic, MAX_PAYLOAD_BYTES>` consumes a fully composed
still-registered runtime, creates exactly one fresh bounded inbox per record,
and freezes the integrated topology. Only `Running` endpoints accept delivery.
Stop and returned callback errors clear only the selected queue before returning
an exact discarded-delivery count; successful restart reconnects an empty inbox.

For applications implementing `MessagingApplication`, `dispatch_one` validates
`Running`, snapshots current lifecycle states, and removes at most one oldest
FIFO message. `ApplicationMessageContext` exposes only the application identity
and bounded publication. Capacity-one self-publication can occupy the freed
slot. Success keeps `Running`; a returned message error drops the attempted
item, fails only the selected record, clears its remaining queue, and retains
peer deliveries already accepted.

`EventQueue<EventId>` separately stores copied structured records with framework
or application source, local severity, mission-defined identifier, and elapsed
`EventTimestamp`. It reserves a positive record limit, preserves emission-order
FIFO, rejects newest at saturation, and frees one slot on dequeue.

`Clock` separately returns `FrameworkInstant` without moving time. `ManualClock`
starts at zero or one explicit controlled instant, advances only by caller-
supplied `Duration`, and preserves its reading on checked overflow. An
`EventTimestamp` can capture a reading through either a concrete or trait-object
clock reference. The clock and event queue are not attached to a runtime.

The runtime creates no thread or executor and has no automatic or batch
dispatch, multi-application fairness rule, schedule, configuration access, or
event integration. The event queue has no filter, fan-out, persistence, or host
drain adapter. RFF-REQ-003 is verified; RFF-REQ-004, RFF-REQ-005, and
RFF-REQ-008 remain partial.

## Source-quality policy

Stable rustfmt owns normal formatting at 100 columns. Clippy enforces a
normally-60-line function threshold for every target. The three current
expectations preserve complete chronological state/error traces and have
item-level reasons; production and service functions need no exception.

Comment prose, exceptional physical lines, progressive source ordering, module
cohesion, abstraction level, and naming remain review responsibilities. A strict
physical-line checker, additional selected lints, validated complexity metric,
dependency-policy tool, CI workflow, and toolchain pin remain deferred to
separate measured increments.

## Work in progress

No implementation work is in progress. The injected manual-clock slice is
complete at its recorded boundary. Caller-driven scheduled work is the next
technical slice; runtime failure-event integration remains separate.

## Highest risks and uncertainties

- `FrameworkInstant` carries no clock identity, so unrelated clock domains can
  be compared accidentally. `EventTimestamp::from_elapsed` also preserves an
  explicit path for arbitrary standalone timestamps.
- Scheduling has no due-work, equal-deadline, overdue, lifecycle, or replay
  policy. Repeatable manual readings do not prove repeatable scheduled work.
- Reject-newest event saturation can omit a later high-severity record. The
  must-use outcome is explicit, but callers can still handle it inadequately.
- `EventSource::Application` cannot validate which runtime issued an identity
  until a runtime-owned integration constructs the event.
- Runtime-owned inbox configurations are positional. Count and identity order
  are proven, but two valid capacity/topic configurations can still be swapped
  by mission composition.
- Callback publication is immediate and non-transactional. Peer deliveries
  remain even when the publisher later returns an error.
- A returned callback error may follow partial application mutation. The
  runtime proves no rollback, cleanup, reinitialisation, panic containment,
  hang containment, or fault tolerance.
- No CI currently executes the local baseline.

## Important unresolved decisions

- No caller-driven scheduling contract, equal-deadline ordering, missed-work
  policy, or runtime clock owner is selected.
- No runtime/application event emission, framework failure-event identifier, or
  host event drain boundary is selected.
- No mission message payload limit, external topic identifier, or wire
  representation is selected; current messages are in-process values only.
- RFF-REQ-007 still needs a host-adapter grammar and validation boundary.
- The v0.1 runtime explicitly does not catch application panics; no future
  containment mechanism or minimum supported Rust version is selected.
- A shared application service context, automatic dispatch order, fairness, and
  event filtering remain unselected.

## Most likely next tasks

1. Add the smallest caller-driven scheduled-work slice with stable equal-
   deadline order and a replayed trace under `ManualClock`.
2. Integrate one clock-captured returned-error event while preserving the
   original error, explicit event-queue saturation, and peer progress.
3. Implement the accepted configuration activation, rejection, revision, and
   rollback behavior after Stage 2 has a coherent stopping point.

## Latest run

2026-08-28: Added the injected manual framework clock in commit
`0ec684f6d50505aea67acef27e54d7175f0a11ee`. Five public tests prove zero and
repeated reads, exact cumulative advancement, zero-duration no-op behavior,
typed non-mutating overflow, object-safe injection, identical manual traces,
and exact event timestamp capture through the existing FIFO queue. The complete
baseline passes 47 tests with warnings denied. No dependency, wall-clock read,
scheduler, runtime/application integration, thread, executor, protocol behavior,
release, or push was added.
