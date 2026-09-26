# Failure-event identity scope review

Date: **2026-09-27**
Scope: **Existing ADR-0016/0020 caller-scoped event attribution contract**

## Decision and inspected boundary

Retain the current opt-in ordinary-work failure-event contract and execute its
documented application-identity limitation through both event-producing owners.
No production mismatch was found in `src/runtime_events.rs` or
`src/messaging_runtime.rs` at starting revision
`3d38bf6bcb5c56ad061561a43ca2d05be639175c`. This review supplements
RFF-REQ-005, RFF-REQ-008, [ADR-0016](../adr/0016-returned-work-failure-events.md),
[ADR-0020](../adr/0020-messaging-work-failure-events.md), and the prior
[application identity review](APPLICATION_ID_SCOPE_REVIEW.md) without changing
behavior.

Both public operations pass the caller-selected `ApplicationId` through the
receiving owner's ordinary-work path. A cooperative returned error retains that
same positional value in `RuntimeWorkError::Application`; shared event
construction then copies it into `EventSource::Application`. Because the key
stores only a record position, a same-position key from another live owner
selects the receiving owner's corresponding local application and is
indistinguishable from its local key in the event.

This remains caller discipline, not authorization to mix identities. A caller
must pair each selector with the runtime or messaging owner that issued it.

## Executed public observations

The two regressions are:

- `equal_position_foreign_identity_records_receiving_runtime_failure_event`
  starts two live direct owners whose first keys compare equal. The foreign key
  invokes and fails only the receiving owner's local application, records one
  event with that positional source, and leaves the foreign owner running and
  uninvoked.
- `equal_position_foreign_identity_records_receiving_messaging_failure_event`
  repeats the observation through `MessagingRuntime`. Only the receiving local
  application fails and its two queued deliveries are cleared. The receiving
  peer and both foreign inboxes remain full; both foreign applications remain
  running and uninvoked.

Both events are recorded once with one injected clock read. Existing out-of-
range tests remain evidence for `UnknownApplication`; they do not prove issuer
validation. No requirement meaning, production API, dependency, event policy,
inbox-cleanup rule, or clock behavior changes.

## Alternatives and limits

An origin-bearing application key would need separate bounded-source, equality,
exhaustion, copying, persistence, and public-API decisions. Accepting a separate
event source would create a second attribution value that could disagree with
the invoked application. Neither redesign is justified by an evidence
checkpoint.

These tests use the first registration position, one cooperative ordinary-work
error, one queue slot, and a non-saturated event path. They do not establish
safe cross-owner mixing, global identity, persistent identity, issuer
validation, scheduled or message-callback event behavior, panic or hang
containment, guaranteed diagnostic delivery, or human API acceptance.

No new external source was needed. The observations follow accepted local ADRs
and executed public interfaces; the existing
[source register](../research/SOURCES.md) remains unchanged.

## Verification

The initial locked baseline passes 132 tests on Rust/Cargo 1.98.0, rustfmt
1.9.0-stable, and Clippy 0.1.98. After formatting, the focused command
`cargo test --locked --test runtime_events --test messaging_work_events`
passes five direct and seven messaging-owned event tests including both
regressions. The focused warnings-denied Clippy command and whitespace check
also pass. The final locked baseline passes 134 workspace tests. Formatting,
all-target checking, warnings-denied Clippy/rustdoc, and whitespace checks pass
without a new exception.

Source, link, traceability, and generated-document audits pass: 32 Rust files
with zero width findings, no block comments, and three unchanged fulfilled
expectations; 50 Markdown files, 230 resolving relative links, 35 source
definitions, and 98 exact traceability test references. Generated HTML passes
structural and exact-content comparisons. Browser visual layout inspection was
not run because browser URL policy blocked local rendered files.

Host adapters, the combined sample, CI workflow, and ADR-0018/0019 experiments
are unchanged, so their separate local commands are not triggered. The full
workspace suite still exercises both host test targets. This checkpoint does
not establish human acceptance.

## Continuation boundary

This checkpoint does not establish issuer validation or human acceptance. A
later checkpoint must carry its own exact revision and hosted evidence. Next
select another bounded Stage 4 service, resource, failure, or public-API gap
after reconciling existing evidence; broader scope remains unapproved.
