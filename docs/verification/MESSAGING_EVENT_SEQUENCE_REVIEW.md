# Messaging-owned failure-event history review

Date: **2026-10-19**
Reviewer: **Codex root, with separate event_gap, records_review and sensitivity agents**
Status: **Complete: checkpoint published and exact-revision CI passed**

## Gap, scope and alternatives

Reviewed input 15f5180f959b2be739ae9db6398a3842ad2bab9d is clean on
codex/nightly and equal to refreshed origin. Fresh exact-input hosted run
39035292887/job 117403763701 passes all 20 steps, 150 workspace tests, separate
14 adapter/five sample tests and the unchanged 222-line sample. Initial full
locked local checks pass 150 workspace tests on Rust/Cargo 1.98.0,
rustfmt 1.9.0-stable and Clippy 0.1.98. ADR-0024 supplies routine review authority.

The fixed messaging_work_events scenarios prefill inboxes before one failure.
The work-failure history model composes publication and clearing without events;
the direct event model has no messaging owner. Neither detects the joint trace
Publish/FaultWork/Publish/EventDequeue/Retry with evolving peer occupancy.
Selected owner bypass, discard-count errors or peer loss could coexist with
correct direct event behavior. No production defect is identified.

Retaining fixed scenarios alone leaves that composition gap. Extending the
direct model would mix owners and repeat two-failure event-identity machinery.
Select a separate, smaller two-application messaging model. Existing ADR-0011,
0012, 0013, 0014 and 0020 specify all expected behavior; no new external source,
upstream claim, dependency or API decision is needed. RFF-REQ-003/005/008 retain
their meanings.

## Frozen domain and oracle

[The plan](../../PLANS.md) freezes two initially Running applications, capacities
[1,2], one shared topic and both fault positions. Selected ordinary work always
returns a concrete error; peer work succeeds and peer dispatch records bytes
without publication. Event capacities one/two begin with a distinctive older
Framework/Warning diagnostic timestamped at 99s. Manual time starts at zero.

Publish(step byte), FaultWork, PeerWork, PeerDispatch, Advance1s, EventDequeue
and RetryFault are enumerated independently through length five: 78,432 traces
and 379,092 generated operations. One caller slot retains a rejected failure
event for optional explicit retry. Rejection retains that copy; acceptance
clears it. Retry without a slot is a harness no-op. This is one test caller
policy, not runtime-owned retry machinery.

Append-only accepted-message histories and consumed ordinals/discard watermark
predict occupancy and peer callback bytes. A symbolic accepted-event ledger and
cursor predict FIFO; separate failure, invocation and integer time histories
predict lifecycle, errors, clock reads and event metadata. The oracle does not
call production transition helpers or reproduce VecDeque storage. Every action
checks exact observable state with fault role, event capacity, trace and step.
Terminal peer work and message/event drains are supplemental probes, outside
generated operation counts.

## Execution and source review

New tests/messaging_event_sequences.rs at blob
6d1d8c2dbcd45ca79d3914cdd7cc5ac73c3fa1af checks the complete frozen domain.
Focused test and warnings-denied Clippy pass; the full locked baseline passes
151 workspace tests, format/check/Clippy, warnings-denied rustdoc and both
whitespace checks. All command exit codes are zero. Root reviews all 581 source
lines; independent event_gap/records_review accept oracle independence and source
form. Separate numeric occupancy enumeration reproduces all asserted counts.
Supplemental observations comprise 78,432 healthy peer calls, 61,384 message
dispatches, 156,864 final empty/lifecycle dispatch checks and 217,988 event dequeues.

Clean isolated control check/test pass. Six selected ownership faults compile
then fail behavioral assertions: inner-runtime bypass with correct reported
discard count, incorrect zero count after actual cleanup, successful-work
cleanup, peer cleanup after selected failure, reporting into a disposable event
queue, and an extra clock read on owner lifecycle rejection. These are selected
sensitivity trials, not mutation completeness or production-defect evidence.
Root independently inspects raw patches/logs and source/test hashes. Temporary
aids, copies, manifest and raw evidence remain under
target/review-2026-10-19-messaging-events/mutations/run-20261018T130908Z.
They are not adopted checker gates. Trials leave root production/test unchanged.

Whole-tree audit covers 43 Rust files/17,101 physical lines/1,138 comment-only
lines, zero width findings and three unchanged fulfilled expectations. All 374
relative links and 115 exact test references resolve. All six changed rendered
documents pass full content/headings/code/links/tables/HTML structure inspection.
An initial inspection caught unquoted Rust paths losing separators in rendered
Markdown; code spans fix them and the complete inspection rerun passes. This is
not pixel or human acceptance. Root and independent Codex records_review accept
the complete source/document diff, raw local/fault evidence and final rendering.

Source review of `MessagingRuntime::work_with_failure_event` confirms delegation
to `Self::work`, which commits the selected failure and clears only that inbox,
before shared error-event construction reads the clock. The wrapper preserves
the exact discard count. A borrowed clock cannot inspect the mutably borrowed
owner, so tests observe returned state/counts and source review supports ordering.

## Limits and disposition

This finite domain excludes arbitrary content, capacities and callbacks,
Registered/Stopped/unknown identities, stop/restart, configuration, schedules,
message-callback failures and multiple returned-work faults. Existing scenarios
and models retain those distinct claims. It checks one bounded input domain,
not all possible executions. The passive peer callback and retry policy are
test choices. Logical bounds do not bound callback allocation or duration;
panics, hangs, allocator failure, concurrency and origin provenance remain
outside the result. No recovery, delivery, qualification or operational claim
is inferred. Invisible repeated attempts against a full event queue require
source review; returned state alone cannot count those attempts.

## Publication evidence

Published checkpoint 27194287b8670b75efac05f3a3f0a7de38a312d5 matches the
refreshed remote after an ordinary fast-forward push. [Run 39130580285](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/39130580285)
passes exact logged checkout, Windows job 117684735602 and all 20 steps. Raw
logs show 151 workspace tests including the new model, separate 14 adapter/five
sample tests, warnings-denied rustdoc, all 222 unchanged sample report lines and
both whitespace checks. Root and independent Codex records_review separately
inspect raw metadata/logs; this is hosted evidence in addition to local review.

Hosted Rust/Cargo 1.99.1, rustfmt 1.10.0-stable, Clippy 0.1.99, runner 2.337.0
and windows-2025-vs2026 image 20261007.264.1 match the input. Local execution
remains on 1.98.0. This later evidence follow-up preserves source/configuration/
workflow blobs and receives its own complete review and exact-revision CI; it
does not inherit the checkpoint's pass.

The follow-up audit resolves 375 relative links and 115 exact test references;
all seven run-changed rendered documents pass full content/HTML structure review.
Source, test, configuration and workflow blobs remain unchanged.
