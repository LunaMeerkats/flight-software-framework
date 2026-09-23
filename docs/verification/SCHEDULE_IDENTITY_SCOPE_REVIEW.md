# Scheduled-work identity scope review

Date: **2026-09-24**
Scope: **Existing ADR-0015/0021 caller-scoped schedule selector contract**

## Decision and inspected boundary

Retain the current finite one-shot schedule contract and execute its documented
issuer limitation through both scheduling owners. No production mismatch was
found in `src/scheduling.rs` at starting revision
`4ef3b74296700259ad80ad5a6792de054fddbea2`. This review supplements
RFF-REQ-004, [ADR-0015](../adr/0015-caller-driven-scheduled-work.md),
[ADR-0021](../adr/0021-messaging-owned-scheduled-work.md), and the prior
[application identity review](APPLICATION_ID_SCOPE_REVIEW.md) without changing
behavior.

`ScheduledWork` copies one `ApplicationId` value without an owner reference.
When an item is due, the schedule passes that value to the receiving owner's
ordinary-work operation. Because the key stores only a record position, a
same-position key from another live runtime selects the receiving owner's
corresponding local application. The scheduler cannot distinguish the issuer
or validate the caller's clock origin.

This remains caller discipline, not authorization to mix identities or clock
domains. A caller must pair each schedule with the runtime or messaging owner
and clock for which its items were configured.

## Executed public observations

The two regressions are:

- `equal_position_foreign_identity_schedules_the_receiving_runtime_application`
  creates two live direct runtimes whose first keys compare equal. A schedule
  containing the foreign key invokes only the receiving runtime's local Alpha
  callback; the foreign runtime's trace and state remain unchanged.
- `equal_position_foreign_identity_schedules_the_receiving_messaging_application`
  repeats the selector observation through `MessagingRuntime`. The receiving
  Alpha callback runs once while both of its full inboxes remain unchanged;
  the foreign owner's callback trace, empty inboxes, and lifecycle state remain
  unchanged.

Both items are consumed once and report successful `Running` outcomes. Existing
out-of-range schedule tests remain evidence for `UnknownApplication`; they do
not prove issuer validation. No requirement meaning, production API,
dependency, scheduling policy, inbox-cleanup rule, or clock behavior changes.

## Alternatives and limits

An origin-bearing application key or clock-domain identity would need separate
bounded-source, equality, exhaustion, copying, persistence, and public-API
decisions. Binding a schedule permanently to one owner would alter the current
borrowed, reusable host composition. Neither redesign is justified by an
evidence checkpoint.

These tests use one due item, the first registration position, successful
ordinary work, and one controlled zero instant. They do not establish safe
cross-owner mixing, global identity, persistent identity, clock-origin
validation, failure cleanup under an aliased selector, panic or hang
containment, or human API acceptance.

No new external source was needed. The observations follow accepted local ADRs
and executed public interfaces; the existing
[source register](../research/SOURCES.md) remains unchanged.

## Verification

The initial locked baseline passes 127 tests on Rust/Cargo 1.98.0, rustfmt
1.9.0-stable, and Clippy 0.1.98. After formatting, the focused command
`cargo test --locked --test scheduled_work --test messaging_scheduled_work`
passes ten direct and eight messaging-owned tests including both regressions.
The final locked baseline passes 129 workspace tests. Source, link,
traceability, rendered-document, and complete-diff audits pass: 32 Rust files
with zero width findings and three unchanged fulfilled expectations; 47
Markdown files, 208 resolving relative links, 35 source definitions, and 93
exact traceability test references. Generated HTML passes structural and exact-
content comparisons; rendered-layout inspection found no visible overflow or
malformed changed section.

Host adapters, the combined sample, CI workflow, and ADR-0018/0019 experiments
are unchanged, so their separate local commands are not triggered. The full
workspace suite still exercises both host test targets. This checkpoint does
not establish human acceptance.

## Published checkpoint and continuation

Commit `9f3aac6947943c3f9affbe38383e5e4ad09f382d` contains both regressions
and the reviewed checkpoint. Ordinary fast-forward publication succeeded and
the remote head matched. On 2026-09-23 UTC (2026-09-24 Sydney),
[hosted run 35914683806](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/35914683806)
passed that exact push and checkout: one Windows job and every configured step.
Logs confirm both new regression names, 129 workspace tests in aggregate, 14
focused adapter tests, five focused sample tests, the sample executable, and
the warnings-denied/whitespace baseline.

Actual runner: 2.337.0; image: `windows-2025-vs2026` version
`20260907.229.1` (requested label `windows-2025`). Actual Rust/Cargo: 1.98.1;
rustfmt: 1.9.0-stable; Clippy: 0.1.98. This matches the previously reviewed
hosted environment. The all-target hosted baseline and companion whole-tree
source review preserve existing policy without a new waiver. Local Rust/Cargo
remain 1.98.0.

This documentation-only follow-up records the completed result with unchanged
Rust, Cargo, workflow, and lint inputs. It does not establish a hosted pass for
its own later revision or human v0.1 acceptance. The next bounded task should
be selected from a reconciled service, resource, failure, or public-API gap;
broader scope remains unapproved.
