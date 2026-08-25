# ADR-0009: Caller-driven synchronous owned work

- Status: Accepted; owned work slice implemented
- Date: 2026-08-23
- Scope: Running-only application work and returned-error shape

## Context

ADR-0001 selects a serial runtime whose host or test harness explicitly asks it
to advance application work. ADR-0003 decides that only `Running` applications
receive work, successful work retains `Running`, and a returned work error
enters terminal `Failed`. The runtime now owns and executes start, stop, and
in-place restart, but it has no operation that exercises application behavior
while running.

Adding that operation requires deciding whether work belongs directly to the
owned `Application` boundary, is supplied as a closure on every call, or waits
for a service context. It also tests whether the existing operation-specific
error wrappers should be replaced by a shared abstraction. This decision is
pre-v0.1 and does not define automatic dispatch, scheduling, fairness, time,
message delivery, cancellation, panic containment, or recovery from `Failed`.

## Decision

Extend `Application` with one synchronous `work` callback and a distinct
associated `WorkError: Error + 'static` type. Retain the start, stop, and restart
error types and expose returned work failures through `RuntimeWorkError`.
`WorkError` need not be `Send` or `Sync` because the accepted runtime is serial
and creates no threads.

The caller selects one application identity for each `Runtime::work` call. The
owned runtime shall:

- validate identity and require `Running` before invoking application code;
- report a typed `NotRunning` lifecycle error for registered, stopped, or failed
  records without invoking work or changing state;
- invoke the selected application's work callback once for a valid request;
- retain `Running` only after the callback returns success;
- preserve a returned concrete work error in the caller-visible result and
  commit the selected record to terminal `Failed`; and
- leave every peer record unchanged, so a running peer remains eligible for
  subsequent caller-selected work.

Work is not added to `LifecycleOperation`. That enum remains the exhaustive LC1
transition vocabulary for start, stop, and restart; successful work is an
operation gated by lifecycle state, not a lifecycle self-transition. The
standalone `LifecycleRegistry` therefore gains no placeholder work method.

Keep the fourth operation-specific runtime error wrapper. All four preserve a
mission-defined concrete source, but their operation names and eligibility
errors remain useful at the public boundary. The small display and source
plumbing duplication is clearer than introducing a generic operation marker or
churning the already verified error types without a demonstrated consumer.

## Alternatives considered

### Add work to the lifecycle transition matrix

Modeling `Running -> Running` as a work transition would reuse
`LifecycleOperation`, but it would make a non-lifecycle callback part of the
standalone logical registry and expand its exhaustive matrix with an operation
the registry cannot execute. A narrow running-state eligibility error preserves
the accepted LC1 model.

### Supply work as a closure on each runtime call

A caller-supplied closure would avoid extending `Application`, but it would let
the behavior associated with one owned application vary at every call site.
Direct application ownership remains the boundary already selected for start,
stop, and restart.

### Add a framework service context now

A context will eventually carry bounded messaging, time, events, and
configuration access. None of those services is implemented yet, so defining
their borrowing and error shape here would be speculative. The unpublished
trait can be revised when the first concrete service must cross it.

### Add batch dispatch or automatic iteration

Iterating all running applications could establish ordering and fairness policy,
but there is no inbox, schedule, or due-work model to select which applications
should run. One explicit caller-selected operation is the smallest observable
boundary and creates the consumer needed for later dispatch evidence.

### Replace all runtime errors with one generic wrapper

A shared wrapper would reduce repeated formatting code, but would need an
operation marker or erase operation-specific public names. Four small explicit
types remain proportionate while the API is pre-v0.1 and each callback has a
different concrete associated error.

## Evidence

- ADR-0001 defines explicit caller-driven serial work and typed outcomes.
- ADR-0003 defines running-only eligibility, success-state retention, and the
  terminal returned-error policy.
- ADR-0006 through ADR-0008 establish direct static ownership and deliberate
  operation-by-operation growth of the unpublished application boundary.
- Public-API tests observe retained state, non-running and unknown callback
  suppression, exact work-error preservation, terminal failure, and successful
  subsequent work by a separately defined peer.
- A two-application integration test observes both applications completing
  registration, start, stop, and restart with work before and after restart.

This is a local architecture decision. No external source prescribes its Rust
API shape, so the source register does not change.

ADR-0012 later reached this decision's first-service revisit condition. It
retains context-free `Application::work` and adds a separate
`MessagingApplication::handle_message` callback with a narrow publish-only
context. This avoids forcing messaging into ordinary caller-selected work or
committing a general service context before time, events, and configuration
exist.

## Consequences and risks

- The runtime can now execute one explicit unit of application work without
  adding hidden work, a scheduler, a context, or a dependency.
- Implementors of the pre-v0.1 `Application` trait must add work behavior and a
  work-error type.
- Application-owned state persists across successful lifecycle and work calls.
  The runtime does not inspect or roll back that state.
- A returned error may follow partial application mutation. `Failed` records the
  cooperative outcome but proves no cleanup, isolation, or recoverability.
- One non-returning or blocking work callback prevents caller progress. Panics,
  hangs, process termination, application-created threads, external I/O, and
  resource exhaustion remain outside this boundary.
- Explicit caller selection defines no stable multi-application dispatch order,
  frequency, deadline, priority, or fairness guarantee.

## Revisit conditions

Revisit the separation when a second framework service crosses the application
boundary, inbox or scheduling evidence requires batch dispatch, a real mission
needs different work capabilities, error handling benefits from a proven shared
shape, or concurrency introduces cancellation and isolation requirements.
