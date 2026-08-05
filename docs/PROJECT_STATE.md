# Project state

Last updated: **2026-08-06**

## Current milestone

Stage 1 is in progress: one bounded application-lifecycle vertical slice. The
logical LC1 model and owned start boundary are implemented; owned work, stop,
and restart behavior are not.

## Verified baseline

- Commit `1696f65a87f9a06ccb80894d2b07ffa211bdc991` contains the
  start-only owned runtime and exact partial RFF-REQ-002/RFF-REQ-008 evidence.
- Formatting, linting with warnings denied, all nine tests, and documentation
  generation pass on rustc/cargo 1.96.1.
- Relative links resolve across all 16 Markdown files; all eight requirement
  identifiers match traceability; every referenced source identifier is
  defined.

## Current architecture

- ADR-0001 selects a provisional caller-driven, serial host runtime.
- ADR-0003 defines four stable LC1 states and stop-gated restart with
  runtime-local identity.
- ADR-0006 selects finite-capacity static application ownership and concrete
  returned start errors for the first owned runtime slice.
- ADR-0004 defines bounded application inbox behavior, and ADR-0005 defines
  configuration revision/rollback behavior; neither service is implemented.

The unpublished, dependency-free library retains a standalone
`LifecycleRegistry` for logical transition evidence. `Runtime<A>` owns bounded
application records, invokes `Application::start` synchronously, commits
`Running` only on success, and commits terminal `Failed` while returning the
concrete application error on cooperative failure. It creates no threads or
executor and has no work dispatch, stop/restart callbacks, service context,
message bus, clock, events, or configuration service.

## Work in progress

No implementation work is in progress. The start-only owned runtime increment
is at its intended stopping point.

## Highest risks and uncertainties

- The start-only public trait may need a recorded pre-v0.1 revision when stop,
  restart, work, or service-context behavior is demonstrated.
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
- Owned stop, restart, work, and future service-context shapes are not selected.
- RFF-REQ-007 still needs a host-adapter grammar and validation boundary.
- No explicit minimum supported Rust version or panic-containment policy has
  been selected.

## Most likely next tasks

1. Add a synchronous owned stop operation proving `Running -> Stopped`, no
   callback on invalid transitions, and terminal `Failed` on a returned error.
2. Add in-place owned restart only after stop behavior is coherent and tested.
3. Add caller-driven work and demonstrate a returned work error while a peer
   remains operable, without claiming arbitrary fault containment.

## Latest run

2026-08-06: Added a finite-capacity generic runtime and a start-only application
boundary. Tests use two independently defined application types in an explicit
mission enum and verify success, exact returned-error preservation, terminal
`Failed`, peer start, invalid/unknown callback suppression, and ownership return
on capacity rejection. Formatting, lint, nine tests, documentation, links,
identifier consistency, and diff checks passed; no broader runtime behavior is
claimed.
