# ADR-0035: Callback-owned selective publication

- Status: Implemented; autonomous source review and exact hosted checks passed
- Date: 2026-11-01
- Scope: One selected subscribed publication from a message callback

## Context

ADR-0034 exposes host publish_to and names demonstrated callback-side feedback
as a revisit condition. A message-driven producer can retain one reply and the
initial broadcast acceptance report, but ApplicationMessageContext currently
cannot retry only the rejected destination. Moving its state to the host breaks
the intended application ownership; rebroadcasting can duplicate accepted copies.
Root Codex and independent consumer_assessment identify this by source review,
not external adoption. RFF-LIB-REQ-010 precedes implementation.

## Decision

Add ApplicationMessageContext::publish_to(application_id, &message), returning
`Result<DeliveryStatus, PublishToError>`. Delegate MessageBus's existing checked
selective primitive using the same immutable pre-callback dispatch-state snapshot
as context.publish. Validation remains membership, subscription, availability
and capacity. The callback cannot mutate lifecycle, dequeue or dispatch again.
No new report allocation, queue owner, retry history or ordinary-work context.

A controlled public-only consumer keeps one reply and two acceptance flags in
its application. An explicit later message triggers rejected-only retry. Busy
new-reply commands are consumed without buffering and need explicit resubmission.
Full/unavailable outcomes are
ordinary retained backpressure, not callback errors. Host policy initiates retry
and decides cancellation. Acceptance remains enqueue, not processing.

## Alternatives considered

- Host-owned ledger: remains suitable for host-produced output, but relocates
  this handler's reply state and exposes it outside the application.
- Per-destination topics: feasible but duplicates routing vocabulary for one
  logical reply and couples producers to that workaround.
- Rebroadcast with receiver deduplication: adds another protocol/state contract
  and does not preserve the existing acceptance meaning.
- Callback selective attempt: selected; reuses the existing routing decision
  and closes this ownership gap with one additive method.
- Multi-producer runner/fairness policy: defer until a consumer demonstrates
  that need; this slice requires neither arbitration nor automatic progress.

## Evidence and consequences

Official Rust borrowing guidance supports simultaneous disjoint struct-field
borrows; std VecDeque documents FIFO queue operations. Reuse the existing safe
bus borrow, immutable lifecycle snapshot and logically bounded enqueue rather
than introducing interior mutability or a parallel routing implementation.
SRC-RUST-CALLBACK-PUBLICATION records these language/library facts and local
adopt/defer choices. No upstream source code or dependency is reused.

The [consumer guide](../CALLBACK_PUBLICATION.md) and
[focused review](../verification/CALLBACK_PUBLICATION_REVIEW.md) will record
actual public/independent consumer and source/diff/hosted evidence. Seven focused
and identical independent consumer tests pass; the locked baseline passes 224
workspace tests. Root and independent callback_review accept source and consumer.
Implementation e2ad3484530091e9b51e5f61ead43318187e802e passes exact hosted
run 41204737701 and all 20 steps; later document revisions receive separate checks.

## Risks and revisit conditions

Membership is not authentication; IDs do not encode origin. Availability refreshes
between callbacks and is fixed within each serial callback. Self-publication
uses the just-freed inbox slot. A later callback error clears selected queued
copies but does not undo accepted peer effects. The ledger cannot detect later
discard or prove processing. Repeated attempts can duplicate without caller
policy. Logical storage/operation bounds do not establish process-memory or
timing bounds, fairness, delivery, adoption, stability or qualification.

Revisit for demonstrated subset validation, callback configuration or bounded
retry scheduling needs that current composition cannot express. Separate
consequential authorization gates remain unchanged.
