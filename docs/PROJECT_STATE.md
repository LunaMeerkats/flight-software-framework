# Project state

Last updated: **2026-11-18**

## Current milestone

Stage 5 practical library use under ADR-0025. RFF-LIB-REQ-017 adds
[a runnable two-application example](PERIODIC_PEER.md) that keeps the
healthy application working after the other returns an error. The reviewed
host v0.1 milestone remains the regression baseline. Package 0.0.0 is
unpublished with provisional APIs.

## Verified baseline

- Input `ca32a428314055f1b2f4eae5214462c6ab129fdb` matched refreshed origin.
  Hosted run 43873841092 passed all 20 steps with 276 workspace tests.
- The locked local baseline passes 278 tests with Rust/Cargo 1.98.0,
  rustfmt 1.9.0-stable and Clippy 0.1.98. Both periodic peer tests and
  the runnable trace pass with identical source in an independent package.

## Current architecture

One unpublished package owns synchronous lifecycle/work, bounded messaging,
manual time, finite schedules, structured events and optional validated
configuration. Both messaging owners expose read-only subscription checks.
The serial command session selects bounded read, admission, dispatch and
output turns. The periodic host uses one downstream slot and one release
cursor. It shares one finite queue across sample and congestion warnings
and returned work errors. A separate output turn adds one pending diagnostic
slot in front of the sink.
That host retains one queue-rejected failure event for explicit FIFO retry.
The periodic peer example uses the direct runtime and two independent
cursors. Returning a work error ends only the selected cursor's releases.

## Work in progress

The periodic peer example passed local and independent-package checks.
Exact hosted verification follows publication.
[Focused review](verification/PERIODIC_PEER_REVIEW.md)

## Highest risks and uncertainties

- Congestion observes occupied host turns on the mission clock. Sample
  progress checks depend on the callback returning. Any queued event can
  consume the available slot and cause a later diagnostic to be rejected.
- The serial host needs each sink call to return. Sink retry requires a
  definite rejection of the whole event. Queue retry depends on the caller
  choosing a turn after space opens. The periodic peer example shows
  continued work after one terminal failure while the host selects turns.
- Attributing a stop count to specific commands requires complete
  knowledge of outstanding admissions. Input and output retry require
  the same retained state and a caller that continues turning.
- Logical bounds leave process memory and execution time unspecified.

## Important unresolved decisions

MSRV and external I/O protocols remain undecided. Release, platform,
security, major API freeze and other consequential decisions retain
separate gates.

## Most likely next task

Assess how a serial host can retain two independent queue-rejected work
failures with an explicit bound and retry order, if a consumer needs it.

## Latest run

2026-11-18. A returned sampler error disables its cursor. The heartbeat
keeps running while diagnostics occupy the FIFO and after dequeue.
