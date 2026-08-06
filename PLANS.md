# Plan: synchronous owned application stop

Status: **Complete**
Date: **2026-08-07**

## Objective

Add the smallest synchronous owned stop boundary to the existing runtime. A
successful callback shall commit `Running -> Stopped`; a returned application
error shall remain available in the exact caller-visible result while the
selected application enters terminal `Failed`.

## Context

The runtime already owns finite-capacity application records and implements
start. ADR-0003 defines stop as the next LC1 transition and requires returned
stop errors to enter `Failed`. The roadmap and project state both identify this
as the next Stage 1 slice.

The application interface needs one deliberate pre-v0.1 extension. ADR-0007
records the comparison between a separate stop capability, a shared lifecycle
error, and the selected operation-specific stop error. No external research is
needed because this is a local API and failure-policy decision under the
accepted ADRs.

This slice deliberately stops before restart, application work, service
contexts, event emission, inboxes, factories, threads, async execution, panic
handling, or cleanup guarantees.

## Acceptance criteria

- The application boundary exposes one synchronous stop callback and a concrete
  stop-error type without changing the existing start-error contract.
- A valid stop invokes the selected application exactly once and commits
  `Running -> Stopped` only after callback success.
- A returned stop error remains programmatically available in the exact runtime
  result and commits the selected application to terminal `Failed`.
- Registered, stopped, and failed targets return typed lifecycle errors before
  any stop callback and retain their current state. Unknown identities also
  return a typed error without invoking application code or mutating records.
- One application's returned stop error does not prevent a peer application
  from completing a valid stop.
- Runtime identity, length, capacity, and application ownership remain stable.
- Documentation describes only cooperative returned-error behavior and keeps
  RFF-REQ-002 and RFF-REQ-008 partial.
- The documented baseline and repository consistency checks pass before local
  commits are created.

## Proposed files and components

- `src/runtime.rs`, `src/lib.rs`, and the narrow shared-vocabulary documentation
  in `src/lifecycle.rs` for `Application::stop`, its associated concrete error,
  `Runtime::stop`, and the typed caller-visible stop error.
- `tests/application_runtime.rs` for success, returned-error, invalid/unknown
  callback suppression, concrete error-source, and peer-progress evidence.
- A focused ADR for the stop callback and operation-specific error shape.
- Existing contributor, architecture, roadmap, state, and traceability
  documents for truthful boundary and evidence updates.

## Verification approach

- Run `cargo fmt --all -- --check`.
- Run `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
- Run `cargo test --workspace --all-features`.
- Run `cargo doc --workspace --no-deps`.
- Resolve relative Markdown links and cross-check requirement/source
  identifiers.
- Review rendered HTML for the changed Markdown documents.
- Run `git diff --check`, inspect the complete diff, and confirm repository
  status before committing.

## Known risks

- Extending the pre-v0.1 `Application` trait is a source-breaking API change for
  implementors, although the package is unpublished and this behavior is the
  next accepted lifecycle responsibility.
- A returned stop error may follow partial application-internal cleanup. The
  runtime can honestly mark the record `Failed`, but cannot promise rollback,
  cleanup, or resource containment.
- Separate start and stop error wrappers duplicate a small amount of code. A
  shared abstraction is deferred until restart or work supplies enough evidence
  to justify one.
- The runtime record bound still does not constrain application-internal
  allocation, blocking, I/O, thread creation, or error size.
- Runtime identities retain the documented origin-alias risk; this slice does
  not change identity representation.

## Safe rollback or stopping point

Stop after owned stop behavior is verified and documented. If the API cannot be
made coherent without widening scope, preserve the decision analysis and tests
but do not commit broken implementation. Do not continue into restart, work
dispatch, framework services, scheduling, concurrency, or dependencies in this
increment.

## Result

The stopping point was reached in commit
`8899cf986ad35ea32b8aff7950a958ca80365d7d`. The dependency-free runtime now
validates and executes synchronous owned stop. Success commits `Stopped`; a
returned concrete stop error remains available through `RuntimeStopError` while
the selected record enters terminal `Failed`.

Five public runtime tests cover start and stop success, exact returned-error
preservation, terminal failure, eligible peer progress, capacity ownership, and
callback suppression for invalid and unknown requests. Together with the
logical lifecycle tests, the suite contains 11 passing tests. ADR-0007 records
the operation-specific error decision and its cleanup limitations.

RFF-REQ-002 and RFF-REQ-008 remain only partially verified. Owned restart,
application work, structured events, and the complete host scenarios remain
outside this increment.
