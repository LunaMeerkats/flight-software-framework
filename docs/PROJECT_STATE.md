# Project state

Last updated: **2026-10-14**

## Current milestone

Stages 1 through 4 and source-quality adoption are complete for experimental
serial host v0.1. The [actual Codex host review](verification/HOST_V0_1_REVIEW.md)
accepts entry points and architecture under ADR-0024 with exact evidence and
limits. Package 0.0.0 remains unpublished; APIs and requirements are provisional.
Current work hardens existing behavior within those boundaries.

## Verified baseline

- Clean input `d45d447786f212a17a353771400a844f6d7754c6` matches refreshed
  origin. Fresh exact-input hosted run 38254355148/job 114828459496 passes all
  20 steps, 145 workspace tests, separate 14/five host tests, rustdoc, full
  unchanged sample and both whitespace checks.
- Initial/final locked local format/check/Clippy/test/warnings-denied rustdoc and
  whitespace pass 145/146 workspace tests. The focused model and focused Clippy
  pass; its 19,682 traces/147,624 generated operations match the reference.
  Local Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy 0.1.98 are unchanged.
- Final source review covers 38 Rust files/15,084 lines, zero width findings and
  three unchanged reasoned expectations. Independent Codex reviewers accept
  oracle/source form with no blocking finding. All 349 relative links and
  110 traceability references resolve. Complete diff and seven changed rendered
  documents pass structure/content review, with no pixel/human claim.
- Hosted runner 2.337.0, windows-2025-vs2026 image 20260925.250.1,
  Rust/Cargo 1.99.0, rustfmt 1.10.0-stable and Clippy 0.1.99 match the reviewed
  baseline. Local checks do not claim execution on that hosted toolchain.
- Published checkpoint `0ca95560041f249911eb9d107cedd06f731ea159` has
  [successful hosted CI](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/38403821340):
  exact logged checkout, all 20 steps, 146 workspace tests, separate 14/five
  host tests, rustdoc, full unchanged sample and both whitespace checks.

## Current architecture

One unpublished package owns synchronous LC1 lifecycle/work, bounded inbox
routing/dispatch, manual time and one-shot scheduling, bounded events and optional
runtime configuration with immutable work visibility and one-use rollback. The
private shared-source host sample composes these services. Separate finite models
strengthen standalone messaging, successful lifecycle, ordinary-work failure,
returned message-callback failure and messaging-owned one-shot scheduling.

## Work in progress

The [schedule sequence checkpoint](verification/MESSAGING_SCHEDULE_SEQUENCE_REVIEW.md)
freezes two Running applications, capacities [1,2], both fault positions,
agenda [(0,1s),(1,1s),(0,2s),(1,2s)] and Publish/Advance1s/Attempt sequences
through length eight: 19,682 traces/147,624 generated operations. Independent
time, consumed-item and accepted-payload histories predict clock reads,
waiting/due/completion, exact failure/rejection, selected cleanup and peer
retention. Supplemental healthy work/passive FIFO drains remain separate.
Implementation, focused/full local verification and independent source review
pass. Seven selected isolated faults compile then fail behavioral checks.
Complete document/diff and rendered structure/content review pass. Checkpoint
publication and exact-checkpoint hosted CI pass; no unfinished implementation
remains. This documentation follow-up preserves the source and receives its own
document/diff review and exact-revision CI. Production, APIs, requirements,
dependencies, licence, lint and workflow are unchanged.

## Highest risks and uncertainties

- Finite models cover only their frozen domains. This schedule domain excludes
  generated lifecycle/dispatch, arbitrary agendas/times and event/configuration
  composition. Failed remains terminal; no scheduled event path is added.
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

Assess a reduced configuration replacement/rejection/rollback sequence model
against existing scenario tests. Freeze its domain only after confirming a
meaningful history-composition gap.

## Latest run

2026-10-14: the messaging-owned schedule model and full locked local baseline
pass with 146 workspace tests. Independent oracle/source review accepts the
finite domain; selected compiling fault copies fail behavioral assertions.
Final document/diff and rendered structure/content review pass. Checkpoint
publication and exact-checkpoint hosted CI pass. This evidence follow-up receives
review of all seven run-changed documents and its own exact-revision hosted CI;
later revisions cannot inherit the checkpoint result.
