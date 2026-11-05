# Project state

Last updated: **2026-11-06**

## Current milestone

Stage 5 practical library use under ADR-0025. RFF-LIB-REQ-015 and
[ADR-0040](adr/0040-read-only-subscription-inspection.md) let a host inspect
copied inbox subscriptions. The command session checks required routes before
startup. The reviewed host v0.1 milestone remains the regression baseline.
Package 0.0.0 remains unpublished with provisional APIs.

## Verified baseline

- Input main `ee1a8ed6cd6dacefc43835caf14d9d013dd5111b` matches fetched
  `origin/main`. Initial locked local baseline passes with Rust/Cargo 1.98.0,
  rustfmt 1.9.0-stable and Clippy 0.1.98.
- Exact input hosted run 41856418810 passes all 20 steps, 253 workspace tests,
  14 focused adapter tests, five focused sample tests and the unchanged
  222-line host sample. Hosted Rust/Cargo 1.99.1 evidence is separate.
- The updated local locked baseline passes 256 tests. The workspace example
  and the identical independent binary pass all 11 command-session tests
  and emit the same 24-line transcript.
- Published implementation `97296afda530bd721c4421bd2aa97d35f6be7ce3`
  passes exact hosted run 42032781940, including all 20 steps and the same
  222-line host sample.
- See the [focused review](verification/COMMAND_SESSION_REVIEW.md#subscription-inspection-follow-up)
  for this increment's exact commands and evidence.

## Current architecture

One unpublished package owns synchronous lifecycle/work, bounded messaging,
manual time, finite schedules, structured events and optional validated
configuration. Both messaging owners inspect the same membership that
publication uses. The command session retains one finite issuer, one pending
reply, a local result slot and two output slots, with explicit host turns.

## Work in progress

The subscription query and startup check are complete. The prior callback and
correlated consumers retain their original tests and ownership boundaries.

## Highest risks and uncertainties

- The host supplies the correct issuing IDs and required route expectations.
  Same-position foreign IDs and other unlisted configuration errors can pass
  this membership check. Membership is independent of availability and capacity.
- Command results and reply output observe distinct local effects. The trusted
  serial host owns drain/retry policy. Taking output before writing it leaves
  continuity across writer failure to the host.
- Logical bounds do not establish process-memory or timing bounds. Panics,
  hangs and process failures remain outside cooperative containment.

## Important unresolved decisions

MSRV and external I/O protocols remain undecided. Release, platform, security,
major API freeze and other consequential decisions retain their separate gates.

## Most likely next task

Assess borrowed setup error recovery. Existing ownership-preserving errors can
keep an application or runtime available for local retry, while examples
currently extract a cause for a one-shot driver. Verify a real recovery path
before considering an API change.

## Latest run

2026-11-06. Local, independent and exact implementation hosted checks pass.
Source and rendered-content review complete. The record revision receives
separate hosted inspection.
