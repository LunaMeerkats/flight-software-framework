# Project state

Last updated: **2026-09-29**

## Current milestone

Stages 1 through 3 and the first source-quality checkpoint are complete.
Stage 4 has sample, hosted CI, dependency/scope, constructor/resource, failure,
schedule, identity-boundary, and public-diagnostics evidence. The current
checkpoint executes the messaging attachment constructor's nested standard
error-source chain. Broader contract review and human v0.1 architecture
acceptance remain open.

## Verified baseline

- Started clean at `f4ffabbef7787b3bbbdede28f1e4bcf50a044dcb`, equal
  to `origin/codex/nightly`; its hosted run 36347110418 succeeded. Initial and
  final locked local Cargo baselines: 134 tests.
- The focused runtime-messaging target passes 11 tests. Formatting, all-target
  check, warnings-denied Clippy/rustdoc, and whitespace checks pass without a
  new exception.
- Local Rust/Cargo: 1.98.0; rustfmt: 1.9.0-stable; Clippy: 0.1.98.
- Final audit: 32 Rust files with zero physical or comment-width findings, no
  block comments, and three unchanged fulfilled expectations; 52 Markdown
  files and 244 resolving relative links; 35 source definitions and 104 exact
  traceability test-name references under the current audit method.
- PowerShell Markdown rendering for the six changed controlling documents
  passes structural inspection. Pixel-level visual acceptance is not claimed.
- The
  [messaging-construction diagnostics review](verification/MESSAGING_CONSTRUCTION_DIAGNOSTICS_REVIEW.md)
  distinguishes standard diagnostic traversal from allocator exhaustion,
  every constructor failure, automatic recovery, or API stabilization.
- Published checkpoint `70beaedd8c315baeea3e399bb6a7cbf0a8ded9a9` has
  successful [hosted CI](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/36477080053):
  one job, every configured step, the strengthened regression, 134 workspace
  tests in aggregate, both focused host targets, and the sample executable.

## Current architecture

One unpublished package owns synchronous LC1 lifecycle/work, bounded inbox
routing/dispatch, manual time and one-shot scheduling, bounded events, and
optional runtime configuration with immutable work visibility and one-use
rollback. The private shared-source host sample composes these services.
This increment strengthens one public messaging-construction regression and
supporting records only; production code and public API remain unchanged.

## Work in progress

No unfinished implementation remains. The messaging-construction diagnostics
checkpoint is published with successful exact-revision CI. This documentation-
only follow-up records that result; later revisions require their own
verification.

## Highest risks and uncertainties

- `ApplicationId` encodes only a record position. Equal-position keys from
  separate live owners compare equal and can select the receiving owner's local
  record, callback, detached inbox, scheduled work target, or failure-event
  attribution. Correct issuer pairing remains caller discipline.
- `FrameworkInstant` carries no clock-origin identity. Equal elapsed values
  from unrelated clocks compare equal and can release work; meaningful clock-
  domain pairing remains caller discipline.
- Constructor source traversal preserves the exact typed cause but provides no
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

2026-09-29: the existing later-inbox capacity-overflow regression now proves
`MessagingRuntimeCreateError -> MessagingRuntimeCreateErrorKind ->
MessageBusCreateError` through `std::error::Error::source`, with exact agreement
to typed access. Returned-runtime ownership, corrected attachment, preserved
application behavior, peer publication, and peer work remain passing. The
focused target and full 134-test locked local baseline pass, as do source and
document audits. No production defect, API change, dependency, external
research, or lint-policy change was needed. Publication and exact-revision
hosted verification succeeded.
