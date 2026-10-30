# ADR-0034: Subscription-respecting selective publication

- Status: Implemented; local verification passed, publication review pending
- Date: 2026-10-31
- Scope: One host-selected subscriber through the existing message owners

## Context

The work-output guide limits retries to one subscriber. Initial publication can
return Delivered for one subscriber and InboxFull for another. Republishing the
same topic then duplicates the accepted copy if that subscriber has drained.
Reports expose destination identities, but existing publication always fans out.
Root Codex and independent consumer_assessment confirmed this by source inspection;
it is controlled consumer friction, not evidence of external adoption.
RFF-LIB-REQ-009 precedes implementation.

## Decision

Add publish_to(application_id, message) to MessageBus and host MessagingRuntime,
returning `Result<DeliveryStatus, PublishToError>`. Validate endpoint membership,
then subscription, then reuse lifecycle-aware bounded enqueue. UnknownApplication
and NotSubscribed are nonmutating typed errors; no direct-address bypass is
introduced. Standalone endpoints remain available. No report allocation, retry
history, queue policy or callback-context capability is added. Existing publish
and its per-destination report retain their behavior.

A public-only consumer retains one message and two acceptance flags. It seeds
the flags from the initial fan-out report and retries only rejected destinations.
It admits no next record until both destinations have accepted. Caller policy
owns retry order, cancellation and eventual disposal. Accepted means enqueued:
later stop/failure can discard a copy, and the ledger must not imply processing.

## Alternatives considered

- Republish the whole topic: rejected for this consumer because it can duplicate
  an earlier acceptance. Receiver deduplication would require another contract.
- Separate per-destination topics: feasible mission workaround, but duplicates
  routing vocabulary and changes the meaning of one shared logical topic.
- Subset or report-driven retry API: defer; it introduces collection validation,
  report/message provenance and resource questions unnecessary for one attempt.
- One selected subscribed destination: selected as the smallest capability that
  lets the consumer express its own bounded acceptance policy.

## Evidence, consequences and risks

Official Rust slice get_mut and VecDeque queue documentation supports checked
endpoint access and reuse of FIFO enqueue. The selective-routing and retry
semantics are local design choices; SRC-RUST-SELECTIVE-PUBLICATION records
provenance. No upstream source code or dependencies are reused.
The [focused review](../verification/SELECTIVE_PUBLICATION_REVIEW.md) will retain
actual local, independent-consumer, source/diff and exact hosted evidence.

IDs remain caller-scoped and cannot detect same-position foreign issuers.
Subscription membership does not authenticate a caller. Two independent attempts
are not transactional; repeated calls can duplicate messages. Successful enqueue
does not guarantee callback execution, retention after stop, eventual delivery,
fairness, timing or crash recovery. No new stability, qualification or operational
claim is made. Revisit for demonstrated callback-side feedback, subset validation
or retry ownership that this primitive cannot express. Separate consequential
authorization gates remain unchanged.
