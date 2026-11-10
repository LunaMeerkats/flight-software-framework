# Project state

Last updated: **2026-11-11**

## Current milestone

Stage 5 practical library use under ADR-0025. RFF-LIB-REQ-014 revision 5
[composes finite framed input](verification/COMMAND_SESSION_REVIEW.md#framed-input-follow-up)
with the saved command slot. The reviewed host v0.1 milestone remains the
regression baseline. Package 0.0.0 is unpublished with provisional APIs.

## Verified baseline

- Input `aaf4b92a15a7a53448e0a59169f87890931fee24` matches refreshed origin.
  Hosted run 42622569988 passes all 20 steps, 260 workspace tests and the
  222-line combined host sample.
- The locked local baseline passes 264 tests with Rust/Cargo 1.98.0,
  rustfmt 1.9.0-stable and Clippy 0.1.98. The workspace example and identical
  independent binary pass 18 tests and preserve the 40-line transcript.
- Implementation `12931b7e936b35bd0f4c9ffd15510213a6629efe` passes all 20
  steps of hosted run 42822018637 with 264 workspace tests and the unchanged
  222-line combined host sample.

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

The framed-input checkpoint is complete with local, independent binary
and hosted implementation evidence. The record revision receives separate
hosted inspection.

## Highest risks and uncertainties

- The reader controls whether a call blocks. The example treats zero as
  finite end of input. The caller decides when to retry a read or abandon
  a saved command. Inbox acceptance can precede lifecycle cleanup.
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

Inspect a downstream application that needs to distinguish a handled
command from one cleared during lifecycle transition. Decide whether
existing dispatch and stop evidence is sufficient for that host.

## Latest run

2026-11-11. The fixed scenario stages a New command from a finite reader.
Tests verify short-read recovery, full-inbox backpressure without read-ahead,
retained identity, malformed records, exhausted IDs and terminal input.
