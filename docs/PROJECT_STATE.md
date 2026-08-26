# Project state

Last updated: **2026-08-27**

## Current milestone

Stage 1 and the first source-quality checkpoint are complete. Stage 2 has
completed bounded application messaging for RFF-REQ-003 and its first standalone
structured-event storage slice. The event queue proves fixed fields, a positive
record bound, emission-order FIFO, and explicit reject-newest saturation, but it
does not produce framework-owned timestamps or receive runtime/application
events. RFF-REQ-005 and RFF-REQ-008 therefore remain partial. The next Stage 2
responsibility is injected manual time.

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
  ADR-0013, and four focused public tests. RFF-REQ-005 is partially verified.
- Repository content is available under `MIT OR Apache-2.0` with the confirmed
  notice `Copyright 2026 Daniel Smith`. Both canonical licence files, Cargo
  metadata, predicted package inventory, and generated package archive have
  been verified while `publish = false` remains in force.
- Formatting, all-target checking, warnings-denied Clippy, all 42 tests, and
  warnings-denied all-feature documentation generation pass with rustc/cargo
  1.98.0, rustfmt 1.9.0-stable, and Clippy 0.1.98.
- Thirteen handwritten Rust files have no physical line over 100 columns and no
  comment-only line over the 80-column review default. The three existing
  reasoned chronological test expectations remain fulfilled; production,
  messaging, and event code need none.

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
- ADR-0013 defines the standalone bounded structured-event queue and its
  explicit elapsed timestamp consumer.
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
or application source, local severity, mission-defined identifier, and explicit
elapsed `EventTimestamp`. Construction reserves a positive record limit.
Emission accepts through the exact limit; saturation preserves older records and
returns a must-use `QueueFull` outcome. FIFO follows emission order even when
timestamps are non-monotonic, and one dequeue immediately frees one slot.

The runtime creates no thread or executor and has no automatic or batch
dispatch, multi-application fairness rule, schedule, clock, configuration
access, or event integration. The standalone queue has no filter, fan-out,
persistence, host drain adapter, or timestamp source. RFF-REQ-003 is verified;
RFF-REQ-005 and RFF-REQ-008 remain partial.

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

No implementation work is in progress. The standalone event queue is complete
at its recorded boundary. Injected manual time is the next technical slice; it
must remain separate from scheduling and runtime failure emission.

## Highest risks and uncertainties

- Runtime-owned inbox configurations are positional. Count and identity order
  are proven, but two valid capacity/topic configurations can still be swapped
  by mission composition.
- Dispatch refreshes a preallocated lifecycle-state snapshot before every
  callback. The serial publish-only context keeps it stable; concurrency would
  require a different design.
- Callback publication is immediate and non-transactional. Peer deliveries
  remain even when the publisher later returns an error.
- Inline message payload slots occupy and copy the configured maximum for short
  payloads. No mission payload size has been measured or selected.
- Reject-newest event saturation can omit a later high-severity record. The
  must-use outcome is explicit, but callers can still handle it inadequately.
- Explicit event timestamps can be non-monotonic until an injected clock owns
  production. Rejected, dequeued, and caller-retained events are outside the
  queue's storage bound.
- `EventSource::Application` cannot validate which runtime issued an identity
  until a runtime-owned integration constructs the event.
- A returned callback error may follow partial application mutation. The
  runtime proves no rollback, cleanup, reinitialisation, panic containment,
  hang containment, or fault tolerance.
- No CI currently executes the local baseline.

## Important unresolved decisions

- No framework-owned clock, runtime/application event emission, or host event
  drain boundary is selected.
- No mission message payload limit, external topic identifier, or wire
  representation is selected; current messages are in-process values only.
- RFF-REQ-007 still needs a host-adapter grammar and validation boundary.
- The v0.1 runtime explicitly does not catch application panics; no future
  containment mechanism or minimum supported Rust version is selected.
- A shared application service context, automatic dispatch order, fairness,
  event filtering, and scheduling remain unselected.

## Most likely next tasks

1. Add the smallest injected manual clock that owns elapsed
   `EventTimestamp` production without adding scheduling.
2. Integrate one framework-timestamped returned-error event while preserving
   the original error, explicit event-queue saturation, and peer progress.
3. Add caller-driven scheduled work only after clock and event ordering are
   independently verified.

## Latest run

2026-08-27: Added the standalone bounded structured-event queue in commit
`cc0e453431588a1f39e147249af701c21c1e1b1d`. Four public tests prove exact
structured fields, positive capacity, non-timestamp-sorted FIFO, reject-newest
saturation, retained older records, and immediate one-slot reuse. The complete
baseline passes 42 tests with warnings denied. No dependency, clock, wall-time
read, runtime/application integration, event text, filter, fan-out, persistence,
protocol behavior, or release was added. Nothing was pushed.
