# Finite runtime message-callback failure sequence review

Date: **2026-10-13**
Status: **Complete: checkpoint published and exact-revision hosted CI passed**

## Contract and reviewed input

Input `d672b42bb3eb1dd77f7b907ba6a6f4a8a7e7b4db` is clean on codex/nightly,
equal to refreshed origin. Exact input hosted run 38142393905 is freshly
successful. Initial locked local baseline passes 144 workspace tests.
The accepted [host review](HOST_V0_1_REVIEW.md), selected
[message-failure scenarios](MESSAGE_FAILURE_REVIEW.md), and earlier finite
models remain separate evidence.

RFF-REQ-002/003, the peer-operability aspect of RFF-REQ-008 and
[ADR-0012](../adr/0012-application-message-dispatch.md) supply the contract.
Running dispatch removes at most the oldest queued message before a callback.
An empty inbox invokes no callback. The publish-only context uses the current
Running snapshot, so one self-publication can occupy the freed slot. Callback
publication has immediate per-destination results; accepted peer deliveries
survive a later callback error. A returned concrete error commits only the
selected application to terminal Failed and clears its remaining queue,
including accepted self-publication. The exact discard count excludes the
attempted in-flight input. Rejected terminal dispatch changes no queue and
invokes no callback.

This increment verifies those existing contracts. It does not change production,
API, requirements, dependencies, licence, policy or workflow. Repository sources
suffice; no external research, source reuse or upstream behavior claim is needed.

## Frozen domain and alternatives

Root Codex and parallel Codex scope_review freeze before implementation:

- Two initially Running applications, capacities [1,2], one shared Command
  topic, both fixed fault roles. All lifecycle/work callbacks succeed.
- Three generated operations: Publish with its operation-position byte,
  Dispatch(fault), Dispatch(peer). Independently initialize every length zero
  through six: 2,186 traces and 12,030 generated operations; assert both counts.
- The fault callback records input, publishes one distinct tagged reply to
  the shared topic, then returns a concrete error. The peer callback records
  input and succeeds without publication.
- Supplemental final passive peer dispatch drains retained FIFO and verifies
  empty suppression. Probe the fault only when already Terminal. These do not
  introduce an uncounted failure into an unfailed prefix and are excluded from
  generated counts. Every generated prefix is independently enumerated.

Both roles cover position/capacity asymmetry. Generating stop/restart would
repeat successful lifecycle coverage; events/schedules/configuration add other
responsibilities. Length six includes saturation, peer consumption before/after
failure, freed-slot self-publication and later terminal rejection. Arbitrary
domains and callback policies remain outside this checkpoint.

## Oracle and observability

The independent reference uses Active/Terminal states, hard-coded
routes [0,1], capacities [1,2], append-only accepted payload histories and retired
cursors. External publication appends only when active and below capacity.
Dispatch consumes one oldest input. The fault's reply is independently routed
before terminal commitment, then exactly its remaining suffix is retired.
Peer dispatch retires only one receipt. Oracle decisions do not query runtime
state or reuse production transitions, queue helpers or classification logic.

The public comparison boundary translates predictions into exact states,
ordered statuses/classifications and typed errors. Compare capacities, pending
counts, callback receipts and publication observations initially and after each
operation. Distinct external/reply bytes expose origin, replacement and order.
Follow the actual returned standard error source chain to its concrete error.
Final peer drains compare the retained FIFO without changing an unfailed
fault application's behavior. Temporal ownership/cleanup order remains source-
inspection evidence; tests observe callback values and post-return outcomes.

Test-only Rc/RefCell/Vec observations are finite fixtures, outside operational
queue bounds. Logical slots and one in-flight record do not bound allocator
overhead, application-retained data, callback duration or whole-process bytes.

## Verification and review

Root Codex owns plan and records; parallel Codex scope_review implements the
test. Parallel evidence_review audits policies/source/durable/rendered documents;
sensitivity reviews oracle independence and selected isolated source faults.
Root Codex and parallel evidence_review/sensitivity accept the final oracle and
source form without a blocking finding. Final source blob is
`8177b53c0367b5a1c3b8e0e03bbd140122456395`; no new expectation/allow is added.
Initial/final full locked baselines pass 144/145 workspace tests. The focused
model passes one test over both asserted aggregate counts. Completed commands:

```text
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
cargo test --locked --test runtime_message_failure_sequences
cargo clippy --locked --test runtime_message_failure_sequences -- -D warnings
git diff --check
git show --format= --check --diff-merges=first-parent HEAD
```

Rustdoc uses RUSTDOCFLAGS=-D warnings. Local tools remain Rust/Cargo 1.98.0,
rustfmt 1.9.0-stable and Clippy 0.1.98. Source audit covers 37 Rust files/14,480
physical lines and 1,091 comment-only lines, zero widths and three unchanged
fulfilled expectations. Conditional host sample/adapter commands, ADR-0018/0019
probes and actionlint are unchanged and not triggered locally. Hosted CI still
executes its configured focused host/sample checks. The initial committed-tip
whitespace check passes; the new tip is checked separately before publication.
Ignored aids/logs remain under target/review-2026-10-13-dispatch.

### Selected fault sensitivity

Parallel Codex sensitivity copies the reviewed final test and production inputs
into isolated packages, never editing tracked production. For each of six
selected source faults, `cargo test --locked --test
runtime_message_failure_sequences --no-run` exits zero and the same focused
command without --no-run exits 101 on a behavioral assertion:

| Source fault | Observed rejection |
| --- | --- |
| Include the in-flight input in discard count | Publish/Dispatch(fault): count 2 instead of 1 |
| Retain selected queue | Same history: selected pending 1 instead of 0 |
| Clear peer queue | Same history: peer pending 0 instead of external input plus accepted reply |
| Reject selected self-publication | Same history: discard count 0 instead of 1 |
| Dequeue newest first | Publish/Publish: final passive peer receipt 1 instead of 0 |
| Return InboxEmpty for terminal dispatch | Terminal probe expects a typed rejection, receives success |

The first witnesses occur with fault role zero. The terminal probe's panic does
not print its trace; the deterministic enumeration/source identifies the first
Publish/Dispatch(fault) history. No claim of logged trace detail is made for
that fault. The other role remains covered by the passing reference traversal,
not by later fault witnesses after each trial stops. These selected detections
do not prove that every incorrect implementation would be rejected. Exact
substitutions, compile/test exit codes, assertion diagnostics and copied-input
provenance remain in mutations/final-copy/mutation_results.json and trial logs.

Root and parallel Codex evidence_review accept the complete diff and durable
records without a blocking finding. All 342 relative links across 66 Markdown
files and 109 exact traceability functions resolve. The six changed local
documents pass rendered HTML structure and complete content comparison:
headings, code, links, tables, normalized text and balanced nesting. This is
document evidence, not pixel or human acceptance. Publication and hosted
execution are recorded separately below; local checks alone establish neither.

## Publication and exact-revision hosted evidence

Checkpoint `fe40d592eb434a0730eac8c14744361e9d685200` contains the accepted
test blob and reviewed local checkpoint. Ordinary fast-forward publication
succeeds and remote/head match; the tree is clean at checkpoint publication.
[Hosted push run 38253819562](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/38253819562)
passes its exact logged checkout, Windows job 114826598201 and all 20 steps.
Raw logs confirm 145 workspace tests including the new model, separate 14
adapter/five sample tests, warnings-denied rustdoc, the executed sample's full
unchanged report and both whitespace checks. Root inspects the raw JSON/logs
and sample comparison; parallel Codex evidence_review reviews them separately.

Runner 2.337.0, windows-2025-vs2026 image 20260925.250.1, Rust/Cargo 1.99.0,
rustfmt 1.10.0-stable and Clippy 0.1.99 match the prior hosted baseline.
Whole-tree source/configuration review finds no policy or waiver drift. Local
execution remains on 1.98.0; it does not claim local 1.99.0 execution.

This later documentation follow-up records completed checkpoint evidence and
requires its own exact-head hosted result. Its final audit/render/complete-diff
review resolves 343 relative links/109 exact functions, inspects all seven
run-changed rendered documents and preserves the source blob.
The checkpoint remains finite test evidence, with no expanded capability claim.

## Limits and next work

This fixed cooperative policy excludes Registered/Stopped, lifecycle/work
errors, arbitrary topics/capacities/payloads/lengths/callbacks, events, schedules,
configuration, panics/hangs, allocator failure, concurrency, performance and
whole-process bounds. Failed remains terminal; no retry, recovery or message-
error event integration follows. Acceptance does not establish human review,
flight qualification, operational suitability, compatibility or release status.

The next candidate is a separately frozen reduced one-shot schedule-consumption
and returned-work-failure model, after assessing its gap against existing tests.
