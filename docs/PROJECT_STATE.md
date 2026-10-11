# Project state

Last updated: **2026-10-12**

## Current milestone

Stages 1 through 4 and source-quality adoption are complete for experimental
serial host v0.1. The [actual Codex host review](verification/HOST_V0_1_REVIEW.md)
accepts entry points and architecture under ADR-0024 with exact evidence and
limits. Package 0.0.0 remains unpublished; APIs and requirements are provisional.
Current work hardens existing behavior within those boundaries.

## Verified baseline

- Clean input `8705408ee9810470b19c7eef796557b5d2b7ee38` matches refreshed
  origin; exact input hosted run 38054936905 is freshly successful.
- Initial/final locked local checks pass 143/144 workspace tests, formatting,
  all-target check, warnings-denied Clippy/rustdoc and whitespace. The focused
  model passes one test over 39,062 traces and 224,610 generated operations.
- Local Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy 0.1.98 are unchanged.
  Hosted execution and tools remain separate evidence.
- Source audit covers 36 Rust files/13,935 lines, zero widths and three unchanged
  expectations; 336 relative links/108 traceability functions resolve. Rendered
  HTML structure/content and source/oracle review pass; no pixel claim is made.
- Production, API, requirement, dependency, licence, lint and workflow are
  unchanged. Publication and exact-head hosted evidence are pending.

## Current architecture

One unpublished package owns synchronous LC1 lifecycle/work, bounded inbox
routing/dispatch, manual time and one-shot scheduling, bounded events and optional
runtime configuration with immutable work visibility and one-use rollback. The
private shared-source host sample composes these services. Separate finite models
strengthen standalone messaging, successful lifecycle and ordinary-work failure.

## Work in progress

The [failure model](verification/RUNTIME_WORK_FAILURE_SEQUENCE_REVIEW.md) uses
both fault roles, capacities [1,2], one topic and five generated operations through
length six. Independent states, histories and discard watermarks predict exact
concrete errors, terminal unavailability, selected clearing and callback counts.
Final healthy work and passive retained FIFO observations are supplemental.
Implementation, local checks, independent Codex oracle/source review and six
compiling isolated fault detections pass.

precedes publication; hosted execution remains separate pending evidence.

## Highest risks and uncertainties

- Each finite model covers only its frozen domain. This failure model excludes
  events/schedules/configuration, generated dispatch, other callback errors,
  arbitrary topology and fault policy. Failed remains terminal.
- Application/clock origins and positional topology pairing are caller duties.
  Copied identifiers may share referents. Logical bounds do not bound process
  bytes, callback allocation or duration.
- Panics/hangs and allocator exhaustion are uncontained/unverified. Queue
  clearing does not prove application/external cleanup. Immediate fan-out is
  nontransactional; accepted peer deliveries survive later failure.
- Saturated events may omit records; one-shot errors are consumed without
  retry/rollback. Host output does not prove physical delivery. Stable tools/
  images float; local reviews and conditional probes remain outside CI.

## Important unresolved decisions

Major API freeze, significant unsafe code, runtime/code-generation/dependency/
security decisions, platform commitments, identity/licences, tags/releases/crate
publication and deployment retain separate gates. MSRV, broader service contexts,
external I/O, hardware, RTOS and no_std remain undecided. Routine host acceptance
is complete; autonomous Codex review does not establish human review.

## Most likely next task

Assess a separate reduced returned-message-callback failure model, including
in-flight consumption and retained peer publication. Freeze a small domain only
after confirming the gap against existing message-failure scenarios.

## Latest run

2026-10-12: both-role ordinary-work failure model and full locked local checks
pass with 144 tests. A planning count typo and one comment width were corrected
before acceptance. Six isolated fault detections and source/document checks pass;
publication evidence is pending. No production change or additional feature work
is planned in this run.
