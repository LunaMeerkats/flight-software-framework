# Project state

Last updated: **2026-08-24**

## Current milestone

Stage 1 and the first source-quality checkpoint are complete. Stage 2 has begun
with a bounded available-endpoint routing core. Its inline payload, immutable
topic topology, FIFO queues, reject-newest saturation, and publisher-visible
fan-out results are implemented and tested. RFF-REQ-003 remains partial because
the core is not owned by the runtime and has no lifecycle-derived availability,
clearing/reconnection, application self-publication, or dispatch boundary.

## Verified baseline

- Lifecycle/work implementation commit
  `43ef56b92da18302eee0c0a0a1f071a29aae0ade` contains the caller-driven work
  boundary and complete RFF-REQ-002 integration evidence.
- Source-quality commit `c8f2f7af9232aa370a3ea9ded211eac1581dd375`
  adds stable rustfmt and Clippy configuration, denies
  `clippy::too_many_lines` at 60 for all targets, and records the initial
  reasoned test expectations.
- Source-order commit `c2aa32772c2fd32a0b87893f913e32ecacc5d051`
  improves public-first reading, callback error documentation, and test
  structure without changing runtime behavior.
- Messaging commit `6c6e0b0b31e7338414f28f330024c2bfbd3eec40`
  adds the bounded message representation, immutable available-endpoint
  routing core, and seven public routing tests.
- Formatting, all-target checking, warnings-denied Clippy, all 23 tests, and
  warnings-denied all-feature documentation generation pass with rustc/cargo
  1.96.1, rustfmt 1.9.0-stable, and Clippy 0.1.96.
- No handwritten Rust physical line exceeds 100 columns and no comment-only
  Rust line exceeds the 80-column review default. The three existing reasoned
  chronological test expectations remain fulfilled; the new messaging source
  needs none.
- All 41 relative links resolve across 21 Markdown files; all eight requirement
  identifiers match traceability; all eight referenced source identifiers are
  defined in the 21-entry register; both Markdown tables have consistent row
  shapes; and all 21 documents render structurally with one top-level heading.

The tool versions record this baseline but do not set a minimum supported Rust
version or a repository toolchain pin. The initial audit remains in
[the source-quality baseline](verification/SOURCE_QUALITY_BASELINE.md).

## Current architecture

- ADR-0001 selects a provisional caller-driven, serial host runtime.
- ADR-0003 defines four stable LC1 states and stop-gated restart with
  runtime-local identity.
- ADR-0006 through ADR-0009 select finite-capacity static application ownership
  and distinct synchronous start, work, stop, and in-place restart callbacks
  with concrete returned errors.
- ADR-0004 defines bounded application inbox behavior;
  [ADR-0010](adr/0010-bounded-message-routing-core.md) implements its first
  available-endpoint routing core.
- ADR-0005 defines configuration revision/rollback behavior; that service is
  not implemented.

The unpublished, dependency-free library retains a standalone
`LifecycleRegistry` for logical transition evidence. `Runtime<A>` owns bounded
application records and invokes one caller-selected callback only after identity
and lifecycle validation. A returned concrete operation error is preserved
while only the selected record enters terminal `Failed`.

`MessageBus<Topic, MAX_PAYLOAD_BYTES>` separately copies a caller-supplied
contiguous-prefix application topology. It reserves each positive-capacity
inbox and unique topic set, processes matching destinations in registration
order, stores inline bounded messages FIFO across topics, rejects the newest
delivery at the exact logical limit, and derives publication classification
from one ordered destination-outcome vector. Report storage is reserved before
any inbox mutation.

The runtime creates no thread or executor and has no automatic dispatch,
service context, schedule, fairness rule, clock, events, or configuration
access. The message core does not observe runtime state and must not be treated
as lifecycle-integrated.

## Source-quality policy

Stable rustfmt owns normal formatting at 100 columns. Clippy enforces a
normally-60-line function threshold for every target. The three current
expectations preserve complete chronological state/error traces and have
item-level reasons; production and messaging functions need no exception.

Comment prose, exceptional physical lines, progressive source ordering, module
cohesion, abstraction level, and naming remain review responsibilities. A
strict physical-line checker, additional selected lints, validated complexity
metric, dependency-policy tool, CI workflow, and toolchain pin remain deferred
to separate measured increments.

## Work in progress

No implementation work is in progress. The available-endpoint routing slice
reached its intended stopping point before runtime ownership or service-context
work.

## Highest risks and uncertainties

- The detached message core cannot prove that every runtime application has an
  inbox, detect a same-position identity from another issuer, or synchronize
  availability with lifecycle state.
- The pre-v0.1 context-free work callback still needs a recorded borrowing
  revision when an application first consumes or publishes a message.
- Inline payload slots consume the configured maximum for short payloads and
  copy that full representation for every accepted destination. No mission
  payload size has been measured or selected.
- Mission-defined topic equality and per-publication report allocation are not
  a general non-blocking execution guarantee. Allocation failure is handled
  before delivery, but arbitrary topic comparison behavior remains caller-
  defined.
- Dequeued messages and publication reports become caller-owned, so the bus
  bounds its topology and queued storage rather than all caller retention.
- A returned application error may follow partial application-internal
  mutation. The runtime records `Failed` but proves no rollback, cleanup,
  reinitialisation, or isolation.
- One blocking or non-returning callback prevents caller progress. Returned
  errors do not contain panics, hangs, process failure, memory exhaustion, or
  hardware faults.
- The function-size gate detects lines, not semantic complexity. Reviewers must
  reject count-gaming and reassess every reasoned expectation when it changes.
- No CI currently executes the local baseline.

## Important unresolved decisions

- The exact copyright-holder text is required before adding the approved MIT
  and Apache-2.0 licence files and Cargo licence expression.
- Runtime ownership of messaging, lifecycle-driven queue clearing, and the
  service-context borrowing shape are not selected.
- No mission payload limit, external topic identifier, or wire representation
  is selected; ADR-0010 deliberately defines only the in-process representation.
- RFF-REQ-007 still needs a host-adapter grammar and validation boundary.
- No explicit minimum supported Rust version or panic-containment policy has
  been selected.
- Additional source lints, a physical-line checker, a complexity metric,
  dependency-policy tooling, CI, and toolchain pinning remain unselected.

## Most likely next tasks

1. Integrate the smallest runtime-owned availability slice: stopped and failed
   endpoints become unavailable with an observable discarded count, and restart
   reconnects an empty inbox.
2. Introduce a messaging work context only after that ownership split proves
   how one application and framework services can be borrowed safely; then test
   true self-publication and one in-flight dispatch.
3. Follow bounded messaging with structured finite event delivery, then inject
   simulated time before scheduling work.

## Latest run

2026-08-24: Added the first ADR-0004 implementation slice. Inline messages now
enforce a const-generic payload maximum, and the available-endpoint routing core
copies immutable configured topology, pre-reserves positive-capacity inboxes,
preserves FIFO across topics, rejects the newest saturated delivery, continues
healthy fan-out, and returns stable ordered destination outcomes. Seven new
public tests bring the suite to 23. Independent review added selective-routing
evidence and narrowed allocation, availability, and topology claims. Formatting,
checking, warnings-denied Clippy, tests, rustdoc, source widths, links, document
rendering, identifiers, tables, and diff checks pass. No dependency, thread,
executor, runtime lifecycle integration, service context, protocol, licence
change, or push was added.
