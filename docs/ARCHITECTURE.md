# Initial architecture analysis

This document separates observed upstream responsibilities from proposed local
design. Source details and adoption notes are in [the source register](research/SOURCES.md).

## Observed responsibility boundaries

The following observations use the official NASA cFS v7.0.1 public release:

- The public cFS bundle contains cFE, OSAL, PSP, tools, and common/example
  applications. NASA distinguishes that bundle from a flight distribution,
  which selects applications for particular mission requirements, and describes
  the bundle as a starting point rather than a fully verified operational
  system. (`SRC-NASA-CFS`)
- cFE provides Executive, Software Bus, Event, Table, and Time services, plus a
  File Service API. (`SRC-NASA-CFE`)
- OSAL is a collection of operating-system abstraction APIs. (`SRC-NASA-OSAL`)
- PSP abstracts platform-specific functionality. (`SRC-NASA-PSP`)
- cFE guidance treats application identity and lifecycle as executive concerns;
  communication normally uses published message interfaces rather than direct
  calls into another application. (`SRC-NASA-CFE`)
- cFE software-bus pipes have explicit depths, and table loading distinguishes
  staged validation from activation. These are useful responsibility examples,
  not local API requirements. (`SRC-NASA-CFE`)

These facts do not make similarly named local components equivalent or
compatible. This analysis adapts problem boundaries; it does not establish or
claim implementation equivalence.

## Proposed Rust-native shape

The first host implementation should be a small composition, not a set of
one-to-one cFS component clones:

1. **Caller-driven runtime:** owns application registry, identity, lifecycle,
   deterministic ordering, and supervisory policy.
2. **Application boundary:** conforming application behavior accesses framework-
   managed effects through an explicit context and returns typed outcomes.
3. **Framework services:** bounded routing, structured events, clock access, and
   validated configuration are added only as vertical slices require them.
4. **Host adapters:** wall-clock, file, terminal, and future network access stay
   behind narrow interfaces; simulated adapters drive tests.
5. **Mission composition:** a sample selects applications, capacities, policies,
   schedules, and adapters without changing framework internals.
6. **External boundaries:** command ingestion, telemetry output, and any future
   protocol codecs validate external data outside core application logic.

Applications should not call one another directly. The runtime owns framework-
managed lifecycle, queue, and adapter resources; applications may own internal
state. Iteration order that affects observable behavior must be stable. The
initial runtime will not spawn hidden work.

## Chosen first execution model

[ADR-0001](adr/0001-caller-driven-host-runtime.md) selects a serial,
caller-driven host runtime for the first v0.1 slices. The current host explicitly
selects one application work call, one message dispatch, or at most the next due
item in a finite one-shot work agenda. Callers can also emit to and dequeue from
the standalone event queue. An injected `Clock` exposes elapsed
`FrameworkInstant` readings, and its manual implementation advances only on
explicit caller requests. The schedule borrows that clock and never advances it.
This keeps work, event, and transition ordering testable without adopting an
async runtime or thread lifecycle early.

Here, **deterministic** means that conforming, terminating applications with
controlled external effects produce the same framework-observable ordering from
the same initial state, simulated time, and ordered inputs. It does not mean
hard real-time execution, bounded wall-clock latency, freedom from OS jitter, or
fault tolerance.

## Approved service policies

- [ADR-0003](adr/0003-stop-gated-lifecycle-and-runtime-local-identity.md)
  defines the LC1 stop-gated lifecycle and bounded runtime-local identity.
- [ADR-0004](adr/0004-bounded-application-inboxes.md) defines one bounded inbox
  per application, reject-newest overflow without waiting for inbox capacity or
  subscriber progress, explicit partial fan-out reporting, and lifecycle-aware
  clearing and reconnection. ADR-0010 through ADR-0012 implement its routing,
  lifecycle ownership, and caller-selected dispatch boundaries; their combined
  evidence verifies RFF-REQ-003.
- [ADR-0005](adr/0005-configuration-revisions-and-rollback.md) defines immutable
  snapshots, monotonic revision assignment, and one consume-once rollback slot.
  ADR-0017 implements the bounded table, and ADR-0018 integrates constructor
  ownership plus read-only visibility through the ordinary-work context. Their
  combined evidence verifies RFF-REQ-006.
- [ADR-0006](adr/0006-static-application-ownership-for-initial-runtime.md)
  selects a finite-capacity generic runtime, explicit static mission
  composition, and concrete returned start errors for the first owned slice.
- [ADR-0007](adr/0007-operation-specific-owned-stop-boundary.md) extends that
  pre-v0.1 boundary with synchronous stop and a distinct concrete stop error.
- [ADR-0008](adr/0008-distinct-in-place-owned-restart.md) completes the owned
  LC1 transition surface with a distinct in-place restart callback and concrete
  restart error.
- [ADR-0009](adr/0009-caller-driven-owned-work.md) adds one explicit
  caller-selected work operation for `Running` applications without defining
  automatic dispatch, scheduling, or a service context.
- [ADR-0010](adr/0010-bounded-message-routing-core.md) selects inline
  const-bounded payloads, mission-defined copied topics, immutable configured
  inbox topology, and one ordered publication report for the first ADR-0004
  slice.
- [ADR-0011](adr/0011-runtime-owned-message-availability.md) couples a fresh
  complete inbox topology to a still-registered runtime, makes only `Running`
  endpoints available, and returns exact queue-clearing counts on stop and
  returned callback errors.
- [ADR-0012](adr/0012-application-message-dispatch.md) adds caller-selected
  one-message dispatch through a separate application callback and a
  publish-only context backed by a refreshed lifecycle-state snapshot.
- [ADR-0013](adr/0013-bounded-structured-event-queue.md) selects structured
  source, severity, mission identifier, and elapsed timestamp values plus a
  positive pre-reserved FIFO queue with caller-visible reject-newest saturation.
- [ADR-0014](adr/0014-injected-manual-framework-clock.md) selects a general
  elapsed instant, an object-safe injected clock read, checked manual
  advancement, and explicit conversion into the event timestamp field.
- [ADR-0015](adr/0015-caller-driven-scheduled-work.md) selects a finite
  nondecreasing one-shot work agenda, stable equal-time order, final consumption
  after each due attempt, and at most one attempted item per caller request.
- [ADR-0016](adr/0016-returned-work-failure-events.md) composes direct work,
  injected time, and bounded event storage for one returned-error event attempt
  that never replaces the original application error.
- [ADR-0017](adr/0017-bounded-configuration-snapshots.md) implements immutable
  byte-bounded configuration snapshots, one retained validator, revision
  assignment, and consume-once rollback.
- [ADR-0018](adr/0018-configuration-aware-work-context.md) integrates one
  optional table owned at runtime construction with immutable configuration
  visibility through every existing ordinary-work path.
- [ADR-0019](adr/0019-host-command-telemetry-boundary.md) implements the private
  mission-local host adapter pair: a validated two-byte echo command, matching
  telemetry, and a borrowed capacity-one host mailbox drained outside dispatch.
  The executable example and integration tests use the same mission source.
- [ADR-0020](adr/0020-messaging-work-failure-events.md) adds opt-in failure
  reporting after messaging-owned ordinary work finishes lifecycle commitment
  and selected-inbox clearing, retaining the exact error and discard count.

## Current implementation boundary

The bounded `LifecycleRegistry` remains the standalone logical LC1 state model:
it allocates opaque identities in registration order and enforces
`Registered -> Running -> Stopped -> Running` without owning application
objects. `Runtime<A, E, MAX_CONFIGURATION_BYTES>` separately owns
finite-capacity application records and optionally one bounded configuration
table. A mission-selected concrete representation, such as an enum, implements
synchronous `Application::start`, `Application::work`, `Application::stop`, and
`Application::restart` boundaries with distinct concrete error types.

Runtime start, work, stop, and restart validate identity and state before
invocation. Lifecycle success commits `Running`, `Stopped`, or `Running`, while
work success retains `Running`. Restart and work mutably borrow the retained
application value. A returned error is preserved in the caller-visible
operation-specific result and commits terminal `Failed`. Public-API tests
suppress callbacks for rejected operations and show that a returned work error
does not mutate peer records or prevent subsequent peer work. A complete
two-application lifecycle integration test now verifies RFF-REQ-002. The
runtime has no automatic dispatch, general service context, or sample mission.
Its ordinary lifecycle/work operations emit no events; the opt-in returned-work
operations below provide failure reporting.

`MessageBus<Topic, MAX_PAYLOAD_BYTES>` remains a standalone available-endpoint
routing core. It copies a caller-supplied contiguous-prefix application
topology, reserves each positive-capacity inbox and unique topic set, routes in
registration order, preserves FIFO across topics, rejects the newest delivery
at saturation, and reports every matching destination in order. Inline payload
storage enforces the selected maximum for each bus type.

`MessagingRuntime` consumes a fully composed runtime whose records are still
`Registered`, assigns one fresh inbox to each record internally, and freezes
both owners behind one API. Publication
derives availability from runtime state: only `Running` accepts delivery;
`Registered`, `Stopped`, and terminal `Failed` remain known but unavailable.
Successful stop and returned callback errors clear only the selected inbox
before returning the exact discarded-delivery count. Successful restart from
`Stopped` reconnects the already-empty inbox. Public tests preserve the
operation-specific concrete error and leave peer queues and work operable.

Applications implementing `MessagingApplication` can now receive one oldest
message through caller-selected `dispatch_one`. The message remains outside its
inbox only during that synchronous callback. A publish-only
`ApplicationMessageContext` uses lifecycle states refreshed immediately before
each dispatch, permits true self-publication into the freed slot, and exposes no
dequeue, nested dispatch, lifecycle operation, or raw bus access. Success keeps
`Running`. A returned message error commits only the selected application to
terminal `Failed`, drops the attempted in-flight record, clears its remaining
queue exactly, and does not roll back peer deliveries already accepted.

`EventQueue<EventId>` is a separate positive-capacity FIFO for structured
`Event` records. Each record has a framework or application source, one of three
local severities, a mission-defined copied identifier, and an explicit elapsed
`EventTimestamp`. Construction reserves the full record limit. Emission accepts
through that exact limit; saturation retains older records and returns a
must-use `QueueFull` outcome without recursive diagnostics. Dequeue frees one
slot. The queue neither creates timestamps nor attaches to an application or
runtime.

`Clock` separately exposes read-only elapsed `FrameworkInstant` values. The
`ManualClock` implementation starts at zero or one explicit controlled instant,
moves only through checked nonnegative duration advances, and returns a typed
non-mutating error on overflow. `EventTimestamp` can capture this injected
reading, while callers can still construct explicit timestamps.

`Runtime::work_with_failure_event` delegates first to the existing direct work
operation. Success and lifecycle rejection read no clock and attempt no event.
After a returned application error commits only the selected record to
`Failed`, the integration captures one injected reading, constructs one
application-sourced error event with a mission-supplied identifier, and calls
the existing bounded queue once. `RuntimeWorkEventError` retains the complete
work error and an optional `FailureEventAttempt` containing the exact event and
`Recorded` or `QueueFull` result. Saturation preserves older records and returns
the rejected event for explicit caller handling; it neither retries nor
prevents later peer work once the operation returns. The runtime borrows rather
than permanently owns the clock and queue.

`MessagingRuntime::work_with_failure_event` first delegates through its own
`work` operation. A returned application error commits `Failed` and clears the
selected inbox before event reporting reads the clock. The existing
`MessagingOperationError<RuntimeWorkEventError<...>>` preserves the exact
discard count, original work error, and optional event attempt. A shared private
constructor keeps direct and messaging-owned event fields and saturation
semantics identical; it does not own lifecycle or queue cleanup. Successful
work and lifecycle rejection produce no event or clock read. Configuration
and peer queues retain ordinary messaging-work behavior. The clock and event
queue remain caller-owned, and neither inner runtime nor bus gains mutable
public access.

`WorkSchedule` separately owns a fixed finite sequence of one-shot
`ScheduledWork` items. Construction requires nondecreasing elapsed instants, so
equal-time items retain configuration order. `Runtime::run_next_scheduled_work`
reads an injected clock once while an item remains, returns without mutation
before that instant, and consumes at most one due or overdue item before
delegating to `Runtime::work`. Success includes the resulting lifecycle state;
an error includes the exact lifecycle or application work error. Identical
manual scenarios reproduce work order, lifecycle outcomes, and clock-captured
event timestamps, verifying RFF-REQ-004 at this finite boundary.

This combined routing, lifecycle-availability, and application-dispatch
evidence verifies RFF-REQ-003. The positional configuration limitation remains:
mission composition must associate each capacity and topic set with the intended
registration position. The runtime-event integration plus existing bounded
queue evidence verifies RFF-REQ-005 and RFF-REQ-008 only for direct cooperative
returned work errors. There is still no automatic or batch message dispatch,
periodic or dynamic work generation, multi-application fairness policy,
messaging-aware scheduled work, application-authored event API, other callback
event path, or host event drain adapter.

`ConfigurationTable<E, MAX_BYTES>` separately owns an active immutable byte
snapshot, at most one rollback snapshot, a revision high-water mark, and a
retained mission validation function. Length rejection precedes validation;
validation precedes revision exhaustion; all complete before retained state
changes. Successful replacement assigns a fresh revision and replaces old
history. Rollback consumes the slot and restores its original revision without
revalidation or revision reuse. It performs no heap allocation itself, but
candidate storage, caller-owned copies, and validator effects need their own
resource budgets. A large inline bound is not a stack-usage guarantee.

This core does not select a schema or typed decoded application value.
ADR-0018 adds a private optional table to `Runtime` only at construction.
`Application::work` receives one `ApplicationWorkContext` whose optional view
exposes the active revision and used bytes but not the validator error type,
inline bound, or mutation. Replacement and rollback remain caller-selected
safe-point operations. Direct, scheduled, failure-event, and messaging-owned
work all delegate through `Runtime::work`; message and lifecycle callbacks
remain outside the access boundary. Tests prove absent and configured contexts,
construction ownership recovery, activation/rejection/rollback visibility,
revision non-reuse, restart retention, no automatic error rollback, peer
progress, event behavior, and inbox cleanup. This combined evidence verifies
RFF-REQ-006. The borrowing probes remain language-shape evidence rather than
runtime integration evidence.

The private `examples/host-echo` mission composes two independently defined
message applications through a static enum, with one capacity-one inbox for
each. Its codec rejects host length, identifier, then percentage before
publication; both application boundaries revalidate topic, used payload length,
then value. Only a privately constructed validated percentage reaches the pure
echo function. Ingress returns the original ordered publication report without
dispatch. The echo callback attempts one telemetry publication and returns its
original allocation error or incomplete report on failure.

The telemetry application borrows a host-owned mailbox containing at most one
validated percentage. Host drain returns one two-byte array outside callbacks.
Full output retains the older record, returns a concrete rejected-value error,
and causes terminal selected-app failure with zero remaining queued discards
in the selected one-slot topology. Stop/restart and failure retain host output;
drain cannot recover a failed application. The example has no callback I/O,
automatic retry, additional library surface, or full-service sample integration.

## Alternatives kept open

- One OS thread per application may later help isolate blocking work, but it
  introduces shutdown, joining, scheduling, and queue semantics that need
  explicit evidence first.
- An async runtime may later suit many concurrent I/O sources, but no current
  workload justifies its dependency, executor, cancellation, or timing model.
- Future embedded work belongs in a separate concrete-target fork after the
  host v0.1 architecture review. This repository will not grow speculative
  `no_std`, RTOS, hardware, or shared-core adapters in preparation for it.
- Standard-library bounded channels are a future option, but publish/subscribe
  fan-out and overflow policy still require framework behavior; selecting a
  primitive is not the same as designing the bus. (`SRC-RUST-CHANNEL`)

## Major technical risks

- The pre-v0.1 application surface has separate narrow contexts: ordinary work
  sees optional configuration, while message dispatch sees publication only.
  Lifecycle callbacks remain context-free. The opt-in failure-event methods
  borrow services for one ordinary work operation but provide no general
  application service-context shape. A later shared context could still cause
  API churn.
- A returned application callback error may follow partial application-internal
  cleanup or mutation; the runtime records `Failed` but provides no rollback or
  cleanup guarantee.
- “Deterministic” may be overstated unless every input and ordering source is
  controlled and the claim remains limited to repeatability.
- The event queue bounds only retained records. Rejected or dequeued events,
  future telemetry, and any caller bypass diagnostic path remain outside that
  bound.
- Reject-newest event saturation can omit a later high-severity record. The
  must-use outcome is explicit but does not provide guaranteed delivery.
- Explicit event timestamps and readings from unrelated clock origins can still
  be non-monotonic when combined. The direct failure path captures one supplied
  clock, but explicit standalone events can still use arbitrary timestamps;
  FIFO remains emission order rather than timestamp sorting.
- Failure-event reporting is opt-in and applies only to returned ordinary-work
  errors. Its timestamp is captured after `Failed` is committed, represents
  framework observation rather than an application-internal fault instant, and
  leaves a caller-owned event copy outside the queue's retained-record bound.
  The messaging-owned path also finishes selected-inbox clearing before that
  observation. Message dispatch and scheduled work do not emit failure events.
- A work schedule cannot detect an identity from another same-shaped runtime or
  an instant from another clock origin. It retains consumed one-shot items until
  drop, has no periodic or reconfiguration policy, and currently schedules only
  through `Runtime::work`; messaging-aware scheduling would need to preserve
  inbox clearing through `MessagingRuntime`.
- Direct standalone routing-core construction can still be paired with a
  same-shaped foreign identity issuer or incomplete runtime topology; lifecycle
  guarantees apply only to `MessagingRuntime`.
- Runtime-owned inbox configurations are positional. The integration proves
  identity order and count, but cannot recognize two valid configurations that
  mission composition accidentally swaps.
- Message dispatch copies lifecycle states into preallocated storage before
  every callback. Serial execution and the publish-only context keep that view
  stable, but concurrency would require a different availability design.
- Callback publications are immediate and non-transactional. Peer deliveries
  remain even when the publishing callback subsequently returns an error.
- Mission-defined topic equality and per-publication report allocation are
  bounded by configured route count but are not a general non-blocking claim.
- A returned application error is not equivalent to containing a panic, hang,
  process failure, memory exhaustion, or hardware fault.
- Platform seams created without a second backend may become decorative layers.
- Borrowed cFS terminology may cause compatibility or NASA-affiliation drift.
- Dependencies may silently define concurrency, allocation, or failure policy.
