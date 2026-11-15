# Project state

Last updated: **2026-11-16**

## Current milestone

Stage 5 practical library use under ADR-0025. RFF-LIB-REQ-016 revision 4
[retains a queued diagnostic across sink rejection](HOST_PROGRESS.md#hand-off-queued-diagnostics-to-a-fallible-sink)
in the serial periodic consumer. The reviewed host v0.1 milestone remains
the regression baseline. Package 0.0.0 is unpublished with provisional APIs.

## Verified baseline

- Input `6a1094d4ef1aa293fe8732246458b0ed177f04e9` matched refreshed origin.
  Hosted run 43598192080 passed all 20 steps with 272 workspace tests.
- The locked local baseline passes 274 tests with Rust/Cargo 1.98.0,
  rustfmt 1.9.0-stable and Clippy 0.1.98. All nine host progress tests pass
  in the workspace and an independent package with identical source.
- Implementation `842aaaace10051886ee4c1c8a2785941fe37cb11` passed all
  20 hosted steps in run 43717651968 with 274 workspace tests and the same
  222-line combined host sample.

## Current architecture

One unpublished package owns synchronous lifecycle/work, bounded messaging,
manual time, finite schedules, structured events and optional validated
configuration. Both messaging owners expose read-only subscription checks.
The serial command session selects bounded read, admission, dispatch and
output turns. The periodic host uses one downstream slot and one release
cursor. It shares one finite queue across sample and congestion warnings
and returned work errors. A separate output turn adds one pending diagnostic
slot in front of the sink.

## Work in progress

Codex accepted the diagnostic handoff with local, independent-package and
exact hosted evidence.
[Focused review](verification/HOST_PROGRESS_REVIEW.md#diagnostic-handoff-follow-up)

## Highest risks and uncertainties

- Congestion observes occupied host turns on the mission clock. Sample
  progress checks depend on the callback returning. Any queued event can
  consume the available slot and cause a later diagnostic to be rejected.
- The serial host needs each sink call to return. Safe retry requires a
  definite rejection of the whole event.
- Attributing a stop count to specific commands requires complete
  knowledge of outstanding admissions. Input and output retry require
  the same retained state and a caller that continues turning.
- Logical bounds leave process memory and execution time unspecified.

## Important unresolved decisions

MSRV and external I/O protocols remain undecided. Release, platform,
security, major API freeze and other consequential decisions retain
separate gates.

## Most likely next task

Assess what a host needs to do with a work-failure diagnostic rejected by
the FIFO. Start with the typed work error, which already exposes the event.

## Latest run

2026-11-16. The sink can reject a dequeued warning while the queue accepts
a later one. The host can retry the retained warning and continue draining
in order. A full queue still exposes a failed callback's rejected event.
