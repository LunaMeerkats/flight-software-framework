# Project state

Last updated: **2026-11-10**

## Current milestone

Stage 5 practical library use under ADR-0025. The RFF-LIB-REQ-014 revision 4
[ingress follow-up](verification/COMMAND_SESSION_REVIEW.md#retained-ingress-follow-up)
shows how a serial host retains one command through ingress rejection and
admits it when a place opens in the inbox. The reviewed host v0.1 milestone
remains the regression baseline. Package 0.0.0 is unpublished with provisional
APIs.

## Verified baseline

- Input `9dbab6ad0432902f319af304d64c35232bc2a2ea` matches refreshed origin.
  Hosted run 42483635231 passes all 20 steps, 259 workspace tests and the
  222-line combined host sample.
- The locked local implementation baseline passes 260 tests with Rust/Cargo
  1.98.0, rustfmt 1.9.0-stable and Clippy 0.1.98. The workspace example and
  identical independent binary pass 14 tests and emit identical 40-line
  transcripts, preserving the exact 24-line input prefix.
- Implementation `b5e5ce0c0b48e23f4668fb512f3eab2094f4fc31` passes all 20
  steps of hosted run 42622185103 with 260 workspace tests and the unchanged
  222-line combined host sample.

## Current architecture

One unpublished package owns synchronous lifecycle/work, bounded messaging,
manual time, finite schedules, structured events and optional validated
configuration. Both messaging owners inspect the membership that publication
uses. A failed attachment returns the original runtime with its applications.
The command session retains one finite issuer, one pending reply, one local
result, two output slots and one caller-owned saved command. The result and
reply gates remain occupied while a presenter holds the writer and retry
offset. The host selects admission and dispatch independently.

## Work in progress

The retained ingress checkpoint is complete with local, independent and
exact hosted implementation evidence. The record revision receives separate
hosted inspection.

## Highest risks and uncertainties

- The caller owns intake order and chooses when to retry or abandon a saved
  command. Acceptance describes enqueue. Lifecycle cleanup may discard
  accepted copies before dispatch.
- A recovering writer needs the same live presenter and writer. The fixed
  script exits on writer error. A stalled line delays other presentation
  through that writer while slots and inboxes bound staged work.
- Logical bounds leave process memory and timing unspecified. Panics, hangs
  and process failures remain outside cooperative containment.

## Important unresolved decisions

MSRV and external I/O protocols remain undecided. Release, platform, security,
major API freeze and other consequential decisions retain separate gates.

## Most likely next task

Assess a concrete downstream handoff between the finite command session and
a fallible host input boundary. Compare a focused integration recipe with
leaving intake entirely caller-owned before introducing another queue or API.

## Latest run

2026-11-10. A full inbox leaves one command with the caller. Dispatch of an
older command opens one place. An explicit retry admits the same ID behind
the remaining queued work, and the callback result and replies follow later.
