# Project state

Last updated: **2026-11-12**

## Current milestone

Stage 5 practical library use under ADR-0025. RFF-LIB-REQ-014 revision 6
[accounts for known command admissions cleared at stop](verification/COMMAND_SESSION_REVIEW.md#command-clearing-follow-up).
The reviewed host v0.1 milestone remains the regression baseline. Package
0.0.0 is unpublished with provisional APIs.

## Verified baseline

- Input `e3884ae1ec883178a01ef9c658dbddb439660e21` matches refreshed origin.
  Hosted run 42822345190 passes all 20 steps with 264 workspace tests.
- The locked local baseline passes 265 tests with Rust/Cargo 1.98.0,
  rustfmt 1.9.0-stable and Clippy 0.1.98. The workspace example passes 19
  tests, including the extended transcript and unexpected-backlog rejection.
- Implementation `1b746222d535eb467236c85c69170f7888576f58` passes all
  20 steps of hosted run 43045074920, with 265 workspace tests and the
  same 222-line combined host sample.

## Current architecture

One unpublished package owns synchronous lifecycle/work, bounded messaging,
manual time, finite schedules, structured events and optional validated
configuration. Both messaging owners inspect the membership used for
publication. Failed attachment returns the original runtime with its
applications. The command session retains one finite issuer, one pending
reply, one local result, two output slots and one saved command. The input
reader retains one two-byte frame and pauses while that command is staged.
The serial host selects read, admission, dispatch and output separately.

## Work in progress

The command-clearing checkpoint is complete with local, independent binary
and hosted implementation evidence. The record revision receives separate
hosted inspection.

## Highest risks and uncertainties

- The reader controls whether a call blocks. The example treats zero as
  finite end of input. The caller decides when to retry a read or abandon
  a saved command.
- Attributing the stop count to specific commands requires knowledge of
  every outstanding admission. Returned callback failures count the
  discarded tail separately from the attempted command.
- A recovering writer needs the same live presenter and writer. A stalled
  line delays other presentation through that writer while slots and
  inboxes bound staged work.
- Logical bounds leave process memory and timing unspecified. Panics,
  hangs and process failures remain outside cooperative containment.

## Important unresolved decisions

MSRV and external I/O protocols remain undecided. Release, platform,
security, major API freeze and other consequential decisions retain
separate gates.

## Most likely next task

Assess whether a downstream supervisor can distinguish missing application
progress across completed host turns from a returned callback failure using
the existing clock, event and lifecycle APIs.

## Latest run

2026-11-12. The fixed scenario matches a producer stop count to two known
New admissions and observes the earlier Retry result after restart.
