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

Status: **Complete (2026-08-23)**

- Create the smallest stable-Rust crate structure needed for one runtime and
  independently defined test applications.
- Implement explicit identity and lifecycle states for registration and start.
- Extend through stop/restart only when transition semantics are tested.
- Use no external runtime or framework dependency unless a recorded need arises.

Completed slices: the single unpublished library provides a bounded logical
lifecycle registry with exhaustive LC1 transition evidence, plus a
finite-capacity generic runtime that owns statically composed applications. The
runtime executes start, caller-selected work, stop, and in-place restart
synchronously, preserves distinct concrete operation errors, commits terminal
`Failed` on any returned error, suppresses callbacks for rejected operations,
and leaves peer records unchanged. Tests observe retained application state,
successful subsequent peer work after a returned work error, and two
independently defined applications completing registration, start, work, stop,
restart, and work.

Exit evidence: applicable Cargo checks and complete transition tests for the
implemented slice plus the complete two-application host scenario. RFF-REQ-002
is verified by the exhaustive logical transition matrix and public integration
evidence. No dependency, thread, executor, service context, or automatic
dispatch was added.

### Source-quality checkpoint before Stage 2

Status: **Complete (2026-08-23)**

The first whole-tree source-form audit found no handwritten Rust line over 100
columns, eight comment-only lines over the 80-column review default, and five
cohesive chronological integration tests over the proposed 60-line Clippy
threshold. Tracked stable rustfmt and Clippy configuration now enforce the
100-column format and individual function-size lint. The comments were
reflowed, and the five initial test findings received narrow reasoned
expectations instead of count-driven fragmentation. A follow-up source-order
cleanup split two independently meaningful behaviors and simplified one
identity fixture, retiring two expectations; three cohesive chronological
scenarios remain explicitly justified.

The contributor policy now records progressive source ordering, module and
naming conventions, waiver retirement, manual review boundaries, exact tool
versions, and a warning-free baseline. Official Rust, JPL, NASA, ECSS, JAXA,
and Australian public-source decisions are recorded without claiming agency
applicability or compliance. Other lints, a strict physical-line checker,
complexity tooling, dependency policy, CI, and toolchain pinning remain
separate measured increments.

## Stage 2 — Bounded interaction under controlled time

Status: **Implementation not started**

- Add publish/subscribe fan-out with an explicit finite capacity and overflow
  contract.
- Add structured events without creating an unbounded side channel.
- Add injected and simulated time, then caller-driven scheduled work.

Next slice: reorient from ADR-0004 and implement the smallest coherent bounded
inbox and publish/subscribe behavior that can feed the existing caller-driven
work boundary, with exact capacity and publisher-visible saturation evidence.

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
