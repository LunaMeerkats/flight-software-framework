# Project state

Last updated: **2026-11-28**

## Current milestone

Stage 5 practical library use under ADR-0025. RFF-LIB-REQ-023 revision 1
adds a public-only fixed startup/cleanup consumer using existing Runtime APIs.
Reviewed host v0.1 remains the regression baseline. Package 0.0.0 stays
unpublished with provisional APIs.

## Verified baseline

- Clean input `82ed58a04aba1bcc0660c3bc386902ff3e0654ac` matches refreshed
  and live origin/main. Exact input run 45933553084 passes all 20 steps.
- Locked input baseline passes 332 tests. Final format/check/Clippy,
  339 workspace tests, warnings-denied rustdoc and whitespace pass.
- Seven focused histories and the same seven independent-binary tests pass.
  Independent check/lint/run/metadata, exact source bytes/SHA256 and all ten
  stdout lines match the root example.
- Local tools remain Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy 0.1.98.
  Input hosted tools are Rust/Cargo 1.100.0, rustfmt 1.11.0-stable,
  Clippy 0.1.100, runner 2.338.2 and image `20261123.321.1`.

## Current architecture

One unpublished package owns synchronous lifecycle/work, bounded messaging,
manual/host elapsed time, schedules, events and optional validated configuration.
The host selects callbacks, waiting and I/O. A private consumer token records
the successful startup prefix; reverse shutdown consumes it and retains each
typed result. Partial-start cleanup preserves the primary and secondary errors.
No framework group abstraction, dependency or recovery policy is added.

## Work in progress

Implementation, local/independent verification and root/independent Codex
complete source/document/diff review pass without findings. Whole-tree widths,
links, exact test references and 12 rendered-document content/structure reviews
pass. Authorized publication and exact hosted inspection remain. The
[focused review](verification/STARTUP_CLEANUP_REVIEW.md) records actual
commands, package path, hashes, test coverage and limits.

## Highest risks and uncertainties

- Failed startup may leave application effects. Terminal Failed cannot be
  stopped/restarted by ordinary operations. Stop success proves callback return
  only, not independent application resource cleanup.
- Fixed group order, initial states and same-owner distinct IDs are consumer
  assumptions. Tokens and IDs do not validate runtime origin.
- Callbacks, clocks, waits and writers must return. Bounded invocation counts
  do not establish wall-clock deadlines or external delivery.

## Important unresolved decisions

MSRV and external I/O protocols remain undecided. Release, platform,
security, major API freeze and other consequential decisions retain gates.

## Most likely next task

Assess a concrete bounded host-input path for validated configuration
activation before adding generic mission orchestration or recovery.

## Latest run

2026-11-28. Startup/cleanup consumer passes local and independent verification.
Complete review passes; authorized publication and exact hosted inspection remain.
