# Project state

Last updated: **2026-11-15**

## Current milestone

Stage 5 practical library use under ADR-0025. RFF-LIB-REQ-016 revision 3
[reports sustained downstream congestion](HOST_PROGRESS.md#report-sustained-downstream-congestion)
in the serial periodic consumer. The reviewed host v0.1 milestone remains
the regression baseline. Package 0.0.0 is unpublished with provisional APIs.

## Verified baseline

- Input `3acc1a9deb1ac0d113d96e6bd27908eaaa0878c2` matched refreshed origin.
  Hosted run 43431354539 passed all 20 steps with 270 workspace tests.
- The locked local baseline passes 272 tests with Rust/Cargo 1.98.0,
  rustfmt 1.9.0-stable and Clippy 0.1.98. All seven host progress tests pass
  in the workspace and an independent package with identical source.
- Implementation `7e1a96738e5cab3b76936cdc2f2c5b45e0895065` passed all
  20 hosted steps in run 43597998608 with 272 workspace tests and the same
  222-line combined host sample.

## Current architecture

One unpublished package owns synchronous lifecycle/work, bounded messaging,
manual time, finite schedules, structured events and optional validated
configuration. Both messaging owners expose read-only subscription checks.
The serial command session selects bounded read, admission, dispatch and
output turns. The periodic host uses one downstream slot and one release
cursor. It shares one finite queue across sample and congestion warnings
and returned work errors.

## Work in progress

The congestion checkpoint has local, independent-package and exact hosted
implementation evidence.
[Focused review](verification/HOST_PROGRESS_REVIEW.md#downstream-congestion-follow-up)

## Highest risks and uncertainties

- Congestion observes occupied host turns on the mission clock. Sample
  progress checks depend on the callback returning. Any queued event can
  consume the available slot and cause a later diagnostic to be rejected.
- Attributing a stop count to specific commands requires complete
  knowledge of outstanding admissions. Input and output retry require
  the same retained state and a caller that continues turning.
- Logical bounds leave process memory and execution time unspecified.

## Important unresolved decisions

MSRV and external I/O protocols remain undecided. Release, platform,
security, major API freeze and other consequential decisions retain
separate gates.

## Most likely next task

Assess a bounded consumer handoff for the three typed diagnostic outcomes,
including how the host reports a rejected event when its sink stalls.

## Latest run

2026-11-15. The first occupied turn at or beyond ten seconds attempts one
congestion warning. The event and queue outcome are returned to the caller.
Observing space re-arms the warning and resumes the retained-release policy.
