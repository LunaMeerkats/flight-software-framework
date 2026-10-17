# Direct-runtime failure-event history review

Date: **2026-10-18**
Reviewer: **Codex**, with independent event_gap and records_review agents
Disposition: **Accepted: published checkpoint and exact-revision CI passed**

## Question and selected boundary

RFF-REQ-005/008, [ADR-0016](../adr/0016-returned-work-failure-events.md) and
the existing direct/messaging scenarios already establish cooperative ordinary-
work failure reporting, saturation/retry, suppression and peer progress. The
[standalone queue history model](EVENT_QUEUE_SEQUENCE_REVIEW.md) adds mixed FIFO
history evidence but excludes runtime event production. None composes two
distinct failures, separate retained rejected events and later retries while
manual time changes. This is an evidence gap, not an identified production bug.

Select one direct-runtime model. A single faulting application plus healthy peer
would be smaller but could not expose crossed retry identities or ordering of
two independent failures. A messaging-owned model would add publication/inbox
history machinery before this simpler event composition is established; retain
its existing scenarios and distinct owner evidence. Length five already contains
two failures separated by time plus a dequeue/retry, and two rejected events
competing for a recovered slot. Length six adds substantial enumeration without
a newly identified question. No new external research, source reuse, production
policy, API, requirement meaning, dependency, licence, lint or workflow change.

## Frozen domain

The plan froze this domain before implementing
[runtime_event_sequences.rs](../../tests/runtime_event_sequences.rs):

- Three initially Running applications in one direct Runtime. A/B fail their
  first work callbacks with distinct codes 11/22 and AlphaFailure/BetaFailure
  identifiers; the third application remains healthy.
- One event queue at each capacity one and two, preloaded with a distinctive
  Framework/Warning diagnostic at elapsed 99 seconds. Generated time is earlier;
  FIFO is acceptance order, independent of timestamp order.
- WorkA, WorkB, PeerWork, Advance1s, Dequeue, RetryA and RetryB; every string
  of lengths zero through five, independently initialized: 39,216 traces and
  189,546 generated operations. The manual clock starts at zero.
- The harness retains a rejected A/B event in that application's caller-owned
  retry slot. Retry with no retained event is a no-op; rejection keeps the same
  record; acceptance clears the slot. Recorded initial attempts are not retried.
  This is one explicit caller policy, not an automatic framework service.
- After each trace, one supplemental healthy peer call and a complete retained
  FIFO drain followed by two empty reads expose final state. The 39,216 peer
  probes and independently enumerated 116,928 dequeues are outside action counts.

## Oracle and observations

The reference records abstract Older or Failure(application, numeric seconds)
values in an append-only accepted ledger and advances one consumed ordinal. It
does not use VecDeque, production storage, transition helpers or observed queue
contents to predict subsequent results. Separate first-failure timestamps,
invocation counts, integer elapsed time and symbolic retry slots predict runtime
and caller effects. Actual retry slots retain the validated returned event;
expected slots retain independently constructed symbolic records. Event mapping
occurs only at the public comparison boundary.

After every generated operation, assertions compare exact work/lifecycle errors,
event source/severity/identifier/timestamp and Recorded/QueueFull outcome, all
application states and callback counts, failure-only clock-read counts, capacity,
pending, empty and retry copies. Dequeue checks exact records and order. Capacity,
trace and step identify failures. Fixed count assertions protect enumeration.

The clock wraps ManualClock solely to count injected reads. The oracle advances
its own integer seconds. Retry neither calls work nor captures a new timestamp;
healthy work and repeated terminal rejection neither read the clock nor emit.
Source inspection separately confirms Runtime::work commits terminal Failed
before returning to the event constructor; the application-error arm captures
one clock reading and calls emit once, while lifecycle rejection captures none.
Public post-call observations cannot establish that internal ordering alone.

## Local verification and review

Input d1e4989b48f71651f393bdbf2503b265b027bf91 matches refreshed origin.
Fresh exact-input hosted run 38906264892 reports success. Initial locked format,
check, Clippy, tests, warnings-denied rustdoc and both whitespace checks pass
149 workspace tests. An initial fixture compile failed because u16 does not
implement Error; a local WorkFailure newtype corrects the fixture before the
successful focused test and focused warnings-denied Clippy checks.

Root and independent event_gap accept final test blob
0a175efa31066e00e05eb6ac7a13203292b1258e: the complete 403-line source,
oracle independence, frozen domain and progressive organization. Independent
enumeration confirms generated counts and separate supplemental observations.
No new width findings, waiver or source-policy change is needed. Ignored review
aids and raw logs are retained under
target/review-2026-10-18-work-events; these are not adopted repository checkers.

Final local checks all pass:

```text
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
cargo test --locked --test runtime_event_sequences
cargo clippy --locked --test runtime_event_sequences -- -D warnings
git diff --check
git show --format= --check --diff-merges=first-parent HEAD
```

Rustdoc uses RUSTDOCFLAGS=-D warnings. The full suite passes 150 workspace tests;
the focused model passes one. Local tools remain Rust/Cargo 1.98.0, rustfmt
1.9.0-stable and Clippy 0.1.98. Unchanged host/sample, workflow and ADR-0018/0019
probes do not trigger their conditional local commands; these are not claimed run.

Whole-tree review covers 42 Rust files/16,520 physical lines/1,129 comment-only
lines, zero width findings and three unchanged fulfilled expectations. All 370
relative links and 114 exact traceability functions resolve. All six changed
rendered documents pass complete content/HTML structure inspection, without a
pixel or human-review claim. Root and independent records_review accept the
complete source/document/diff, raw local/fault evidence and source hash identity.

## Selected fault sensitivity

Clean isolated control check/test exits zero. All six selected source variants
compile successfully, then fail assertions in the final focused test:

| Source fault | First observed witness |
| --- | --- |
| Omit emit and report Recorded | Capacity 1, WorkA: QueueFull outcome mismatch |
| Emit the same event twice | Capacity 2, Dequeue/WorkA: pending 2 instead of 1 |
| Attribute to Framework | WorkA: exact source mismatch |
| Use Warning severity | WorkA: exact severity mismatch |
| Read clock on lifecycle rejection | WorkA/WorkA: two reads instead of one |
| Read clock but replace timestamp with zero | Advance/WorkA: zero instead of 1 second |

The sensitivity agent retains exact patches, copied sources/test hashes, command
exit codes and raw logs under mutations/run-20261017T130030Z. Root independently
checks the source replacements, production/test hash identity and behavioral
witnesses. These are six selected faults, not mutation completeness or evidence
of a production defect. Source remains unchanged after the final successful
focused/full checks.

## Publication evidence

Checkpoint 9fc7e0240b36f7eca76f81c5b48030ff8904a338 is published by ordinary
fast-forward to codex/nightly; the remote head matches.
[Push run 39035017652](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/39035017652)
passes its exact logged checkout, Windows job 117402970574 and all 20 steps.
Raw logs confirm 150 workspace tests including this model, separate 14 adapter/
five sample tests, warnings-denied rustdoc, all 222 unchanged sample report lines
and both whitespace checks. Root and independent Codex records_review inspect
raw hosted JSON/logs separately from local/source acceptance.

Hosted Rust/Cargo 1.99.1, rustfmt 1.10.0-stable, Clippy 0.1.99, runner 2.337.0
and windows-2025-vs2026 image 20261007.264.1 match the input. Local execution
remains on 1.98.0. No toolchain-policy change or new waiver is needed. This
evidence follow-up preserves source/configuration/workflow blobs; its hosted
result must be checked independently rather than inherited from the checkpoint.

The evidence follow-up audit resolves 371 relative links/114 exact test references;
all seven run-changed rendered documents pass complete content/HTML structure
inspection. The source/configuration/workflow blobs remain unchanged.

## Limits and next candidate

Finite evidence does not cover arbitrary applications, capacities, identifiers,
content, initial states, Registered/Stopped/unknown requests, start/stop/restart
errors, configuration, schedules, messaging ownership/cleanup, allocator failure,
panics/hangs, concurrency, provenance, recovery or delivery guarantees. Existing
scenarios retain error-source-chain and identity-scope evidence. A duplicate
emit on an already-full queue is observationally equivalent here; source review
supports exact attempt count alongside detectable duplication with spare room.
The harness retry policy does not prove arbitrary caller retry handling.

Next, assess whether messaging-owned event histories add meaningful integration
evidence beyond this direct model and the existing inbox-cleanup/failure-event
scenarios. Select that work only if it justifies the additional oracle machinery.
