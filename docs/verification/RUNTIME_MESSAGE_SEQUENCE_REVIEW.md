# Finite runtime lifecycle sequence review

Date: **2026-10-11**
Scope: **Successful runtime-owned publication/stop/restart compositions**

## Contract and reviewed input

Input `181cfa913c17177e2696d35fadb790883e4d0ed7` starts clean on codex/nightly,
equal to refreshed origin. Exact input hosted run 37935576798 is freshly
confirmed successful. The fresh locked local baseline passes 142 tests.
The completed [host review](HOST_V0_1_REVIEW.md) and standalone
[message model](MESSAGE_SEQUENCE_REVIEW.md) remain separate accepted evidence.

RFF-REQ-002/003 and ADR-0003/0004/0011 define the contract: only Running accepts
delivery; configured unavailable subscribers remain ordered destinations;
successful stop clears exactly the selected outstanding deliveries; successful
Stopped-to-Running restart reconnects empty; rejected lifecycle requests invoke
no callback, preserve all inboxes and return zero discarded. This checkpoint
adds verification without changing production, requirements, APIs, dependencies,
licences, lint policy or workflow. No new external research or source reuse is
needed; repository decisions are the oracle authority.

## Frozen finite domain and alternatives

Before implementation, root Codex and parallel Codex model_review freeze:

- Two successful passive applications, both Running after checked start setup;
  one shared subscribed Command topic, capacities [1,2].
- Five generated operations: Publish with its position as a one-byte payload,
  Stop(0), Stop(1), Restart(0), Restart(1).
- Every sequence length zero through six, independently initialized: 19,531
  traces and 112,305 generated operations. Assert both counts in the test.
- Supplemental final dispatch observations are additional operations, excluded
  from those counts. No generated dispatch or callback publication is included.

Adding two generated dispatch operations would produce 137,257 traces and
800,667 operations at length six and broaden the chosen lifecycle responsibility.
Reducing to length five would omit six-operation repeated clear/reconnect
witnesses. More topics duplicate the standalone routing domain. The reduced
five-operation model preserves the precise gap and a small execution cost.

## Independent oracle and observability

`tests/runtime_messaging_sequences.rs` models lifecycle as independent
Active/Paused values, with explicit routes [0,1]. Append-only accepted payload
histories and discard watermarks describe retained suffixes without using the
production queue, lifecycle transition helper or report-classification logic.
Successful stop advances only that inbox's watermark; restart changes only its
model state. Expected errors and reports are translated into public vocabulary
at the comparison boundary. Oracle decisions never consult runtime state.

Initially and after every generated operation, compare both states, capacities,
pending counts and start/stop/restart callback counters. Every publication
compares exact ordered identities/statuses and classification. Every lifecycle
request checks exact state/error and any returned discarded count. Position
payloads expose
retained-record replacement and reordering across repeated publications.

The owner exposes no public dequeue. At each trace end, public dispatch_one
presents each retained record to a passive recording callback; compare complete
topic/payload receipt prefixes and pending counts, then exact empty/rejected
dispatch and final receipt equality. Paused inboxes remain empty and their
dispatch attempt reports NotRunning with zero discarded. Each generated prefix
also occurs as an independently initialized trace, so these final observations
expose retained identity/FIFO at every prefix in the frozen domain.
After all observations, recheck both lifecycle states and callback counters.

Recording observers use test-only Rc/RefCell/Vec; these are finite test histories,
not operational storage or a whole-process bound. No callback publishes or
returns an error, and final drains do not expand generated sequence coverage.

## Review and selected sensitivity

Root Codex implements and reviews source form and complete diff. Parallel Codex
model_review independently reviews the frozen domain and implementation;
state_review checks durable scope, rendered documents and evidence. Selected
source fault trials use isolated copies under the ignored
target/review-2026-10-11-lifecycle directory. Production source is never faulted.
The independent oracle review accepts enumeration, history/state independence,
exact outcomes and every-prefix observation. Its optional final lifecycle/counter
recheck is included and passes focused/Clippy checks. Reviewed test blob is
`71f5782c215de08f0e8034cacdead2b262c21555`. No independent human review is claimed.

Six isolated source faults compile and fail behavioral assertions against that
final test copy: stop retains the selected inbox, stop also clears its peer,
stop overcounts discard, publication admits stopped endpoints, rejected restart
clears an inbox, and rejected restart invokes the callback. Small detected traces
are respectively [Publish, Stop(0)], [Publish, Stop(0)], [Stop(0)],
[Stop(0), Publish], [Publish, Restart(0)] and [Restart(0)]. Exact substitutions,
copies, logs and results are retained under the ignored mutations/final-copy
directory. This is selected sensitivity evidence, not comprehensive mutation
testing or a production behavior change.

## Executed local verification

The focused target passes one test, all frozen traces and asserted counts.
Initial/final full locked baselines pass 142/143 tests, formatting, all-target
check, warnings-denied Clippy/rustdoc and whitespace. No new exception is needed.
Source audit covers 35 Rust files and 13,353 physical lines with zero width
findings and three unchanged fulfilled expectations. All 331 relative links
across 64 Markdown files and 107 exact traceability function references resolve.
Root and parallel Codex sensitivity/source and durable-state reviewers accept
the complete checkpoint diff. Six changed documents pass rendered HTML heading,
code, link, table, complete normalized text and nesting inspection. One
intermediate relative-link count is corrected after the plan adds its review
link; final render/audit is regenerated. This is structure/content evidence,
not pixel-level or human acceptance.

```text
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
cargo test --locked --test runtime_messaging_sequences
python target/review-2026-10-11-lifecycle/mutations/mutation_trials.py
python target/review-2026-10-11-lifecycle/inventory_audit.py
& ./target/review-2026-10-11-lifecycle/render.ps1
python target/review-2026-10-11-lifecycle/inspect_rendered.py
git diff --check
git show --format= --check --diff-merges=first-parent HEAD
```

Rustdoc uses RUSTDOCFLAGS=-D warnings. Local tools remain Rust/Cargo 1.98.0,
rustfmt 1.9.0-stable and Clippy 0.1.98. Source widths, reasoned expectations,
links, exact traceability references and changed rendered HTML structure/content
are reviewed separately from Cargo. No pixel-level acceptance is claimed.
Unchanged sample/adapters, ADR-0018/0019 experiments and workflow do not trigger
extra local sample, probe or actionlint commands.

## Publication and hosted evidence

Checkpoint `c3eaa10cb013d1d4f5dbd11f6f0c4a3d4d98d37b` was published by ordinary
fast-forward after source/document/diff acceptance. At checkpoint publication,
local/remote heads matched and the tree was clean. Root Codex inspects run/job metadata
and logs from
[push run 38054603275](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/38054603275).
The exact logged checkout, one Windows job 114220376706 and all 20 reported
steps pass. Logs confirm 143 workspace tests including the new model, separate
14 adapters/five sample tests, warnings-denied rustdoc, the executed sample and
both committed-tip/working-tree whitespace checks.
Parallel Codex sensitivity independently verifies the same saved metadata/logs
and accepts the exact-checkpoint evidence without a finding.

Runner 2.337.0, windows-2025-vs2026 image 20260925.250.1, Rust/Cargo 1.99.0,
rustfmt 1.10.0-stable and Clippy 0.1.99 match the prior reviewed hosted baseline.
Local tools remain 1.98.0/1.9.0-stable/0.1.98; no local 1.99.0 execution is claimed.
No workflow, toolchain, lint configuration or source-policy change is needed.

This documentation follow-up records the exact checkpoint's completed evidence.
Its later revision still needs its own CI. Regenerated final documentation
inspection remains separate from hosted execution and independent human review.
All seven run-changed documents pass HTML structure/content inspection, with
332 relative links across 64 Markdown files and 107 exact traceability functions
resolving. Source width and waiver evidence is unchanged.

## Limits and next candidate

The test is exhaustive only over the frozen successful-callback domain and its
step-correlated valid payloads. It excludes generated start/dispatch/work,
Registered/Failed states, returned callback errors, callback publication,
arbitrary topologies or capacities, foreign identity, allocation failure,
concurrency, performance, real-time behavior and whole-process bounds.
Existing scenario tests retain their separate evidence for excluded behavior.

The next candidate is a separate reduced returned-work-failure model, including
terminal unavailability and peer preservation. Freeze it independently and
reassess priority; stop this run after the one lifecycle sequence checkpoint.
