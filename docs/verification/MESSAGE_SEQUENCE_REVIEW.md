# Finite message publication/dequeue review

Date: **2026-10-10**
Status: **Local checkpoint reviewed; publication and hosted evidence pending**

## Question, inputs and decision

Does composition of the standalone ADR-0004/0010 operations preserve routing,
reject-newest pressure, reports and complete cross-topic FIFO? Twelve existing
message-bus regressions exercise selected scenarios. The completed
[host review](HOST_V0_1_REVIEW.md) proposes finite sequence hardening as the
next bounded increment. RFF-REQ-003 and the recorded decisions define expected
behavior; no external research or new runtime decision is needed.

Input `81ca906e97c28db53ca601d34c1b2d1b2e7926fb` is clean on codex/nightly,
equal to refreshed origin. Fresh initial locked baseline passes 141 tests.
[Input hosted run 37782992360](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/37782992360)
is freshly confirmed successful at that exact head; it does not cover this test.

Codex root freezes the contract in [PLANS.md](../../PLANS.md) before adding
`tests/message_bus_sequences.rs`. A separate integration target keeps the
sequence oracle coherent alongside the existing scenario-test target. It adds
one test, no production code, API, dependency, lint exception or workflow change.

## Frozen finite domain and assertions

| Parameter | Explored values |
| --- | --- |
| Inbox 0 | Command only, capacity 1 |
| Inbox 1 | Telemetry only, capacity 2 |
| Inbox 2 | Command and Telemetry, shared capacity 2 |
| Publication operations | Command, Telemetry, Unrouted |
| Consumption operations | Dequeue inbox 0, 1 or 2 |
| Payload | One valid byte equal to the operation position |
| Lengths | Every sequence from zero through six operations |
| Count | 55,987 fresh traces, 324,726 generated operations |

Final observation drains are additional and excluded from the generated count.
Base-six decoding is a bijection for every fixed length; the test asserts both
counts. Increasing lengths mean any first mismatch has the shortest explored
generated length, although final drains add consumption beyond that length.

The oracle uses explicit routes Command=[0,2], Telemetry=[1,2], Unrouted=[].
It appends semantic records to accepted histories and advances a separate
consumption cursor. It does not use production `VecDeque`, endpoint lookup,
message storage or report classification. A comparison adapter translates
independent record/outcome values only at the public-API assertion boundary.

Every publication checks exact destination identities, order and statuses,
plus classification. Every generated operation checks all logical capacities
and pending counts. Every dequeue compares full topic and payload. Each trace
then consumes and compares every outstanding suffix, checks an additional empty
dequeue, and ends with zero pending in every inbox. Position-valued payloads
distinguish repeated publications and expose reorder/replacement faults.

## Independent review and sensitivity

Root Codex implements and reviews the complete diff. Parallel Codex
`model_design_review` independently freezes the finite contract and accepts
the enumeration/oracle/invariants. Codex `source_review` independently checks
the implementation against requirements and ADRs; one 82-column opening
comment is wrapped, and diagnostic context is improved. Codex
`durable_state_review` confirms recorded host acceptance/policies and the
default-branch revisit. Final source review covers test blob
`7ea211edafd8884556a3e3e536ef46fe628f7e1a`. No independent human review is claimed.

Six temporary isolated source copies compile, then fail this new test with
behavioral assertion traces: replace the oldest while reporting full, LIFO
dequeue, stop fan-out at a full inbox, reverse destination reports, route to
nonmatching inboxes, and misclassify wholly-undelivered as partial. These are
selected sensitivity trials, not comprehensive mutation testing. The script,
copied sources and logs remain ignored under target/review-2026-10-10-model;
no fault was introduced into production source.

The default branch is now main. The dated [ADR-0023 revisit](../adr/0023-host-ci-baseline.md)
verifies its exact unchanged workflow and retains existing nightly push/PR
triggers. No setting, workflow or manual-dispatch execution is changed.

## Executed verification

```text
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
cargo test --locked --test message_bus_sequences
python target/review-2026-10-10-model/mutation_trials.py
git diff --check
```

Initial full baseline passes 141 tests; final full baseline passes 142, with
formatting, all-target check, warnings-denied Clippy/rustdoc and whitespace
passing. The focused target passes one test, all frozen traces and counts.
Rustdoc uses `RUSTDOCFLAGS=-D warnings`. Local versions remain Rust/Cargo
1.98.0, rustfmt 1.9.0-stable, Clippy 0.1.98. Source audit covers 34 Rust files
and 12,913 physical lines: zero physical/comment width findings and the same
three fulfilled function-size expectations. Relative Markdown links and 106
exact traceability function references resolve. Seven changed documents pass
rendered HTML heading/code/link/table/text and nesting inspection. Root and
parallel Codex source, complete-diff, evidence and rendered-content reviews
accept the checkpoint after the comment correction. No pixel-level acceptance
is claimed. Publication evidence remains pending. Unchanged sample/adapters,
ADR-0018/0019 experiments and workflow do not trigger extra local
sample/probe/actionlint commands.

## Limits and next candidate

This is exhaustive only for the declared topology, capacities, alphabet,
step-correlated valid payloads and length bound. It does not prove larger
domains, all Rust values, hostile payloads, lifecycle availability/clearing,
dispatch, allocation failures, concurrency, performance, real-time behavior
or whole-process bounds. Existing host acceptance and APIs remain provisional.
The next candidate is a separate small model of runtime-owned lifecycle
availability and exact inbox clearing. Freeze its scope before implementation;
do not infer those behaviors from this standalone test.
