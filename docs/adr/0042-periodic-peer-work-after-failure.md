# ADR-0042 — Continue periodic peer work after a returned failure

Date **2026-11-18**. Decision accepted for the example under RFF-LIB-REQ-017.

## Context

[ADR-0030](0030-host-owned-periodic-releases.md) assigns cursor polling and
disposal to the host. The [periodic integration tests](../../tests/periodic_work.rs)
exercise selected timer disposal and peer work with the messaging owner.
The [direct runtime tests](../../tests/runtime_events.rs) exercise typed
failure diagnostics and peer work with a saturated FIFO. A downstream
developer needs a runnable driver that continues calling the peer's
independent cursor while diagnostics are pending.

## Decision

Compose one direct runtime, one manual clock, two optional cursors and one
capacity-one event FIFO in a serial host example. Select one cursor per
turn. For a released item, call the direct runtime with the ID carried by
the item. On a work error, discard that cursor and return the typed result,
including its failure-event attempt if present. A later disabled turn
returns before polling. Diagnostic dequeue is a separate choice.

Propagating the first application error through the whole driving function
would end the observed peer's work. The example instead handles that
expected error at the call site and advances the peer separately. A shared
agenda or framework supervisor would require choosing fairness, lifecycle
and recovery rules for consumers ahead of evidence that they need those
rules in the library.

## Evidence and consequences

[Sources and provenance](../research/SOURCES.md#src-rff-periodic-peer--periodic-work-after-returned-failure)
records the reuse. The [guide](../PERIODIC_PEER.md) documents the serial
turn policy and the [review](../verification/PERIODIC_PEER_REVIEW.md)
records the workspace and downstream evidence. A diagnostic can be
rejected while the original work error remains available. Progress depends
on the caller selecting turns and each callback returning. Revisit this
policy when a downstream use demonstrates a need for multi-application
turn selection, recovery or bounded diagnostic retention.

## Two retained failure events — 2026-11-19

RFF-LIB-REQ-017 revision 2 adds a second independently registered sampler
alongside the healthy heartbeat. The concrete need is to preserve both
terminal failure diagnostics when the shared FIFO is occupied, without
preventing heartbeat work. The one-slot host-progress consumer cannot store
this history. Existing library APIs expose both exact rejected records.

Keep two inline retry slots in host observation order. Each failing cursor
is disabled after its one returned error; the heartbeat cannot return an
application error in this mission. These facts bound retained failures at
two. Re-offer only the oldest on one explicit retry turn, preserving both
slots on QueueFull and removing only an accepted record. Keep the original
typed error available to the caller. Recorded attempts and lifecycle
rejections add no retry record.

Alternatives considered: independent per-role retry order would reorder
reversed failures; an unbounded list would hide resource policy; a library
retry service would impose mission ownership and scheduling rules. A
separate test consumer could prove the policy, but extending the existing
executable also makes the recovery sequence runnable without copied private
fixtures. Retain its original histories and trace prefix.

This orders retained rejected events only. A newly generated event can enter
the shared FIFO before an older rejected event if the caller selects work
after freeing space and before retry. The host does not reserve capacity or
promise global occurrence order. Caller-selected retry and returning
callbacks/writes remain necessary. Revisit the fixed bound before adding
another failing role, restart/recovery, or another diagnostic source to the
retention policy. The guide and focused review record the exact evidence.

## Retained admission priority — 2026-11-20

Revision 3 addresses the observed gap between rejected-event order and
fresh failure admission. A host choosing diagnostic priority needs to defer
new failure-producing work while preserving independent healthy work.
Add an optional `turn_after_retained_failures` policy to this example.
An enabled sampler returns `DeferredForDiagnostics` before polling when
the compacted retry head is occupied. A disabled sampler still reports
`Disabled`; heartbeat delegates through the existing work path.

Keep retry and dequeue explicit. Once the last retained event is admitted,
fresh sampler work may proceed even while that event occupies the FIFO.
Its newer rejected failure is retained through the existing mechanism.
Keep the unguarded turn for callers choosing a different order.

Alternatives: automatic retry followed by work combines two effects in
one turn; gating every application prevents useful heartbeat work;
gating on FIFO occupancy waits for dequeue rather than admission. Reject
those stronger policies for this consumer. A framework scheduler would
prematurely generalize mission knowledge about which roles can fail.

The existing cursor, work-error and event-queue contracts suffice. Tests
compare original release/state and exact event histories under both role
orders. The guide and focused review record actual downstream evidence.
Revisit this role policy if heartbeat can fail, cursors can recover, or
another source shares retention. Selecting only guarded turns prioritizes
retained admission; it neither schedules retries nor establishes fairness,
global occurrence order, durable storage or external delivery.
