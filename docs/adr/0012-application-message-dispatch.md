# ADR-0012: One-message application dispatch and self-publication

- Status: Accepted and implemented
- Date: 2026-08-26
- Scope: Caller-selected consumption of runtime-owned application inboxes

## Context

ADR-0004 places one serial in-flight delivery outside the sum of configured
inbox capacities and requires application self-publication evidence. ADR-0010
implements FIFO dequeue only as a standalone routing-core probe. ADR-0011 then
couples one fresh inbox to each application and derives availability from
lifecycle state, but deliberately exposes no application dequeue or messaging
context.

The context-free `Application::work` callback cannot borrow the selected
application mutably while the same owner is also queried for every destination's
lifecycle state. A broad general-purpose service context would settle time,
events, configuration, and other unimplemented service APIs speculatively. The
smallest useful boundary needs only one inbound message and publication through
the already-owned bus.

## Decision

Add a separate `MessagingApplication<Topic, MAX_PAYLOAD_BYTES>` trait extending
`Application`. Its `handle_message` callback receives:

- a borrow of exactly one message already removed from its inbox; and
- a mutable `ApplicationMessageContext` that exposes the selected application
  identity and bounded `publish`, but no lifecycle mutation, inbox inspection,
  dequeue, nested dispatch, or inner bus access.

The separate callback preserves context-free `Application::work` for work not
caused by a queued delivery. Both remain unpublished pre-v0.1 APIs. A common
framework-service context is deferred until another implemented service proves
a coherent shared borrowing requirement.

`MessagingRuntime::dispatch_one` is synchronous and caller-selected. It applies
this order:

1. Validate that the identity exists and is `Running`. Rejection changes no
   queue and invokes no application code.
2. Refresh a fixed-size preallocated snapshot of every runtime lifecycle state.
3. Remove at most the selected inbox's oldest FIFO message. An empty inbox
   returns `MessageDispatchOutcome::InboxEmpty` without invoking the callback.
4. Hold the removed message as the sole framework-owned in-flight delivery and
   invoke `handle_message` once.
5. On success, drop the completed in-flight record, retain `Running`, and keep
   all accepted callback publications queued.
6. On a returned callback error, drop the attempted in-flight record, commit
   only the selected application to terminal `Failed`, clear that
   application's remaining queued deliveries, and return the original concrete
   error plus the exact queued discard count.

The 2026-09-15 source review clarifies steps 5 and 6: the callback closure
releases its local in-flight message before the runtime commits the returned
state. This describes the existing serial implementation and does not change
the public post-return contract or add panic/unwind containment.

The in-flight message is attempted callback input, not a queued discard, so it
is excluded from `discarded_deliveries`. Callback publications take effect
immediately. A later callback error does not roll back accepted peer deliveries;
self-publications and older queued deliveries in the failed endpoint are
cleared together.

### Borrowing and availability

`MessagingRuntime` reserves one `ApplicationState` snapshot slot per application
during construction. Allocation failure is typed and returns the unchanged
owned runtime. Immediately before each non-rejected dispatch, the snapshot is
overwritten from `Runtime`, the lifecycle source of truth.

The narrow callback cannot change lifecycle state, and the caller cannot access
the mutably borrowed runtime until the synchronous callback returns. Therefore
the snapshot is stable for that publication interval and is not an independently
mutable availability authority. It permits disjoint mutable borrows of the
selected application and `MessageBus` without unsafe code or temporarily
removing the application from its runtime record.

Self-publication uses the selected application's snapshotted `Running` state.
Because the inbound message has already left the queue, a self-publication may
occupy the freed slot. The framework still owns no more than the sum of inbox
capacities plus the one in-flight delivery record.

## Alternatives considered

- Replacing `Application::work` with a general service context was rejected for
  this slice because it would revise every application and commit APIs for
  services that do not exist.
- Returning an application-defined outbox after the callback was rejected
  because it is not true in-callback publication and cannot return immediate
  saturation or availability results to the application.
- Temporarily taking the application out of its runtime record was rejected
  because panic unwinding could leave the record empty without a restoration
  mechanism whose complexity exceeds this cooperative boundary.
- Separating all runtime states and applications into parallel storage or a
  split-slice view remains a possible future simplification, but it would
  rewrite the proven lifecycle owner and introduce a larger correspondence
  invariant for this one dispatch need.
- Batch or automatic dispatch was deferred because ordering, fairness,
  scheduling, and due-work policy require time and scheduling evidence.

## Evidence

Implementation commit `150b924e6391c9adcc14f23bf21138011b747313`
adds the callback, context, preallocated state snapshot, caller-selected
dispatch, and five focused public-API tests. They demonstrate:

- non-running and unknown dispatch rejection before queue or callback mutation;
- an explicit empty-inbox outcome without callback invocation;
- oldest-first, exactly-one-message callback invocation;
- capacity-one self-publication occupying the slot freed by the in-flight
  delivery, plus explicit saturation on another self-publication;
- publication to a running peer and unavailable reporting for a stopped peer;
- callback success retaining `Running`; and
- a returned callback error preserving its source, committing only the selected
  application to `Failed`, excluding the in-flight item from the discard count,
  clearing selected queued/self-published deliveries, and retaining accepted
  peer deliveries.

Existing lifecycle, routing, availability, clearing, and restart tests remain
regression evidence. The complete suite passes 38 tests with no dependency,
thread, executor, automatic dispatch, scheduler, clock, event service, or
protocol boundary added by this decision.

## Consequences and risks

- Caller selection provides no automatic dispatch order, fairness, priority,
  frequency, deadline, or progress guarantee.
- The application may copy or retain message contents and publication reports;
  those application-owned values are outside framework inbox bounds.
- Immediate peer publication followed by publisher failure can expose peer work
  derived from a callback that did not complete. This is explicit attempted
  behavior, not transactionality or rollback.
- A returned error may follow arbitrary partial application mutation. Panics,
  hangs, process failure, memory exhaustion, cleanup, and isolation remain
  outside this cooperative boundary.
- The state snapshot and publication report use bounded-by-topology host
  allocation. No real-time or general non-blocking claim follows.

## Revisit conditions

Revisit when a second framework service needs the application boundary, a
sample mission demonstrates that separate work and message callbacks are
awkward, dispatch ordering or fairness becomes required, concurrency invalidates
the serial snapshot, or message failure needs retry, dead-letter, rollback, or
transactional semantics.
