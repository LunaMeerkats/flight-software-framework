# Project state

Last updated: **2026-10-10**

## Current milestone

Stages 1 through 4 and source-quality adoption are complete for the
experimental serial host v0.1 target. The [actual Codex host review](verification/HOST_V0_1_REVIEW.md)
accepts current entry points and architecture under ADR-0024 with exact
reviewer, revision, evidence and limitation dispositions. The package remains
unpublished at 0.0.0; APIs and requirements are provisional. Current work
hardens existing behavior within those boundaries.

## Verified baseline

- Input `81ca906e97c28db53ca601d34c1b2d1b2e7926fb` started clean, equal to
  refreshed origin. Exact input hosted run 37782992360 is freshly confirmed
  successful. It predates the new sequence test.
- Initial locked local baseline passes 141 tests; final baseline passes 142,
  formatting, all-target check, warnings-denied Clippy/rustdoc and whitespace.
  The new focused sequence target passes one test covering 55,987 traces,
  324,726 generated operations and additional final drains.
- Six isolated faulty source copies compile and fail the new test's behavioral
  assertions. This is selected sensitivity evidence, not comprehensive mutation
  testing. Production source, APIs, dependencies, licences and policy are unchanged.
- Local Rust/Cargo 1.98.0, rustfmt 1.9.0-stable, Clippy 0.1.98 remain unchanged.
  The recorded input hosted toolchain is separately 1.99.0/1.10.0/0.1.99.
- Source audit covers 34 Rust files and 12,913 physical lines, no width findings,
  and the same three fulfilled function-size expectations. Relative Markdown
  links and all 106 exact traceability function references resolve.
- Checkpoint `82078b3656481ec9527aaa059e159ca446c54aec` is published.
  [Hosted run 37935055379](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/37935055379)
  passes the exact logged checkout/all 20 steps, 142 workspace tests, separate
  14/five host targets, rustdoc, sample and whitespace. Hosted runner/image/tools
  match the input baseline; local 1.99.0 execution is not claimed.

## Current architecture

One unpublished package owns synchronous LC1 lifecycle/work, bounded inbox
routing/dispatch, manual time and one-shot scheduling, bounded events, and
optional runtime configuration with immutable work visibility and one-use
rollback. The private shared-source host sample composes these services.
The new standalone test compares append-only accepted histories and consumption
cursors against the public message-bus API over a frozen finite domain.

## Work in progress

Sequence implementation, focused/full locked checks, and root/parallel Codex
source and complete-diff review are complete. Seven changed documents pass
rendered HTML structure/content inspection; no pixel-level acceptance is
claimed. Checkpoint publication and exact-head hosted verification pass in the
[sequence review](verification/MESSAGE_SEQUENCE_REVIEW.md). This follow-up
records that completed checkpoint; later revisions need their own CI. The dated
[ADR-0023 revisit](adr/0023-host-ci-baseline.md) confirms main contains the
unchanged workflow; no setting or workflow change was needed.

## Highest risks and uncertainties

- Sequence evidence covers only one topology, capacities [1,2,2], six operations
  and lengths zero through six. Lifecycle and arbitrary-domain claims remain
  outside this new test.
- Application identity and clock-origin pairing remain caller discipline;
  positional inbox configurations cannot detect swapped valid mission entries.
- Generic copied identifiers may retain shared referents. Logical capacities
  do not bound whole-process bytes, callback allocations, archives or duration.
  Actual allocator exhaustion and allocation/deallocation counts are unverified.
- Callback/clock panics and hangs remain outside containment. Stop clearing
  does not prove application-internal/external cleanup; Failed is terminal.
- Callback fan-out is immediate and nontransactional. Accepted peer deliveries
  survive later failure; event saturation may omit an event. Scheduled errors
  are consumed without automatic retry, events or rollback. Host output arrays
  do not prove physical delivery.
- Stable Rust and runner images float; source/document and conditional probes
  remain outside CI. Codex review does not establish independent human review.

## Important unresolved decisions

Major API freeze, significant unsafe code, major runtime/code-generation/
dependency/security decisions, platform commitments, licences/identity,
tags/releases/crate publication and deployment retain separate authorization
gates. MSRV, broader contexts/events, external I/O, hardware, RTOS and no_std
remain undecided. Routine host acceptance is complete under ADR-0024.

## Most likely next task

Consider one separate reduced model of runtime-owned availability and exact
inbox clearing through publication, stop and restart. Freeze its lifecycle
alphabet and bounds first; reassess the priority against fresh evidence.

## Latest run

2026-10-10: adds one finite standalone message-bus sequence test and independent
oracle, with passing focused/full checks and six selected fault detections.
Records default-branch revisit with unchanged CI. Source/document/diff reviews,
checkpoint publication and exact-checkpoint hosted verification pass.
No unfinished implementation remains; the linked review records evidence limits.
