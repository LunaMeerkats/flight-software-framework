# Plan: synchronous in-place owned application restart

Status: **In progress**
Date: **2026-08-07**

## Objective

Add the smallest synchronous in-place restart boundary to the existing runtime.
A successful callback shall reuse the stopped application value and commit
`Stopped -> Running`; a returned application error shall remain available in
the exact caller-visible result while the selected application enters terminal
`Failed`.

## Context

The runtime already owns finite-capacity application records and implements
start and stop. ADR-0003 defines in-place restart as the final successful LC1
transition and requires a returned restart error to enter `Failed`. The roadmap
and project state both identify this as the next Stage 1 slice.

The application interface needs one deliberate pre-v0.1 extension. ADR-0008
records the comparison between a distinct restart callback, reusing `start`, and
a construction boundary. No external research is needed because this is a local
API and failure-policy decision under the accepted ADRs.

This slice deliberately stops before application work, service contexts, event
emission, inboxes, factories, threads, async execution, panic handling,
recovery from `Failed`, or cleanup guarantees.

## Acceptance criteria

- The application boundary exposes one synchronous restart callback and a
  concrete restart-error type without changing the start- or stop-error
  contracts.
- A valid restart invokes the selected stopped application exactly once and
  commits `Stopped -> Running` only after callback success.
- A restart callback observes state retained in the same owned application
  value across its preceding start and stop callbacks; no replacement or
  factory path is introduced.
- A returned restart error remains programmatically available in the exact
  runtime result and commits the selected application to terminal `Failed`.
- Registered, running, and failed targets return typed lifecycle errors before
  any restart callback and retain their current state. Unknown identities also
  return a typed error without invoking application code or mutating records.
- One application's returned restart error does not prevent a stopped peer
  application from completing a valid restart.
- Runtime identity, length, capacity, and application ownership remain stable.
- Documentation describes only cooperative returned-error behavior and keeps
  RFF-REQ-002 and RFF-REQ-008 partial.
- The documented baseline and repository consistency checks pass before local
  commits are created.

## Proposed files and components

- `src/runtime.rs`, `src/lib.rs`, and the narrow shared-vocabulary documentation
  in `src/lifecycle.rs` for `Application::restart`, its associated concrete
  error, `Runtime::restart`, and the typed caller-visible restart error.
- `tests/application_runtime.rs` for retained-state success, returned-error,
  invalid/unknown callback suppression, concrete error-source, and peer-
  progress evidence.
- A focused ADR for the distinct in-place restart callback and operation-
  specific error shape.
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

- Extending the pre-v0.1 `Application` trait is another source-breaking API
  change for implementors, although the package is unpublished and this
  behavior is the next accepted lifecycle responsibility.
- A returned restart error may follow partial application-internal mutation.
  The runtime can honestly mark the record `Failed`, but cannot promise
  rollback, reinitialisation, cleanup, or resource containment.
- Separate start, stop, and restart error wrappers duplicate a small amount of
  code. A shared abstraction is deferred until application work supplies more
  evidence about whether it is stable and clearer.
- The runtime record bound still does not constrain application-internal
  allocation, blocking, I/O, thread creation, or error size.
- Runtime identities retain the documented origin-alias risk; this slice does
  not change identity representation.

## Safe rollback or stopping point

Stop after owned in-place restart behavior is verified and documented. If the
API cannot be made coherent without widening scope, preserve the decision
analysis and tests but do not commit broken implementation. Do not continue
into work dispatch, framework services, scheduling, concurrency, factories, or
dependencies in this increment.
