# ADR-0013: Bounded structured-event queue

- Status: Accepted and implemented as the first RFF-REQ-005 slice
- Date: 2026-08-27
- Scope: Standalone in-process structured-event storage and delivery

## Context

RFF-REQ-005 requires machine-inspectable source, severity, event identifier,
and framework timestamp fields plus a finite storage or delivery policy. Stage
2 also needs an event timestamp consumer before choosing an injected clock.
The repository has neither an event representation nor an event overflow
contract.

The existing application message bus is not a diagnostic path. It fans out to
mission-selected application subscriptions, rejects unavailable lifecycle
endpoints, and clears a failed application's inbox. Reusing it for runtime
failure events could make the diagnostic unavailable with its subject or turn
bus saturation into recursive publication. ADR-0004 already requires overflow
diagnostics to return directly rather than enter the saturated bus.

NASA cFE's Event Service informs only the responsibility boundary already
recorded under `SRC-NASA-CFE`. The representation and queue policy below are
independent local Rust decisions and do not claim cFE behavior or compatibility.

## Decision

Add a standalone `EventQueue<EventId>` and these structured record types:

- `EventSource` distinguishes framework-originated events from events
  attributed to one runtime-local `ApplicationId`. A standalone queue stores
  that identity as caller-supplied data and cannot validate its issuer.
- `EventSeverity` provides `Informational`, `Warning`, and `Error` categories.
  The categories carry no response, priority, filtering, or compatibility
  semantics in this slice.
- The mission supplies a copied Rust `EventId`, normally an enum. The framework
  does not choose a numeric width or imply a wire, CCSDS, cFS, or persistent
  identifier.
- `EventTimestamp` wraps elapsed `Duration` from an origin selected by a future
  framework clock. Producers supply it explicitly in this slice. It is not
  wall-clock time, and the queue neither generates timestamps nor enforces
  monotonicity.
- `Event<EventId>` contains exactly source, severity, identifier, and timestamp.
  It has no free-form text or payload whose encoding and byte limit would need
  a separate policy.

The queue applies these storage and delivery rules:

- construction requires a positive logical capacity in event records and
  reserves storage for the complete bound before returning a queue;
- accepted events are copied into FIFO order;
- emission through the exact configured capacity returns `Recorded`;
- a full queue retains every older record, rejects the newest event, and
  returns `QueueFull { capacity }` without mutation;
- the event is borrowed for emission, so the caller retains it for explicit
  handling or retry after either outcome;
- saturation never emits another event, waits for consumer progress, retries,
  spills, overwrites, persists, or creates hidden work; and
- dequeue removes at most the oldest event and frees one logical slot.

Records removed from the queue become caller-owned and no longer count against
the framework queue bound. Retention by a consumer is outside this service's
storage policy.

This slice does not attach the queue to `Runtime` or `MessagingRuntime`, add an
application service context, or emit runtime failures. RFF-REQ-005 therefore
remains partial until a framework event path uses an injected clock and host
integration produces the timestamps. RFF-REQ-008 remains partial until a
returned application error attempts a structured event while a peer remains
operable.

## Alternatives considered

### Reuse the bounded application message bus

Rejected. Application routing, lifecycle availability, subscriptions, and
queue clearing are different policies from host diagnostic visibility. A
failure event must not depend on the failed application's inbox or recursively
diagnose bus saturation through that bus.

### Use an unbounded vector, logger, or formatted text stream

Rejected. An unbounded collection violates the finite operational-storage
direction. A logger or formatted string would make tests parse presentation
text instead of asserting structured fields and would leave text bounds,
encoding, blocking, and host-effect behavior undefined.

### Overwrite the oldest record when full

Rejected for the first policy. It keeps the newest diagnostic but silently
destroys the earliest retained causal evidence. Reject-newest preserves the
accepted trace and gives the producer an explicit outcome. A later measured
consumer may justify a different queue with a separate contract.

### Deliver immediately through a callback sink

Rejected. A caller-defined sink could block, allocate, re-enter framework code,
or retain events without a framework-owned bound. A bounded queue establishes
the behavior that later host adapters can drain.

### Add the clock and application/runtime integration now

Deferred. Combining timestamp generation, service-context borrowing, automatic
failure emission, event saturation reporting, and queue storage would exceed
one reviewable increment. The standalone record defines the clock consumer; an
injected clock is the next separate boundary.

## Evidence

Public-API tests demonstrate:

- exact machine-inspectable framework and application sources, all three
  severities, a mission enum identifier, and elapsed timestamp values;
- typed rejection of zero capacity and observable positive capacity;
- acceptance through the exact record limit and FIFO dequeue independent of
  timestamp order; and
- reject-newest saturation retaining older events, followed by successful
  explicit retry as soon as one dequeue frees a slot.

The implementation adds no dependency, thread, executor, wall-clock read,
runtime mutation, application callback change, recursive diagnostic, text
payload, filter, fan-out, persistence, protocol boundary, or compatibility
claim.

## Subsequent returned-work integration

[ADR-0016](0016-returned-work-failure-events.md) later composes this queue with
an injected clock and direct runtime work. One cooperative returned work error
now attempts a structured event while retaining the complete original error and
exact `Recorded` or `QueueFull` outcome. Statements above that RFF-REQ-005 and
RFF-REQ-008 remained partial describe this decision's original standalone
checkpoint; both are now verified only at ADR-0016's narrow returned-work
boundary.

## Consequences and risks

- Queue storage and event metadata are bounded by the configured record count
  and fixed Rust type sizes. An `EventId` may still contain a copied reference
  to caller-owned static data, so this is not a whole-process byte bound.
- Reject-newest can omit a later high-severity event while saturated. The
  outcome is explicit, but a caller can ignore it; this is not guaranteed
  delivery or a durable audit log.
- Explicit timestamps can arrive out of order because standalone callers and
  integrated runtime paths can use different injected clocks. FIFO represents
  emission-call order, not timestamp sorting.
- `EventSource::Application` carries the existing runtime-local identity risk:
  equal-position keys from separate issuers are indistinguishable here.
- The fixed severity set may be revised before v0.1 if filtering or a sample
  mission demonstrates another category is necessary.
- No application can emit through a framework context yet, and runtime failure
  paths produce no event in this slice.

## Revisit conditions

Revisit this decision when injected time supplies timestamps, application or
runtime failure paths cross the event boundary, a host adapter drains events,
filtering or multiple consumers become required, measured event content needs
bounded structured detail, or a different overflow policy has concrete mission
evidence.
