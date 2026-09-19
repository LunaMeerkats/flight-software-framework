# Project state

Last updated: **2026-09-20**

## Current milestone

Stages 1 through 3 and the first source-quality checkpoint are complete.
Stage 4 has sample, hosted CI, dependency/scope, messaging-constructor,
returned-message failure, and schedule-construction evidence. The current
checkpoint verifies event-queue construction and repeated saturation/reuse.
Broader contract review and human v0.1 architecture acceptance remain open.

## Verified baseline

- Started clean at `4bfbda5907b92b3b71cded0422c25dfca8d987a7`, equal to
  refreshed `origin/codex/nightly`; its hosted run 35018337047 succeeded.
  Initial locked local Cargo baseline: 120 tests.
- Final locked baseline: 122 tests; focused event-queue target: six. Formatting,
  all-target check, warnings-denied Clippy/rustdoc, and whitespace checks pass
  without new exceptions. Independent complete-diff review found no blocker.
- Local Rust/Cargo: 1.98.0; rustfmt: 1.9.0-stable; Clippy: 0.1.98.
  Final audit: 32 Rust files with zero width findings and three unchanged
  expectations; 43 Markdown files, 187 resolving links, 86 exact test references.
  Seven changed rendered documents pass content/DOM/layout review. Screenshot
  capture timed out, leaving pixel inspection unavailable.
- The [event review](verification/EVENT_QUEUE_REVIEW.md) distinguishes
  executed capacity-overflow/FIFO evidence from source-inspected allocation
  properties. Earlier [schedule](verification/SCHEDULE_CONSTRUCTION_REVIEW.md),
  [dispatch](verification/MESSAGE_FAILURE_REVIEW.md),
  [messaging construction](verification/MESSAGING_CONSTRUCTION_REVIEW.md), and
  [dependency/scope](verification/DEPENDENCY_SCOPE_REVIEW.md) records retain
  their evidence. The [CI baseline](verification/CI_BASELINE.md) records the
  hosted acceptance policy; earlier passes do not prove later revisions.

## Current architecture

One unpublished package owns synchronous LC1 lifecycle/work, bounded inbox
routing/dispatch, manual time and one-shot scheduling, bounded events, and
optional runtime configuration with immutable work visibility and one-use
rollback. The private shared-source host sample composes these services.
This increment changes two event-queue tests and supporting records only.

## Work in progress

Complete authorized publication and exact-revision hosted verification of the
locally reviewed event-queue checkpoint. No unfinished implementation remains.

## Highest risks and uncertainties

- Impossible-capacity rejection is verified; actual allocator exhaustion and
  allocation counts are not. Logical bounds do not bound whole-process bytes.
- Runtime and clock origins remain caller-scoped. A full event queue can reject
  a later high-severity event; delivery is not guaranteed.
- Callback/clock panics and hangs remain outside containment. Host saturation
  is terminal; no recovery, real-time, or physical delivery claim is made.
- Stable Rust and runner images float. Source/document and conditional ADR
  checks remain outside CI; an empty Cargo graph does not audit the toolchain.

## Important unresolved decisions

Human v0.1 acceptance remains pending. APIs and local grammar are unfrozen.
MSRV, message/lifecycle configuration access, broader events, external I/O,
hardware, RTOS, and no_std remain open; no scope expansion is approved here.

## Most likely next tasks

1. Review lifecycle registration/construction ownership and resource boundaries
   after reconciling existing tests, or select a stronger bounded Stage 4 gap.
2. Record human entry-point/architecture acceptance before broadening scope.

## Latest run

2026-09-20: six focused and 122 workspace tests pass, verifying exact event
reservation-overflow rejection and repeated FIFO preservation/reuse. Review
passes with the recorded screenshot limitation. Publication is pending;
no production defect or API change was needed.
