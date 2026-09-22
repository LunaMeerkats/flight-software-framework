# Rust Flight Framework

This repository is an experiment in building a small, Rust-native framework for
host-based flight-software research. It studies responsibilities also addressed
by NASA's Core Flight System (cFS), while independently choosing designs that
fit Rust's ownership, type, error, and testing models.

> **Safety and affiliation notice:** This project is not flight-qualified,
> safety-certified, a NASA product, or NASA-endorsed. It has no demonstrated
> Technology Readiness Level and is not suitable for operational spacecraft,
> safety-critical systems, or human-rated systems. It does not claim automatic
> compatibility with cFS, cFE, OSAL, PSP, CCSDS, or any RTOS.

## Current status

The repository contains an initial research and architecture baseline, five
bounded Rust lifecycle/work increments, three Stage 2 messaging increments,
two structured-event increments, an injected manual-time increment, and a
finite scheduled-work increment, plus standalone and runtime-integrated
configuration lifecycle increments. A no-dependency `LifecycleRegistry`
verifies the logical LC1 transition table. A finite-capacity `Runtime` owns
statically composed application values and executes start, caller-selected
work, stop, and in-place restart synchronously.
Successful lifecycle operations enter `Running`, `Stopped`, and `Running`;
successful work retains `Running`. Restart and work retain application-owned
state, while a returned concrete operation error enters terminal `Failed`
without mutating peer records.
The [lifecycle construction review](docs/verification/LIFECYCLE_CONSTRUCTION_REVIEW.md)
adds exact capacity-overflow diagnostics for both the standalone registry and
unconfigured owned runtime constructors. Those impossible-capacity tests do not
claim actual allocator exhaustion or whole-process memory bounds.
The checkpoint at `15e4166bbead2d824e732460fa1a67266386f648` has successful
[hosted CI](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/35534767664)
for 124 workspace tests, both focused host targets, and the sample executable.
The [application identity scope review](docs/verification/APPLICATION_ID_SCOPE_REVIEW.md)
now executes the distinct caller-discipline boundary: equal-position keys from
separate owners compare equal and select the corresponding local record. They
do not carry issuer provenance, and callers must not mix them across owners.
Checkpoint `9ea7f48f827031fcbeb63f712c97dfbe14cc629c` has successful
[hosted CI](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/35658016416)
for 126 workspace tests and the configured host/sample evidence.

The standalone `MessageBus` uses mission-selected topics and inline
const-bounded payloads. It copies immutable positive-capacity inbox topology,
preserves FIFO order across topics, applies reject-newest saturation, continues
fan-out to unaffected subscribers, and returns stable ordered outcomes. The
[detached-message identity review](docs/verification/MESSAGE_IDENTITY_SCOPE_REVIEW.md)
executes its caller-scoped boundary: a same-position foreign key passes
registration-order validation and addresses the configured inbox because the
key carries no issuer provenance. This is not permission to mix identities. The
owning `MessagingRuntime` now consumes a fully composed still-registered
runtime, constructs exactly one fresh inbox per application, derives delivery
availability from lifecycle state, clears queues after successful stop or a
returned callback error with an exact discarded-delivery count, and reconnects
an empty inbox after successful restart. A separate `MessagingApplication`
callback now receives one oldest in-flight delivery plus a publish-only context.
It can self-publish through the same bounded bus without gaining lifecycle,
dequeue, or nested-dispatch access. Routing-core, runtime-messaging, and
message-dispatch tests cover these boundaries; the
[returned-message failure review](docs/verification/MESSAGE_FAILURE_REVIEW.md)
records the complete retained peer FIFO and post-failure availability boundary.

A standalone `EventQueue<EventId>` stores typed source, severity,
mission-defined copied identifier, and explicit elapsed `EventTimestamp`
values. It pre-reserves a positive record limit, preserves emission-order FIFO,
rejects the newest event at saturation with a caller-visible outcome, and frees
one slot on dequeue. Six public tests cover its fields, exact reservation-error
diagnostics, storage boundary, and repeated saturation/reuse; the
[event-queue review](docs/verification/EVENT_QUEUE_REVIEW.md) records their
resource and failure limits. `EventTimestamp` can now capture an injected
`Clock` reading. The manually advanced implementation starts at zero by default
or at one explicit
controlled instant, changes only on explicit nonnegative advances, and rejects
representational overflow without mutation. Five public tests prove zero and
repeated reads, cumulative and replayed traces, typed overflow, object-safe
injection, and event timestamp capture.

`Runtime::work_with_failure_event` now composes those boundaries for one opt-in
direct work operation. Success and lifecycle rejection read no clock and emit
nothing. After an application returns a work error and its record enters
terminal `Failed`, the operation reads the injected clock once and attempts one
application-sourced error event. Its returned error retains the complete
`RuntimeWorkError`, exact event, and `Recorded` or `QueueFull` outcome. A full
queue preserves its older event and returns the rejected event for explicit
retry while leaving a healthy peer operable for later work. Four public tests
cover exact fields, the error-source chain, single emission, lifecycle
suppression, saturation, retry, and peer progress. `Runtime` still does not
permanently own the clock or event queue.

`MessagingRuntime::work_with_failure_event` adds the same opt-in reporting to
ordinary work under the inbox owner. It delegates to `MessagingRuntime::work`
first, so terminal failure and exact selected-inbox clearing finish before the
single clock read and event attempt. The existing `MessagingOperationError`
retains the discard count around the existing `RuntimeWorkEventError`; success
and lifecycle rejection emit nothing. Direct and messaging-owned operations
share event construction while preserving separate ownership responsibilities.
This does not add events to message dispatch or scheduled work.

A `WorkSchedule` copies a finite agenda of one-shot application work items in
nondecreasing elapsed-time order. `Runtime::run_next_scheduled_work` reads an
injected clock once while an item remains, waits without mutation before its
instant, and consumes at most one due or overdue item per caller request. Equal-
time items retain configuration order. Success returns the lifecycle state;
lifecycle rejection or a returned work error preserves the exact runtime error
and consumes only the attempted item so a due peer can still progress. Seven
public tests cover order validation, exact clock-read counts, inclusive and
overdue work, lifecycle/error handling, and replay-equivalent work, lifecycle,
and structured-event timestamp traces under manual time.

`MessagingRuntime::run_next_scheduled_work` uses that same private timing and
consumption decision, then delegates through messaging-owned ordinary work.
The existing nested errors retain the item, observed instant, original work
error, and exact discarded-delivery count. Failure clears only the selected
inbox; later due peers can still work and dispatch their retained messages.
This operation leaves successful inboxes intact and emits no failure event.

`ConfigurationTable<E, MAX_BYTES>` validates and copies in-memory byte content
through one retained mission function. It exposes immutable active snapshots,
assigns fresh revisions even to equal content, and retains only the former
active snapshot for consume-once rollback. Rejection preserves active content,
rollback history, and revision high-water; rollback restores the original
revision without reusing numbers. Exact byte bounds, semantic rejection,
ownership, history replacement, and checked revision exhaustion have tests.
`Runtime<A, E, MAX_CONFIGURATION_BYTES>` can now take ownership of a complete
validated table at construction. Ordinary work receives an
`ApplicationWorkContext` with an optional immutable revision/bytes view;
unconfigured work sees explicit absence. Narrow replacement and rollback
operations retain the table's validation and revision rules, while construction
failure returns the unchanged table. The same configured runtime composes with
scheduled work, failure-event reporting, and `MessagingRuntime` work without a
parallel callback. Returned work errors do not roll configuration back, and
stop/restart retains the full lineage. Ten focused runtime tests plus the
standalone table tests verify RFF-REQ-006. No schema, wire format, host loader,
or message/lifecycle callback access is selected.

A public integration test runs two independently defined applications through
registration, start, work, stop, restart, and work, completing the bounded
RFF-REQ-002 lifecycle evidence. The combined routing, lifecycle-availability,
and caller-selected dispatch evidence now verifies RFF-REQ-003, including
capacity-one self-publication, per-dispatch availability refresh, and exact
selected-queue clearing after a returned message error. The runtime still has
no automatic or batch dispatch. The private combined sample now composes these
services with a fixed caller-driven scenario.
Periodic scheduling, application-authored events, other
callback event paths, and application configuration access outside ordinary
work remain unimplemented.
The private `host-echo` example preserves ADR-0019's two-byte validated echo
command and matching telemetry. Its two applications use capacity-one inboxes
and a borrowed capacity-one host mailbox drained outside dispatch. The same
mission source is exercised by integration tests; no library API is added.
ADR-0022 combines the adapters with both applications' lifecycle, finite
scheduling at manual time 10 ms, observed configuration activation/rejection/
rollback, and a separate cooperative work-failure event at 20 ms. The binary
and sample tests execute the same fixed driver and retain a structured report.
The finite scheduling and manual-time evidence verifies RFF-REQ-004. The direct
returned-work event and peer-progress evidence, extended through the messaging
owner, verifies RFF-REQ-005 and RFF-REQ-008 at those cooperative ordinary-work
boundaries. Traceability distinguishes this
evidence from event delivery guarantees or containment of panics, hangs,
cleanup failures, and other arbitrary faults.

The first source-quality checkpoint tracks stable rustfmt at 100 columns and
denies Clippy functions over a 60-line review threshold across all targets.
Three cohesive chronological integration tests carry narrow reasoned
expectations; two avoidable expectations were retired through focused test
structure. Production code needs no function-length waiver. Source ordering,
module cohesion, names, comment prose, and exceptional physical lines remain
review responsibilities rather than unsupported automated claims.

## Development

The package is unpublished and uses stable Rust with no external Cargo
dependencies or package features. The standard library, host environment,
toolchain, and CI tooling remain outside that Cargo graph. The
[dependency and scope review](docs/verification/DEPENDENCY_SCOPE_REVIEW.md)
records the exact inventory and autonomous review boundary; human v0.1
entry-point and architecture acceptance remain pending.

```text
cargo fmt --all -- --check
cargo check --workspace --all-targets --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo doc --workspace --all-features --no-deps
```

Run rustdoc with warnings denied; in PowerShell, set
`$env:RUSTDOCFLAGS = "-D warnings"` before the documentation command. Contributor
guidance records the remaining structural and document checks.

[The host CI workflow](.github/workflows/ci.yml) encodes these checks with
locked dependency resolution, both focused host tests, and the sample on
Windows. [Its verification record](docs/verification/CI_BASELINE.md) separates
local validation from successful exact-revision hosted execution. Verified
source pushes to the existing `codex/nightly` branch are authorized without
per-push human review. Autonomous source/document review and automated checks
remain required; separate human v0.1 release reviews remain outstanding.

## Combined host sample

Run `cargo run --example host-echo` from the repository root. The
[example entry point](examples/host-echo/main.rs) prints the experimental scope
notice and processes percentages 0, 42, and 100 through the
[shared mission source](examples/host-echo/mission.rs):

```text
command [01, 00] -> telemetry [81, 00]
command [01, 2A] -> telemetry [81, 2A]
command [01, 64] -> telemetry [81, 64]
```

Each input is one complete caller-framed two-byte slice: identifier `0x01`,
then a percentage in `0..=100`. Length, identifier, and value validation precede
publication. The host inspects the returned delivery report, explicitly
dispatches the echo application once and the telemetry application once, then
drains one exact `[0x81, percentage]` array. Ingress does not execute work or
drain output. The local identifiers have no external protocol meaning.

Full inboxes reject the newest delivery and preserve older messages. Full host
output preserves the older record but returns a callback error: the telemetry
application becomes terminal `Failed`. Draining that output does not recover
the application. Normal example execution drains between commands. No retry,
physical delivery, stream framing, authentication, or execution rollback is
promised. Diagnostic stdout writes occur after drain and outside callbacks;
a write error ends the host example without undoing execution.

The executable then prints the fixed scenario report: both applications'
lifecycle states, manual-time schedule outcomes, fresh callback configuration
observations, and an injected echo work failure that clears one queued command
while preserving the peer's work and queued telemetry. Configuration is one
validated observation byte; it does not alter the command grammar.

`cargo test --test host_adapters` exercises the adapters and fault fixture.
`cargo test --test host_sample` verifies the same combined driver as the binary,
including complete fresh-run report equality. See the
[sample guide](docs/verification/HOST_SAMPLE.md) for exact expected fields and
resource limits. CI, scope/dependency review, and human v0.1 architecture review
remain separate release gates.

## Start here

- [Project charter](docs/CHARTER.md)
- [Initial architecture analysis](docs/ARCHITECTURE.md)
- [v0.1 requirements](docs/REQUIREMENTS.md)
- [Roadmap](docs/ROADMAP.md)
- [Current project state](docs/PROJECT_STATE.md)
- [Caller-driven runtime decision](docs/adr/0001-caller-driven-host-runtime.md)
- [Project identity and dual licensing](docs/adr/0002-project-name-and-licensing-intent.md)
- [Lifecycle and identity decision](docs/adr/0003-stop-gated-lifecycle-and-runtime-local-identity.md)
- [Bounded inbox decision](docs/adr/0004-bounded-application-inboxes.md)
- [Configuration rollback decision](docs/adr/0005-configuration-revisions-and-rollback.md)
- [Initial application ownership decision](docs/adr/0006-static-application-ownership-for-initial-runtime.md)
- [Owned stop boundary decision](docs/adr/0007-operation-specific-owned-stop-boundary.md)
- [In-place owned restart decision](docs/adr/0008-distinct-in-place-owned-restart.md)
- [Caller-driven owned work decision](docs/adr/0009-caller-driven-owned-work.md)
- [Bounded message-routing core decision](docs/adr/0010-bounded-message-routing-core.md)
- [Runtime-owned message availability decision](docs/adr/0011-runtime-owned-message-availability.md)
- [One-message application dispatch decision](docs/adr/0012-application-message-dispatch.md)
- [Bounded structured-event queue decision](docs/adr/0013-bounded-structured-event-queue.md)
- [Injected manual framework clock decision](docs/adr/0014-injected-manual-framework-clock.md)
- [Finite caller-driven scheduling decision](docs/adr/0015-caller-driven-scheduled-work.md)
- [Returned-work failure-event decision](docs/adr/0016-returned-work-failure-events.md)
- [Bounded configuration snapshot decision](docs/adr/0017-bounded-configuration-snapshots.md)
- [Configuration-aware work-context decision](docs/adr/0018-configuration-aware-work-context.md)
- [Host command/telemetry boundary decision](docs/adr/0019-host-command-telemetry-boundary.md)
- [Messaging-owned work failure-event decision](docs/adr/0020-messaging-work-failure-events.md)
- [Messaging-owned scheduling decision](docs/adr/0021-messaging-owned-scheduled-work.md)
- [Lifecycle construction review](docs/verification/LIFECYCLE_CONSTRUCTION_REVIEW.md)
- [Research sources and provenance](docs/research/SOURCES.md)
- [Verification traceability](docs/verification/TRACEABILITY.md)
- [Source-quality baseline](docs/verification/SOURCE_QUALITY_BASELINE.md)
- [Contributor and automation guidance](AGENTS.md)

## Licence

This repository is licensed under either the
[MIT licence](LICENSE-MIT) or the
[Apache License, Version 2.0](LICENSE-APACHE), at your option. The confirmed
notice is `Copyright 2026 Daniel Smith`. Cargo publication remains disabled.
