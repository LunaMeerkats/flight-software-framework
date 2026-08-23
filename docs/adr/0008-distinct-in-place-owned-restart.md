# ADR-0008: Distinct synchronous in-place owned restart

- Status: Accepted; owned restart slice implemented
- Date: 2026-08-07
- Scope: Owned application restart callback and returned-error shape

## Context

ADR-0003 defines `Stopped -> Running` as the only valid LC1 restart transition,
requires reuse of the same application object, and sends a returned restart
error to terminal `Failed`. ADR-0006 and ADR-0007 grew the unpublished
application boundary one operation at a time so observed behavior, rather than
a speculative broad trait, determines its shape.

The runtime now needs to decide whether restart invokes a distinct application
callback, invokes `start` again, or constructs a fresh value. This decision does
not define recovery from `Failed`, asynchronous cancellation, cleanup
completion, panic containment, or a general application factory.

## Decision

Extend `Application` with one synchronous `restart` callback and a distinct
associated `RestartError: Error + 'static` type. Retain the existing associated
start and stop error types and expose returned restart failures through a
separate `RuntimeRestartError`. `RestartError` need not be `Send` or `Sync`
because the accepted runtime is serial and creates no threads.

The owned runtime shall:

- validate identity and `Stopped -> Running` before invoking application code;
- mutably borrow the same application value retained through its successful
  start and stop, without constructing or substituting a value;
- invoke the selected application's restart callback once for a valid request;
- commit `Running` only after the callback returns success;
- preserve a returned concrete restart error in the caller-visible result and
  commit the selected record to terminal `Failed`;
- invoke no restart callback for unknown identities or invalid transitions; and
- leave every peer record unchanged, so stopped peers remain restartable.

In-place retention does not promise a pristine, reset, or newly constructed
application. The application defines how its retained internal state changes on
restart. If the callback returns an error after mutation, the runtime does not
attempt rollback, retry, compensation, or replacement.

## Alternatives considered

### Invoke `start` again

Reusing `start` would avoid one callback and associated error type, but it would
make first start and restart share semantics despite restart receiving retained
post-stop state. It would also obscure which operation returned an error.

### Construct a fresh application value

A fresh-incarnation restart could provide a stronger recovery boundary, but it
requires a factory, ownership rules for construction failure, identity-
generation policy, and cleanup semantics. ADR-0003 explicitly selected the
smaller in-place LC1 behavior and reserves fresh incarnation for later review.

### Share one lifecycle error type or wrapper

A common type would reduce small amounts of repeated plumbing but would couple
three application-defined failure domains and churn the already verified start
and stop contracts. Operation-specific types remain explicit until work
dispatch provides enough evidence to reassess the pattern.

### Permit restart from `Running` or `Failed`

Restart from `Running` would silently combine stop and restart failure paths.
Restart from `Failed` would claim recovery without a construction, cleanup, or
containment boundary. Both conflict with the accepted LC1 transition table.

## Evidence

- ADR-0001 selects synchronous caller-driven application operations and typed
  returned outcomes.
- ADR-0003 defines the LC1 restart transition, same-object retention, callback
  suppression, and terminal returned-error policy.
- ADR-0006 and ADR-0007 record direct static ownership and deliberate
  operation-by-operation growth of the pre-v0.1 boundary.
- Public-API tests observe retained application state, exact restart-error
  preservation, state commitment, callback suppression, and an eligible peer
  restart.

This decision is local architecture; no external source prescribes its Rust API
shape.

## Consequences and risks

- Mission applications can distinguish initial start from in-place restart and
  retain a concrete restart error.
- Implementors of the pre-v0.1 `Application` trait must add restart behavior.
- The runtime preserves application-owned state but cannot establish whether
  that state is clean, internally consistent, or safe to resume.
- Four operation-specific runtime error wrappers now duplicate some display and
  source plumbing. ADR-0009 reassesses and retains that explicit public shape;
  later evidence may still justify a shared internal or public type.
- A returned error contains only cooperative failure. Panics, hangs, process
  termination, application-created threads, and external resource cleanup are
  outside this boundary.

## Revisit conditions

Revisit when a service context or mission exposes a clearer common error
structure, recovery from `Failed` needs a fresh incarnation, cleanup requires a
separate protocol, or concurrency introduces cancellation and forced-
termination semantics.
