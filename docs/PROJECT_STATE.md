# Project state

Last updated: **2026-10-01**

## Current milestone

Stages 1 through 3 and the first source-quality checkpoint are complete.
Stage 4 has sample, hosted CI, dependency/scope, constructor/resource, failure,
schedule, identity-boundary, and public-diagnostics evidence. The current
checkpoint executes the runtime-owned configuration operations' standard error
source shapes for absence, semantic rejection, and exhausted rollback while
retaining their revision lineage. Broader contract review and human v0.1
architecture acceptance remain open.

## Verified baseline

- Started clean at `dc7c2e67`, equal to `origin/codex/nightly`; its hosted run
  36624544756 succeeded. The initial
  locked local Cargo baseline passes 134 tests.
- The focused configuration-runtime target passes 11 tests. Formatting,
  all-target check, warnings-denied Clippy/rustdoc, and whitespace checks pass
  without a new exception. The final locked Cargo baseline passes 134 tests.
- Local Rust/Cargo: 1.98.0; rustfmt: 1.9.0-stable; Clippy: 0.1.98.
- Final audit: 32 Rust files with zero physical or comment-width findings, no
  block comments, and three unchanged fulfilled expectations; 54 Markdown
  files and 255 resolving relative links.
- PowerShell Markdown rendering for the changed controlling documents passes
  structural inspection. Pixel-level visual acceptance is not claimed.
- The
  [configuration-operation diagnostics review](verification/CONFIGURATION_OPERATION_DIAGNOSTICS_REVIEW.md)
  distinguishes standard diagnostic traversal from automatic recovery,
  logging policy, API stabilization, or broader configuration behavior.
- Published checkpoint `5372a52f888b38b0d725b74bbd330f40ef4a0a31` has
  successful [hosted CI](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/36770444600):
  one job, every configured step, 134 workspace tests, both focused host
  targets, and the sample executable.

## Current architecture

One unpublished package owns synchronous LC1 lifecycle/work, bounded inbox
routing/dispatch, manual time and one-shot scheduling, bounded events, and
optional runtime configuration with immutable work visibility and one-use
rollback. The private shared-source host sample composes these services.
This increment strengthens two public configuration-operation regressions and
supporting records only; production code and public API remain unchanged.

## Work in progress

No unfinished implementation remains. The configuration-operation diagnostics
checkpoint is published with successful exact-revision CI. This
documentation-only follow-up records that result; later revisions require their
own verification.

## Highest risks and uncertainties

- `ApplicationId` encodes only a record position. Equal-position keys from
  separate live owners compare equal and can select the receiving owner's local
  record, callback, detached inbox, scheduled work target, or failure-event
  attribution. Correct issuer pairing remains caller discipline.
- `FrameworkInstant` carries no clock-origin identity. Equal elapsed values
  from unrelated clocks compare equal and can release work; meaningful clock-
  domain pairing remains caller discipline.
- Constructor source traversal preserves exact typed causes but provides no
  automatic recovery. Actual allocator exhaustion and allocation counts remain
  unverified; logical limits do not bound whole-process bytes.
- Scheduled errors are finally consumed. Their source chain preserves exact
  diagnostics, but does not add automatic retry, event reporting, or rollback.
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

2026-10-01: existing public regressions now prove terminal unconfigured
configuration errors, the semantic `RuntimeConfigurationError ->
ConfigurationError -> MissionValidationError` chain, and the structural chain
ending at `NoRollbackAvailable`. Active revision 2, rollback revision 1, and
next replacement revision 3 remain passing. No production defect, API change,
dependency, external research, or lint-policy change was needed. Local
verification, publication, and exact-revision hosted verification succeeded.
