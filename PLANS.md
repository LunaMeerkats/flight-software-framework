# Runtime message-callback failure sequence checkpoint

Date: **2026-10-13**
Status: **Complete: checkpoint published and exact-revision hosted CI passed**

## Objective and authority

Strengthen existing RFF-REQ-002/003 and the peer-operability aspect of
RFF-REQ-008 with a reduced independent model of returned message-callback
failure. Prior finite models exclude generated dispatch and callback
publication; selected message-failure scenarios do not enumerate composition
with peer saturation and consumption. ADR-0012 supplies the contract.

Input `d672b42bb3eb1dd77f7b907ba6a6f4a8a7e7b4db` is clean on codex/nightly,
equal to refreshed origin. Exact input hosted run 38142393905 is freshly
successful. Initial locked local format/check/Clippy/test/rustdoc and whitespace
checks pass 144 workspace tests on Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and
Clippy 0.1.98. Encoded source policies remain unchanged. Routine autonomous
review follows ADR-0024; completed host v0.1 acceptance remains separate.

## Frozen finite domain

- Two initially Running applications, capacities [1,2], one shared Command
  topic; traverse both fixed fault roles. Lifecycle/work callbacks succeed.
- Three generated operations: Publish with a unique operation-position byte,
  Dispatch(fault), Dispatch(peer). Enumerate lengths zero through six,
  independently initialized: 2,186 traces and 12,030 generated operations.
- A fault callback records its oldest input, publishes one distinct tagged
  reply through the shared topic while still Running, then returns a concrete
  error. The peer message callback only records input and succeeds.
- Independent Active/Terminal states and append-only accepted histories with
  consumption/discard cursors predict FIFO receipts, ordered external/callback
  publication reports and classifications, exact concrete errors/source chains,
  queued discard counts, states, capacities, pending counts and callbacks.
- Supplemental final passive peer drains expose retained FIFO at every prefix.
  Probe fault dispatch only if already Terminal. These observations do not
  introduce an uncounted fault into an unfailed prefix and are excluded from
  generated counts. Every generated prefix is independently enumerated.

One fault role misses position/capacity asymmetry. Adding stop/restart would
repeat prior lifecycle evidence; events, schedules and configuration introduce
other composition responsibilities. Length six includes saturation, freed-slot
self-publication, accepted peer replies, peer consumption and terminal rejection.

## Components and acceptance

Add tests/runtime_message_failure_sequences.rs and a focused verification
review. Update AGENTS layout, this plan, traceability, roadmap and concise state.
Root Codex owns durable records. Parallel Codex scope_review implements the
test; evidence_review audits policy/source/document evidence. A separate Codex
reviewer examines oracle independence and selected isolated source faults.
No production API, dependency, lint exception, requirement or ADR is planned.
Existing repository contracts suffice; no external research or source reuse.

Acceptance requires asserted frozen counts/exact outcomes, focused and full
locked baseline, complete source/diff review, resolving relative links and
traceability function references, whole-tree width/waiver audit, and changed
rendered document structure/content inspection. Selected isolated source fault
trials must compile and fail behavioral assertions without editing production.

```text
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
cargo test --locked --test runtime_message_failure_sequences
git diff --check
git show --format= --check --diff-merges=first-parent HEAD
```

Rustdoc uses RUSTDOCFLAGS=-D warnings. Unchanged host sample/adapters,
ADR-0018/0019 experiments and workflow do not trigger extra local commands.
Ignored aids/logs remain under target/review-2026-10-13-dispatch. Commit and
ordinary fast-forward push only after applicable local verification and actual
Codex review pass; inspect and record exact-revision hosted CI separately.

## Limits and safe stopping point

This is a fixed cooperative returned-error policy and finite domain. It excludes
Registered/Stopped states, start/stop/restart/work errors, arbitrary topics,
capacities, payloads, lengths or callback policies, event/schedule/configuration
composition, panic/hang/allocation failures, concurrency and whole-process or
timing bounds. Peer delivery remains immediate and nontransactional. Failed
remains terminal; no recovery or message-error event integration is introduced.

If a mismatch appears, preserve its smallest trace and resolve it within this
objective. Preserve unrelated changes and revert only current-run implementation
if needed to restore baseline. Stop after this one completed checkpoint.

## Completed local evidence

The focused model passes one test over all asserted counts; initial/final full
locked baselines pass 144/145 workspace tests. Final source blob is
`8177b53c0367b5a1c3b8e0e03bbd140122456395`. Root Codex and parallel
evidence_review/sensitivity independently accept oracle/source form with no
blocking finding. No new lint exception is needed. Whole-tree inspection finds
37 Rust files/14,480 physical lines, zero width findings and three unchanged
fulfilled expectations.

Six isolated copied-package faults each compile with the focused --no-run
command and then fail behavioral assertions: include in-flight in discard,
retain selected queue, clear peer queue, reject self-publication, newest-first
dequeue and misclassify terminal dispatch as InboxEmpty. These are selected
sensitivity trials, not general mutation coverage; production is unchanged.
Root and parallel evidence_review accept the complete diff and durable records.
All 342 relative links across 66 Markdown files and 109 exact traceability
functions resolve. The six changed local documents pass rendered HTML structure
and complete content comparison; no pixel/human acceptance is claimed.

## Publication and stopping point

Checkpoint `fe40d592eb434a0730eac8c14744361e9d685200` is published by ordinary
fast-forward; remote/head match and the tree is clean at checkpoint publication.
[Push run 38253819562](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/38253819562)
passes that exact logged checkout, one Windows job 114826598201 and all 20
reported steps: 145 workspace tests including the model, separate 14 adapters/
five sample tests, warnings-denied rustdoc, full sample and both whitespace checks.
Root inspects raw job JSON/logs; parallel evidence_review reviews them separately.

Runner/image/tools match the prior hosted baseline: 2.337.0, windows-2025-vs2026
20260925.250.1, Rust/Cargo 1.99.0, rustfmt 1.10.0-stable and Clippy 0.1.99.
Local checks remain on 1.98.0. The source blob and all production inputs are
unchanged. This documentation follow-up records completed checkpoint evidence;
its later revision requires its own exact-head hosted CI. Final link/render/diff
review resolves 343 relative links/109 exact functions and inspects all seven
run-changed rendered documents. Stop after this one checkpoint.
