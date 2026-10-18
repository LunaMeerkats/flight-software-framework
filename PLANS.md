# Messaging-owned failure-event history checkpoint

Date: **2026-10-19**
Status: **Complete: checkpoint published and exact-revision CI passed**

## Objective and context

Join ADR-0011 inbox cleanup and peer retention with ADR-0020 ordinary-work
failure events. Existing messaging-work scenarios use one fixed failure with
prefilled inboxes; the work-failure history model has no event queue, and the
direct event history model has no messaging owner. A separate reduced model
can expose owner bypass, incorrect discard counts and peer message loss amid
event saturation, changing time and explicit retry. No production defect is
identified. This supplements RFF-REQ-003/005/008 without changing their meaning.

Input 15f5180f959b2be739ae9db6398a3842ad2bab9d is clean on codex/nightly
and matches refreshed origin. Fresh input hosted run 39035292887/job
117403763701 passes its exact checkout, all 20 steps, 150 workspace tests,
separate 14 adapter/five sample tests and the unchanged 222-line sample.
Initial locked format/check/Clippy/test/warnings-denied rustdoc and both
whitespace checks pass 150 workspace tests. Local Rust/Cargo 1.98.0,
rustfmt 1.9.0-stable and Clippy 0.1.98 remain unchanged. Codex root and
event_gap assess the gap; records_review independently checks input evidence.

## Frozen domain and oracle

- Two initially Running applications, inbox capacities [1,2], one shared topic
  and both selected fault positions. Selected ordinary work returns one
  concrete error; the peer remains healthy and passively records messages.
- Event capacities one/two, preloaded with an older diagnostic at elapsed 99s.
  Manual time starts at zero. Publish uses the generated step as a unique byte.
- Publish, FaultWork, PeerWork, PeerDispatch, Advance1s, EventDequeue and
  RetryFault; every independently initialized string through length five:
  78,432 traces and 379,092 generated operations across four domains.
- Independent append-only message acceptance histories, consumed ordinals and
  failure discard watermark predict inboxes and peer callback bytes. Symbolic
  event acceptance/cursor, failure/invocation history and integer elapsed time
  predict exact events, outcomes, callback counts, states and clock reads.
  No production transition/storage helper or VecDeque supplies the oracle.
- One caller retry slot retains only a rejected failure event, preserving its
  original timestamp. Retry without a slot is a harness no-op; successful
  retry clears the slot. This is explicit caller policy, not a framework API.
- Compare after every action with domain/trace/step context. Terminal healthy
  peer work and message/event FIFO drains are separate supplemental probes.

## Files, acceptance and verification

Add tests/messaging_event_sequences.rs and a focused verification review.
Reconcile AGENTS layout, project state, roadmap, traceability and CI evidence.
Production, APIs, requirements, dependencies, licences, lint and workflow stay
unchanged. Existing repository contracts suffice; no external research or source
reuse is needed. ADR-0024 supplies routine autonomous review authority.

Acceptance requires focused/full locked checks, independent oracle/source and
complete-diff review, whole-tree widths/waivers, resolving relative links and
exact traceability references, and changed rendered-document inspection.
Selected isolated ownership faults must compile then fail behavioral assertions;
retain exact patches/logs and source/test hashes outside tracked source.

```text
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
cargo test --locked --test messaging_event_sequences
cargo clippy --locked --test messaging_event_sequences -- -D warnings
git diff --check
git show --format= --check --diff-merges=first-parent HEAD
```

Rustdoc uses RUSTDOCFLAGS=-D warnings. Unchanged host/sample, ADR-0018/0019
experiments and workflow do not trigger extra local commands. Ignored review
aids and logs belong under target/review-2026-10-19-messaging-events.

## Risks and safe stopping point

The finite domain excludes arbitrary callbacks/content/capacities, other
lifecycle states, stop/restart, configuration, schedules, message callback
failure and multiple returned-work faults. Existing evidence covers those
separately. Commit/cleanup-before-clock ordering still requires source review;
the borrowed owner is inaccessible from the clock. Logical capacities do not
bound callback allocations/duration. No allocator, panic, hang, concurrency,
recovery or delivery guarantee is inferred. Preserve unrelated work, publish
reviewed commits by ordinary fast-forward, inspect exact-head CI and stop after
this one checkpoint.

## Local execution

Test blob 6d1d8c2dbcd45ca79d3914cdd7cc5ac73c3fa1af implements the frozen
domain. Focused test and Clippy pass. The final full locked baseline passes
151 workspace tests, warnings-denied rustdoc and both whitespace checks.
Root and independent records_review accept the oracle/source; separate numeric
occupancy enumeration reproduces every count. Supplemental probes comprise
78,432 healthy peer calls, 61,384 message dispatches, 156,864 final dispatch
checks and 217,988 event dequeues outside generated operations.

Clean isolated control check/test pass; six selected faults compile then fail
behavioral assertions: owner bypass with a correct reported count, zero discard
count after real cleanup, successful-work cleanup, clearing the healthy peer,
reporting into a disposable event queue, and a clock read on owner lifecycle
rejection. Root separately inspects raw logs, patches and source/test hashes.
Copies and manifests remain in mutations/run-20261018T130908Z under the ignored
review directory. Production and test hashes remain unchanged by the trials.

Whole-tree audit finds 43 Rust files/17,101 physical lines/1,138 comment-only
lines, zero widths and three unchanged fulfilled expectations. All 374 relative
links and 115 exact test references resolve. Six changed rendered documents pass
full content/HTML structure inspection, without pixel or human-review claims.
An initial rendering inspection found two unquoted Rust paths consumed as
Markdown syntax; code spans correct them and the complete rerun passes.
Root and independent Codex records_review accept the complete source/document
diff and raw local/fault evidence.

## Publication and stopping point

Checkpoint 27194287b8670b75efac05f3a3f0a7de38a312d5 is published by ordinary
fast-forward; refreshed origin/live remote match. [Push run 39130580285](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/39130580285)
passes exact logged checkout, job 117684735602 and all 20 steps, 151 workspace
tests including the model, separate 14 adapter/five sample tests, warnings-denied
rustdoc, all 222 unchanged sample report lines and both whitespace checks.
Root and independent Codex records_review inspect raw hosted JSON/logs separately
from local source/document/diff acceptance.

Hosted Rust/Cargo 1.99.1, rustfmt 1.10.0-stable, Clippy 0.1.99, runner 2.337.0
and windows-2025-vs2026 image 20261007.264.1 match the input; local execution
remains on 1.98.0. This evidence follow-up preserves source/configuration/workflow
blobs and receives complete document/diff review and its own exact-revision CI.
Stop after this one completed checkpoint.

The evidence follow-up resolves 375 relative links/115 exact test references.
All seven run-changed rendered documents pass full content/HTML structure
inspection; source/configuration/workflow blobs remain unchanged.
