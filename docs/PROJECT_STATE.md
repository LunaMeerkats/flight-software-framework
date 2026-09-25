# Project state

Last updated: **2026-09-26**

## Current milestone

Stages 1 through 3 and the first source-quality checkpoint are complete.
Stage 4 has sample, hosted CI, dependency/scope, constructor/resource, returned-
message failure, schedule, and identity-boundary evidence. The current
checkpoint executes configuration ownership across a failed messaging
attachment: the returned runtime retains its active revision, rollback history,
revision high-water, and registered application. Broader contract review and
human v0.1 architecture acceptance remain open.

## Verified baseline

- Started clean at `6a60f381736d2b7c7d069088d494000017ceaa75`, equal to
  refreshed `origin/codex/nightly`; its hosted run 36053484748 succeeded.
  Initial locked local Cargo baseline: 131 tests.
- Final locked local baseline: 132 tests; the focused configuration-runtime
  target passes eleven. Formatting, all-target check, warnings-denied Clippy/
  rustdoc, and whitespace checks pass without new exceptions.
- Local Rust/Cargo: 1.98.0; rustfmt: 1.9.0-stable; Clippy: 0.1.98.
- Final audit: 32 Rust files with zero width findings, no block comments, and
  three unchanged fulfilled expectations; 49 Markdown files, 222 resolving
  relative links, 35 source definitions, and 96 exact test references.
- Generated HTML for all documents passes structural and source-content checks.
  The six changed documents preserve exact content; inspected 1280-pixel
  browser views have no visible layout defect.
- The [configured attachment review](verification/CONFIGURATION_ATTACHMENT_REVIEW.md)
  distinguishes the executed ownership path from allocator exhaustion, general
  recovery, or every constructor failure.
- Published checkpoint `b21b59bff68fe6c16b0d153661865a67a28f8df2` has
  successful [hosted CI](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/36184310315):
  one job, all configured steps, the regression, 132 workspace tests in
  aggregate, both focused host targets, and the sample executable.

## Current architecture

One unpublished package owns synchronous LC1 lifecycle/work, bounded inbox
routing/dispatch, manual time and one-shot scheduling, bounded events, and
optional runtime configuration with immutable work visibility and one-use
rollback. The private shared-source host sample composes these services.
This increment adds one public configuration/messaging composition regression
and supporting records only; production code and public API remain unchanged.

## Work in progress

No unfinished implementation remains. The configured attachment checkpoint is
published with successful exact-revision CI. This documentation-only follow-up
records that result; later revisions require their own verification.

## Highest risks and uncertainties

- `ApplicationId` encodes only a record position. Equal-position keys from
  separate live owners compare equal and can select the receiving owner's
  local record, callback, detached inbox, or scheduled work target. Correct
  issuer pairing remains caller discipline.
- `FrameworkInstant` carries no clock-origin identity. Equal elapsed values
  from unrelated clocks compare equal and can release work; meaningful clock-
  domain pairing remains caller discipline.
- Adding issuer or clock provenance needs bounded origin sources and explicit
  equality, exhaustion, persistence, and public-API decisions; no redesign is
  approved by these evidence checkpoints.
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

2026-09-26: the focused configuration-runtime target passes eleven tests and
the final locked workspace baseline passes 132. The new regression recovers a
configured runtime after failed inbox reservation, then observes active
revision 2, rollback revision 1, and next revision 3. Local source/document
review passes; publication and exact-revision hosted verification succeeded. No
production defect, API change, dependency, external research, or lint-policy
change was needed.
