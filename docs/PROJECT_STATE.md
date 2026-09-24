# Project state

Last updated: **2026-09-25**

## Current milestone

Stages 1 through 3 and the first source-quality checkpoint are complete.
Stage 4 has sample, hosted CI, dependency/scope, constructor/resource, returned-
message failure, schedule, and identity-boundary evidence. The current
checkpoint executes the remaining clock-origin consequence: an instant from
one manual clock can be released by an unrelated clock with the same elapsed
value through both scheduling APIs. Broader contract review and human v0.1
architecture acceptance remain open.

## Verified baseline

- Started clean at `f5b6ab82f888b45ea122ab1fb84aed2e202187c2`, equal to
  refreshed `origin/codex/nightly`; its hosted run 35915158895 succeeded.
  Initial locked local Cargo baseline: 129 tests.
- Current locked local baseline: 131 tests; focused direct/messaging scheduled-
  work targets: eleven and nine. Formatting, all-target check, warnings-denied
  Clippy/rustdoc, and whitespace checks pass without new exceptions.
- Local Rust/Cargo: 1.98.0; rustfmt: 1.9.0-stable; Clippy: 0.1.98.
- Final audit: 32 Rust files with zero width findings, no block comments, and
  three unchanged fulfilled expectations; 48 Markdown files, 216 resolving
  relative links, 35 source definitions, and 95 exact test references.
- Generated HTML for all Markdown files passes heading, table, link, and source-
  content checks. The six changed documents preserve exact content and have no
  horizontal overflow; the new review's layout has no visible defect.
- Complete source and diff review found no production mismatch. The
  [clock-origin review](verification/CLOCK_IDENTITY_SCOPE_REVIEW.md)
  distinguishes executed elapsed-value comparison from permission or evidence
  for cross-clock ordering.
- Publication and exact-revision hosted CI evidence for this checkpoint remain
  pending. The prior exact-revision hosted baseline remains the last published
  evidence.

## Current architecture

One unpublished package owns synchronous LC1 lifecycle/work, bounded inbox
routing/dispatch, manual time and one-shot scheduling, bounded events, and
optional runtime configuration with immutable work visibility and one-use
rollback. The private shared-source host sample composes these services.
This increment adds two public scheduling regressions and supporting records
only; production code and public API remain unchanged.

## Work in progress

Local implementation, focused tests, required Cargo baseline, complete diff,
source-form, and rendered-document reviews pass. Publication and exact-
revision hosted verification must complete before this checkpoint is final.

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

2026-09-25: eleven direct-schedule, nine messaging-schedule, and 131 workspace
tests pass. The two new regressions execute equal elapsed values from unrelated
manual clocks through both scheduling owners while preserving successful
messaging inboxes. No production defect, API change, dependency, external
research, or lint-policy change was needed. Publication remains pending.
