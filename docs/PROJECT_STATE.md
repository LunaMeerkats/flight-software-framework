# Project state

Last updated: **2026-11-07**

## Current milestone

Stage 5 practical library use under ADR-0025. The RFF-LIB-REQ-001
[quickstart follow-up](verification/QUICKSTART_REVIEW.md#borrowed-setup-recovery-follow-up)
shows how to recover a borrowed application from a rejected inbox attachment
before startup. The reviewed host v0.1 milestone remains the regression
baseline. Package 0.0.0 remains unpublished with provisional APIs.

## Verified baseline

- Input `e100be6e06439e394c665466808f758de96f6eb1` matches fetched origin.
  Exact hosted run 42033160214 passes all 20 steps, 256 workspace tests,
  focused adapters and sample checks, and the 222-line host sample.
- The updated locked local baseline passes 257 tests with Rust/Cargo 1.98.0,
  rustfmt 1.9.0-stable and Clippy 0.1.98. The workspace and identical
  independent quickstart each pass four tests and emit identical output.

## Current architecture

One unpublished package owns synchronous lifecycle/work, bounded messaging,
manual time, finite schedules, structured events and optional validated
configuration. Both messaging owners inspect the same membership that
publication uses. A failed attachment returns the original still-owned
runtime to its caller. The command session retains one finite issuer, one
pending reply, a local result slot and two output slots, with explicit host turns.

## Work in progress

The quickstart's pre-start retry path is verified locally and in the
independent package. The normal one-shot driver still exits on setup failure.

## Highest risks and uncertainties

- The host supplies configuration corrections, issuing IDs and required route
  expectations. Membership is independent of availability and capacity.
- Command results and reply output observe distinct local effects. The serial
  host owns drain/retry policy. Taking output before writing it leaves
  continuity across writer failure to the host.
- Logical bounds do not establish process-memory or timing bounds. Panics,
  hangs and process failures remain outside cooperative containment.

## Important unresolved decisions

MSRV and external I/O protocols remain undecided. Release, platform, security,
major API freeze and other consequential decisions retain their separate gates.

## Most likely next task

Assess whether a recoverable command-session writer error needs retained local
output. Use the event-output writer cursor as comparison evidence and isolate
one explicit short-write or error boundary before expanding the host driver.

## Latest run

2026-11-07. A corrected inbox allows the same registered application to deliver
to its original host slot. Local and independent verification pass. The
focused record separates hosted evidence from local results.
