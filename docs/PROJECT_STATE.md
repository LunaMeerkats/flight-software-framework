# Project state

Last updated: **2026-10-21**

## Current milestone

Reviewed experimental serial host v0.1 remains the regression baseline. Stage 5
advances practical library use under ADR-0025. The first public-API quickstart
is verified, reviewed and published. The second consumer composes two independent
application implementations with mission enum delegation and typed concrete
message errors. Package 0.0.0 remains unpublished and APIs provisional.

## Verified baseline

- Clean input f539755e50b0f15aaa4361cba2e2eae4e7b3dca9 matches refreshed origin;
  exact hosted run 39258514095/job 118079807809 passes all 20 steps and 154
  workspace tests. Existing finite-domain and independent quickstart evidence
  is retained in its focused records.
- Fresh initial/final locked format/check/Clippy/test and warnings-denied rustdoc
  pass 154/158 workspace tests. Focused composition passes four tests and
  quickstart three; both executables and both whitespace checks pass.
- Identical composition source passes independent package check/Clippy/test/run;
  metadata and SHA256 confirm its own workspace and sole framework path dependency.
  Root and independent Codex composition_review accept complete source/document/
  diff and consumer evidence. Exact evidence is
  in [the focused record](verification/COMPOSITION_REVIEW.md).
- Local Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy 0.1.98 are unchanged.
  Input hosted tools remain Rust/Cargo 1.99.1, rustfmt 1.10.0-stable/Clippy 0.1.99,
  runner 2.337.0/windows-2025-vs2026 image 20261007.264.1. Local results do not
  claim execution on the hosted toolchain.
- Published checkpoint d63d4f88d23a70cfd0fd4badf3efeba88ca3bb8d passes exact
  hosted run 39429032917/job 118648900866: all 20 steps, 158 workspace tests,
  separate 14 adapter/five sample tests, rustdoc, both whitespace checks and
  unchanged 222-line sample. Hosted tools/runner match input; Windows image is
  now 20261018.275.1. This documentation-only follow-up receives its own CI.

## Current architecture

One unpublished package owns synchronous LC1 lifecycle/work, bounded inbox
routing/dispatch, manual time and one-shot scheduling, bounded events and optional
runtime configuration with immutable work visibility and one-use rollback.
The private combined host sample joins those services. Public-only quickstart
and composition examples use the supported APIs. The second has independently
implemented relay/sink types, mission enum forwarding and separate concrete
message errors. Existing finite sequence models retain their frozen domains.

## Work in progress

The RFF-LIB-REQ-002/ADR-0027 consumer and guidance are implemented, verified,
autonomously reviewed and published with exact-checkpoint hosted CI success.
No unfinished implementation remains. The documentation-only evidence follow-up
receives its own review and CI. No new library API, dependency, platform
commitment or workflow change is introduced.

## Highest risks and uncertainties

- Two controlled consumers do not prove external adoption, broad usability or API
  stability. Static composition shares topic/payload definitions and infallible
  lifecycle/work errors; ordinary-work effects need further consumer assessment.
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

Assess a consumer whose ordinary work produces a message or event: compare
explicit bounded host-mediated effects with a narrow public service capability.
Use concrete friction to select the next slice; this pair alone does not justify
a generic sum adapter or another finite model.

## Latest run

2026-10-21: selected independent application composition after root and separate
Codex source assessment. Requirement and ADR precede implementation. The example
and guide are implemented; full locked local and independent-package checks pass.
Complete autonomous source/document/diff review passes. Source publication and
exact-checkpoint hosted verification pass; this documentation-only evidence
follow-up receives its own review and CI.
