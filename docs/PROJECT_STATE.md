# Project state

Last updated: **2026-10-06**

## Current milestone

Stages 1 through 3 and the first source-quality checkpoint are complete.
Stage 4 has sample, hosted CI, dependency/scope, resource, failure, schedule,
identity, diagnostics, and message ownership evidence. This run verifies event
producer replacement and caller retention after nonempty-queue destruction.
Broader contract review and human v0.1 architecture acceptance remain open.

## Verified baseline

- Started clean at `d5ddb7e0869fe953ff54213d4f5311b21b9124e2`, equal to
  refreshed `origin/codex/nightly`; exact hosted run 37204967826 succeeded.
- Initial locked local baseline: 139 tests; final baseline: 141. Formatting,
  all-target check, warnings-denied Clippy/rustdoc, and whitespace pass.
  Focused event-queue target passes eight tests including two new regressions.
- Local Rust/Cargo: 1.98.0; rustfmt: 1.9.0-stable; Clippy: 0.1.98.
- Previous hosted Rust/Cargo: 1.99.0; rustfmt: 1.10.0-stable; Clippy: 0.1.99.
  The [source-policy re-audit](verification/CI_BASELINE.md) retains the existing
  format, width, function-size, and waiver policy.
- Final inventory passes 33 Rust files and 12,617 physical lines with zero
  width findings and three unchanged fulfilled expectations. All 280 relative
  links across 58 Markdown files and 105 traceability function references resolve.
  Six changed documents pass rendered HTML structure/content inspection;
  no pixel-level acceptance is claimed. Author/independent source/diff review pass.
- Publication and this checkpoint's hosted verification are pending; a local
  pass is not hosted evidence.

## Current architecture

One unpublished package owns synchronous LC1 lifecycle/work, bounded inbox
routing/dispatch, manual time and one-shot scheduling, bounded events, and
optional runtime configuration with immutable work visibility and one-use
rollback. The private shared-source host sample composes these services.
This checkpoint adds tests and records; production code, public API,
dependencies, lint policy, and workflow remain unchanged.

## Work in progress

The [event-record review](verification/EVENT_RECORD_OWNERSHIP_REVIEW.md)
records the observations and their overlap with earlier scheduled replay.
Local implementation, verification, and review are complete. Publication and
exact-revision hosted verification are in progress.

## Highest risks and uncertainties

- Application identity and clock-origin pairing remain caller discipline:
  `ApplicationId` holds a record position; `FrameworkInstant` holds elapsed time.
- Copied topics and event identifiers can retain arbitrary shared referents;
  plain enum ownership tests cover values rather than deep reference isolation.
- Logical capacities do not bound whole-process bytes. Actual allocator
  exhaustion and allocation/deallocation counts remain unverified; consumer
  retention lies outside queue bounds.
- Callback/clock panics and hangs remain outside containment. Stop clearing
  does not prove application-internal/external cleanup; Failed is terminal.
- Scheduled errors are consumed without automatic retry, events, or rollback.
  Host output saturation is terminal.
- Stable Rust and runner images float. Source/document and conditional ADR
  checks remain outside CI.

## Important unresolved decisions

Human v0.1 acceptance remains pending, including caller-scoped application and
clock identity before API stabilization. APIs and local grammar are unfrozen.
MSRV, message/lifecycle configuration access, broader events, external I/O,
hardware, RTOS, and no_std remain open; no scope expansion is approved here.

## Most likely next tasks

1. Reconcile remaining existing Stage 4 contract coverage before selecting
   another bounded gap; inspect immutable work configuration-view lifetime
   evidence if it remains incomplete.
2. Record human entry-point/architecture acceptance before broadening scope.

## Latest run

2026-10-06: two standalone regressions prove exact copied event metadata/FIFO
after producer replacement and a retained record after slot reuse and
nonempty-queue destruction. No production defect was found. Focused tests and
the final 141-test locked baseline pass without a new lint exception.
Source/document/diff review passes; publication and exact-checkpoint hosted
verification are pending. Human v0.1 acceptance remains open.
