# ADR-0006: Static application ownership for the initial runtime

- Status: Accepted; initial start slice implemented and extended by ADR-0007 and ADR-0008
- Date: 2026-08-06
- Scope: Owned application representation and returned start errors

## Context

The logical lifecycle registry cannot demonstrate application execution or the
returned-error boundary selected by ADR-0001 and ADR-0003. The first owned
runtime must compose independently defined applications without introducing a
dynamic plugin system, hidden allocation policy, concurrency requirement, or a
broad lifecycle interface before those behaviors exist.

The choice is intentionally pre-v0.1 and concerns only the serial host runtime.
It does not define a portable ABI, public plugin mechanism, task model, or
embedded target strategy.

## Decision

The initial owned runtime is generic over one concrete application
representation `A` and stores application values directly in finite-capacity
runtime records. A mission that needs different concrete application types may
define an enum or another explicit static sum type and implement the application
boundary for that type.

The first `Application` boundary contains only a synchronous `start` operation
and one associated concrete error type:

- the runtime owns a value after successful registration;
- a valid start borrows that value mutably exactly for the call;
- success commits the lifecycle record to `Running`;
- a returned error is moved unchanged into a typed runtime error and commits the
  record to terminal `Failed`; and
- lifecycle validation happens before the call, so invalid transitions do not
  invoke application code.

The associated error must implement `std::error::Error` and have no borrowed
data. It is not required to be `Send` or `Sync` because the selected runtime is
serial and creates no threads. The runtime does not catch panics.

Runtime registration failure returns the rejected application value to preserve
caller ownership. Capacity reserves storage for the configured number of
runtime records during construction. This bounds record count, not allocation
inside application values or error values.

## Alternatives considered

### Type-erased boxed applications

`Box<dyn Application<StartError = E>>` would accept unrelated application types
that share one error type; erasing both application and error types would need a
further common boundary. Either form adds per-application allocation and
indirection before dynamic composition is needed. It is deferred until a
concrete mission cannot express its composition statically or measured evidence
favors that tradeoff.

### One generic runtime with caller-supplied start closures

Passing a closure to each runtime start call would avoid an application trait,
but it would let lifecycle behavior vary at the call site instead of belonging
to the owned application representation. That weakens the boundary this slice
is intended to test.

### A broad lifecycle trait now

Defining start, work, stop, restart, context, and recovery methods together
would predict interfaces and error relationships that have no implementation
evidence. Each operation is added only with its own coherent behavior and tests.

## Evidence

- ADR-0001 requires runtime-owned lifecycle, explicit application behavior, and
  typed outcomes in a serial caller-driven model.
- ADR-0003 defines a returned application error as the transition into terminal
  `Failed` and excludes panic, hang, process, and hardware-fault containment.
- Compile-time and public-API tests for this slice verify static composition,
  concrete error preservation, callback count, state commitment, and peer
  progress. No external source prescribes this local ownership choice.

## Consequences and risks

- The runtime has no per-registration trait-object allocation and retains the
  application's concrete error type.
- Static mission composition is explicit, but an enum record is sized to its
  largest variant and all applications in one runtime share each associated
  operation-error type.
- The framework record bound does not constrain application-internal
  allocation, blocking, I/O, thread creation, or error size.
- The original start-only public trait was extended by ADR-0007 for stop and
  ADR-0008 for restart and remains deliberately pre-v0.1. It may change when
  work or service contexts are demonstrated.
- A returned error contains only cooperative failure. It is not fault tolerance
  and does not isolate arbitrary application behavior.

## Revisit conditions

Revisit when a real mission cannot reasonably use static composition, dynamic
loading becomes a requirement, heterogeneous error preservation needs a
different boundary, record-size evidence favors indirection, or concurrency
requires `Send`, `Sync`, cancellation, or isolation semantics.
