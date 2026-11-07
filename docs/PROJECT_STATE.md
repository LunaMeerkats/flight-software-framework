# Project state

Last updated: **2026-11-08**

## Current milestone

Stage 5 practical library use under ADR-0025. The RFF-LIB-REQ-014 revision 2
[command-result follow-up](verification/COMMAND_SESSION_REVIEW.md#retained-command-result-follow-up)
retains one typed result and its byte offset until its writer accepts the
complete line. The reviewed host v0.1 milestone remains the regression
baseline. Package 0.0.0 remains unpublished with provisional APIs.

## Verified baseline

- Input `fb2d728954980e042b7ca1d6af4278ed3b958767` matches refreshed origin.
  Hosted run 42222654426 passes all 20 steps, 257 workspace tests and the
  222-line host sample.
- The locked local baseline passes 258 tests with Rust/Cargo 1.98.0,
  rustfmt 1.9.0-stable and Clippy 0.1.98. The workspace and identical
  independent command session each pass 12 tests and emit the same 24 lines.

## Current architecture

One unpublished package owns synchronous lifecycle/work, bounded messaging,
manual time, finite schedules, structured events and optional validated
configuration. Both messaging owners inspect the membership that publication
uses. A failed attachment returns the original runtime with its applications.
The command session retains one finite issuer, one pending reply, one local
result and two output slots. Its serial host can retain the result line and
retry its suffix while the existing slot gates producer dispatch.

## Work in progress

The retained command-result checkpoint has passed local and independent
verification. Exact hosted results are recorded separately from local evidence.

## Highest risks and uncertainties

- The host supplies configuration corrections, issuing IDs and route
  expectations. Membership is independent of availability and capacity.
- Recovery needs the same presenter and writer in a live serial host. The
  fixed script exits on writer error. Subscriber diagnostics still consume
  the slot before writing. Writer acceptance describes a local byte boundary.
- Logical bounds leave process memory and timing unspecified. Panics, hangs
  and process failures remain outside cooperative containment.

## Important unresolved decisions

MSRV and external I/O protocols remain undecided. Release, platform, security,
major API freeze and other consequential decisions retain separate gates.

## Most likely next task

Assess whether subscriber output needs the same retained-writer boundary.
Check how keeping a partially written line affects independent subscriber
gates and ordering when their diagnostics share one stream.

## Latest run

2026-11-08. An injected short write and three rejected attempts preserve the
typed result and queued command. The suffix completes exactly once, and the
producer dispatches the queued command after the result slot clears.
