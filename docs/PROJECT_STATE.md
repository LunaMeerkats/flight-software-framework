# Project state

Last updated: **2026-09-28**

## Current milestone

Stages 1 through 3 and the first source-quality checkpoint are complete.
Stage 4 has sample, hosted CI, dependency/scope, constructor/resource, failure,
schedule, and identity-boundary evidence. The current checkpoint executes the
direct finite schedule's standard diagnostic source chains. Broader contract
review and human v0.1 architecture acceptance remain open.

## Verified baseline

- Started clean at `bb3ec40cac4d72e9ca8b9d38c3a191fa55cdb070`, equal
  to `origin/codex/nightly`; its hosted run 36276641746 succeeded. Initial and
  final locked local Cargo baselines: 134 tests.
- Focused direct and messaging-owned schedule targets pass 11 and nine tests.
  Formatting, all-target check, warnings-denied Clippy/rustdoc, and whitespace
  checks pass without a new exception.
- Local Rust/Cargo: 1.98.0; rustfmt: 1.9.0-stable; Clippy: 0.1.98.
- Final audit: 32 Rust files with zero physical or comment-width findings, no
  block comments, and three unchanged fulfilled expectations; 51 Markdown
  files, 237 resolving relative links, 35 source definitions, and 101 exact
  traceability test references.
- PowerShell Markdown rendering for the five changed controlling documents
  passes structural inspection. Pixel-level visual acceptance is not claimed.
- The
  [scheduled-failure diagnostics review](verification/SCHEDULE_FAILURE_DIAGNOSTICS_REVIEW.md)
  distinguishes standard diagnostic traversal from retry, event reporting,
  arbitrary fault containment, or API stabilization.

## Current architecture

One unpublished package owns synchronous LC1 lifecycle/work, bounded inbox
routing/dispatch, manual time and one-shot scheduling, bounded events, and
optional runtime configuration with immutable work visibility and one-use
rollback. The private shared-source host sample composes these services.
This increment strengthens two direct public scheduled-work regressions and
supporting records only; production code and public API remain unchanged.

## Work in progress

The scheduled-failure diagnostics checkpoint is locally complete. Publication
and exact-revision hosted CI remain pending; a local pass does not establish
either result.

## Highest risks and uncertainties

- `ApplicationId` encodes only a record position. Equal-position keys from
  separate live owners compare equal and can select the receiving owner's local
  record, callback, detached inbox, scheduled work target, or failure-event
  attribution. Correct issuer pairing remains caller discipline.
- `FrameworkInstant` carries no clock-origin identity. Equal elapsed values
  from unrelated clocks compare equal and can release work; meaningful clock-
  domain pairing remains caller discipline.
- Scheduled errors are finally consumed. Their source chain preserves exact
  diagnostics, but does not add automatic retry, event reporting, or rollback.
- Impossible-capacity rejection is verified; actual allocator exhaustion and
  allocation counts are not. Logical limits do not bound whole-process bytes.
- Callback/clock panics and hangs remain outside containment. Host saturation
  is terminal; no recovery, real-time, or physical delivery claim is made.
- Stable Rust and runner images float. Source/document and conditional ADR
  checks remain outside CI; an empty Cargo graph does not audit the toolchain.

## Important unresolved decisions

Human v0.1 acceptance remains pending, including whether caller-scoped
application and clock identity are sufficient before API stabilization. APIs
and local grammar are unfrozen. MSRV, message/lifecycle configuration access,
broader events, external I/O, hardware, RTOS, and no_std remain open; no scope
expansion is approved here.

## Most likely next tasks

1. Select another bounded Stage 4 service, resource, failure, or public-API gap
   after reconciling existing evidence.
2. Record human entry-point/architecture acceptance before broadening scope.

## Latest run

2026-09-28: direct scheduled lifecycle rejection now proves
`ScheduledWorkError -> RuntimeWorkError -> LifecycleError`; direct cooperative
application failure proves the corresponding chain to the concrete application
error. Existing item consumption, terminal-state behavior, and later peer
progress remain unchanged. Focused targets and the full 134-test locked local
baseline pass, as do source/document audits. No production defect, API change,
dependency, external research, or lint-policy change was needed. Publication
and hosted verification remain pending.
