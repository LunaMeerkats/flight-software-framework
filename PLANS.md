# Direct-runtime failure-event history checkpoint

Date: **2026-10-18**
Status: **Complete: checkpoint published and exact-revision CI passed**

## Objective and context

Supplement ADR-0016 and RFF-REQ-005/008 evidence with mixed direct-runtime work,
event dequeue, manual time and explicit rejected-event retry histories. Existing
fixed direct/messaging scenarios and the standalone event model cover individual
contracts. They do not compose two distinct returned failures and retained retry
records across changing time. No production defect is identified.

Input d1e4989b48f71651f393bdbf2503b265b027bf91 is clean on codex/nightly,
equal to refreshed origin; fresh input hosted run 38906264892 reports success at
that exact head. Initial locked format/check/Clippy/test, warnings-denied rustdoc
and both whitespace checks pass 149 workspace tests. Local tools remain
Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy 0.1.98. Codex root and
independent event_gap/records_review assess existing contracts and alternatives.
ADR-0024 supplies routine autonomous review authority.

## Frozen domain and oracle

- One direct Runtime with three initially Running applications. A and B return
  distinct errors on their first work callbacks; the third remains healthy.
- One shared event queue at capacity one or two, preloaded with a distinctive
  Framework/Warning diagnostic at elapsed 99 seconds, after generated times.
- WorkA, WorkB, PeerWork, Advance1s, Dequeue, RetryA, RetryB; every independently
  initialized operation string of lengths zero through five: 39,216 traces and
  189,546 generated operations. The manual clock starts at zero.
- Caller policy retains only rejected A/B events in separate retry slots.
  Retry without a retained event is a harness no-op; rejection retains the same
  copy, successful retry clears that slot. This is test policy, not a new API.
- An independent symbolic accepted-record ledger/consumed ordinal, failure and
  invocation history, and integer elapsed time predict exact outcomes, errors,
  event fields, lifecycle states, callback counts and failure-only clock reads.
  No production transition/storage helper or VecDeque is used by the oracle.
- Compare observations after each action with capacity/trace/step context.
  Terminal FIFO drains and one healthy peer-work probe are separate observations
  outside generated counts. Retry never calls work or reads the clock.

## Files and acceptance

Add tests/runtime_event_sequences.rs and a focused verification review; reconcile
AGENTS layout, project state, roadmap, traceability and CI evidence. No production,
API, requirement, dependency, licence, lint or workflow change. Existing local
contracts suffice; no new external research or source reuse is needed.

Acceptance requires exact counts, focused/full locked checks, independent oracle/
source/complete-diff review, whole-tree width/waiver review, resolving relative
links and exact traceability references, and changed rendered-document inspection.
Selected isolated faults must compile then fail behavioral assertions, with exact
patches and logs retained outside tracked source.

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

Rustdoc uses RUSTDOCFLAGS=-D warnings. Unchanged host/sample, ADR-0018/0019
experiments and workflow do not trigger extra local commands. Ignored aids/logs
belong under target/review-2026-10-18-work-events.

## Risks and safe stopping point

The finite domain excludes arbitrary application policies, capacities/content,
Registered/Stopped/unknown identities, configuration, scheduling and messaging
ownership/cleanup. Source review, rather than observing the mutably borrowed owner
inside Clock::now, supports commit-before-clock ordering. Existing scenarios
retain error-source-chain and caller-origin evidence. No allocator, panic, hang,
concurrency, recovery or delivery guarantee is inferred. Preserve unrelated work;
publish only reviewed commits by ordinary fast-forward and inspect exact-head CI.
Stop after this one checkpoint.

## Local execution

Final test blob 0a175efa31066e00e05eb6ac7a13203292b1258e implements the
frozen domain. Focused test/Clippy and the final full locked baseline pass 150
workspace tests; all command exit codes are zero. The initial fixture's missing
Error implementation was corrected before acceptance. Root and independent
event_gap accept all 403 source lines and oracle independence; separate enumeration
confirms 39,216 supplemental peer calls and 116,928 terminal dequeues.

Clean isolated control check/test passes. Six selected faults compile then fail
behavioral assertions: omitted/duplicated emission, Framework source, Warning
severity, clock read on lifecycle rejection and discarded nonzero timestamp.
Root separately inspects raw patches/logs/source and test hashes. Trials remain
under mutations/run-20261017T130030Z in the ignored review directory.

Whole-tree audit covers 42 Rust files/16,520 physical lines/1,129 comment-only
lines, zero width findings and three unchanged fulfilled expectations. All 370
relative links and 114 exact traceability functions resolve. All six changed
rendered documents pass complete content/HTML structure inspection; this is not
pixel or human acceptance. Root and independent records_review accept the complete
source/document/diff and independently inspect raw local/fault evidence.

## Publication and stopping point

Checkpoint 9fc7e0240b36f7eca76f81c5b48030ff8904a338 is published by ordinary
fast-forward; its remote head matches. [Push run 39035017652](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/39035017652)
passes exact logged checkout, job 117402970574 and all 20 steps, 150 workspace
tests including the model, separate 14 adapter/five sample tests, warnings-denied
rustdoc, all 222 unchanged sample report lines and both whitespace checks.
Root and independent Codex records_review inspect the raw hosted evidence
separately from local source/document/diff acceptance.

Hosted Rust/Cargo 1.99.1, rustfmt 1.10.0-stable, Clippy 0.1.99, runner 2.337.0
and windows-2025-vs2026 image 20261007.264.1 match the input. Local execution
remains on 1.98.0. This evidence follow-up preserves source/configuration/workflow
blobs and receives complete document/diff review and its own exact-revision hosted
result. Stop after this one completed checkpoint.

The evidence follow-up audit resolves 371 relative links/114 exact test references.
All seven run-changed rendered documents pass complete content/HTML structure
inspection; source/configuration/workflow blobs remain unchanged.
