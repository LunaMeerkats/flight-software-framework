# Project state

Last updated: **2026-09-27**

## Current milestone

Stages 1 through 3 and the first source-quality checkpoint are complete.
Stage 4 has sample, hosted CI, dependency/scope, constructor/resource, returned-
message failure, schedule, and identity-boundary evidence. The current
checkpoint executes caller-scoped application identity through both opt-in
ordinary-work failure-event owners. Broader contract review and human v0.1
architecture acceptance remain open.

## Verified baseline

- Started clean at `3d38bf6bcb5c56ad061561a43ca2d05be639175c`, equal
  to refreshed `origin/codex/nightly`; its hosted run 36184729804 succeeded.
  Initial locked local Cargo baseline: 132 tests.
- Final locked local baseline: 134 tests; the focused direct and messaging-owned
  event targets pass five and seven. Formatting, all-target check, warnings-
  denied Clippy/rustdoc, and whitespace checks pass without new exceptions.
- Local Rust/Cargo: 1.98.0; rustfmt: 1.9.0-stable; Clippy: 0.1.98.
- Final audit: 32 Rust files with zero width findings, no block comments, and
  three unchanged fulfilled expectations; 50 Markdown files, 230 resolving
  relative links, 35 source definitions, and 98 exact test references.
- Generated HTML for all documents passes structural checks. The six changed
  documents preserve exact content. Visual browser layout inspection was not
  run because browser URL policy blocked the local rendered files.
- The [failure-event identity review](verification/FAILURE_EVENT_IDENTITY_SCOPE_REVIEW.md)
  distinguishes executed positional attribution from issuer validation or
  authorization to mix identities.
- Published checkpoint `ce232c6a9d1a49dcf6764e0cd6ac2a1d64331241` has
  successful [hosted CI](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/36276474765):
  one job, all configured steps, both regressions, 134 workspace tests in
  aggregate, both focused host targets, and the sample executable.

## Current architecture

One unpublished package owns synchronous LC1 lifecycle/work, bounded inbox
routing/dispatch, manual time and one-shot scheduling, bounded events, and
optional runtime configuration with immutable work visibility and one-use
rollback. The private shared-source host sample composes these services.
This increment adds two public failure-event identity regressions and supporting
records only; production code and public API remain unchanged.

## Work in progress

No unfinished implementation remains. The failure-event identity checkpoint is
published with successful exact-revision CI. This documentation-only follow-up
records that result; later revisions require their own verification.

## Highest risks and uncertainties

- `ApplicationId` encodes only a record position. Equal-position keys from
  separate live owners compare equal and can select the receiving owner's local
  record, callback, detached inbox, scheduled work target, or failure-event
  attribution. Correct issuer pairing remains caller discipline.
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

2026-09-27: the focused direct and messaging-owned event targets pass five and
seven tests, and the final locked workspace baseline passes 134. Two new
regressions prove that a foreign same-position key fails only the receiving
owner's local application and is copied into its event; the foreign owner stays
running and uninvoked, and messaging cleanup remains receiver-local. Local
source/document review passes except that browser visual layout inspection is
not run due local-file URL policy. Publication and exact-revision hosted
verification succeeded. No production defect, API change, dependency, external
research, or lint-policy change was needed.
