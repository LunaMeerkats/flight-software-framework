# Roadmap

The roadmap is evidence-driven. A later stage may be reordered when repository
evidence changes, but scope changes must be recorded rather than implied.

## Stage 0 — Foundation and responsibility checkpoint

Status: **Complete (2026-08-05)**

- Record charter, limitations, provenance, v0.1 behavior, and traceability.
- Decide the first execution model and its revisit conditions.
- Defer Cargo structure until the first vertical slice defines a real boundary.

Exit evidence: internally consistent documents, resolving relative links, clean
diff checks, and a local commit on `codex/nightly`.

## Stage 1 — One lifecycle vertical slice

Status: **In progress**

- Create the smallest stable-Rust crate structure needed for one runtime and
  independently defined test applications.
- Implement explicit identity and lifecycle states for registration and start.
- Extend through stop/restart only when transition semantics are tested.
- Use no external runtime or framework dependency unless a recorded need arises.

Completed slices: the single unpublished library provides a bounded logical
lifecycle registry with exhaustive LC1 transition evidence, plus a
finite-capacity generic runtime that owns statically composed applications. The
runtime executes start synchronously, preserves a returned concrete start error,
commits terminal `Failed`, and permits a peer application to start afterward.

Next slice: add the smallest owned stop operation that can prove
`Running -> Stopped`, callback suppression for invalid transitions, and terminal
`Failed` on a returned stop error without adding work dispatch, restart,
factories, threads, or async execution.

Exit evidence: applicable Cargo checks and complete transition tests for the
implemented slice. This stage begins RFF-REQ-002; it need not claim the entire
requirement before all transitions exist.

## Stage 2 — Bounded interaction under controlled time

- Add publish/subscribe fan-out with an explicit finite capacity and overflow
  contract.
- Add structured events without creating an unbounded side channel.
- Add injected and simulated time, then caller-driven scheduled work.

Exit evidence: exact-boundary, ordering, saturation, disconnect, and simulated-
time integration tests for RFF-REQ-003 through RFF-REQ-005.

## Stage 3 — Configuration and mission boundaries

- Add validated, versioned configuration activation and rollback.
- Add one command-ingest and telemetry-output host adapter pair.
- Demonstrate defined application-error containment in the sample mission.

Exit evidence: hostile-input, rollback, and fault-injection tests for
RFF-REQ-006 through RFF-REQ-008.

## Stage 4 — v0.1 integration and review

- Run the complete sample mission and verification suite in CI.
- Audit all resource bounds, failure paths, dependency licences, public APIs,
  provenance, and unsupported claims.
- Record a v0.1 architecture review before widening scope.

After v0.1, prefer hardening, property tests, fuzzing where byte parsers exist,
concurrency analysis, API simplification, and measured portability experiments
before adding a large feature surface.
