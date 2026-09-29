# Project state

Last updated: **2026-09-30**

## Current milestone

Stages 1 through 3 and the first source-quality checkpoint are complete.
Stage 4 has sample, hosted CI, dependency/scope, constructor/resource, failure,
schedule, identity-boundary, and public-diagnostics evidence. The current
checkpoint executes the configured-runtime constructor's standard error source
for both deterministic storage failures while retaining its returned table
lineage. Broader contract review and human v0.1 architecture acceptance remain
open.

## Verified baseline

- Started clean at `9cad622987f47f0ab14e9922a8ab2b4e29b20f6a`, equal
  to `origin/codex/nightly`; its hosted run 36477422468 succeeded. The initial
  locked local Cargo baseline passes 134 tests.
- The focused configuration-runtime target passes 11 tests. Formatting,
  all-target check, warnings-denied Clippy/rustdoc, and whitespace checks pass
  without a new exception. The final locked Cargo baseline passes 134 tests.
- Local Rust/Cargo: 1.98.0; rustfmt: 1.9.0-stable; Clippy: 0.1.98.
- Final audit: 32 Rust files with zero physical or comment-width findings, no
  block comments, and three unchanged fulfilled expectations; 53 Markdown
  files and 249 resolving relative links; 35 source identifiers and 107 exact
  traceability test-name references under the current audit method.
- PowerShell Markdown rendering for the six changed controlling documents
  passes structural inspection. Pixel-level visual acceptance is not claimed.
- The
  [configured-runtime construction diagnostics review](verification/CONFIGURATION_CONSTRUCTION_DIAGNOSTICS_REVIEW.md)
  distinguishes standard diagnostic traversal and explicit ownership recovery
  from allocator exhaustion, automatic recovery, or API stabilization.
- Published checkpoint `887f9965bc8db0939dc161ffc3a58ad7387d2910` has
  successful [hosted CI](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/36624191212):
  one job, every configured step, the strengthened regression, 134 workspace
  tests in aggregate, both focused host targets, and the sample executable.

## Current architecture

One unpublished package owns synchronous LC1 lifecycle/work, bounded inbox
routing/dispatch, manual time and one-shot scheduling, bounded events, and
optional runtime configuration with immutable work visibility and one-use
rollback. The private shared-source host sample composes these services.
This increment strengthens one public configured-construction regression and
supporting records only; production code and public API remain unchanged.

## Work in progress

No unfinished implementation remains. The configured-runtime construction
diagnostics checkpoint is published with successful exact-revision CI. This
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

2026-09-30: the existing configured-construction regression now proves
`RuntimeConfigurationCreateError -> RuntimeCreateError` through
`std::error::Error::source` for zero capacity and deterministic `usize::MAX`
record-capacity overflow, with exact agreement to typed access. Returned table
ownership, active revision 2, rollback revision 1, and next replacement revision
3 remain passing. No production defect, API change, dependency, external
research, or lint-policy change was needed. Local verification is complete;
publication and exact-revision hosted verification succeeded.
