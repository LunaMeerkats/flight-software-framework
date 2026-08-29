# Project state

Last updated: **2026-08-30**

## Current milestone

Stages 1 and 2 and the first source-quality checkpoint are complete. Bounded
application messaging verifies RFF-REQ-003. Injected manual time and a fixed
one-shot work agenda verify RFF-REQ-004 through stable equal-time order, explicit
overdue/error behavior, exact clock reads, and a replayed trace.

The standalone event queue proves structured fields, a positive record bound,
emission-order FIFO, and explicit reject-newest saturation. An opt-in direct
runtime work operation now constructs one clock-captured application error event
after a cooperative returned work error, preserves the complete original error
and exact event attempt under saturation, and leaves a healthy peer operable.
That combined evidence verifies RFF-REQ-005 and RFF-REQ-008 only at this direct
cooperative returned-work boundary. Stage 3 configuration behavior is next.

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
  five focused public tests.
- Scheduling implementation commit
  `63654d57936617e63731a906a049777420f91913` adds `ScheduledWork`, a fixed
  `WorkSchedule`, one-item due-work execution, ADR-0015, and seven public tests.
  Combined manual-time and replay evidence verifies RFF-REQ-004.
- Runtime-event implementation commit
  `a04c5bd3f929934b7578b14f181138ae0be56e9b` adds
  `Runtime::work_with_failure_event`, preserved error/event-attempt context,
  ADR-0016, and four public tests. Together with the bounded queue evidence it
  verifies RFF-REQ-005 and RFF-REQ-008 at the direct returned-work boundary.
- Repository content is available under `MIT OR Apache-2.0` with the confirmed
  notice `Copyright 2026 Daniel Smith`. Both canonical licence files, Cargo
  metadata, predicted package inventory, and generated package archive have
  been verified while `publish = false` remains in force.
- Formatting, all-target checking, warnings-denied Clippy, all 58 tests, and
  warnings-denied all-feature documentation generation pass with rustc/cargo
  1.98.0, rustfmt 1.9.0-stable, and Clippy 0.1.98.
- Nineteen handwritten Rust files have no physical line over 100 columns and no
  comment-only line over the 80-column review default. The three existing
  reasoned chronological test expectations remain fulfilled; production,
  messaging, event, clock, scheduling, and runtime-event code need none.

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
- ADR-0015 defines a finite nondecreasing one-shot work agenda, stable
  equal-time order, one attempted item per caller request, and final consumption
  after success, lifecycle rejection, or returned work failure.
- ADR-0016 defines an opt-in direct returned-work failure-event attempt that
  preserves the complete work error and exact queue outcome.
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
clock reference. Neither runtime permanently owns the clock or event queue.

`Runtime::work_with_failure_event` borrows both services and delegates first to
ordinary direct work. Success and lifecycle rejection read no clock and emit no
event. After a returned application work error commits only that record to
`Failed`, it captures one reading, constructs one application-sourced error
event with a mission-supplied identifier, and attempts the bounded queue once.
The returned integration error retains the complete `RuntimeWorkError`, exact
event, and `Recorded` or `QueueFull` outcome. Saturation retains older events and
the rejected event remains available for explicit caller retry.

`WorkSchedule` owns an immutable finite sequence of one-shot `ScheduledWork`
items. Nondecreasing instants preserve caller order for equal times.
`Runtime::run_next_scheduled_work` reads an injected clock once while an item
remains, waits without mutation, or consumes and attempts at most one due or
overdue item through `Runtime::work`. Success reports `Running`; lifecycle and
application errors retain exact schedule and runtime context. Consumed failures
do not block later due peers.

The runtime creates no thread or executor and has no automatic or batch
dispatch, periodic or dynamic work generation, multi-application fairness rule,
or configuration access. Scheduled work currently applies only to the
lifecycle-only runtime, not `MessagingRuntime`. Event integration is limited to
the opt-in direct returned-work path; the queue has no filter, fan-out,
persistence, or host drain adapter. RFF-REQ-003, RFF-REQ-004, RFF-REQ-005, and
RFF-REQ-008 are verified only at their recorded boundaries.

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

No implementation work is in progress. Stage 2 is complete at its recorded
messaging, event, time, scheduling, and direct cooperative failure boundaries.
The accepted configuration lifecycle is the next Stage 3 technical slice.

## Highest risks and uncertainties

- `FrameworkInstant` carries no clock identity, so a schedule can compare an
  instant from an unrelated clock domain. `ApplicationId` likewise cannot
  detect a same-position identity from another runtime.
  `EventTimestamp::from_elapsed` also preserves an explicit path for arbitrary
  standalone timestamps.
- The schedule is a fixed one-shot agenda. It has no periodic phase, drift,
  missed-release catch-up or coalescing, reconfiguration, priority, or fairness
  policy, and retains consumed items in its fixed allocation until drop.
- Scheduled work is exposed only on bare `Runtime`. A future messaging-aware
  operation must delegate through `MessagingRuntime::work` to preserve its
  failed-endpoint inbox clearing.
- Reject-newest event saturation can omit a later high-severity record. The
  must-use outcome and rejected event are explicit, but callers can still handle
  them inadequately; there is no delivery guarantee.
- Direct failure-event reporting is opt-in and applies only to returned work
  errors. Its timestamp is captured after `Failed` is committed and represents
  framework observation rather than an application-internal fault instant. The
  returned caller-owned event copy is outside the queue's retained-record bound.
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

- No periodic scheduling contract, missed-release policy, runtime clock owner,
  or messaging-aware scheduled-work composition is selected.
- No application-authored event API, other callback failure-event policy,
  persistent runtime event/clock owner, or host event drain boundary is selected.
- No mission message payload limit, external topic identifier, or wire
  representation is selected; current messages are in-process values only.
- RFF-REQ-007 still needs a host-adapter grammar and validation boundary.
- The v0.1 runtime explicitly does not catch application panics; no future
  containment mechanism or minimum supported Rust version is selected.
- A shared application service context, automatic dispatch order, fairness, and
  event filtering remain unselected.

## Most likely next tasks

1. Implement the accepted configuration activation, rejection, revision, and
   rollback behavior as the first Stage 3 slice.
2. Record the command/telemetry host grammar and validation boundary before
   beginning RFF-REQ-007 implementation.
3. Compose the verified services into a small sample mission without widening
   the event, scheduling, messaging, or containment claims.

## Latest run

2026-08-30: Added direct returned-work failure-event reporting in commit
`a04c5bd3f929934b7578b14f181138ae0be56e9b`. Four public tests prove exact
structured fields, one injected timestamp read and event attempt, complete error
and source-chain preservation, no event on success/lifecycle rejection,
saturation retention and explicit retry, failed state, and later peer work. The
complete implementation baseline passes 58 tests with warnings denied. No
dependency, permanent runtime clock/event owner, other callback event path,
application event API, host drain, thread, executor, protocol behavior, release,
or push was added.
