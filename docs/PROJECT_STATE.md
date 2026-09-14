# Project state

Last updated: **2026-09-15**

## Current milestone

Stages 1 through 3 and the first source-quality checkpoint are complete.
The sample, hosted CI, dependency/scope review, and messaging-constructor
checkpoint have recorded evidence. Stage 4 now examines returned-message
failure: exact peer FIFO, concrete error preservation, selected cleanup, and
fresh callback availability. Broader contract review and human v0.1
entry-point/architecture acceptance remain open.

## Verified baseline

- Started clean at `ee63b4a7a7ec6c556bc76536298211fd3ff0d253`, equal to
  refreshed `origin/codex/nightly`. Initial locked Cargo baseline: 117 tests,
  warnings-denied Clippy/rustdoc, rustc/Cargo 1.98.0, rustfmt 1.9.0-stable,
  and Clippy 0.1.98.
- Final required baseline: 118 tests; focused message-dispatch target: six.
  Whole-tree audit: 32 Rust files, zero physical/comment width findings, three
  unchanged lint expectations, 41 Markdown files, 177 resolved relative links,
  and 82 exact traceability test references.
- The [dispatch failure review](verification/MESSAGE_FAILURE_REVIEW.md)
  records the selected boundary and proof limits. Eight changed rendered
  documents match source content and pass browser DOM/layout/screenshot review.
  Independent complete-diff review found no blocking defect.
- The [constructor review](verification/MESSAGING_CONSTRUCTION_REVIEW.md)
  retains its capacity-overflow and returned-owner evidence. The
  [dependency/scope record](verification/DEPENDENCY_SCOPE_REVIEW.md) retains
  the empty external Cargo graph and unchanged approved licences.
- The [CI baseline](verification/CI_BASELINE.md) records hosted provenance;
  earlier hosted passes do not prove later revisions.

## Current architecture

One unpublished package owns synchronous LC1 lifecycle/work, bounded inbox
routing/dispatch, manual time and one-shot scheduling, bounded events, and
optional runtime configuration with immutable work visibility and one-use
rollback. The private shared-source host sample composes these services.
The current increment changes tests and supporting records only.

## Work in progress

The returned-message failure checkpoint passes local checks and complete
source/document review. Source publication and exact hosted CI remain pending.

## Highest risks and uncertainties

- Returned-message failure preserves already accepted peer effects. This is
  immediate publication with selected cleanup, not a transaction or rollback.
- Capacity-overflow tests do not inject allocator exhaustion. Logical retained
  bounds exclude caller-held values, stack, and callback/topic effects.
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

1. Select another bounded Stage 4 contract after reconciling existing evidence,
   such as finite schedule construction or event-queue resource/failure behavior.
2. Record human entry-point/architecture acceptance before broadening scope.

## Latest run

2026-09-15: strengthened full peer FIFO and actual returned-error observations;
added callback-availability evidence after message failure. Six focused and
118 workspace tests pass. Source review found no production contract defect.
ADR-0012's source-order prose is clarified. Local review passes; publication
remains pending, and no human acceptance is inferred.
