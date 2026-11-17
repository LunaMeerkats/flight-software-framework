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
