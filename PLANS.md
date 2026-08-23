# Plan: prove bounded available-endpoint fan-out

Status: **In progress**
Date: **2026-08-24**

## Objective

Implement the smallest available-endpoint routing core from ADR-0004: an
inline payload with an enforced maximum, immutable bounded inbox topology,
stable publish/subscribe fan-out, exact reject-newest saturation, and an
ordered publisher-visible result.

## Context

Stage 1 and the first source-quality checkpoint are complete. The clean
pre-change baseline passes with 16 tests. ADR-0004 already fixes queue, routing,
ordering, and overflow behavior, but the first payload representation was still
unselected.

This slice will use a mission-selected `Copy + Eq` topic type and an inline
const-generic byte payload. It will model configured endpoints as available so
the queue and fan-out policy can be verified without prematurely changing
`Application::work`. Runtime ownership, lifecycle-derived unavailability,
queue clearing on stop/failure, restart reconnection, and application access
remain later integration work. RFF-REQ-003 will therefore remain partial.

## Acceptance criteria

- Enforce the message payload maximum, accepting exactly the configured bound
  and rejecting one byte beyond it.
- Construct the caller-supplied configured inboxes and unique topic
  subscriptions as one immutable topology with positive per-inbox capacities.
- Require application identities in registration order and document the
  existing same-slot foreign-identity limitation.
- Reserve queue and topology storage during construction; add no dependency,
  thread, executor, wait for inbox capacity or subscriber progress, retry,
  spill path, or hidden work.
- Deliver accepted messages FIFO within each inbox across topics.
- Retain older entries and reject the newest delivery at the exact logical
  capacity while continuing fan-out to unaffected subscribers.
- Return stable registration-order destination outcomes whose derived class is
  `NoSubscribers`, `Complete`, `Partial`, or `WhollyUndelivered`.
- Reserve the complete report before mutating an inbox so report-allocation
  failure delivers nothing.
- Test duplicate topics, invalid topology, unknown identities, payload bounds,
  exact saturation, cross-topic FIFO, partial fan-out, all-full publication,
  stable result ordering, and no-subscriber publication.
- Keep new and touched Rust within the source-form policy without adding a lint
  suppression.

## Files and components

- `src/messaging.rs`: bounded messages, topology, fan-out outcomes, and inbox
  access.
- `src/lib.rs`: the intentional public pre-v0.1 surface.
- `tests/message_bus.rs`: public behavior and boundary evidence.
- `docs/adr/0010-bounded-message-routing-core.md`: representation and slice
  boundary.
- `README.md`, ADR-0004, project state, roadmap, and traceability: truthful
  partial implementation and evidence.
- This plan: acceptance, verification, risks, and stopping point.

## Verification approach

- Run `cargo fmt --all -- --check`.
- Run `cargo check --workspace --all-targets --all-features`.
- Run `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
- Run `cargo test --workspace --all-features`.
- Run warnings-denied all-feature rustdoc generation.
- Audit handwritten Rust physical and comment-only widths and all reasoned
  expectations.
- Resolve relative Markdown links; render changed documents; check requirement
  and source identifiers and table shapes.
- Run `git diff --check` and review the complete diff.

## Known risks

- The routing core is not yet owned by `Runtime`; callers could pair it with a
  different same-shaped identity issuer or fail to synchronize lifecycle state.
- Treating configured endpoints as available is only a routing test boundary,
  not the accepted stopped/failed availability policy.
- Inline payload slots consume the configured maximum even for short payloads;
  a poorly chosen maximum can waste storage or stack space.
- External code can retain dequeued messages and publish reports, so this slice
  bounds framework-owned inbox/topology storage rather than all caller memory.
- A returned report is allocated per publish. Allocation failure must occur
  before any delivery; process-wide allocation failure remains outside broader
  fault-containment claims.

## Safe rollback or stopping point

Stop after the available-endpoint routing core, its public tests, ADR, and
partial traceability are coherent and verified. Do not add lifecycle hooks,
service contexts, automatic dispatch, event reporting, time, scheduling,
runtime mutation APIs, or external protocol identifiers in this run.
