# Project state

Last updated: **2026-10-17**

## Current milestone

Stages 1 through 4 and source-quality adoption are complete for experimental
serial host v0.1. The [Codex host review](verification/HOST_V0_1_REVIEW.md)
accepts entry points and architecture under ADR-0024 with recorded evidence and
limits. Package 0.0.0 remains unpublished; APIs and requirements are provisional.
Current work hardens existing behavior within those boundaries.

## Verified baseline

- Clean input 21a0f02b5f2b23ef19813c7e37812f748157f812 matches refreshed
  origin. Fresh exact-input hosted run 38740423411/job 116440378554 passes all
  20 steps, 148 workspace tests, separate 14 adapter/five sample tests, rustdoc,
  unchanged sample and both whitespace checks.
- Initial/final locked local format/check/Clippy/test/warnings-denied rustdoc
  and whitespace pass 148/149 workspace tests. Focused event model and focused
  Clippy pass. Local Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy 0.1.98
  are unchanged.
- Independent source/oracle review accepts test blob
  d865bbab4187415cd2460c8bd3ab13bfdbd06e54. Exact-count assertions check
  29,523 traces and 221,436 generated operations; terminal FIFO drains remain
  separate. Clean isolated control passes; six selected compiling source faults
  fail behavioral assertions. Root separately inspects raw logs/patches/hashes.
  Whole-tree audit covers 41 Rust files/16,117 lines, zero width findings and
  three unchanged fulfilled expectations. All 365 relative links and 113 exact
  test references resolve; seven changed rendered documents pass full content/
  HTML structure review, without a pixel or human-review claim. Complete
  source/document/diff review accepts the checkpoint.
- Input hosted runner 2.337.0, windows-2025-vs2026 image 20260925.250.1,
  Rust/Cargo 1.99.0, rustfmt 1.10.0-stable and Clippy 0.1.99 match the recorded
  baseline. Local checks do not claim execution on that hosted toolchain.
- Published checkpoint 7137e924819b8cc35f0c4f3464f80c3d8df1b149 has
  [successful hosted CI](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/38905783350):
  exact logged checkout, all 20 steps, 149 workspace tests, separate 14/five host
  tests, rustdoc, unchanged 222-line sample and both whitespace checks. Hosted
  Rust/Cargo advances to 1.99.1; rustfmt 1.10.0-stable/Clippy 0.1.99 remain.
  Runner image advances to 20261007.264.1. Required configuration/whole-tree
  re-audit retains the policy with no source or waiver changes.

## Current architecture

One unpublished package owns synchronous LC1 lifecycle/work, bounded inbox
routing/dispatch, manual time and one-shot scheduling, bounded events and optional
runtime configuration with immutable work visibility and one-use rollback. The
private shared-source host sample composes these services. Separate finite models
strengthen messaging, configuration, lifecycle/work failures, messaging-owned
scheduling, direct-runtime configuration histories and standalone event histories.

## Work in progress

The [event sequence checkpoint](verification/EVENT_QUEUE_SEQUENCE_REVIEW.md)
supplements existing saturation and ownership scenarios with mixed emission,
rejection and dequeue histories at capacities one, two and three. Two fixed
records differ in all metadata fields and oppose timestamp order; repeated equal
records remain separate accepted entries. The independent accepted-record ledger
and consumed ordinal check all histories through length eight. Local and exact-
checkpoint hosted acceptance are complete. This documentation follow-up preserves
Rust and receives complete review and its own exact-revision CI. No unfinished
implementation remains. Production, APIs, requirements, dependencies,
licence, lint and workflow are unchanged.

## Highest risks and uncertainties

- Finite models cover only their frozen domains. The event model excludes
  arbitrary capacities/content/identifier types, construction failures,
  allocation behavior and runtime event-production ordering.
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

Assess whether mixed returned-work failure-event histories reveal a meaningful
integration gap beyond the existing direct and messaging-owned saturation/retry
scenarios. Select a new model only if its added evidence justifies its cost.

## Latest run

2026-10-17: the standalone event history model and full locked local baseline
pass 149 workspace tests. Independent oracle/source review accepts the frozen
domain; selected isolated fault trials pass their defined expectations. Whole-tree,
rendered-document and complete source/document/diff reviews accept the checkpoint.
Publication and exact-checkpoint hosted CI pass; hosted toolchain/image changes
receive the required configuration/whole-tree re-audit. This documentation follow-up
receives its own exact-revision CI. Stop after this one completed checkpoint.
