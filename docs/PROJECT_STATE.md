# Project state

Last updated: **2026-10-04**

## Current milestone

Stages 1 through 3 and the first source-quality checkpoint are complete.
Stage 4 has sample, hosted CI, dependency/scope, resource, failure, schedule,
identity-boundary, and public-diagnostics evidence. This run adds copied
message-topology ownership evidence after caller storage changes and drops.
Broader contract review and human v0.1 architecture acceptance remain open.

## Verified baseline

- Started clean at `565c9d44a0cc18bf9717c64416e5f265b5dda9e2`, equal to
  refreshed `origin/codex/nightly`; hosted run 37019302807 succeeded.
- Initial locked local baseline: 135 tests; final baseline: 137. Formatting,
  all-target check, warnings-denied Clippy/rustdoc, and whitespace pass.
  Focused topology targets pass 10 standalone and 12 runtime tests.
- Local Rust/Cargo: 1.98.0; rustfmt: 1.9.0-stable; Clippy: 0.1.98.
- Hosted Rust/Cargo: 1.99.0; rustfmt: 1.10.0-stable; Clippy: 0.1.99.
  Its required [source-policy re-audit](verification/CI_BASELINE.md) retains
  existing format, width, function-size, and waiver policy.
- Final audit passes 33 Rust files, zero physical/comment-width findings,
  no block comments, three unchanged expectations, and 270 relative links
  across 56 Markdown files. Six changed documents render structurally;
  pixel-level acceptance is not claimed. Complete source/diff review passes.
- Checkpoint `ab9914618a739d4b7c8a46f0e14d7e12c55b6649` is published.
  [Hosted run 37128965033](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/37128965033)
  passes its exact logged checkout and every configured step: 137 workspace
  tests, both focused host targets, and the executed sample.

## Current architecture

One unpublished package owns synchronous LC1 lifecycle/work, bounded inbox
routing/dispatch, manual time and one-shot scheduling, bounded events, and
optional runtime configuration with immutable work visibility and one-use
rollback. The private shared-source host sample composes these services.
This checkpoint adds tests to existing targets and records; production code,
public API, dependencies, lint policy, and workflow remain unchanged.

## Work in progress

The [copied topology review](verification/MESSAGE_TOPOLOGY_OWNERSHIP_REVIEW.md)
records completed observations. No unfinished implementation remains. The
checkpoint is published with successful exact-revision CI. This documentation
follow-up records that result; later revisions require their own verification.

## Highest risks and uncertainties

- Application identity and clock-origin pairing remain caller discipline:
  `ApplicationId` holds a record position; `FrameworkInstant` holds elapsed time.
- Copied `Copy + Eq` topics need not isolate arbitrary shared referents or
  equality dependent on external mutable state. Plain-enum tests cover values.
- Logical capacities do not bound whole-process bytes. Actual allocator
  exhaustion and allocation counts remain unverified.
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

1. Reconcile another Stage 4 message/resource ownership gap, such as inline
   payload and queued-message independence after publisher storage changes.
2. Record human entry-point/architecture acceptance before broadening scope.

## Latest run

2026-10-04: two public regressions retain original topology after caller buffer
mutation and destruction. Standalone evidence consumes exact FIFO; runtime
owner evidence covers availability, saturation, selected clearing, and restart.
No production defect was found. Focused tests and Clippy pass after correcting
one test accessor and shortening the runtime test without a lint exception.
The full locked baseline passes 137 tests, final review passes, and publication
and exact-checkpoint hosted verification succeed. Hosted tool/runner evidence
matches the previous checkpoint; no new policy adaptation is needed.
