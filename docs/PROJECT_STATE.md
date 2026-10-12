# Project state

Last updated: **2026-10-13**

## Current milestone

Stages 1 through 4 and source-quality adoption are complete for experimental
serial host v0.1. The [actual Codex host review](verification/HOST_V0_1_REVIEW.md)
accepts entry points and architecture under ADR-0024 with exact evidence and
limits. Package 0.0.0 remains unpublished; APIs and requirements are provisional.
Current work hardens existing behavior within those boundaries.

## Verified baseline

- Clean input `d672b42bb3eb1dd77f7b907ba6a6f4a8a7e7b4db` matches refreshed
  origin; exact input hosted run 38142393905 is freshly successful.
- Initial/final locked local checks pass 144/145 workspace tests, formatting,
  all-target check, warnings-denied Clippy/rustdoc and whitespace. The focused
  message-failure model passes one test over 2,186 traces/12,030 operations.
- Local Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy 0.1.98 are unchanged.
  Hosted execution and tools remain separate evidence.
- Source audit covers 37 Rust files/14,480 lines, zero widths and three unchanged
  expectations. Independent Codex source/oracle review accepts the finite model.
  Six compiling isolated source faults fail behavioral assertions. All 343
  relative links/109 traceability functions resolve; complete diff and all
  seven run-changed rendered documents pass structure/content review, with no
  pixel claim.
- Production, APIs, requirements, dependencies, licence, lint and workflow are
  unchanged. Published checkpoint `fe40d592eb434a0730eac8c14744361e9d685200` has
  [successful hosted CI](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/38253819562):
  exact logged checkout, all 20 steps, 145 workspace tests, separate 14/five host
  tests, rustdoc, full sample and whitespace. Runner/image/tools remain
  2.337.0/20260925.250.1, Rust/Cargo 1.99.0, rustfmt 1.10.0-stable, Clippy 0.1.99.

## Current architecture

One unpublished package owns synchronous LC1 lifecycle/work, bounded inbox
routing/dispatch, manual time and one-shot scheduling, bounded events and optional
runtime configuration with immutable work visibility and one-use rollback. The
private shared-source host sample composes these services. Separate finite models
strengthen standalone messaging, successful lifecycle, ordinary-work failure and
returned message-callback failure.

## Work in progress

The [message-failure model](verification/RUNTIME_MESSAGE_FAILURE_SEQUENCE_REVIEW.md)
uses both fault roles, capacities [1,2], one topic and three generated operations
through length six. Independent states, accepted histories/cursors, callback
receipts and publication observations predict oldest input, freed-slot self-
publication, exact selected discards excluding in-flight, preserved peer replies
and terminal rejection. Final passive peer drains observe retained FIFO without
introducing failure into an unfailed prefix. Implementation, focused/full local
checks, independent source review, selected sensitivity trials and complete
document/diff review pass. Publication and exact-checkpoint hosted CI pass.
No unfinished implementation remains. This evidence follow-up receives repeated
document/diff/render review and needs its own exact-revision CI.

## Highest risks and uncertainties

- Each finite model covers only its frozen domain. This model excludes lifecycle
  operations, arbitrary callback policies, events/schedules/configuration, other
  errors, domains and longer histories. Failed remains terminal.
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

Assess a separate reduced one-shot schedule-consumption/returned-work-failure
model against existing scenario tests. Freeze a small timing and operation domain
only after confirming a meaningful composition gap.

## Latest run

2026-10-13: both-role returned message-callback failure model and full locked
local checks pass with 145 workspace tests. Source/oracle review, width audit
and six isolated compiling fault detections pass. Complete document/diff and
rendered structure/content review pass. Checkpoint publication and exact-checkpoint
hosted CI pass; all seven run-changed documents are included in final follow-up
review. Finite-domain limits and the likely next task are recorded above.
