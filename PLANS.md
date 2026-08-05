# Plan: start-only owned application execution

Status: **Complete**
Date: **2026-08-06**

## Objective

Add the smallest synchronous runtime boundary that owns registered application
values and executes one valid start operation. A successful start shall enter
`Running`; a returned application error shall be preserved in the caller-visible
result while the affected application enters terminal `Failed`.

## Context

The existing `LifecycleRegistry` exhaustively verifies logical LC1 transitions
but owns no application values and cannot enter `Failed` from returned
application behavior. ADR-0001 selects a serial caller-driven runtime, and
ADR-0003 already defines returned errors as the initial failure boundary.

This slice deliberately stops before application work, stop/restart callbacks,
contexts, factories, threads, async execution, panic handling, or framework
services. It compares static generic ownership with type-erased ownership before
committing the first application interface.

## Acceptance criteria

- A finite-capacity runtime owns every successfully registered application and
  exposes opaque identities in registration order.
- The application boundary contains only the start behavior demonstrated by
  this slice and returns a concrete associated error type.
- A valid start invokes the selected application exactly once and commits
  `Registered -> Running` only after success.
- A returned start error remains programmatically available in the exact runtime
  result and commits the selected application to terminal `Failed`.
- An invalid start returns the existing typed lifecycle error without invoking
  application behavior or changing state.
- A two-application public-API test uses independently defined application types
  and proves that one returned start error does not prevent the other from
  starting successfully.
- Runtime capacity exhaustion is explicit and returns ownership of the rejected
  application value.
- Documentation and traceability describe this as partial RFF-REQ-002 and
  partial RFF-REQ-008 evidence, not as fault tolerance or a complete runtime.
- The documented baseline and repository consistency checks pass before a local
  commit is created.

## Proposed files and components

- `src/runtime.rs`, plus narrow shared-type updates in `src/lifecycle.rs` and
  `src/lib.rs`, for the owned `Runtime`, minimal `Application` start boundary,
  and typed runtime errors.
- `tests/application_runtime.rs` for public-boundary success, returned-error,
  invalid-transition, capacity, and peer-progress evidence.
- A focused ADR for static ownership and concrete error preservation.
- Existing architecture, roadmap, state, and traceability documents for
  truthful boundary and evidence updates.

## Verification approach

- Run `cargo fmt --all -- --check`.
- Run `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
- Run `cargo test --workspace --all-features`.
- Run `cargo doc --workspace --no-deps`.
- Resolve relative Markdown links and cross-check requirement/source
  identifiers.
- Run `git diff --check`, inspect the complete diff, and confirm repository
  status before committing.

## Known risks

- A generic runtime stores one concrete application representation; a mission
  that composes distinct application types must provide an explicit enum or
  similar static sum type.
- Reserved runtime storage is bounded by record count but scales with the size
  of that representation and does not bound allocations made internally by an
  application or its returned error.
- The start-only `Application` trait is an early pre-v0.1 API and may require a
  recorded revision when work, stop, restart, or context behavior is proven.
- Runtime and standalone registry identities retain the documented origin-
  alias risk because the opaque key does not encode its issuer.
- Returned errors do not contain panics, hangs, process failure, allocation
  failure inside application code, or hardware faults.

## Safe rollback or stopping point

Stop after owned registration and start behavior is verified and documented.
Do not extend into work dispatch, stop/restart callbacks, event emission, a
message bus, scheduling, configuration, concurrency, or external dependencies in
this increment.

## Result

The stopping point was reached in commit
`1696f65a87f9a06ccb80894d2b07ffa211bdc991`. The dependency-free runtime owns a
finite number of statically composed application values and implements only
synchronous start. Three public runtime tests cover success, concrete returned
errors, terminal `Failed`, peer progress, invalid and unknown callback
suppression, and capacity rejection with application ownership preserved. The
existing six logical lifecycle tests continue to pass, for nine tests total.

ADR-0006 records the static-ownership decision and alternatives. RFF-REQ-002 and
RFF-REQ-008 remain only partially verified; application work, owned stop/restart,
events, and the complete host scenarios remain outside this increment.
