# ADR-0016: Returned-work failure events

- Status: Accepted and implemented as the completing Stage 2 event slice
- Date: 2026-08-30
- Scope: One bounded structured-event attempt after cooperative work failure

## Context

RFF-REQ-005 requires a host scenario to emit machine-inspectable events with a
framework timestamp and finite storage or delivery policy. RFF-REQ-008 requires
a fault-injection scenario to observe a failed application's state and event
while a peer remains operable. ADR-0013 supplies structured records and a
positive-capacity reject-newest queue. ADR-0014 supplies injected elapsed time.
The existing `Runtime::work` path preserves a concrete returned error, commits
only that application to terminal `Failed`, and leaves a peer available, but it
does not compose those three boundaries.

The smallest integration must preserve `Runtime::work` as the single lifecycle
authority, keep event saturation visible without replacing the application
error, and avoid choosing a shared service context or a second wrapper that
owns the runtime, clock, and event queue.

## Decision

Add an opt-in `Runtime::work_with_failure_event` operation. It receives one
runtime-local application identity, a mission-defined copied failure-event
identifier, an injected clock reference, and a mutable reference to an existing
`EventQueue`.

The operation first delegates to `Runtime::work`:

1. Successful work returns `Running`. It does not read the clock or attempt an
   event.
2. Lifecycle rejection returns the complete `RuntimeWorkError::Lifecycle`. It
   occurs before application code, does not read the clock, and does not attempt
   an event.
3. A returned application work error first completes the existing transition of
   only the selected record to terminal `Failed`.
4. The integration then reads the injected clock exactly once and constructs
   one event with `EventSource::Application(application_id)`,
   `EventSeverity::Error`, the supplied identifier, and the captured timestamp.
5. It calls the bounded queue's existing `emit` operation exactly once and does
   not retry, wait, overwrite, spill, or emit a recursive diagnostic.

`RuntimeWorkEventError` retains the complete `RuntimeWorkError` unchanged. Its
optional `FailureEventAttempt` exists exactly for the application-error case and
contains both the exact event and `EventEmitOutcome`. A recorded event is copied
both into the queue and into caller-owned error context. If the queue is full,
older records remain unchanged and the caller receives the rejected event with
its original failure timestamp for explicit handling or retry. No retry is
required or automatic.

The runtime constructs the application source from the selected record instead
of accepting a caller-invented event source. The caller still selects the
mission event identifier and supplies the intended clock and queue. Neither
resource becomes permanently owned by `Runtime`.

This integration applies only to direct `Runtime::work`. Ordinary work remains
available without event reporting. Start, stop, restart, scheduled work, and
message-dispatch failures do not emit through this method. Applications cannot
author general events, and the event queue still has no filtering, fan-out,
persistence, host drain adapter, or guaranteed-delivery policy.

## Alternatives considered

### Make every `Runtime::work` error emit automatically

Rejected. The existing runtime owns neither a clock nor an event queue, and
changing the established operation would require hidden resource selection or
a new permanent owner. An explicit integration method keeps effects and
borrowing visible while the multi-service composition remains provisional.

### Add a wrapper that owns runtime, clock, and event queue

Deferred. A second owning wrapper would compete with `MessagingRuntime`, and no
sample mission yet demonstrates the stable combined ownership and borrowing
shape. Borrowing the two services for one operation proves the required
behavior without freezing that larger API.

### Emit lifecycle rejections

Rejected for this slice. RFF-REQ-008 concerns an application-returned error.
Lifecycle rejection occurs before application code and already has a typed
caller-visible result. Emitting it would broaden framework diagnostic policy and
consume clock and queue resources for API misuse without current evidence.

### Return only the event-queue outcome

Rejected. A `QueueFull` result without the event would discard the exact
runtime-generated timestamp and prevent explicit retry of the same record. It
could also encourage callers to treat diagnostic saturation as the primary
failure. Returning the complete work error and event attempt keeps both facts
independent and observable.

### Route failure events through application messaging

Rejected. Event observation must not depend on the failed application's inbox,
subscription, lifecycle availability, or queue-clearing policy. ADR-0013 keeps
diagnostic storage separate from application message delivery.

## Evidence

Four public integration tests demonstrate:

- a returned concrete work error commits only the selected application to
  `Failed`, captures exactly one injected clock reading, records the exact
  application source, error severity, mission identifier, and timestamp, and
  leaves a running peer able to complete later work; a repeated call for the
  failed application neither invokes it nor duplicates the diagnostic;
- a saturated event queue retains its older event, returns the complete original
  work error plus `QueueFull` and the rejected event, permits explicit retry
  after one dequeue, and still leaves the peer operable;
- registered-state and unknown-identity lifecycle rejections read the clock zero
  times and emit no event; and
- successful running work also reads the clock zero times and emits no event.

Together with the existing exact-capacity event-queue tests, this verifies
RFF-REQ-005 at the narrow runtime-generated returned-work failure boundary. The
fault-injection state, event, original-error, saturation, and peer-progress
evidence verifies RFF-REQ-008 for cooperative returned work errors. This does
not extend either claim to panics, hangs, process termination, memory
exhaustion, hardware faults, cleanup, rollback, or guaranteed event delivery.

No external research was needed. This decision composes existing local runtime,
event, and clock contracts using only safe stable Rust and adds no dependency.

## Consequences and risks

- The integration is opt-in. Callers that use `Runtime::work` directly receive
  no failure event, and no other callback path emits one.
- The event timestamp is captured after the application returns and the runtime
  commits `Failed`. It represents the framework's observation time, not the
  exact instant of an application-internal fault.
- Reject-newest saturation can omit the failure event from retained storage.
  Returning the event makes loss explicit but provides no delivery guarantee.
- The caller-owned event in the returned error is outside the queue's retained-
  record bound, as are other values already returned to callers.
- `ApplicationId` and `FrameworkInstant` still do not encode runtime or clock
  origin. The caller can supply a same-shaped foreign queue, clock, or event
  identifier; this method validates only the application identity through its
  owning runtime.
- The application may mutate itself before returning its error. Reporting adds
  no rollback, cleanup, retry, recovery, panic containment, hang containment,
  or fault-tolerance behavior.
- A future scheduled or messaging-aware event path must delegate through the
  corresponding owner so it preserves final schedule consumption and inbox
  clearing.

## Revisit conditions

Revisit when the sample mission needs application-authored events, more
callback operations to report failures, a combined runtime/clock/event owner,
messaging-aware or scheduled event reporting, event filtering or priority, a
host drain adapter, guaranteed diagnostic delivery, or explicit clock/runtime
domain identity. Any broader design must retain the original application error,
finite storage behavior, visible saturation, and successful peer isolation.
