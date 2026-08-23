# Plan: caller-driven owned application work

Status: **Complete**
Date: **2026-08-23**

## Objective

Add the smallest synchronous caller-driven work boundary to the existing owned
runtime. Only a `Running` application shall receive work. Successful work shall
retain `Running`; a returned concrete work error shall remain available in the
caller-visible result while the selected application enters terminal `Failed`.

Use the public integration evidence to run two independently defined
applications through registration, start, work, stop, restart, and work. This
shall complete the remaining RFF-REQ-002 host scenario without introducing a
sample binary or widening the runtime surface.

## Context

ADR-0001 selects a serial, caller-driven host runtime. ADR-0003 already decides
that only `Running` applications receive work, successful work retains
`Running`, and a returned work error enters terminal `Failed`. The runtime now
owns and executes start, stop, and in-place restart, but it has no work
operation. The roadmap and project state identify this as the next Stage 1
slice and as the concrete dispatch consumer needed before bounded inbox work.

This is a local pre-v0.1 API decision. A focused ADR will compare a direct
`Application::work` callback with caller-supplied work and a premature service
context. No external research is needed because accepted local decisions
already define the observable behavior.

This slice deliberately stops before service contexts, time, scheduling,
messages, events, configuration, threads, async execution, panic handling,
recovery from `Failed`, or automatic work iteration.

## Acceptance criteria

- The application boundary exposes one synchronous work callback and a
  concrete work-error type without changing the start, stop, or restart error
  contracts.
- `Runtime::work` invokes the selected `Running` application exactly once.
  Success leaves the record in `Running` and preserves application-owned state.
- Registered, stopped, and failed targets return a typed state error before
  application code runs and retain their current state. Unknown identities
  also return a typed error without invoking application code or mutating
  records.
- A returned work error remains programmatically available in the exact runtime
  result and commits the selected application to terminal `Failed`.
- One application's returned work error does not prevent a separately defined
  running peer from completing subsequent work and remaining `Running`.
- A public integration test registers two independently defined applications
  and observes both completing start, stop, and restart, with eligible work
  before and after restart. Together with the existing exhaustive LC1 tests,
  this satisfies RFF-REQ-002 without claiming broader sample-mission evidence.
- Runtime identity, length, capacity, application ownership, serial execution,
  and dependency-free construction remain unchanged.
- Documentation describes only cooperative returned-error behavior. RFF-REQ-008
  remains partial until a structured failure event exists.
- The documented baseline and repository consistency checks pass before local
  commits are created.

## Proposed files and components

- `src/runtime.rs`, `src/lib.rs`, and the narrow shared state-error vocabulary
  in `src/lifecycle.rs` for `Application::work`, `Runtime::work`, and the typed
  caller-visible work error.
- `tests/application_runtime.rs` for retained-state success, complete
  two-application lifecycle evidence, non-running and unknown callback
  suppression, exact concrete error/source preservation, and peer progress.
- A focused ADR for direct synchronous work and its operation-specific error.
- Existing contributor, architecture, roadmap, state, requirements, and
  traceability documents for truthful boundary and evidence updates.

## Verification approach

- Run `cargo fmt --all -- --check`.
- Run `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
- Run `cargo test --workspace --all-features`.
- Run `cargo doc --workspace --no-deps`.
- Resolve relative Markdown links and cross-check requirement/source
  identifiers.
- Render changed Markdown to HTML and inspect its document structure; record
  any environment limitation instead of treating it as a pass.
- Run `git diff --check`, inspect the complete diff, and confirm repository
  status before committing.

## Known risks

- Extending the pre-v0.1 `Application` trait is source-breaking for
  implementors, although the package is unpublished and this behavior is
  already selected by ADR-0003.
- A returned work error may follow partial application-internal mutation. The
  runtime records `Failed` but cannot promise rollback, cleanup, isolation, or
  a recoverable state.
- Four operation-specific error wrappers duplicate a small amount of display
  and source plumbing. This slice will reassess that pattern but will not force
  a shared abstraction without clearer evidence.
- Direct `work` has no framework context, deadline, fairness, or automatic
  iteration semantics. Those remain deliberately undefined.
- The runtime record bound still does not constrain application-internal
  allocation, blocking, I/O, thread creation, or error size.
- Runtime identities retain the documented origin-alias risk; this slice does
  not change identity representation.

## Safe rollback or stopping point

Stop after caller-driven work and the complete two-application LC1 integration
scenario are verified and documented. If the API cannot remain coherent
without service contexts or broader execution policy, preserve the decision
analysis but do not commit broken implementation. Do not continue into inboxes,
time, events, scheduling, configuration, concurrency, or dependencies in this
increment.

## Result

The stopping point was reached in implementation commit
`43ef56b92da18302eee0c0a0a1f071a29aae0ade`. The dependency-free runtime now
validates and executes one synchronous caller-selected work operation.
Successful work mutably borrows the retained application value and preserves
`Running`; a returned concrete work error remains available through
`RuntimeWorkError` while only the selected record enters terminal `Failed`.

Two new public tests plus the extended unknown-identity test cover retained
state, registered/stopped/failed/unknown callback suppression, exact returned-
error and source preservation, terminal failure, and successful subsequent peer
work. One test runs separately defined healthy and stateful applications through
registration, start, work, stop, restart, and work. Together with the existing
logical transition matrix, this completes RFF-REQ-002. The suite contains 15
passing tests; RFF-REQ-008 remains partial only because no structured failure
event exists.

ADR-0009 records the direct work callback, narrow `NotRunning` eligibility
error, operation-specific error wrapper, and deferral of service contexts and
automatic dispatch. Stage 1 is complete. Formatting, warnings-denied linting,
tests, rustdoc, all 33 relative links across 19 Markdown files, requirement and
source identifier consistency, Markdown table shapes, rendered-document review,
and Git whitespace checks pass. No external research, dependency, context,
message bus, clock, event service, thread, executor, or push was added.
