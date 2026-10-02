# Project state

Last updated: **2026-10-03**

## Current milestone

Stages 1 through 3 and the first source-quality checkpoint are complete.
Stage 4 has sample, hosted CI, dependency/scope, constructor/resource, failure,
schedule, identity-boundary, and public-diagnostics evidence. This run adds a
stop-specific full-inbox failure and retained peer-dispatch checkpoint. Broader
contract review and human v0.1 architecture acceptance remain open.

## Verified baseline

- Started clean at `2b138e11415695129e6162954600911f71ec2eaf`, equal to
  refreshed `origin/codex/nightly`. Hosted run 36770717562 succeeded.
- The initial locked local baseline passes 134 tests; the final baseline passes
  135. The focused stop-failure target passes one test. Formatting, all-target
  checking, warnings-denied Clippy/rustdoc, and whitespace checks pass.
- Local Rust/Cargo: 1.98.0; rustfmt: 1.9.0-stable; Clippy: 0.1.98.
- Whole-tree review: 33 Rust files, zero physical/comment-width findings, no
  block comments, three unchanged fulfilled expectations; 55 Markdown files
  and 261 resolving relative links. Six changed documents render structurally
  in PowerShell; pixel-level visual acceptance is not claimed.
- The [returned-stop failure review](verification/STOP_FAILURE_REVIEW.md)
  records completed local observations and their evidence limits. Publication
  and exact-revision hosted verification remain pending.

## Current architecture

One unpublished package owns synchronous LC1 lifecycle/work, bounded inbox
routing/dispatch, manual time and one-shot scheduling, bounded events, and
optional runtime configuration with immutable work visibility and one-use
rollback. The private shared-source host sample composes these services.
This checkpoint adds a stop-specific public integration test and records;
production code, public API, and dependencies remain unchanged.

## Work in progress

The selected regression passes local verification. Publication and hosted CI
are the remaining steps; no current-run hosted result is claimed yet.

## Highest risks and uncertainties

- `ApplicationId` encodes only a record position. Equal-position foreign keys
  select the receiving owner's corresponding local record; issuer pairing
  remains caller discipline.
- `FrameworkInstant` carries no clock-origin identity. Meaningful clock-domain
  pairing remains caller discipline.
- Logical capacities do not bound whole-process bytes. Actual allocator
  exhaustion and allocation counts remain unverified.
- Callback/clock panics and hangs remain outside containment. Stop queue
  clearing does not prove application-internal or external resource cleanup.
  Failed records cannot recover under LC1.
- Scheduled errors are finally consumed without automatic retry, events, or
  rollback. Host output saturation is terminal.
- Stable Rust and runner images float. Source/document and conditional ADR
  checks remain outside CI.

## Important unresolved decisions

Human v0.1 acceptance remains pending, including whether caller-scoped
application and clock identity are sufficient before API stabilization. APIs
and local grammar are unfrozen. MSRV, message/lifecycle configuration access,
broader events, external I/O, hardware, RTOS, and no_std remain open; no scope
expansion is approved here.

## Most likely next tasks

1. Reconcile another bounded Stage 4 service/resource/failure gap against
   existing evidence before selecting more work.
2. Record human entry-point/architecture acceptance before broadening scope.

## Latest run

2026-10-03: the new public regression proves exact full selected-inbox cleanup,
terminal callback gates, retained peer FIFO dispatch, later publication,
peer work, and the concrete stop-error source chain. Focused and full local
verification pass; no production defect, API, dependency, or policy change was
needed. Exact publication and hosted evidence remain pending.
