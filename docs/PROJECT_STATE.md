# Project state

Last updated: **2026-10-15**

## Current milestone

Stages 1 through 4 and source-quality adoption are complete for experimental
serial host v0.1. The [actual Codex host review](verification/HOST_V0_1_REVIEW.md)
accepts entry points and architecture under ADR-0024 with exact evidence and
limits. Package 0.0.0 remains unpublished; APIs and requirements are provisional.
Current work hardens existing behavior within those boundaries.

## Verified baseline

- Clean input `ecfe954cd7550a933bf08821d9a4154c71023953` matches refreshed
  origin. Fresh exact-input hosted run 38404703855/job 115321155694 passes all
  20 steps, 146 workspace tests, separate 14/five host tests, rustdoc, full
  unchanged sample and both whitespace checks.
- Initial/final locked local format/check/Clippy/test/warnings-denied rustdoc
  and whitespace pass 146/147 workspace tests. Focused model and focused Clippy
  pass; 19,531 traces/112,305 generated operations and 58,593 supplemental probes
  match the reference. Local Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy
  0.1.98 are unchanged.
- Final whole-tree audit covers 39 Rust files/15,333 lines, zero width findings
  and three unchanged reasoned expectations. Independent Codex review accepts
  oracle/source/complete diff. Six selected compiling fault copies fail model
  assertions; clean isolated control passes. All 352 relative links/111 exact
  traceability functions resolve; six changed rendered documents pass complete
  content/HTML structure inspection, with no pixel or human-review claim.
- Hosted runner 2.337.0, windows-2025-vs2026 image 20260925.250.1,
  Rust/Cargo 1.99.0, rustfmt 1.10.0-stable and Clippy 0.1.99 match the reviewed
  baseline. Local checks do not claim execution on that hosted toolchain.

## Current architecture

One unpublished package owns synchronous LC1 lifecycle/work, bounded inbox
routing/dispatch, manual time and one-shot scheduling, bounded events and optional
runtime configuration with immutable work visibility and one-use rollback. The
private shared-source host sample composes these services. Separate finite models
strengthen standalone messaging, successful lifecycle, ordinary-work failure,
returned message-callback failure, messaging-owned one-shot scheduling and
standalone configuration histories.

## Work in progress

The [configuration sequence checkpoint](verification/CONFIGURATION_SEQUENCE_REVIEW.md)
freezes one standalone table, bound two, initial [1], accepted [1]/[2,3], rejected
[0]/oversize [0,0,0] and Rollback through length six. Independent append-only
acceptance/predecessor and movement histories predict exact results, original
revisions, used bytes, predecessor eviction and consume-once rollback. Three
supplemental probes expose retained history and next revision after each trace.
Focused/full local verification, independent source/document/diff review and
selected isolated fault trials pass their defined expectations. No unfinished
implementation remains. Publication and exact-revision hosted CI are pending.
Production, APIs, requirements, dependencies, licence, lint and workflow are
unchanged.

## Highest risks and uncertainties

- Finite models cover only their frozen domains. This configuration model
  excludes arbitrary candidates/validators/bounds, initial rejection, exhaustion
  and runtime service composition. Existing scenarios remain separate evidence.
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

Assess whether generated runtime configuration/work histories expose a meaningful
gap beyond existing integration scenarios before selecting another model or
widening the domain.

## Latest run

2026-10-15: the configuration history model and full locked local baseline pass
147 workspace tests. Independent source/oracle review accepts the frozen scope;
six selected compiling faults fail model assertions. Complete document/diff and
rendered content/structure review pass. Publication and exact-revision hosted CI
remain. Stop after this one bounded checkpoint.
