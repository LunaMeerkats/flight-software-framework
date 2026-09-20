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

Status: **Complete (2026-09-12)**

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

ADR-0019 now implements the command-ingest and telemetry-output host adapter
pair in the private `host-echo` example. The codec and two applications share
source with integration tests covering malformed records, explicit dispatch,
publication diagnostics, bounded host output, terminal saturation failure,
lifecycle retention, and replay. The host-owned borrowed mailbox is drained
outside callbacks. RFF-REQ-007 has adapter-boundary evidence; the original
ownership probe remains separate. ADR-0022 now composes and documents the
complete service sample, including the narrow returned-work failure/event
scenario, using that same private mission source.

ADR-0020 adds the prerequisite opt-in failure-event operation through the
messaging owner. It completes ordinary-work lifecycle commitment and exact
selected-inbox clearing before reading the borrowed clock and attempting one
event, retaining the existing nested error types.

ADR-0021 adds messaging-owned one-shot scheduling through ordinary work. Both
owners share the private clock/consumption decision; messaging errors retain
the consumed item, observed instant, original error, and exact selected-inbox
discard count. Later due peers retain their queues and can progress. This
completes the recorded scheduling prerequisite without adding scheduled events.

ADR-0022 completes the combined host scenario: two-application lifecycle,
command/telemetry exchange, finite scheduling under manual time, fresh work
configuration observations through activation/rejection/rollback, and a
cooperative work failure with exact inbox cleanup, event, and peer progress.
Five shared-driver integration tests and the executed binary establish the
sample behavior. CI and human release reviews remain in Stage 4.

Exit evidence: hostile-input and rollback tests for RFF-REQ-006,
command/telemetry adapter tests for RFF-REQ-007, and sample-mission
re-demonstration of the narrow RFF-REQ-008 behavior.

## Stage 4 — v0.1 integration and review

ADR-0023 configures one Windows GitHub Actions job for the existing baseline
and sample. Local command replay and workflow validation are a configuration
checkpoint; the first [hosted run](verification/CI_BASELINE.md) subsequently
passes the baseline and sample at `abb1293790136128d5f27d8c48c1e3d98a355540`.
The user's 2026-09-12 approval now authorizes verified source pushes without
per-push human review under AGENTS.md. Hosted results must be recorded
separately; source publication does not complete v0.1 release review.

- Run the complete sample mission and verification suite in CI.
- Audit all resource bounds, failure paths, dependency licences, public APIs,
  provenance, and unsupported claims.
- Record a v0.1 architecture review before widening scope.

The 2026-09-13 [dependency and scope checkpoint](verification/DEPENDENCY_SCOPE_REVIEW.md)
records an empty external Cargo dependency/feature graph, unchanged approved
licences, separate host/CI tool dependencies, and an autonomous entry-point
scope inventory. Crate notice omissions and stale sample CI wording are
corrected. This completes the bounded dependency/features review and prepares
scope evidence. The 2026-09-14
[messaging construction review](verification/MESSAGING_CONSTRUCTION_REVIEW.md)
adds exact later-inbox reservation-overflow and returned-runtime reuse evidence.
This closes one constructor boundary. The 2026-09-15
[returned-message failure review](verification/MESSAGE_FAILURE_REVIEW.md)
examines immediate peer publication, selected cleanup, complete retained FIFO,
and post-failure callback availability. The 2026-09-16
[schedule construction review](verification/SCHEDULE_CONSTRUCTION_REVIEW.md)
adds first-descent diagnostics and copied-agenda ownership observations.
The 2026-09-20 [event-queue review](verification/EVENT_QUEUE_REVIEW.md) adds
exact capacity-overflow rejection and repeated saturation/reuse observations
at logical capacities one and three without changing the queue policy.
The 2026-09-21
[lifecycle construction review](verification/LIFECYCLE_CONSTRUCTION_REVIEW.md)
adds exact reservation-overflow diagnostics through both the standalone
registry and unconfigured owned runtime constructors. Existing zero-capacity,
logical saturation, returned-application ownership, and configured-runtime
table-lineage evidence remains unchanged. Other lifecycle/service resource,
failure, and public-API reviews plus human entry-point/architecture acceptance
remain open. Stage 4 is not complete.

After v0.1, prefer hardening, property tests, fuzzing where byte parsers exist,
concurrency analysis, API simplification, and measured portability experiments
before adding a large feature surface.
