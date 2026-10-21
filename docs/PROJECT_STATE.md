# Project state

Last updated: **2026-10-22**

## Current milestone

Reviewed experimental serial host v0.1 remains the regression baseline. Stage 5
advances practical library use under ADR-0025. Two public-only consumers are
verified, reviewed and published. The third selected consumer demonstrates
configured ordinary-work output, finite scheduling and explicit host transfer.
Package 0.0.0 remains unpublished and APIs provisional.

## Verified baseline

- Clean input 0848e32821046b91048ad607a7aba267a8850fb4 matches refreshed origin.
  Re-read exact hosted run 39429862057/job 118651777288 passes all 20 steps and
  158 workspace tests. Earlier focused evidence retains its exact boundaries.
- Fresh initial locked format/check/Clippy/test and warnings-denied rustdoc pass
  158 workspace tests and both whitespace checks. Final locked checks pass 168
  workspace tests, ten focused tests/run and ten identical-source external tests.
- External metadata confirms one independent workspace member, two graph packages
  and the sole framework path dependency. Matching SHA256 and stdout are checked.
  Whole-tree widths, relative links, test mappings and twelve rendered Markdown
  documents pass content/structure inspection. Actual evidence is in
  [the focused record](verification/WORK_OUTPUT_REVIEW.md).
- Local Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy 0.1.98 are unchanged.
  Input hosted tools are Rust/Cargo 1.99.1, rustfmt 1.10.0-stable/Clippy 0.1.99,
  runner 2.337.0/windows-2025-vs2026 image 20261007.264.1. Local results do not
  claim execution on the hosted toolchain.

## Current architecture

One unpublished package owns synchronous LC1 lifecycle/work, bounded inbox
routing/dispatch, manual time and one-shot scheduling, bounded events and optional
runtime configuration with immutable work visibility and one-use rollback.
The private combined host sample joins those services. Public-only quickstart
and composition examples use the supported APIs. The new consumer keeps one
caller-owned pending output slot and explicitly publishes after ordinary work;
it adds no library service context, dependency or workflow change.

## Work in progress

RFF-LIB-REQ-003 and ADR-0028 are established before source implementation.
The work-output consumer and guide are implemented and locally verified.
Root and independent Codex independent_review accept complete source/document/
diff, external-package and raw baseline evidence with no blocking findings.
Exact publication/hosted evidence remains. No implementation, local verification
or routine autonomous review is unfinished.

## Highest risks and uncertainties

- Controlled consumers do not prove external adoption, broad usability or API
  stability. Immediate in-callback publication feedback remains unprovided.
- Retained retry is safe only for this frozen sole subscriber. General partial
  fan-out can duplicate deliveries. Caller-owned output can outlive producer
  stop/failure; transfer is explicit mission policy, not automatic recovery.
- Application/clock origins and positional inbox pairing remain caller duties.
  Logical counts do not bound process bytes, callback allocation or duration.
- Panics/hangs and allocator exhaustion are uncontained. Cooperative queue cleanup
  does not prove external cleanup. Event saturation can omit records; retries are
  caller policy. One-shot errors are consumed. In-process output is not physical
  delivery. Finite models prove only their frozen domains. Hosted stable tools
  and images float; local reviews and conditional probes remain outside CI.

## Important unresolved decisions

Major API freeze, significant unsafe code, runtime/code-generation/dependency/
security decisions, platform commitments, identity/licences, tags/releases/crate
publication and deployment retain separate gates. MSRV, broader service contexts,
external I/O, hardware, RTOS and no_std remain undecided. Autonomous Codex review
does not establish human review or operational suitability.

## Most likely next task

After this checkpoint, assess a concrete ordinary-work event producer needing
immediate saturation feedback before callback success, or another independently
valuable capability. Use that need to decide whether a narrow service capability
is justified; avoid another example or model without a concrete adoption gap.

## Latest run

2026-10-22: initial safety, remote and exact input hosted checks pass. The full
locked local baseline passes. Root and independent consumer_need assess the
ordinary-work output gap; requirement and ADR precede source. Implementation,
full local and independent-package verification pass. Source/consumer/baseline
review passes; source publication and exact hosted evidence are pending.
