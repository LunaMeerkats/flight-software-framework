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

Status: **Complete (2026-08-30)**

- Add publish/subscribe fan-out with an explicit finite capacity and overflow
  contract.
- Add structured events without creating an unbounded side channel.
- Add injected and simulated time, then caller-driven scheduled work.

Completed messaging slices: the standalone routing core uses inline const-bounded
payloads, immutable unique topic sets, pre-reserved positive-capacity inboxes,
cross-topic FIFO, reject-newest saturation, continued partial fan-out, and
ordered publisher-visible classifications. The owning integration constructs
one fresh inbox per still-registered runtime application, makes only `Running`
endpoints available, returns exact clearing counts after stop or returned
callback error, and reconnects an empty inbox after successful restart from
`Stopped`. Caller-selected dispatch now removes at most one oldest message for
a running application and supplies a publish-only context. Tests prove bounded
self-publication, refreshed lifecycle availability, explicit empty and rejected
dispatch, and non-transactional peer delivery when a callback later fails.
RFF-REQ-003 is verified by the combined evidence.

Completed event slices: machine-inspectable records enter a standalone positive-
capacity FIFO queue with direct reject-newest saturation reporting. An opt-in
direct runtime work operation now constructs one clock-captured application
error event after a cooperative returned work error, preserves the complete
original error and exact event attempt under saturation, and leaves a peer able
to complete later work. Ordinary work, lifecycle rejection, and other callback
paths emit no event.

Completed time and scheduling slices: a general `FrameworkInstant` and injected
`Clock` boundary have one manually advanced implementation. Reads never move
time; checked overflow is typed and non-mutating; and `EventTimestamp` can
capture the injected reading. A finite `WorkSchedule` now preserves configured
order for equal elapsed instants, and one caller request attempts at most one
due or overdue item through the running-only work boundary. Identical manual
scenarios reproduce scheduled outcomes, application work order, lifecycle
states, and clock-captured event timestamps. RFF-REQ-004 is verified. RFF-REQ-005
is verified at the direct cooperative returned-work failure boundary. The same
failed-state, event, original-error, saturation, and peer-progress scenario
verifies RFF-REQ-008 at that narrow boundary.

Exit evidence: exact-boundary, ordering, saturation, and unavailable/clearing
tests for RFF-REQ-003; structured-event and simulated-time integration tests
for RFF-REQ-004 and RFF-REQ-005; and direct cooperative fault-injection evidence
for RFF-REQ-008. No application-authored events, other callback event path,
panic or hang containment, guaranteed event delivery, thread, or executor was
added.

## Stage 3 — Configuration and mission boundaries

Status: **In progress**

- Add validated, versioned configuration activation and rollback.
- Add one command-ingest and telemetry-output host adapter pair.
- Integrate and re-demonstrate the already verified direct cooperative
  application-error behavior in the sample mission.

The configuration core validates const-bounded byte snapshots, preserves state
on rejection, assigns never-reused revisions, and restores one consume-once
rollback snapshot. ADR-0018 now integrates that table as optional
constructor-owned runtime state. One immutable ordinary-work context exposes
the active revision and bytes through direct, scheduled, failure-event, and
messaging-owned work. Tests cover construction ownership recovery, absence,
safe-point activation/rejection/rollback visibility, restart retention, no
automatic error rollback, peer progress, event behavior, and inbox cleanup.
Together with the standalone transition and exhaustion tests, this verifies
RFF-REQ-006. No schema, host loading adapter, or wire format has been selected.

The next Stage 3 boundary is the command-ingest and telemetry-output host
adapter pair. ADR-0019 selects a two-byte validated echo command and matching
telemetry, with a borrowed capacity-one host mailbox drained outside callbacks.
The executable mailbox probe checks ownership and full-output behavior; the
codec, adapter pair, and sample mission remain unimplemented. Next implement
one shared mission source with malformed-input and end-to-end evidence.

Exit evidence: hostile-input and rollback tests for RFF-REQ-006,
command/telemetry adapter tests for RFF-REQ-007, and sample-mission
re-demonstration of the narrow RFF-REQ-008 behavior.

## Stage 4 — v0.1 integration and review

- Run the complete sample mission and verification suite in CI.
- Audit all resource bounds, failure paths, dependency licences, public APIs,
  provenance, and unsupported claims.
- Record a v0.1 architecture review before widening scope.

After v0.1, prefer hardening, property tests, fuzzing where byte parsers exist,
concurrency analysis, API simplification, and measured portability experiments
before adding a large feature surface.
