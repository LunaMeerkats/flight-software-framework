# Project state

Last updated: **2026-11-21**

## Current milestone

Stage 5 practical library use under ADR-0025. RFF-LIB-REQ-017 revision 3
adds optional retained-failure admission priority to the
[periodic peer example](PERIODIC_PEER.md). Enabled samplers defer before
polling while retained failures await admission; heartbeat stays selectable.
The reviewed host v0.1 milestone remains the regression baseline. Package
0.0.0 is unpublished with provisional APIs.

## Verified baseline

- Clean input `dba77f744f9151d515d7110259f425a64254abb1` matched fetched
  and live origin. Exact hosted run 44311715530 passed all 20 steps.
- Input locked local baseline passed 281 workspace tests, formatting,
  all-target check/Clippy, warnings-denied rustdoc and whitespace.
- Local tools remain Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy
  0.1.98. Input whole-tree width review found zero findings.
- Completed locked local replay passes 285 workspace tests, formatting,
  all-target check/Clippy, warnings-denied rustdoc and whitespace.
- The focused example and identical independent binary pass nine tests
  each, check, lint and run. Their 23-line output matches exactly and
  preserves the prior 14-line trace byte for byte.

## Current architecture

One unpublished package owns synchronous lifecycle/work, bounded messaging,
manual time, finite schedules, structured events and optional validated
configuration. Serial command-session turns separate input, admission,
dispatch and output. The periodic peer consumer retains two terminal
failures in observation order alongside a capacity-one FIFO and three
independent cursors. An optional host gate prioritizes retained admission
before fresh sampler polling; healthy work remains caller-selected.
Retry and dequeue stay explicit. The original unguarded turn remains.

## Work in progress

Local/downstream checks and independent source review are complete.
Finish document review and ordinary main publication for retained priority;
record exact hosted results before accepting the checkpoint.
[Focused review](verification/PERIODIC_PEER_REVIEW.md#retained-admission-priority-follow-up)

## Highest risks and uncertainties

- Priority relies on the mission's healthy heartbeat, terminal cursor
  disablement and compacted retry slots. Changed roles require review.
- A host must choose retries/drains; sampler deferral can be indefinite.
  Selecting unguarded turns bypasses this optional ordering policy.
- Callbacks and writes must return. Logical bounds do not bound bytes or
  execution time; FIFO admission/dequeue does not prove external delivery.
- Attributing discarded commands requires complete serial admission knowledge.

## Important unresolved decisions

MSRV and external I/O protocols remain undecided. Release, platform,
security, major API freeze and other consequential decisions retain gates.

## Most likely next task

Assess a concrete downstream need beyond this fixed retry/admission policy,
including whether integrations need public lifecycle-safe application access.
Do not expand retry machinery without evidence from a consumer.

## Latest run

2026-11-21. Preserved and completed the interrupted retained-priority
implementation. Local and downstream verification passed. Exact main
publication and hosted evidence remain pending.
