# Project state

Last updated: **2026-09-14**

## Current milestone

Stages 1 through 3 and the first source-quality checkpoint are complete.
The sample, hosted CI, and dependency/scope review have recorded evidence.
Stage 4 now has a bounded messaging-constructor resource review with two new
capacity-overflow and ownership-return regressions. Broader resource/failure/
public-API review and human v0.1 entry-point/architecture acceptance remain open.

## Verified baseline

- Started clean at `553fb50832a2aa4948944d051575cc5fb704ea26`, equal to
  refreshed `origin/codex/nightly`. Initial locked Cargo baseline: 115 tests,
  warnings-denied Clippy/rustdoc, rustc/Cargo 1.98.0, rustfmt 1.9.0-stable,
  and Clippy 0.1.98.
- New focused messaging tests pass eight standalone and 11 owner tests.
  The [constructor review](verification/MESSAGING_CONSTRUCTION_REVIEW.md)
  separates capacity overflow from allocator exhaustion and source review.
- Final required baseline passes 117 tests. Source widths, 170 relative links,
  81 exact test references, rendered content/DOM/layout, and independent diff
  review pass. Browser screenshot capture timed out; pixel inspection was
  unavailable. The plan records that rendered-review limitation.
- The [dependency/scope record](verification/DEPENDENCY_SCOPE_REVIEW.md)
  retains the empty external Cargo graph and unchanged approved licences.
  The [CI baseline](verification/CI_BASELINE.md) records hosted provenance;
  earlier hosted passes do not prove this run's changes.

## Current architecture

One unpublished package owns synchronous LC1 lifecycle/work, bounded inbox
routing/dispatch, manual time and one-shot scheduling, bounded events, and
optional runtime configuration with immutable work visibility and one-use
rollback. The private shared-source host sample composes these services.
Runtime source, public APIs, dependencies, workflow, and lint policy are
unchanged by this test/review increment.

## Work in progress

The constructor checkpoint is locally verified and independently reviewed.
Publication and exact-revision hosted CI remain to be completed this run.

## Highest risks and uncertainties

- Capacity-overflow tests do not inject allocator exhaustion. Endpoint, topic,
  dispatch-state, and publication-report reservation failures remain covered
  here by source inspection only.
- Logical retained bounds do not bound the process, caller-held values, stack,
  or callback/topic effects. IDs, revisions, and instants remain caller-scoped.
- Callback/clock panics and hangs remain outside containment. Host saturation
  is terminal; no recovery, real-time, or physical delivery claim is made.
- Stable Rust and runner images float. Source/document and conditional ADR
  probe checks remain outside CI. An empty Cargo graph does not audit the
  host/toolchain/CI supply chain.

## Important unresolved decisions

Human v0.1 acceptance remains pending. APIs and local grammar are unfrozen.
MSRV, message/lifecycle configuration access, broader events, external I/O,
hardware, RTOS, and no_std remain open; no scope expansion is approved here.

## Most likely next tasks

1. Select one remaining lifecycle/dispatch interaction or another service's
   resource/failure/public-API boundary for the Stage 4 audit.
2. Record human entry-point/architecture acceptance before broadening scope.

## Latest run

2026-09-14: no constructor contract defect found. Added deterministic
later-inbox overflow tests, including returned-runtime attachment with corrected
capacities and preserved application behavior. The review records untouched
allocation-error branches and whole-process-bound limits. Local checks and
review pass; publication is pending. No human acceptance is inferred.
