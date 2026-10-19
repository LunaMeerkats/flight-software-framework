# Project state

Last updated: **2026-10-20**

## Current milestone

Reviewed experimental serial host v0.1 remains the regression baseline. Stage 5
advances practical library use under ADR-0025. The first controlled public-API
consumer is implemented and locally verified: a runnable quickstart plus setup
instructions and actual independent Cargo path-dependency use. This demonstrates
one composition, not external adoption or broad consumer readiness. Package
0.0.0 remains unpublished and APIs provisional.

## Verified baseline

- Clean input 61e3f0e54509ab4a2f45b94648c83d66e245b233 matches refreshed origin;
  exact hosted run 39248169214/job 118045097712 passes all 20 steps. Prior
  messaging/event finite-domain evidence is retained in its focused records.
- Fresh initial/final locked format/check/Clippy/test and warnings-denied rustdoc
  pass 151/154 workspace tests. Focused quickstart passes three tests and its
  executable prints the guide's exact seven lines. Both whitespace checks pass.
- The same quickstart source independently builds/lints/tests/runs in an external
  Cargo package; metadata confirms its own workspace and sole framework path
  dependency. Source SHA256 and actual commands are in the
  [focused verification record](verification/QUICKSTART_REVIEW.md).
- Local Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy 0.1.98 are unchanged.
  Input hosted tools remain Rust/Cargo 1.99.1, rustfmt 1.10.0-stable/Clippy 0.1.99,
  runner 2.337.0/windows-2025-vs2026 image 20261007.264.1. Local results do not
  claim execution on the hosted toolchain.
- Root and independent Codex quickstart_review accept full source/document/diff,
  raw local/consumer evidence and generated crate documentation. Whole-tree review
  finds zero widths/three unchanged expectations; all 411 relative links/118 exact
  test references resolve and twelve run-changed rendered documents pass content/
  structure review, including the CI baseline evidence follow-up.
- Published checkpoint c772dd960fe45b8cbc440dd1101587e101d1244d passes exact
  hosted run 39258022861/job 118078130174: all 20 steps, 154 workspace tests
  including all three quickstart regressions, separate 14 adapter/five sample
  tests, rustdoc, unchanged 222-line sample and both whitespace checks. Tools and
  image match input. The documentation-only follow-up receives its own exact CI.

## Current architecture

One unpublished package owns synchronous LC1 lifecycle/work, bounded inbox
routing/dispatch, manual time and one-shot scheduling, bounded events and optional
runtime configuration with immutable work visibility and one-use rollback.
The private combined host sample joins those services. The new standalone tested
quickstart uses public APIs only; independent consumer verification adds no
maintained package. Existing finite sequence models retain their frozen domains.

## Work in progress

The public-API quickstart checkpoint is implemented, reviewed, verified and
published with successful exact-revision hosted CI. No unfinished implementation
remains. This documentation-only evidence follow-up receives its own review and
exact CI. RFF-LIB-REQ-001 and ADR-0026 preserve v0.1 meanings; no new runtime API,
dependency, platform commitment or workflow change is introduced.

## Highest risks and uncertainties

- One controlled consumer does not prove external adoption, broad usability or API
  stability. Heterogeneous composition and ordinary-work effects need concrete
  consumer assessment; the quickstart uses one application and borrowed host slot.
- Application/clock origins and positional inbox pairing remain caller duties.
  Logical counts do not bound process bytes, callback allocation or duration.
- Panics/hangs and allocator exhaustion are uncontained. Cooperative queue cleanup
  does not prove external cleanup. Immediate fan-out is nontransactional.
- Saturated events can omit records; retry is caller policy. One-shot errors are
  consumed. In-process output is not physical delivery. Finite models prove only
  their frozen domains. Stable hosted tools/images float; local reviews and
  conditional experiments remain outside CI.

## Important unresolved decisions

Major API freeze, significant unsafe code, runtime/code-generation/dependency/
security decisions, platform commitments, identity/licences, tags/releases/crate
publication and deployment retain separate gates. MSRV, broader service contexts,
external I/O, hardware, RTOS and no_std remain undecided. Autonomous Codex review
does not establish human review or operational suitability.

## Most likely next task

Assess a consumer needing two independently defined application types or ordinary
work that produces messages/events. Identify actual composition friction, compare
consequential alternatives and implement one justified improvement. Avoid adding
service breadth or another model solely because the first consumer is complete.

## Latest run

2026-10-20: selected public-API getting started after independent consumer
assessment. Added the standalone recipe, runnable setup/error/resource guidance,
requirement and ADR. Three new regression tests run in the unchanged workspace
baseline; actual independent package checks pass. Full autonomous source/document
review accepts the checkpoint. Source publication and exact-checkpoint hosted CI
pass; this documentation-only evidence follow-up receives its own review and CI.
