# Finite messaging-owned schedule sequence review

Date: **2026-10-14**
Status: **Complete: checkpoint published and exact-revision hosted CI passed**

## Contract and reviewed input

Input `d45d447786f212a17a353771400a844f6d7754c6` is clean on codex/nightly,
equal to refreshed origin. Fresh push run 38254355148/job 114828459496 succeeds
at that exact logged checkout: all 20 steps, 145 workspace tests, separate
14 adapter/five sample tests, warnings-denied rustdoc, full sample and whitespace.
Initial locked local baseline passes 145 tests on Rust/Cargo 1.98.0,
rustfmt 1.9.0-stable and Clippy 0.1.98. Hosted tools remain separate evidence.

[ADR-0015](../adr/0015-caller-driven-scheduled-work.md) and
[ADR-0021](../adr/0021-messaging-owned-scheduled-work.md) permit at most one
due-item attempt per caller request. A complete agenda reads no clock. A pending
decision reads once; a future item waits without mutation. Equality and overdue
time are due.
Each due item is consumed before the owning ordinary-work operation, including
returned failure or lifecycle rejection. A returned application-work error commits
terminal Failed, clears exactly the selected inbox and preserves peer queues.
Scheduled work emits no event and does not dispatch messages.

This checkpoint supplements RFF-REQ-003/004 and peer operability under
RFF-REQ-008. Existing event evidence and the [accepted host review](HOST_V0_1_REVIEW.md)
remain separate. Production, APIs, requirements, dependencies, licence, lint
policy and workflow are unchanged. Repository contracts suffice; no external
research, source reuse or upstream behavior claim is needed.

## Frozen domain and alternatives

Root Codex and parallel gap_review freeze the following before implementation:

- Two initially Running applications, capacities [1,2], one Command topic,
  both fixed fault positions. Lifecycle and message callbacks succeed.
- Fixed agenda [(0,1s),(1,1s),(0,2s),(1,2s)] in physical registration order.
  The fault application returns its concrete error at first eligible ordinary
  work; the healthy peer records work and succeeds.
- Three generated operations: Publish with an operation-position byte, Advance
  by one second, Attempt. Independently initialize all lengths zero through
  eight for both roles: 19,682 traces and 147,624 generated operations.
- Supplemental final healthy ordinary work and passive peer FIFO drain observe
  retained progress without introducing a fault into an unfailed prefix.
  These probes are excluded from generated counts.

The domain reaches waiting, inclusive/overdue attempts, equal-time order,
saturation, returned failure, later terminal rejection, later peer items and
completion. Physical order covers a healthy item both before and after failure
across roles; a role-relative fault-first agenda misses the former. Lifecycle
operations, configuration and events would introduce other responsibilities.
Existing direct-owner scenarios continue to protect the shared timing policy.

## Oracle and observability

Independent numeric elapsed seconds, consumed-item history, Active/Terminal
states, callback order, append-only accepted payload histories and discard
watermarks predict decisions. Oracle decisions do not query runtime state or
reuse production clock, queue, transition, schedule or classification helpers.
Only the public comparison boundary translates predictions into framework
types. The actual counting clock wraps ManualClock; independent time predicts
its advancement and counts only injected schedule reads.

Compare initial and every-operation lifecycle states, capacities, pending
counts, callback order, schedule remaining/completion, actual elapsed time and
clock reads. Compare exact complete/wait/completed outcomes, consumed item and
observed instant, nested concrete errors and source chains, selected discards
and zero-discard terminal rejection. Final healthy work and topic/payload FIFO
observations cover every independently enumerated prefix.

Consumption-before-callback is source inspection evidence: the shared private
decision advances its cursor before invoking the single-use owner closure, and
the messaging closure delegates to its authoritative ordinary-work method.
Tests observe post-return effects. No panic-containment claim follows. Test
Rc/RefCell/Vec histories are finite fixtures outside operational queue bounds;
logical slots do not establish process-byte or callback-duration bounds.

## Verification and review

Root Codex and parallel gap_review/baseline_review/sensitivity accept the final
oracle and source form with no blocking finding. The reviewed test blob is
`fa4c467828c56f929fd5aeddb0ca522ae16be3d8`; no new lint exception is added.
Initial/final full locked local baselines pass 145/146 workspace tests; the
focused model and focused warnings-denied Clippy pass. Completed commands:

```text
cargo fmt --all
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
cargo test --locked --test messaging_schedule_sequences
cargo clippy --locked --test messaging_schedule_sequences -- -D warnings
git diff --check
git show --format= --check --diff-merges=first-parent HEAD
```

Rustdoc uses RUSTDOCFLAGS=-D warnings. Local tools remain Rust/Cargo 1.98.0,
rustfmt 1.9.0-stable and Clippy 0.1.98. Whole-tree review covers 38 Rust files,
15,084 physical lines and 1,098 comment-only lines, zero width findings and three
unchanged fulfilled expectations. Conditional host sample/adapter commands,
ADR-0018/0019 probes and actionlint are unchanged and not triggered locally.
Hosted CI still executes its configured focused host/sample checks.

### Selected fault sensitivity

Independent Codex sensitivity copies the final test and production inputs into
isolated packages under target/review-2026-10-14-schedule/mutations, never
editing tracked production. Each of seven selected executions compiles with
`cargo test --locked --test messaging_schedule_sequences --no-run` (exit zero)
then fails the same command without --no-run (exit 101) on a behavioral check:

| Source fault | Observed rejection |
| --- | --- |
| Equality waits instead of executing | Expected error receives Waiting at exactly 1s |
| Waiting consumes an item | Attempt: remaining 3 instead of 4 |
| Complete reads the clock | Four attempts plus complete: reads 5 instead of 4 |
| Returned error retains its item | Advance/Attempt: remaining 4 instead of 3 |
| Schedule bypasses messaging owner | Advance/Publish/Attempt: discards 0 instead of 1 |
| Discard count increased by one | Advance/Attempt: discards 1 instead of 0 |
| Bypass reports queued count without clearing | Advance/Publish/Attempt: selected pending 1 instead of 0 |

The final companion separates actual selected-inbox retention from incorrect
discard reporting. All first witnesses occur with fault role zero; passing
reference traversal covers both roles. The equality diagnostic omits its trace;
the deterministic enumeration identifies Advance/Attempt, rather than a logged
trace claim. These selected detections do not prove general mutation adequacy.
Exact substitutions, source/test/manifest hashes, compile/test exit codes and
raw diagnostics remain in final-copy/mutation_results.json and
owner-boundary-copy/mutation_results.json beneath the ignored mutation path.
Root Codex independently inspects all seven raw failure diagnostics and results.

Root and parallel baseline_review accept the complete diff and durable records.
All 348 relative links across 67 Markdown files and 110 exact traceability
functions resolve. Six changed documents pass rendered HTML structure and
complete content inspection: headings, code, links, tables, text and balanced
nesting. This is document evidence, not pixel or human acceptance. Publication
and exact-revision hosted execution are separate completed evidence below.

## Publication and exact-revision hosted evidence

Checkpoint `0ca95560041f249911eb9d107cedd06f731ea159` contains the reviewed
test blob and local evidence. Ordinary fast-forward publication succeeds;
remote/head match and the tree is clean at checkpoint publication.
[Push run 38403821340](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/38403821340)
passes that exact logged checkout, Windows job 115318053007 and all 20 steps.
Raw logs confirm 146 workspace tests including the new model, separate
14 adapter/five sample tests, warnings-denied rustdoc, the executed sample's
unchanged full report and both whitespace checks. Root Codex and parallel
baseline_review inspect raw JSON/logs independently from local verification.

Runner 2.337.0, windows-2025-vs2026 image 20260925.250.1, Rust/Cargo 1.99.0,
rustfmt 1.10.0-stable and Clippy 0.1.99 match the reviewed input. Whole-tree
source/configuration review finds no policy or waiver drift. Local execution
remains on 1.98.0; it does not claim local 1.99.0 execution.

This documentation follow-up records completed checkpoint evidence and preserves
the source blob. Follow-up audit resolves 349 relative links/110 exact functions;
all seven run-changed rendered documents pass structure/content inspection.
Review covers the complete diff. Its own published revision requires a completed
exact-head hosted result; earlier CI does not cover a later documentation head.

## Limits and next work

This finite domain excludes Registered/Stopped/unknown identities, generated
dispatch, other agendas/timing steps/topics/capacities/callback policies,
configuration, scheduled events, panics/hangs/allocator failure, concurrency,
performance and whole-process bounds. Failed remains terminal; no retry or
recovery is introduced. Acceptance is Codex review, not human review, release
authorization, operational suitability, qualification or compatibility.

After this checkpoint, assess a reduced configuration replacement/rejection/
rollback sequence model against existing scenarios before selecting its domain.
