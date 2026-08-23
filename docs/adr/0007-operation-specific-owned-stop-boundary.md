# ADR-0007: Operation-specific synchronous owned stop boundary

- Status: Accepted; owned stop slice implemented
- Date: 2026-08-07
- Scope: Owned application stop callback and returned-error shape

## Context

ADR-0003 defines `Running -> Stopped` as the only valid LC1 stop transition and
requires a returned stop error to enter terminal `Failed`. ADR-0006 deliberately
recorded the historical first boundary as start-only and required later
operations to earn their shape through evidence. Adding stop now requires
deciding whether to extend that application boundary, introduce a separate
capability trait, or make all lifecycle operations share one error type.

This is a pre-v0.1 source-level API decision for an unpublished package. It does
not define asynchronous cancellation, forced termination, cleanup completion,
panic containment, or restart behavior.

## Decision

Extend `Application` with one synchronous `stop` callback and a distinct
associated `StopError: Error + 'static` type. Retain the existing associated
`StartError` and operation-specific `RuntimeStartError`; expose returned stop
failures through a separate `RuntimeStopError`. `StopError` need not be `Send`
or `Sync` because the accepted runtime is serial and creates no threads.

The owned runtime shall:

- validate identity and `Running -> Stopped` before invoking application code;
- invoke the selected application's stop callback once for a valid request;
- commit `Stopped` only after the callback returns success;
- preserve a returned concrete stop error in the caller-visible result and
  commit the selected record to terminal `Failed`;
- invoke no stop callback for unknown identities or invalid transitions; and
- leave every peer record unchanged, so peers in eligible states remain
  operable.

The runtime does not attempt compensation, retry, forced cleanup, or recovery
after a returned stop error. The application may have changed internal state
before returning the error, so `Failed` records only the cooperative lifecycle
outcome; it is not evidence that resources were cleaned up or isolated.

## Alternatives considered

### Separate stoppable capability trait

A `StoppableApplication: Application` trait would preserve the start-only base
trait, but every owned LC1 application must eventually support stop. Splitting
the immediately required lifecycle into a trait hierarchy adds surface without
a demonstrated runtime that benefits from start-only applications.

### One shared application lifecycle error type

Replacing `StartError` with a common error would couple unrelated operation
failures and break the already verified start contract. Separate associated
types preserve concrete errors without requiring mission code to manufacture a
premature common hierarchy.

### One generic runtime-operation error wrapper

A wrapper carrying a lifecycle operation could reduce small amounts of
repeated code. It would also replace or coexist awkwardly with the existing
public `RuntimeStartError` before restart and work show whether the abstraction
is stable. Operation-specific wrappers remain explicit for this slice.

### Enter `Stopped` or retain `Running` after callback error

`Stopped` would claim success that the callback denied. Retaining `Running`
would claim continued work eligibility despite an unsuccessful stop attempt and
possibly changed application state. Terminal `Failed` matches ADR-0003 and is
the honest bounded outcome.

## Evidence

- ADR-0001 selects synchronous caller-driven application operations and typed
  returned outcomes.
- ADR-0003 defines the LC1 stop transition, callback suppression, and terminal
  returned-error policy.
- ADR-0006 records direct static application ownership and deliberate
  operation-by-operation growth of the pre-v0.1 boundary.
- Public-API tests verify exact stop-error preservation, state commitment,
  callback suppression, and an eligible peer lifecycle operation.

This decision is local architecture; no external source prescribes its Rust API
shape.

## Consequences and risks

- Missions can use distinct concrete start and stop errors while the runtime
  preserves each unchanged.
- Implementors of the pre-v0.1 `Application` trait must add stop behavior.
- Operation-specific error wrappers duplicate a small amount of display and
  source plumbing. ADR-0009 later reassesses and retains the explicit public
  shape after work is implemented.
- A returned error contains only cooperative failure. Panics, hangs, process
  termination, application-created threads, and external resource cleanup are
  outside this boundary.

## Revisit conditions

Revisit when a service context or mission exposes a stable common error
structure, a real mission needs independently composable lifecycle
capabilities, cleanup needs a separate state or protocol, or concurrency
introduces cancellation and forced-termination semantics.
