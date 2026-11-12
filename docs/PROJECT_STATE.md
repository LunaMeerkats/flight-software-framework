# Project state

Last updated: **2026-11-13**

## Current milestone

Stage 5 practical library use under ADR-0025. RFF-LIB-REQ-016 revision 1
[observes overdue sample output on completed host turns](HOST_PROGRESS.md).
The reviewed host v0.1 milestone remains the regression baseline. Package
0.0.0 is unpublished with provisional APIs.

## Verified baseline

- Input `336ecfa17a22fabe93cd376e84f9194a1eaa8fbe` matches refreshed origin.
  Hosted run 43045448136 passes all 20 steps with 265 workspace tests.
- The locked local baseline passes 268 tests with Rust/Cargo 1.98.0,
  rustfmt 1.9.0-stable and Clippy 0.1.98. All three host progress tests pass
  in the workspace and an independent package with identical source.
- Implementation `d8b77724ea1b63a954c5becb64371c8e37de35ea` passes all
  20 steps in hosted run 43225151848 with 268 workspace tests and the
  same 222-line combined host sample.

## Current architecture

One unpublished package owns synchronous lifecycle/work, bounded messaging,
manual time, finite schedules, structured events and optional validated
configuration. Both messaging owners expose read-only subscription checks.
The serial command session selects bounded read, admission, dispatch and
output turns. The host progress consumer composes one sample slot, one
manual clock and one event queue with the existing runtime work error path.

## Work in progress

The host progress checkpoint is complete with local, independent-package
and hosted implementation evidence. The record revision receives separate
hosted inspection. [Focused review](verification/HOST_PROGRESS_REVIEW.md)

## Highest risks and uncertainties

- A serial progress check depends on the selected callback returning. The
  host defines expected output and chooses when to observe it. Event queue
  saturation can reject a warning.
- Attributing a stop count to specific commands requires complete
  knowledge of outstanding admissions. Input and output retry require the
  same retained state and a caller that continues turning.
- Logical bounds leave process memory and execution time unspecified.

## Important unresolved decisions

MSRV and external I/O protocols remain undecided. Release, platform,
security, major API freeze and other consequential decisions retain
separate gates.

## Most likely next task

Assess how a periodic host should account for intentionally deferred work
when downstream backpressure prevents a fair progress observation.

## Latest run

2026-11-13. A completed turn without a sample can warn while Running.
A returned callback error at the same threshold preserves Failed and the
existing typed failure-event attempt.
