# Project state

Last updated: **2026-11-19**

## Current milestone

Stage 5 practical library use under ADR-0025. RFF-LIB-REQ-017 revision 2
extends the [periodic peer example](PERIODIC_PEER.md) with two fixed retry
slots for independent queue-rejected work failures. Healthy work remains
selectable throughout. The reviewed host v0.1 milestone is the regression
baseline. Package 0.0.0 is unpublished with provisional APIs.

## Verified baseline

- Clean input `c2da285952d99710adb61eb76ca872213e32edbd` matched refreshed
  origin. Exact hosted run 44089559879 passed 20 steps and 278 workspace tests.
- The locked local baseline passes 281 tests on Rust/Cargo 1.98.0,
  rustfmt 1.9.0-stable and Clippy 0.1.98. Five focused tests and the
  runnable trace pass with identical source in an independent binary.
- Local formatting, all-target check/Clippy, warnings-denied rustdoc and
  whitespace pass. The combined host sample matches 222 input output lines.
- Implementation `4e37bced92142092193b1bb334984c9d427d4ce6` passed all
  20 steps in exact hosted run 44311260522 with 281 workspace tests.

## Current architecture

One unpublished package owns synchronous lifecycle/work, bounded messaging,
manual time, finite schedules, structured events and optional validated
configuration. Serial command-session turns separate input, admission,
dispatch and output. The host-progress consumer retains one rejected failure
and one diagnostic awaiting a fallible sink. The periodic peer consumer has
three independent cursors and a capacity-one event FIFO. Two inline retry
slots retain the two samplers' rejected terminal events in observation order.
Retry offers only the oldest once; heartbeat work remains caller-selected.

## Work in progress

Codex accepted the two-failure retry checkpoint after local, independent
consumer, complete source/document and exact hosted verification.
[Focused review](verification/PERIODIC_PEER_REVIEW.md#two-failure-retry-follow-up)

## Highest risks and uncertainties

- The two-event bound depends on two terminally failing roles and permanently
  disabled cursors. Changing roles or adding recovery requires a new policy.
- Retained events retry in observation order; the FIFO orders acceptance.
  Selecting fresh work before retry can admit a newer event first.
- Continued serial progress depends on returning callbacks/sink calls and
  caller-selected retry. Logical bounds do not bound bytes or execution time.
- Attributing discarded commands requires complete serial admission knowledge.

## Important unresolved decisions

MSRV and external I/O protocols remain undecided. Release, platform,
security, major API freeze and other consequential decisions retain gates.

## Most likely next task

Assess whether a downstream consumer needs to service retained diagnostics
before fresh failure-producing work, and what that policy does to peer work.

## Latest run

2026-11-19. Two rejected terminal failures retain their exact identities and
timestamps through repeated full retries, then enter the FIFO once each.
