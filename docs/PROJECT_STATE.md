# Project state

Last updated: **2026-10-16**

## Current milestone

Stages 1 through 4 and source-quality adoption are complete for experimental
serial host v0.1. The [actual Codex host review](verification/HOST_V0_1_REVIEW.md)
accepts entry points and architecture under ADR-0024 with exact evidence and
limits. Package 0.0.0 remains unpublished; APIs and requirements are provisional.
Current work hardens existing behavior within those boundaries.

## Verified baseline

- Clean input `ce8bd7e86423a69435c6ba4f71a011df5b62ce99` matches refreshed
  origin. Fresh exact-input hosted run 38570555687/job 115874303603 passes all
  20 steps, 147 workspace tests, separate 14/five host tests, rustdoc, full
  unchanged sample and both whitespace checks.
- Initial/final locked local format/check/Clippy/test/warnings-denied rustdoc
  and whitespace pass 147/148 workspace tests. Focused model and focused Clippy
  pass; 74,898 traces/363,792 generated operations and 524,286 separate terminal
  probes match the reference. Local Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and
  Clippy 0.1.98 are unchanged.
- Whole-tree audit covers 40 Rust files/15,912 lines, zero width findings and
  three unchanged fulfilled expectations. Independent Codex review accepts
  oracle and complete source at blob ca64c5a09f1ab93e046e612fd869937b8b3ecff6.
  All 357 relative links and 112 exact traceability function references resolve.
  Six changed rendered documents pass complete content/HTML structure review;
  no pixel or human-review claim. Complete diff review accepts this increment.
  Clean isolated control passes; six selected compiling source faults fail
  behavioral assertions. Root separately verifies raw logs, patches and hashes.
- Input hosted runner 2.337.0, windows-2025-vs2026 image 20260925.250.1,
  Rust/Cargo 1.99.0, rustfmt 1.10.0-stable and Clippy 0.1.99 match the reviewed
  baseline. Local checks do not claim execution on that hosted toolchain.

## Current architecture

One unpublished package owns synchronous LC1 lifecycle/work, bounded inbox
routing/dispatch, manual time and one-shot scheduling, bounded events and optional
runtime configuration with immutable work visibility and one-use rollback. The
private shared-source host sample composes these services. Separate finite models
strengthen standalone messaging/configuration, successful lifecycle, work and
message-callback failures, messaging-owned scheduling and direct-runtime
configuration/work/lifecycle histories.

## Work in progress

The [runtime configuration checkpoint](verification/RUNTIME_CONFIGURATION_SEQUENCE_REVIEW.md)
freezes two initially Running applications, one bound-two table and eight
configuration/work/selected-stop/restart operations through length five for both
selected indices. The selected application fails on its second invoked work.
Independent configuration and lifecycle/callback histories check fresh work
views, retained application state, suppression, terminal failure and healthy
peer progress. Supplemental peer work and table operations expose retained
lineage after every terminal prefix. Focused/full local checks, independent
source/oracle/document/diff review and selected isolated fault trials pass their
defined expectations. No unfinished implementation remains. Publication and
exact-revision hosted execution are pending.
Production, APIs, requirements, dependencies, licence, lint and workflow are
unchanged.

## Highest risks and uncertainties

- Finite models cover only their frozen domains. This direct-runtime model
  excludes arbitrary candidates/validators/bounds, initial rejection, exhaustion,
  unknown/Registered applications, other callback failures and service owners.
  Existing scenarios retain their separate evidence.
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

Assess whether finite structured-event enqueue/dequeue histories add meaningful
saturation/FIFO evidence beyond existing scenarios before selecting another
model. Preserve the current event contract and alternate-owner evidence.

## Latest run

2026-10-16: the direct-runtime configuration/work history model and full locked
local baseline pass 148 workspace tests. Independent source/oracle review accepts
the frozen scope; six selected compiling faults fail behavioral assertions.
Complete document/diff and rendered content/structure reviews pass. Publication
and exact-revision hosted execution are pending. Stop after this one checkpoint.
