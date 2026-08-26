# Project state

Last updated: **2026-08-26**

## Current milestone

Stage 1 and the first source-quality checkpoint are complete. Stage 2 has now
completed bounded messaging for RFF-REQ-003: bounded payloads and inboxes,
ordered partial fan-out, lifecycle-aware availability and clearing, and
caller-selected one-message application dispatch with true self-publication.
The next Stage 2 responsibility is structured finite event delivery before
injected time and scheduling.

## Verified baseline

- Lifecycle/work implementation commit
  `43ef56b92da18302eee0c0a0a1f071a29aae0ade` contains the caller-driven work
  boundary and complete RFF-REQ-002 integration evidence.
- Source-quality commit `c8f2f7af9232aa370a3ea9ded211eac1581dd375`
  adds stable rustfmt and Clippy configuration and denies
  `clippy::too_many_lines` at 60 for all targets.
- Messaging-core commit `6c6e0b0b31e7338414f28f330024c2bfbd3eec40`
  adds the bounded message representation and available-endpoint routing core.
- Messaging-ownership commit `65bd4fa458bb6f82fe73af291f90e90ee582e3d5`
  adds complete inbox attachment, lifecycle-derived availability, clearing, and
  restart reconnection.
- Application-dispatch commit
  `150b924e6391c9adcc14f23bf21138011b747313` adds the separate message
  callback, publish-only context, preallocated per-dispatch lifecycle snapshot,
  one-in-flight dispatch, and five focused public tests.
- Repository content is available under `MIT OR Apache-2.0` with the confirmed
  notice `Copyright 2026 Daniel Smith`. Both canonical licence files, Cargo
  metadata, predicted package inventory, and the generated package archive have
  been verified while `publish = false` remains in force.
- Formatting, all-target checking, warnings-denied Clippy, all 38 tests, and
  warnings-denied all-feature documentation generation pass with rustc/cargo
  1.98.0, rustfmt 1.9.0-stable, and Clippy 0.1.98.
- Eleven handwritten Rust files have no physical line over 100 columns and no
  comment-only line over the 80-column review default. The three existing
  reasoned chronological test expectations remain fulfilled; none of the
  messaging increments needs one.

The active toolchain changed from the 1.96.1 policy-establishment evidence. The
whole-tree re-audit passes; this records evidence rather than a minimum supported
Rust version or toolchain pin.

## Current architecture

- ADR-0001 selects a provisional caller-driven, serial host runtime.
- ADR-0003 defines four stable LC1 states and stop-gated restart with
  runtime-local identity.
- ADR-0006 through ADR-0009 select finite-capacity static application ownership
  and distinct synchronous start, work, stop, and in-place restart callbacks
  with concrete returned errors.
- ADR-0004 defines bounded application inbox behavior. ADR-0010 implements its
  routing core, ADR-0011 its lifecycle owner, and ADR-0012 its one-message
  application dispatch boundary.
- ADR-0005 defines configuration revision and rollback behavior; that service
  is not implemented.

`MessagingRuntime<A, Topic, MAX_PAYLOAD_BYTES>` consumes a fully composed
still-registered runtime, creates exactly one fresh bounded inbox per record,
and freezes the integrated topology. Only `Running` endpoints accept delivery.
Stop and returned callback errors clear only the selected queue before returning
an exact queued-delivery count; successful restart reconnects an empty inbox.

For applications implementing `MessagingApplication`, `dispatch_one`
validates `Running`, snapshots current lifecycle states, and removes at most
one oldest FIFO message. `ApplicationMessageContext` exposes only the
application identity and bounded publication. Capacity-one self-publication can
occupy the slot freed by the in-flight item. Success keeps `Running`; a
returned message error drops the attempted item, commits only the selected
record to terminal `Failed`, clears its remaining queue, and retains peer
deliveries already accepted.

The runtime creates no thread or executor and has no automatic or batch
dispatch, multi-application fairness rule, schedule, clock, events, or
configuration access. RFF-REQ-003 is verified; RFF-REQ-008 remains partial
until a structured failure event exists.

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

No implementation work is in progress. The approved dual licence has been
applied without changing Rust behavior or enabling Cargo publication. The next
technical slice remains structured events.

## Highest risks and uncertainties

- Runtime-owned inbox configurations are positional. Count and identity order
  are proven, but two valid capacity/topic configurations can still be swapped
  by mission composition.
- Dispatch refreshes a preallocated lifecycle-state snapshot before every
  callback. The serial publish-only context keeps it stable; concurrency would
  require a different design.
- Callback publication is immediate and non-transactional. Peer deliveries
  remain even when the publisher later returns an error.
- Inline payload slots occupy and copy the configured maximum for short
  payloads. No mission payload size has been measured or selected.
- Mission-defined topic equality and per-publication report allocation are not
  a general non-blocking or real-time guarantee.
- A returned callback error may follow partial application mutation. The runtime
  proves no rollback, cleanup, reinitialisation, panic containment, hang
  containment, or fault tolerance.
- Separate context-free work and message callbacks are proportionate now, but a
  later common service context may require pre-v0.1 API revision.
- No CI currently executes the local baseline.

## Important unresolved decisions

- No structured event representation, event queue bound, or overflow policy is
  selected.
- No mission payload limit, external topic identifier, or wire representation
  is selected; current messages are in-process values only.
- RFF-REQ-007 still needs a host-adapter grammar and validation boundary.
- No explicit minimum supported Rust version or panic-containment policy has
  been selected.
- A shared application service context, automatic dispatch order, fairness, and
  scheduling remain unselected.

## Most likely next tasks

1. Record and implement the smallest bounded structured-event path without a
   recursive or unbounded diagnostic side channel.
2. Add injected manual time only after the event timestamp consumer is defined,
   then address caller-driven scheduled work.
3. Preserve RFF-REQ-003 routing, lifecycle, self-publication, and dispatch
   regressions while later services cross the application boundary.

## Latest run

2026-08-26: Applied the human-approved recipient-choice `MIT OR Apache-2.0`
licence after Daniel Smith confirmed the exact notice and his authority to
license all current content. The canonical MIT and Apache files match their
audited byte counts and SHA-256 hashes. Cargo metadata and the actual generated
package retain the SPDX expression, both licence files, and `publish = false`.
The complete baseline still passes 38 tests with warnings denied. No Rust source,
dependency, NOTICE, per-file header, name or trademark claim, publication,
release, protocol, technical behavior, or push changed.
