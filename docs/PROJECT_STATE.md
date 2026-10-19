# Project state

Last updated: **2026-10-19**

## Current milestone

Stages 1 through 4 and source-quality adoption are complete for experimental
serial host v0.1. The [Codex host review](verification/HOST_V0_1_REVIEW.md)
accepts entry points and architecture under ADR-0024 with recorded evidence and
limits. Package 0.0.0 remains unpublished; APIs and requirements are provisional.
The [2026-10-19 practical-library direction](adr/0025-practical-library-development.md)
makes that milestone a baseline rather than a scope ceiling. Stage 5 targets
downstream use, usable APIs, useful capabilities and integration guidance through
researched, autonomously reviewed increments. Consumer readiness is still to be
demonstrated.

## Verified baseline

- Clean input 1809ce3e1cafd390c017debabfd9933146d70b61 has exact hosted
  run 39130850495/job 117685516290 passing all 20 steps, 151 workspace tests,
  separate 14 adapter/five sample tests, rustdoc, unchanged 222-line sample and
  both whitespace checks. The messaging event history evidence remains in its
  focused verification record.
- Initial/final locked local format/check/Clippy/test/warnings-denied rustdoc and
  both whitespace checks pass 151 workspace tests for this direction checkpoint.
  Local Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy 0.1.98 are unchanged.
  Runtime behavior, APIs, dependencies, licence and workflow are unchanged;
  src/lib.rs changes crate documentation only.
- Whole-tree review covers 43 Rust files/17,104 physical lines/1,138 comment-only
  lines, zero widths and three unchanged fulfilled expectations. All 388 relative
  links/115 exact test references resolve. Ten changed rendered Markdown documents
  and generated crate documentation pass content/structure review. Codex root and
  independent library_direction_review accept the complete direction/source diff.
- The nightly prompt is saved through the supported tool and exactly read back;
  only prompt/updated_at change. Schedule, model, reasoning effort, project,
  ACTIVE state and notification preference remain unchanged.
- Input hosted tools remain Rust/Cargo 1.99.1, rustfmt 1.10.0-stable and
  Clippy 0.1.99, runner 2.337.0/windows-2025-vs2026 image 20261007.264.1.
  Local checks do not claim execution on that hosted toolchain. This checkpoint
  still requires its own published exact-revision hosted result.

## Current architecture

One unpublished package owns synchronous LC1 lifecycle/work, bounded inbox
routing/dispatch, manual time and one-shot scheduling, bounded events and optional
runtime configuration with immutable work visibility and one-use rollback. The
private shared-source host sample composes these services. Separate finite models
strengthen messaging, configuration, lifecycle/work failures, messaging-owned
scheduling, direct configuration histories, standalone/direct event histories,
and messaging-owned inbox/event history composition.

## Work in progress

The messaging event history checkpoint is complete at 1809ce3 with exact-head
hosted run 39130850495/job 117685516290 passing all 20 steps, 151 workspace
tests, separate 14 adapter/five sample tests and unchanged sample output.
The practical-library direction checkpoint reconciles active instructions,
charter, roadmap, architecture and nightly prompt. It changes no runtime behavior
or existing requirement meaning. Initial/final locked baseline passes 151 tests;
ten changed rendered Markdown documents and generated crate documentation pass
content/structure review. Exact saved automation readback matches, preserving all
other settings. Codex root and independent library_direction_review accept the
direction/source/document diff. Publication and exact-checkpoint CI remain.

## Highest risks and uncertainties

- Finite models cover only frozen domains. The new model has one work fault,
  fixed capacities/topic and initially Running states. Other lifecycle/service
  composition retains separate evidence. Cleanup-before-clock and invisible
  full-queue attempt counts include source review.
- Application/clock origins and positional topology pairing are caller duties.
  Copied identifiers may share referents. Logical bounds do not bound process
  bytes, callback allocation or duration.
- Panics/hangs and allocator exhaustion are uncontained/unverified. Queue
  clearing does not prove application/external cleanup. Immediate fan-out is
  nontransactional; accepted peer deliveries survive later failure.
- Saturated events may omit records; explicit caller retry is optional. One-shot
  errors are consumed without retry/rollback. Host output does not prove physical
  delivery. Stable tools/images float; local reviews/conditional probes are outside CI.

## Important unresolved decisions

Major API freeze, significant unsafe code, runtime/code-generation/dependency/
security decisions, platform commitments, identity/licences, tags/releases/crate
publication and deployment retain separate gates. MSRV, broader service contexts,
external I/O, hardware, RTOS and no_std remain undecided. Routine host acceptance
is complete; autonomous Codex review does not establish human review.

## Most likely next task

Assess a representative downstream consumer through the public API and current
getting-started guidance. Rank integration friction, missing capability and
relevant defect risks, then select one executable improvement. The earlier
command-ingest coverage candidate remains available if that evidence supports it.

## Latest run

2026-10-19 user follow-up: practical reusable-library development beyond v0.1 is
authorized. ADR-0025 records the direction and preserves the separate consequential
gates. Active documents and the nightly prompt are reconciled. Initial/final
locked checks pass 151 tests; 388 links/115 exact test references and all ten
changed rendered Markdown documents pass content/structure review. Independent
Codex review and saved automation readback pass; source publication and exact-
checkpoint hosted inspection remain.
