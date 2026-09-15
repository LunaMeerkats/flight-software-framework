# Project state

Last updated: **2026-09-16**

## Current milestone

Stages 1 through 3 and the first source-quality checkpoint are complete.
Stage 4 has recorded sample, hosted CI, dependency/scope, messaging-constructor,
and returned-message failure evidence. The current checkpoint strengthens
schedule construction diagnostics and copied-agenda ownership. Broader contract
review and human v0.1 entry-point/architecture acceptance remain open.

## Verified baseline

- Started clean at `75f7cbdc659d73d2430b9987bfe64b1afa62ff35`, equal to
  refreshed `origin/codex/nightly`; its hosted run 34901623549 succeeded.
  Initial locked local Cargo baseline: 118 tests.
- Final locked local baseline: 120 tests; focused schedule target: nine.
  Formatting, all-target check, warnings-denied Clippy/rustdoc pass on local
  Rust/Cargo 1.98.0, rustfmt 1.9.0-stable, and Clippy 0.1.98.
- The [schedule review](verification/SCHEDULE_CONSTRUCTION_REVIEW.md) separates
  exact public observations from source-inspected resource/failure properties.
  Independent complete-diff review found no blocking defect. The audit passes
  42 Markdown files, 181 relative links, 84 exact test references, and 32 Rust
  files with zero width findings and three unchanged lint expectations.
  Five changed rendered documents pass content/DOM/layout review; screenshot
  capture timed out twice, leaving pixel inspection unavailable.
- The [dispatch review](verification/MESSAGE_FAILURE_REVIEW.md),
  [constructor review](verification/MESSAGING_CONSTRUCTION_REVIEW.md), and
  [dependency/scope record](verification/DEPENDENCY_SCOPE_REVIEW.md) retain
  their earlier bounded evidence. The [CI baseline](verification/CI_BASELINE.md)
  records hosted provenance; earlier passes do not prove later revisions.
- Published checkpoint `1b7d8281f9a6f987ce8608ad51bcf375837acbbd` has
  successful [hosted CI](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/35017990579):
  one job/all 20 steps, 120 workspace tests, both focused host targets, and the
  sample. The schedule review records the actual hosted environment.

## Current architecture

One unpublished package owns synchronous LC1 lifecycle/work, bounded inbox
routing/dispatch, manual time and one-shot scheduling, bounded events, and
optional runtime configuration with immutable work visibility and one-use
rollback. The private shared-source host sample composes these services.
This increment changes two schedule tests and supporting records only.

## Work in progress

No unfinished implementation remains. The schedule-construction checkpoint
is published with successful exact-revision CI. This documentation-only
follow-up records that result; later revisions need their own verification.

## Highest risks and uncertainties

- Schedule allocation-error execution remains unverified. The configured item
  count bounds logical retained storage, not allocator bytes or callback data.
- Runtime and clock origins remain caller-scoped; construction cannot detect
  mismatched origins. Consumed schedule items retain storage until drop.
- Callback/clock panics and hangs remain outside containment. Host saturation
  is terminal; no recovery, real-time, or physical delivery claim is made.
- Stable Rust and runner images float. Source/document and conditional ADR
  probe checks remain outside CI. The empty Cargo graph does not audit the
  host/toolchain/CI supply chain.

## Important unresolved decisions

Human v0.1 acceptance remains pending. APIs and local grammar are unfrozen.
MSRV, message/lifecycle configuration access, broader events, external I/O,
hardware, RTOS, and no_std remain open; no scope expansion is approved here.

## Most likely next tasks

1. Select the event queue's construction/saturation/reuse contract after
   reconciling existing tests, or another bounded Stage 4 gap with stronger need.
2. Record human entry-point/architecture acceptance before broadening scope.

## Latest run

2026-09-16: exact first-descent diagnostics and independent copied-agenda
ownership pass nine focused and 120 workspace tests. No production defect or
API change was needed. Source/document review passes with the recorded
screenshot limitation. The published checkpoint's exact hosted CI passes;
human v0.1 acceptance remains open.
