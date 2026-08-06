# Project state

Last updated: **2026-08-07**

## Current milestone

Stage 1 is in progress: one bounded application-lifecycle vertical slice. The
logical LC1 model and owned synchronous start/stop/in-place-restart boundaries
are implemented; application work is not.

## Verified baseline

- Commit `f957233e790058af20a49ffdd4b9e637c21ef195` contains the owned
  restart implementation and exact partial RFF-REQ-002/RFF-REQ-008 evidence.
- Formatting, linting with warnings denied, all 13 tests, and documentation
  generation pass on rustc/cargo 1.96.1.
- All 31 relative links resolve across 18 Markdown files; all eight requirement
  identifiers match traceability; every referenced source identifier is
  defined.
- Source-structure review of the README, project state, ADR-0008, and
  traceability found the expected headings, lists, and table shape. A fresh
  browser-rendered Markdown review was blocked because the available browser
  policy rejects local rendered content; the previous documents' rendered
  baseline remains historical evidence only.

## Current architecture

- ADR-0001 selects a provisional caller-driven, serial host runtime.
- ADR-0003 defines four stable LC1 states and stop-gated restart with
  runtime-local identity.
- ADR-0006 selects finite-capacity static application ownership and concrete
  returned start errors; ADR-0007 and ADR-0008 extend that pre-v0.1 boundary
  with synchronous stop and in-place restart plus distinct concrete errors.
- ADR-0004 defines bounded application inbox behavior, and ADR-0005 defines
  configuration revision/rollback behavior; neither service is implemented.

The unpublished, dependency-free library retains a standalone
`LifecycleRegistry` for logical transition evidence. `Runtime<A>` owns bounded
application records and invokes `Application::start`, `Application::stop`, and
`Application::restart` synchronously after lifecycle validation. Successful
callbacks commit `Running`, `Stopped`, and `Running` respectively. Restart
mutably borrows the same retained application value. Any returned concrete
error is preserved in an operation-specific caller result while the selected
record enters terminal `Failed`; peer records are unchanged. The runtime creates
no threads or executor and has no work dispatch, service context, message bus,
clock, events, or configuration service.

## Work in progress

No implementation work is in progress. The synchronous owned restart increment
is at its intended stopping point.

## Highest risks and uncertainties

- The pre-v0.1 public trait may need a recorded revision when work or service-
  context behavior is demonstrated.
- A returned stop or restart error may follow partial application-internal
  cleanup or mutation. The runtime records `Failed` but does not prove rollback,
  cleanup, reinitialisation, or isolation.
- Three operation-specific error wrappers duplicate a small amount of display
  and source plumbing; work evidence may or may not justify a shared shape.
- Static mission composition sizes each runtime record to its concrete
  representation's largest variant; record capacity does not bound allocation
  inside application or error values.
- IDs are valid only with their origin registry/runtime, but the opaque index
  cannot detect a numerically aliased ID from another issuer.
- A returned application error does not contain a panic, hang, process failure,
  memory exhaustion, or hardware fault.
- Queue-slot limits will not constitute memory bounds until message payloads
  receive a separate enforced bound.

## Important unresolved decisions

- The exact copyright-holder text is required before adding the approved MIT
  and Apache-2.0 licence files and Cargo licence expression.
- Application work and future service-context shapes are not selected.
- RFF-REQ-007 still needs a host-adapter grammar and validation boundary.
- No explicit minimum supported Rust version or panic-containment policy has
  been selected.
- No CI configuration exists; all current verification is local.

## Most likely next tasks

1. Add caller-driven work and demonstrate a returned work error while a peer
   remains operable, without claiming arbitrary fault containment.
2. Reassess whether operation-specific error wrappers remain clearer than a
   shared abstraction after restart and work provide evidence.
3. Begin the bounded inbox vertical slice only after the running-work boundary
   supplies a concrete dispatch consumer.

## Latest run

2026-08-07: Added synchronous in-place owned restart. Tests verify retained
application state across start/stop/restart, exact concrete returned-error
preservation, terminal `Failed`, peer restart after failure, and callback
suppression for registered/running/failed/unknown requests. Formatting, lint,
13 tests, documentation, links, identifier consistency, Markdown source
structure, and diff checks passed. Browser-rendered Markdown review was blocked
by the local-content URL policy; application work remains unimplemented.
