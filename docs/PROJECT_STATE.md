# Project state

Last updated: **2026-11-17**

## Current milestone

Stage 5 practical library use under ADR-0025. RFF-LIB-REQ-016 revision 5
[retries a queue-rejected work failure](HOST_PROGRESS.md#retry-a-work-failure-rejected-by-the-queue)
in the serial periodic consumer. The reviewed host v0.1 milestone remains
the regression baseline. Package 0.0.0 is unpublished with provisional APIs.

## Verified baseline

- Input `d7e50431dfe89d331ef3077dc3baec36b9dc28ae` matched refreshed origin.
  Hosted run 43717910335 passed all 20 steps with 274 workspace tests.
- The locked local baseline passes 276 tests with Rust/Cargo 1.98.0,
  rustfmt 1.9.0-stable and Clippy 0.1.98. All 11 host progress tests pass
  in the workspace and an independent package with identical source.
- Implementation `4801928432def9b0003eaf8f219d6adb27f4b77b` passed all
  20 hosted steps in run 43873421289 with 276 workspace tests and the
  same 222-line combined host sample.

## Current architecture

One unpublished package owns synchronous lifecycle/work, bounded messaging,
manual time, finite schedules, structured events and optional validated
configuration. Both messaging owners expose read-only subscription checks.
The serial command session selects bounded read, admission, dispatch and
output turns. The periodic host uses one downstream slot and one release
cursor. It shares one finite queue across sample and congestion warnings
and returned work errors. A separate output turn adds one pending diagnostic
slot in front of the sink.
The host retains one queue-rejected failure event for explicit FIFO retry.

## Work in progress

Codex accepted the failure event retry after local, independent-package
and exact hosted verification.
[Focused review](verification/HOST_PROGRESS_REVIEW.md#queue-rejected-failure-follow-up)

## Highest risks and uncertainties

- Congestion observes occupied host turns on the mission clock. Sample
  progress checks depend on the callback returning. Any queued event can
  consume the available slot and cause a later diagnostic to be rejected.
- The serial host needs each sink call to return. Sink retry requires a
  definite rejection of the whole event. Queue retry depends on the caller
  choosing a turn after space opens. The example has one terminal failure.
- Attributing a stop count to specific commands requires complete
  knowledge of outstanding admissions. Input and output retry require
  the same retained state and a caller that continues turning.
- Logical bounds leave process memory and execution time unspecified.

## Important unresolved decisions

MSRV and external I/O protocols remain undecided. Release, platform,
security, major API freeze and other consequential decisions retain
separate gates.

## Most likely next task

Assess a second public-API consumer with several applications and an
explicit policy for continuing healthy work after one application fails.

## Latest run

2026-11-17. A host can retry the original work-failure event after an older
warning frees the FIFO entry. Sink output preserves both older warnings.
