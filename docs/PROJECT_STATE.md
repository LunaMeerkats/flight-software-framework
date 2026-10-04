# Project state

Last updated: **2026-10-05**

## Current milestone

Stages 1 through 3 and the first source-quality checkpoint are complete.
Stage 4 has sample, hosted CI, dependency/scope, resource, failure, schedule,
identity-boundary, public-diagnostics, and copied-topology evidence. This run
verifies inline payload construction and standalone queued-copy ownership.
Broader contract review and human v0.1 architecture acceptance remain open.

## Verified baseline

- Started clean at `4cff66c2ee1f9931bd445b2a077e6028c6766729`, equal to
  refreshed `origin/codex/nightly`; exact hosted run 37129162904 succeeded.
- Initial locked local baseline: 137 tests; final baseline: 139. Formatting,
  all-target check, warnings-denied Clippy/rustdoc, and whitespace pass.
  Focused message-bus target passes 12 tests including two new regressions.
- Local Rust/Cargo: 1.98.0; rustfmt: 1.9.0-stable; Clippy: 0.1.98.
- Last verified hosted Rust/Cargo: 1.99.0; rustfmt: 1.10.0-stable; Clippy: 0.1.99.
  Its [source-policy re-audit](verification/CI_BASELINE.md) retains the existing
  format, width, function-size, and waiver policy.
- Final inventory passes 33 Rust files and 12,545 physical lines with no width
  findings, block comments, or allows; three fulfilled expectations remain.
  All 275 relative links across 57 Markdown files resolve. Six changed
  documents pass HTML structure/content inspection; no pixel-level acceptance
  is claimed. Complete diff and independent source review pass.

## Current architecture

One unpublished package owns synchronous LC1 lifecycle/work, bounded inbox
routing/dispatch, manual time and one-shot scheduling, bounded events, and
optional runtime configuration with immutable work visibility and one-use
rollback. The private shared-source host sample composes these services.
This checkpoint adds tests to an existing target and records; production code,
public API, dependencies, lint policy, and workflow remain unchanged.

## Work in progress

The [inline payload review](verification/MESSAGE_PAYLOAD_OWNERSHIP_REVIEW.md)
records completed observations. Full local/source/document acceptance passes;
publication and exact-revision hosted evidence remain pending.

## Highest risks and uncertainties

- Application identity and clock-origin pairing remain caller discipline:
  `ApplicationId` holds a record position; `FrameworkInstant` holds elapsed time.
- Copied `Copy + Eq` topics need not isolate arbitrary shared referents or
  equality dependent on external mutable state. Plain-enum tests cover values.
- Logical capacities do not bound whole-process bytes. Actual allocator
  exhaustion and allocation counts remain unverified; caller-retained values
  lie outside queue bounds.
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

1. Reconcile another existing Stage 4 ownership gap, such as event-record
   independence after producer binding reuse and queue destruction.
2. Record human entry-point/architecture acceptance before broadening scope.

## Latest run

2026-10-05: two standalone public regressions prove retained constructor bytes
and queued FIFO values after publisher storage reuse and scope exit. No
production defect was found. Focused tests and the final 139-test locked
baseline pass after stable rustfmt reformats one expression; no lint exception
is added. Source/document/diff review passes. Publication and exact-checkpoint
hosted verification are pending.
