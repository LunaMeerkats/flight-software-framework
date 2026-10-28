# Project state

Last updated: **2026-10-29**

## Current milestone

Reviewed experimental host v0.1 remains the regression baseline. Stage 5 advances
practical library use under ADR-0025. RFF-LIB-REQ-008 composes recurring work,
bounded transfer and deferred host observation in the existing public-only
work_output consumer. Package 0.0.0 remains unpublished and APIs provisional.

## Verified baseline

- Clean input c899aef39a8df854d14b48484bd044ee14bba120 matches refreshed origin.
  Live exact run 40511047519/job 122212738703 passes all 20 steps, 201 workspace
  tests, separate 14 adapter/five sample tests and the unchanged 222-line sample.
  Root raw logs and connector step inspection agree.
- Fresh initial/final locked format/check/Clippy/test/warnings-denied rustdoc and
  both whitespace checks pass, with 201/208 workspace tests. All 17 work_output
  tests and its runnable ordinary/recurring scenarios pass.
- Local Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy 0.1.98 are unchanged.
  Input hosted tools are 1.99.1, rustfmt 1.10.0-stable/Clippy 0.1.99, runner
  2.338.0/windows-2025-vs2026 image 20261021.278.1. Local checks do not claim
  execution on those hosted tools. Actual current evidence is retained in
  [the focused review](verification/HOST_COORDINATION_REVIEW.md).

## Current architecture

One unpublished package owns synchronous LC1 lifecycle/work, bounded inbox
routing/dispatch, manual time, finite one-shot scheduling, bounded events and
optional configuration with immutable work visibility/one-use rollback. All work
shares selected identity/context construction. PeriodicWork supplies one
observed-time release for host delegation. EventFilter applies optional severity
admission; failure reports remain unfiltered. The separate event presenter owns
one writer and retained line/cursor. The work-output host turn guards dispatch,
transfers pending output and polls recurrence only when production has space.
It holds no hidden worker and changes no library contract or dependency.

## Work in progress

RFF-LIB-REQ-008/ADR-0033 precede implementation. Source and seven new regressions
pass the full local baseline and identical independent-package verification.
Root/independent Codex complete source/document/diff review accepts with no
remaining findings. Ordinary publication and exact hosted inspection follow. No additional capability is selected for this run.

## Highest risks and uncertainties

- Controlled consumers do not establish external adoption or broad usability.
  The new host policy is serial, nontransactional and limited to one subscriber.
  Paused polling provides no fairness or drain deadline. Earlier effects survive
  a later error. Physical/durable output requires separate evidence and policy.
- Application/clock origin and slot pairing remain caller duties. Logical bounds
  do not bound process memory, allocation, blocking or callback time. Panics,
  hangs, allocator exhaustion and process failure remain uncontained.
- Finite models prove frozen domains. Hosted tools/images float; source/doc
  review and conditional probes remain outside the CI job.

## Important unresolved decisions

Major API freeze, unsafe/runtime/code-generation/dependency/security decisions,
platform commitments, identity/licences, tags/releases/crate publication and
deployment retain separate gates. MSRV, external I/O protocols, hardware, RTOS
and no_std remain undecided. Codex review is not human acceptance or qualification.

## Most likely next task

Assess a concrete multi-producer or selective fan-out consumer before choosing
further host policy or library changes. Compare existing per-destination reports
with retry ownership needs; do not add a generic runner or more models by default.

## Latest run

2026-10-29: implemented bounded host coordination to prevent ordinary downstream
backpressure from invoking full producer/sink callbacks. Seven new regressions
and the runnable four-value recurring scenario pass. Identical independent
consumer and complete root/independent review pass; exact publication/hosted
evidence follows actual execution.
