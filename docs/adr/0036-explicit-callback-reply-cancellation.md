# ADR-0036: Explicit callback reply cancellation

- Status: Implemented; local checks, Codex review and exact hosted checks pass
- Date: 2026-11-02
- Scope: Cancellation policy in the public-only callback reply consumer

## Context

The ADR-0035 consumer retains one reply until both subscribers accept it. A
permanently Failed destination can therefore keep that reply pending forever;
new-reply commands are consumed without buffering their requested values. Root
Codex and independent consumer_assessment identify this friction by inspecting
the controlled consumer, not from external adoption. RFF-LIB-REQ-011 records the
new policy before implementation; RFF-LIB-REQ-010 revision 1 evidence remains.

## Decision

Add a Cancel topic to the consumer's existing bounded command inbox. Dispatching
it clears the application's current optional reply and its acceptance flags,
without publication or destination queue mutation. Idle and repeated Cancel
commands succeed. Retry while no reply is pending does nothing, and a later
new-reply command may be admitted. This is mission policy using existing public
APIs, with no new library method, queue owner or retry service.

Cancel targets whatever reply exists when its callback runs. It carries no
operation identity and has no special admission priority. The host must retain
and resubmit a command rejected by InboxFull or Unavailable, choose dispatch
order, and avoid delayed commands cancelling a later reply unintentionally.
Enqueue acceptance does not prove that cancellation ran. Successful producer
stop clears its command inbox, including queued Cancel commands, while this
consumer's no-op stop/restart callbacks retain its application-owned reply.

Accepted destination copies remain queued or already processed. Cancellation
only ends future attempts for the retained reply; it cannot retract effects or
restore an accepted subscriber's capacity. No acknowledgement or authentication
protocol is added.

## Alternatives considered

- Automatic lifecycle cancellation: reject for this consumer. A stopped
  destination may restart, and lifecycle clearing does not reveal processing.
  This would replace an explicit mission decision with a different policy.
- Host mutation of the producer: reject. Moving or exposing pending state would
  weaken the demonstrated application ownership and is unnecessary here.
- Acknowledgements and operation IDs: defer. They could correlate delayed
  commands or report callback completion, but require another bounded protocol
  and error/backpressure contract beyond cancelling one current reply.
- Explicit command: select. It uses existing publication, FIFO and dispatch
  behavior, preserves ownership, and provides a bounded termination choice.

## Evidence and consequences

Official Option documentation distinguishes Some from None; VecDeque documents
removal from the front. SRC-RUST-CALLBACK-CANCELLATION records these language and
collection facts. Cancelling by replacing the optional application state with
None and ordering commands through the existing inbox are local design choices,
not Rust, cFS or agency requirements. No upstream implementation is reused.

The [consumer guide](../CALLBACK_PUBLICATION.md#cancel-future-attempts) describes
the policy and limits. The
[focused review](../verification/CALLBACK_CANCELLATION_REVIEW.md) records
12 focused and identical independent-package tests and the 229-test locked
baseline passing. Root and independent cancellation_review accept the complete
source/document diff with no findings. Implementation
38e1d9ab8c5548ce5cabf2356b5af161ec2e044b passes exact hosted run 41309924576
and all 20 steps; later document revisions receive separate checks.
Existing selective publication and lifecycle regression evidence is retained.

## Risks and revisit conditions

Cancellation is subject to ordinary command saturation, lifecycle availability,
FIFO order, explicit dispatch and stop/failure clearing. A stale Cancel can
target a newer reply because it has no operation ID. Stop/restart does not itself
cancel the retained reply, and terminal Failed producers cannot process Cancel.
No priority, timing, fairness, eventual processing, rollback or delivery guarantee
is introduced. Existing identity, resource and qualification limits remain.

Revisit for a demonstrated need to correlate delayed cancellation, acknowledge
processing, cancel while the producer is stopped, or coordinate multiple pending
operations. Separate consequential authorization gates remain unchanged.
