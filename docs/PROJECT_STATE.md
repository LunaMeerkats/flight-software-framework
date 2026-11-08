# Project state

Last updated: **2026-11-09**

## Current milestone

Stage 5 practical library use under ADR-0025. The RFF-LIB-REQ-014 revision 3
[subscriber follow-up](verification/COMMAND_SESSION_REVIEW.md#retained-subscriber-output-follow-up)
retains a typed reply and its byte offset until its writer accepts the complete
line. The reviewed host v0.1 milestone remains the regression baseline.
Package 0.0.0 remains unpublished with provisional APIs.

## Verified baseline

- Input `785c6610260c05c66b3c52207487084eb085a715` matches refreshed origin.
  Hosted run 42375973171 passes all 20 steps, 258 workspace tests and the
  222-line host sample.
- The locked local baseline passes 259 tests with Rust/Cargo 1.98.0,
  rustfmt 1.9.0-stable and Clippy 0.1.98. The workspace and identical
  independent command session each pass 13 tests and emit the same 24 lines.
- Implementation `70180da0af12745b666962def0202f655a76ced4` passes all 20
  steps of hosted run 42483458584, with 259 workspace tests and the unchanged
  222-line host sample.

## Current architecture

One unpublished package owns synchronous lifecycle/work, bounded messaging,
manual time, finite schedules, structured events and optional validated
configuration. Both messaging owners inspect the membership that publication
uses. A failed attachment returns the original runtime with its applications.
The command session retains one finite issuer, one pending reply, one local
result and two output slots. One active presenter retains the writer and
retry offset while the selected slot gates dispatch. Peer callbacks can
stage output in their own slots during a pending subscriber line.

## Work in progress

The retained subscriber checkpoint is complete. The implementation has local,
independent and exact hosted evidence. The record revision receives separate
hosted inspection.

## Highest risks and uncertainties

- The host supplies configuration corrections, issuing IDs and route
  expectations. Membership is independent of availability and capacity.
- Recovery needs the same presenter and writer in a live serial host. The
  fixed script exits on writer error. A stalled line delays further writes
  through that object, while existing slots and inboxes bound staged work.
- Logical bounds leave process memory and timing unspecified. Panics, hangs
  and process failures remain outside cooperative containment.

## Important unresolved decisions

MSRV and external I/O protocols remain undecided. Release, platform, security,
major API freeze and other consequential decisions retain separate gates.

## Most likely next task

Assess how a bounded host retains a rejected command while occupied output
gates delay dispatch. Use the existing rejection/FIFO evidence to decide
whether a runnable recovery turn closes a distinct consumer gap.

## Latest run

2026-11-09. A short write and rejected attempts preserve the selected reply.
The peer stages independently. Completing the suffix releases only the
selected gate, and both lines reach the writer in order.
