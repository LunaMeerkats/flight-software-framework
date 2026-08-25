# ADR-0011: Runtime-owned lifecycle-aware message availability

- Status: Accepted and implemented as the ADR-0004 lifecycle integration slice
- Date: 2026-08-25
- Scope: Ownership and lifecycle availability for in-process application inboxes

## Context

ADR-0004 requires one runtime-owned inbox per application, explicit unavailable
outcomes, clearing when an endpoint stops or fails, and empty reconnection after
restart. ADR-0010 proves bounded available-endpoint routing separately, but its
caller-supplied identities and detached lifecycle state allow an incomplete or
foreign-identity topology.

The context-free `Application::work` callback does not yet provide a safe
borrowing shape for framework services. Runtime ownership and lifecycle
synchronization can be proved before exposing message access to application
code, avoiding a larger callback and dispatch decision in the same increment.

The accepted ADRs did not explicitly classify a newly `Registered` endpoint.
Accepting pre-start messages would retain deliveries for an application that
cannot yet work and could make initial behavior depend on when mission setup
publishes. The integration therefore needs one complete state-to-availability
rule rather than inheriting the detached bus default.

## Decision

`MessagingRuntime<A, Topic, MAX_PAYLOAD_BYTES>` consumes a fully composed
`Runtime<A>` and constructs a fresh `MessageBus` internally. It applies these
construction rules:

- the caller supplies one `RuntimeInboxConfig` per registered application in
  registration order, without supplying application identities;
- the configuration count must exactly equal the runtime record count;
- every record must still be `Registered` before attachment;
- the fresh bus assigns the runtime's contiguous identities internally and
  starts with every inbox empty;
- attachment failure returns ownership of the unchanged runtime; and
- successful attachment freezes registration and exposes neither inner owner
  mutably, so lifecycle synchronization cannot be bypassed through this API.

Mission composition remains responsible for associating each positional
capacity and topic set with the intended application. The integration proves
identity order and complete inbox count; it cannot detect two valid positional
configurations that the caller accidentally swaps.

The detached `MessageBus` remains available for its routing-core evidence and
continues to model its configured endpoints as available. Lifecycle claims in
this decision apply only to `MessagingRuntime`.

### Availability table

| Runtime state | Matching destination status | Queue invariant |
| --- | --- | --- |
| `Registered` | `Unavailable` | Empty because construction is fresh and delivery is rejected |
| `Running` | `Delivered` or `InboxFull` | Retains accepted deliveries up to its configured slot limit |
| `Stopped` | `Unavailable` | Cleared before the stop operation returns |
| `Failed` | `Unavailable` | Cleared before the callback error returns |

Configured but unavailable subscribers remain matching destinations in stable
registration order. They receive `DeliveryStatus::Unavailable`; a publication
with only unavailable destinations is `WhollyUndelivered`, not
`NoSubscribers`. `Complete`, `Partial`, and `WhollyUndelivered` continue to be
derived from the ordered destination outcomes.

Availability is derived from `Runtime::state` for every publication rather
than stored as a second mutable flag. The report still reserves space for all
matching destinations before any inbox is changed.

### Lifecycle and clearing order

The integration delegates each callback and state transition to `Runtime`, then
applies the queue effect before returning:

- successful start commits `Running`; the initial empty inbox becomes
  available;
- successful work retains `Running` and does not change the inbox;
- successful stop commits `Stopped`, clears that inbox, and returns the exact
  discarded-delivery count;
- successful restart commits `Running` and reconnects the inbox already
  cleared by stop;
- any returned callback error commits terminal `Failed` and clears only that
  application's inbox before the error is returned; and
- lifecycle rejection invokes no callback, changes no state, clears no queue,
  and reports zero discarded deliveries.

`Failed` is terminal under LC1. “Restart reconnection” means only a successful
`Stopped -> Running` restart; it is not a recovery path from `Failed`.

The generic `MessagingOperationError<E>` retains the existing operation-specific
runtime error as `E`, preserves its error-source chain, and adds the exact
number of queued delivery records cleared by that operation. Successful stop
returns `MessagingStopOutcome` with the committed state and count. Counts are
queued delivery records, not payload bytes, rejected publications, future
in-flight messages, or evidence of secure erasure.

## Alternatives considered

- Passing `&mut MessageBus` into individual `Runtime` methods was rejected
  because callers could omit synchronization or pair unrelated owners.
- Public connect and disconnect methods or a stored availability flag were
  rejected because they create a second mutable lifecycle truth.
- Accepting an already-created bus was rejected because it could contain stale
  deliveries and retain the detached identity/topology ambiguity.
- Adding topic and payload parameters directly to `Runtime` was deferred
  because it would couple lifecycle-only composition to messaging and force a
  broader constructor and registration change.
- Moving message storage into each runtime record remains a possible future
  simplification, but it would rewrite the proven lifecycle and routing cores
  rather than add the smallest ownership checkpoint.
- Changing `Application::work` in this slice was rejected because
  self-publication, dequeue ownership, one in-flight delivery, and service
  borrowing still need their own evidence.

## Evidence

Public-API tests demonstrate:

- count mismatch, non-registered attachment, and invalid inbox configuration
  preserve the owned runtime;
- registered subscribers are ordered, known, unavailable, and empty;
- running fan-out followed by successful stop returns the exact cleared count,
  leaves the peer queue intact, and produces partial unavailable/delivered
  publication;
- successful restart reconnects an empty reusable inbox;
- returned work and stop errors clear only the failed endpoint while the peer
  remains available;
- lifecycle rejection returns zero discarded deliveries without clearing a
  running inbox; and
- start and restart callback failure leave terminal unavailable empty
  endpoints while preserving the concrete operation error.

Existing detached message-bus tests continue to verify available-endpoint
routing without lifecycle integration. This slice adds no dependency, thread,
executor, wait, retry, spill path, dynamic subscription, application dispatch,
event service, clock, scheduler, or protocol boundary.

## Subsequent application boundary

[ADR-0012](0012-application-message-dispatch.md) preserves this owner and adds
caller-selected one-message dispatch. It refreshes lifecycle states before each
callback and supplies only a publish-capable context, so application
self-publication cannot bypass this decision's availability or clearing rules.

## Consequences and risks

- Mission composition becomes a two-step pre-v0.1 process: register every
  application, then consume the still-registered runtime into its immutable
  message topology. A future sample may provide a narrower builder if this is
  demonstrably awkward.
- At this slice boundary, only external callers could publish and inspect queue
  counts. ADR-0012 now adds application consumption and self-publication through
  a separate messaging callback; ordinary `Application::work` remains
  context-free.
- Clearing happens only after a synchronous callback returns. Panics, hangs,
  process failure, memory exhaustion, application-internal cleanup, and secure
  payload erasure remain outside this cooperative behavior.
- The integration prevents foreign identities and incomplete inbox counts
  through its own API, but cannot recognize swapped positional mission
  configurations. The detached `MessageBus` and standalone `Runtime` remain
  intentionally usable experimental building blocks without the integration
  guarantees.
- Inline payload-copy and per-publication report-allocation costs from ADR-0010
  are unchanged.

## Revisit conditions

Revisit this wrapper when mission composition demonstrates a need for an atomic
application-plus-inbox builder, message storage belongs more coherently inside
runtime records, a common multi-service context becomes justified, dynamic
topology becomes necessary, or concurrency changes the serial state and
clearing order.
