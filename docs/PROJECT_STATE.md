# Project state

Last updated: **2026-08-25**

## Current milestone

Stage 1 and the first source-quality checkpoint are complete. Stage 2 now has a
bounded routing core and its first runtime-owned lifecycle integration. Inline
payloads, immutable topic topology, FIFO, reject-newest saturation, ordered
fan-out results, lifecycle-derived unavailability, exact stop/returned-error
clearing, and empty restart reconnection are implemented and tested.
RFF-REQ-003 remains partial because applications have no messaging work
context, self-publication, or one-in-flight dispatch boundary.

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
- Messaging-core commit `6c6e0b0b31e7338414f28f330024c2bfbd3eec40`
  adds the bounded message representation, immutable available-endpoint
  routing core, and seven public routing tests.
- Messaging-ownership commit `65bd4fa458bb6f82fe73af291f90e90ee582e3d5`
  adds complete registered-count/order attachment, runtime-derived availability,
  stop/returned-error clearing, restart reconnection, and ten public integration
  tests.
- Formatting, all-target checking, warnings-denied Clippy, all 33 tests, and
  warnings-denied all-feature documentation generation pass with rustc/cargo
  1.96.1, rustfmt 1.9.0-stable, and Clippy 0.1.96.
- Nine handwritten Rust files have no physical line over 100 columns and no
  comment-only line over the 80-column review default. The three existing
  reasoned chronological test expectations remain fulfilled; neither messaging
  increment needs one.
- All 46 relative links resolve across 22 Markdown documents; all eight
  requirement identifiers match traceability; all eight referenced source
  identifiers resolve in the 21-entry register; all three Markdown tables have
  consistent shapes; and all eleven changed documents render structurally with
  one top-level heading.

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
  [ADR-0010](adr/0010-bounded-message-routing-core.md) implements the routing
  core and
  [ADR-0011](adr/0011-runtime-owned-message-availability.md) implements its
  first lifecycle-aware ownership boundary.
- ADR-0005 defines configuration revision/rollback behavior; that service is
  not implemented.

The unpublished, dependency-free library retains a standalone
`LifecycleRegistry` for logical transition evidence. `Runtime<A>` owns bounded
application records and invokes one caller-selected callback only after identity
and lifecycle validation. A returned concrete operation error is preserved
while only the selected record enters terminal `Failed`.

Standalone `MessageBus<Topic, MAX_PAYLOAD_BYTES>` copies caller-supplied
contiguous-prefix topology and treats every endpoint as available.
`MessagingRuntime<A, Topic, MAX_PAYLOAD_BYTES>` instead consumes a fully
composed still-registered runtime, constructs one fresh inbox per record, and
freezes both owners. Only `Running` accepts delivery. `Registered`, `Stopped`,
and terminal `Failed` remain known but unavailable; stop and returned callback
errors clear only the selected queue before returning the exact delivery count.
Successful restart from `Stopped` reconnects the already-empty inbox.

The runtime creates no thread or executor and has no automatic dispatch,
service context, schedule, fairness rule, clock, events, or configuration
access. The integrated message surface exposes publication and queue counts,
not application dequeue or self-publication.

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

No implementation work is in progress. The lifecycle-availability slice
reached its intended stopping point before application work-context or dispatch
changes.

## Highest risks and uncertainties

- The pre-v0.1 context-free work callback still needs a recorded borrowing
  revision before an application can consume or publish a message.
- Runtime-owned inbox configurations are positional. The integration proves
  complete identity order and count but cannot detect two valid capacity/topic
  configurations that mission composition accidentally swaps.
- Direct standalone `MessageBus` construction still has the recorded
  same-position foreign-identity and incomplete-topology limitation; lifecycle
  guarantees apply only to `MessagingRuntime`.
- Inline payload slots consume the configured maximum for short payloads and
  copy that full representation for every accepted destination. No mission
  payload size has been measured or selected.
- Mission-defined topic equality and per-publication report allocation are not
  a general non-blocking execution guarantee. Allocation failure is handled
  before delivery, but arbitrary topic comparison remains caller-defined.
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
- The application messaging context, dequeue ownership, self-publication, and
  one-in-flight dispatch shape are not selected.
- No mission payload limit, external topic identifier, or wire representation
  is selected; ADR-0010 deliberately defines only the in-process representation.
- RFF-REQ-007 still needs a host-adapter grammar and validation boundary.
- No explicit minimum supported Rust version or panic-containment policy has
  been selected.
- Additional source lints, a physical-line checker, a complexity metric,
  dependency-policy tooling, CI, and toolchain pinning remain unselected.

## Most likely next tasks

1. Record the smallest messaging work-context and one-in-flight dispatch
   borrowing shape, then prove one application can consume and self-publish
   without bypassing lifecycle or queue bounds.
2. Follow bounded messaging with structured finite event delivery, then inject
   simulated time before scheduling work.
3. Preserve runtime composition and message integration regression evidence
   while keeping RFF-REQ-003 partial until true application access is tested.

## Latest run

2026-08-25: Added the first lifecycle-aware ADR-0004 integration. A fresh
`MessagingRuntime` now consumes a fully composed still-registered runtime,
assigns exactly one inbox per record, makes only `Running` endpoints available,
clears stopped and callback-failed queues with exact returned counts, and
reconnects an empty inbox only after successful restart from `Stopped`. Ten new
public tests bring the suite to 33 and cover construction failure ownership,
ordered unavailable results, exact two-delivery clearing, unaffected peers,
lifecycle rejection, and all returned callback-error paths. Independent review
narrowed positional-topology claims, strengthened exact-count and route-order
evidence, and restored public-first helper placement. Formatting, checking,
warnings-denied Clippy, tests, rustdoc, source widths, links, rendered document
structure, identifiers, tables, and diff checks pass. No dependency, thread,
executor, application message context, dispatch, protocol, licence change, or
push was added.
