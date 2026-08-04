# Project state

Last updated: **2026-08-05**

## Current milestone

Stage 0 is complete. The next planned milestone is Stage 1: one lifecycle
vertical slice.

## Verified baseline

- The workspace began empty and was initialized as a Git repository on
  `codex/nightly`.
- No Rust implementation, Cargo manifest, executable, test, CI, benchmark, or
  runtime behavior exists yet.
- Relative links resolved across all 11 Markdown files; eight requirement IDs
  matched traceability rows; eight referenced source IDs were defined; and
  `git diff --cached --check` passed.
- Cargo formatting, lint, test, and documentation commands were not applicable
  because no Cargo manifest exists.

## Current architecture

ADR-0001 selects a provisional caller-driven, serial host runtime for the first
v0.1 vertical slices. The host/test harness explicitly advances work; time and
external effects are injected; no hidden thread or async executor is assumed.
This is not a frozen public API or a real-time determinism claim.

## Work in progress

No implementation work is in progress.

## Highest risks and uncertainties

- Lifecycle interfaces may be committed before ownership and restart behavior
  are adequately characterized.
- Queue bounds and overflow outcomes have not been selected.
- Event and telemetry paths could bypass resource bounds.
- Defined returned-error containment could be confused with panic, hang, or
  process-fault containment.
- Public terminology could imply NASA affiliation or cFS/CCSDS compatibility.
- A future dependency could silently define concurrency or cancellation policy.

## Important unresolved decisions

- Project name, public branding, and licence require human review.
- Minimum supported Rust version has not been selected or tested.
- Application identity allocation and complete lifecycle transitions are not
  designed yet.
- Bus overflow/backpressure policy is deliberately undecided.
- Panic policy and any stronger isolation boundary remain out of current scope.

## Most likely next tasks

1. Characterize and test a minimal application lifecycle state machine in one
   small crate without adding concurrency or external dependencies.
2. Decide only the application identity and error semantics needed by that
   slice.
3. Update traceability with exact evidence; keep unimplemented requirements
   marked as such.

## Latest run

2026-08-05: Established the initial charter, architecture analysis, eight
provisional v0.1 requirements, roadmap, ADR-0001, source provenance, and honest
pre-implementation traceability. All applicable repository/document checks
passed; no Rust behavior was implemented or claimed.
