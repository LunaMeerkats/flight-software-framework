# Project state

Last updated: **2026-11-14**

## Current milestone

Stage 5 practical library use under ADR-0025. RFF-LIB-REQ-016 revision 2
[accounts for deliberate downstream deferral](HOST_PROGRESS.md#defer-periodic-work-when-the-downstream-slot-is-full)
in periodic sample observation.
The reviewed host v0.1 milestone remains the regression baseline. Package
0.0.0 is unpublished with provisional APIs.

## Verified baseline

- Input `93118f295cc89c9374e3429f2775939d6213f22b` matched refreshed origin.
  Hosted run 43367462734 passed all 20 steps with 268 workspace tests.
- The locked local baseline passes 270 tests with Rust/Cargo 1.98.0,
  rustfmt 1.9.0-stable and Clippy 0.1.98. All five host progress tests pass
  in the workspace and an independent package with identical source.

## Current architecture

One unpublished package owns synchronous lifecycle/work, bounded messaging,
manual time, finite schedules, structured events and optional validated
configuration. Both messaging owners expose read-only subscription checks.
The serial command session selects bounded read, admission, dispatch and
output turns. The periodic progress consumer composes one downstream slot
and one periodic cursor with the direct progress observer and runtime work
error path.

## Work in progress

The periodic progress checkpoint has local and independent-package evidence.
The implementation revision receives exact hosted inspection.
[Focused review](verification/HOST_PROGRESS_REVIEW.md#periodic-deferral-follow-up)

## Highest risks and uncertainties

- A serial progress check depends on the selected callback returning. The
  host defines expected output and chooses when to observe it. Event queue
  saturation can reject a warning. The periodic sample window excludes
  observed deferral, so a long blocked interval needs its own diagnostic
  if the mission requires one.
- Attributing a stop count to specific commands requires complete
  knowledge of outstanding admissions. Input and output retry require the
  same retained state and a caller that continues turning.
- Logical bounds leave process memory and execution time unspecified.

## Important unresolved decisions

MSRV and external I/O protocols remain undecided. Release, platform,
security, major API freeze and other consequential decisions retain
separate gates.

## Most likely next task

Assess whether the same periodic host needs a distinct diagnostic for a
downstream slot that stays occupied across turns.

## Latest run

2026-11-14. Repeated deferred turns retain the staged sample and due release.
Late resumption preserves the time accrued before deferral. A returned
error at the adjusted threshold retains its failure event and disables the
consumer cursor.
