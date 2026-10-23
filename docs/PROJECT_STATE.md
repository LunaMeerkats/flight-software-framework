# Project state

Last updated: **2026-10-24**

## Current milestone

Reviewed experimental serial host v0.1 remains the regression baseline. Stage 5
advances practical library use under ADR-0025. Three public-only example consumers
remain verified. RFF-LIB-REQ-004 now adds runtime-issued ordinary-work identity
with public-only diagnostic consumer tests and independent-package evidence.
Package 0.0.0 remains unpublished and APIs provisional.

## Verified baseline

- Input HEAD a9afb328905eb2d413a868bc7e0371a06b5c6c20 matches refreshed origin.
  Exact input run 39606324526/job 119243331220 passes all 20 steps, 168 workspace
  tests, separate 14 adapter/five sample tests and the 222-line sample report.
  Prior unfinished requirement/ADR/guide/source changes are preserved.
- The unfinished source's initial formatting failure is corrected. Fresh locked
  initial/final format/check/Clippy/test and warnings-denied rustdoc pass with
  168/176 workspace tests. Eight focused and eight identical-source independent
  consumer tests pass. Metadata confirms one independent workspace member, two
  graph packages and the sole library path dependency; source bytes/hashes match.
- The composition guide's identity correction passes four focused and four
  independent tests, check/Clippy/run, metadata and source/output comparison.
- Local Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy 0.1.98 are unchanged.
  Input hosted tools are Rust/Cargo 1.99.1, rustfmt 1.10.0-stable/Clippy 0.1.99,
  runner 2.338.0/windows-2025-vs2026 image 20261018.275.1. Local results do not
  claim execution on the hosted toolchain.
  Actual commands, source identities and limitations are in
  [the focused record](verification/WORK_IDENTITY_REVIEW.md).
- Published checkpoint b3ebba331e73d2d4b46189accfc75e9ef9a6281b passes exact
  hosted run 39959273438/job 120432846468: all 20 steps, 176 workspace tests,
  separate 14 adapter/five sample tests, rustdoc, both whitespace checks and
  unchanged 222-line sample. Hosted tools/runner match input; image is
  20261021.278.1. Whole-tree source and configuration policy remains unchanged.

## Current architecture

One unpublished package owns synchronous LC1 lifecycle/work, bounded inbox
routing/dispatch, manual time and one-shot scheduling, bounded events and optional
runtime configuration with immutable work visibility and one-use rollback.
The ordinary-work context now exposes selected runtime-local identity alongside
configuration, through one construction path shared by all work owners. The
diagnostic consumer uses checked mission-owned services and separate diagnostic
and framework failure-report queues. No new framework service owner, trait,
dependency, maintained crate, workflow or lint exception is introduced.

## Work in progress

The unfinished 2026-10-23 item is complete. Root Codex and independent
context_review accept complete source/document/diff, baseline, rendered and
independent consumer evidence with no findings. Source publication and exact
checkpoint hosted inspection pass. No unfinished implementation remains. The
documentation-only evidence follow-up receives its own review and exact hosted
inspection. No broader service or scheduling capability is started in this run.

## Highest risks and uncertainties

- Controlled consumers do not prove adoption, broad usability or API stability.
  Immediate in-callback message publication feedback remains unprovided.
- Application/clock origins and positional inbox pairing remain caller duties.
  Copied context identity is not authenticated, persistent or origin-checked.
  Logical queue counts do not bound process memory, callback allocation or time.
- Cell/RefCell sharing is serial mission policy. Busy/full diagnostics are
  terminal work failures in this fixture; recording before an effect is not
  atomic delivery. This fixture uses a separate report queue for framework
  failure wrappers.
- Panics/hangs and allocation exhaustion are uncontained. Cooperative inbox
  cleanup does not prove external cleanup. Event saturation can omit records;
  retries remain caller policy. One-shot errors are consumed. In-process output
  is not physical delivery. Finite models prove only their frozen domains.
  Hosted tools/images float; manual reviews and conditional probes remain local.

## Important unresolved decisions

Major API freeze, significant unsafe code, runtime/code-generation/dependency/
security decisions, platform commitments, identity/licences, tags/releases/crate
publication and deployment retain separate gates. MSRV, broader service contexts,
external I/O, hardware, RTOS and no_std remain undecided. Autonomous Codex review
does not establish human review or operational suitability.

## Most likely next task

Assess a concrete consumer needing repeated work without rebuilding finite
one-shot agendas. Compare explicit host re-arming with a narrow periodic
scheduling policy before selecting any API change; preserve the current one-shot
contract and avoid a generic executor or model without a demonstrated need.

## Latest run

2026-10-24: resumed preserved unfinished identity work, corrected formatting and
queue-borrow guidance, and completed public-only consumer tests. Initial/final
locked baseline and actual identity/composition independent packages pass.
Complete autonomous review, ordinary source publication and exact checkpoint
hosted inspection pass. Evidence follow-up receives its own document/diff and
exact hosted review; no unrelated work is begun.
