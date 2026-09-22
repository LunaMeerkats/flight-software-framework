# Detached message identity scope review

Date: **2026-09-23**
Scope: **Existing ADR-0010 caller-scoped detached-topology contract**

## Decision and inspected boundary

Retain the current detached `MessageBus` contract and execute its documented
issuer limitation. No production mismatch was found in `src/messaging.rs` at
starting revision `3041407580c09236adbc46ad05819c66984b2c9e`. This review
supplements RFF-REQ-003, [ADR-0010](../adr/0010-bounded-message-routing-core.md),
and the prior [application identity review](APPLICATION_ID_SCOPE_REVIEW.md)
without changing behavior.

`MessageBus::new` validates each supplied identity's private index against its
configuration position. Because `ApplicationId` carries no issuer provenance,
a first-position key from any registry satisfies the first topology position.
The bus stores that copied value; equal-position keys from another issuer are
indistinguishable for reports, inspection, and dequeue.

This remains caller discipline, not authorization to mix identities.
`MessagingRuntime` mitigates detached-topology composition error by consuming a
complete runtime, accepting positional inbox configuration without identities,
and assigning its internal bus identities itself. Operation selectors still use
ADR-0003's caller-scoped `ApplicationId` and do not gain origin validation.

## Executed public observation

`equal_position_foreign_identity_passes_topology_and_addresses_the_inbox`
creates two live one-record registries and observes that their independently
issued keys compare equal. It configures a detached bus with the foreign key,
publishes one message, and then uses the equal local key to observe and dequeue
the same inbox. The foreign key subsequently observes that inbox empty. Both
registries remain `Registered`, proving that the bus has no ownership or state
link to either issuer.

The regression executes the previously documented constructor and lookup
boundary. Existing out-of-range tests remain evidence for
`InboxAccessError::UnknownApplication`; they do not prove issuer validation.
No requirement meaning, production API, dependency, allocation policy, or
lifecycle behavior changes.

## Alternatives and limits

Changing `ApplicationId` to carry origin would affect every service and needs a
separate bounded-source, equality, exhaustion, copying, persistence, and public-
API decision. Removing the detached bus would also discard useful routing-core
evidence without improving the already-integrated owner's selectors. Neither
change is justified by an evidence checkpoint.

This test uses one inbox, one topic, one delivery, and two live registries. It
does not establish safe cross-owner mixing, global uniqueness, persistence,
serialization, swapped mission-configuration detection, allocation-failure
behavior, or human API acceptance.

No new external source was needed. The observation follows the accepted local
ADRs and executed public interfaces; the existing
[source register](../research/SOURCES.md) remains unchanged.

## Verification

The initial locked baseline passes 126 tests on Rust/Cargo 1.98.0, rustfmt
1.9.0-stable, and Clippy 0.1.98. After formatting,
`cargo test --locked --test message_bus` passes nine tests including the new
regression. The final locked baseline and source, link, traceability, rendered-
document, and complete-diff reviews pass: 127 workspace tests, 32 Rust files
with zero width findings and three unchanged fulfilled expectations, 46
Markdown files, 200 resolving relative links, and 91 exact traceability test
references. Rendered structure and exact source-content comparisons pass;
pixel-level visual acceptance is not claimed.

Host adapters, the combined sample, CI workflow, and ADR-0018/0019 experiments
are unchanged, so their separate local commands are not triggered. The full
workspace suite still exercises both host test targets. This checkpoint does
not establish human acceptance.
