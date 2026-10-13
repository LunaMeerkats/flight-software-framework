# Messaging-owned one-shot schedule sequence checkpoint

Date: **2026-10-14**
Status: **Local checkpoint verified; publication and hosted evidence pending**

## Objective and authority

Strengthen existing RFF-REQ-003/004 and the peer-operability aspect of
RFF-REQ-008 with an independent finite model of messaging-owned one-shot
schedule consumption and returned ordinary-work failure. Existing scenarios
cover selected timing, rejection and cleanup traces separately; prior generated
models exclude scheduling. ADR-0015/0021 supply the contract. No production or
requirement change is planned.

Input `d45d447786f212a17a353771400a844f6d7754c6` is clean on codex/nightly,
equal to refreshed origin. Fresh input hosted run 38254355148 passes its exact
checkout and all 20 steps. Initial locked local format/check/Clippy/test/rustdoc
and whitespace checks pass 145 workspace tests. Local Rust/Cargo 1.98.0,
rustfmt 1.9.0-stable and Clippy 0.1.98 are unchanged. Root Codex and parallel
gap_review/baseline_review reorient and audit; ADR-0024 authorizes routine review.
The existing accepted host v0.1 milestone remains separate evidence.

## Frozen finite domain

- Two initially Running applications, capacities [1,2], one Command topic,
  both fixed fault positions. All lifecycle and message callbacks succeed.
- Fixed physical agenda: application 0 at 1s, application 1 at 1s, application
  0 at 2s, application 1 at 2s. The first eligible fault work returns a concrete
  error; healthy ordinary work only records its application position.
- Three generated operations: Publish with its unique operation-position byte,
  Advance by one second, Attempt the next scheduled item. Independently
  initialize lengths zero through eight for each fault position: 19,682 traces
  and 147,624 generated operations. Assert both aggregate counts.
- Independent numeric elapsed time, consumed-item history, Active/Terminal
  states, callback order, accepted payload histories and discard watermarks
  predict complete/wait/due outcomes, exact errors/source chains, clock reads,
  final consumption, terminal availability, cleanup and preserved peer queues.
- Compare initial and every-operation state. Supplemental final healthy work
  and passive peer FIFO drains expose retained progress without introducing a
  failure into an unfailed prefix. They are excluded from generated counts.

Length eight reaches saturated publication, waiting/equality/overdue decisions,
four item attempts, later terminal rejection and completion. Fixed physical
order covers healthy equal-time work both before and after failure across roles.
Role-relative fault-first agendas miss the former; generating stop/restart,
events or configuration would add separate responsibilities. Direct-owner
schedule scenarios continue to protect the shared private timing decision.

## Components and acceptance

Add tests/messaging_schedule_sequences.rs and a focused verification review.
Update AGENTS layout, traceability, roadmap, concise state and CI evidence.
Root Codex owns records; parallel gap_review implements the test. Separate
Codex review checks oracle independence, selected isolated source faults,
source form and the complete diff. Existing repository contracts suffice;
no external research, source reuse, dependency, API, ADR or lint waiver is needed.

Acceptance requires exact frozen counts/outcomes, focused and full locked
checks, source/diff review, relative-link/traceability resolution, whole-tree
width/waiver audit and changed rendered-document structure/content inspection.
Selected isolated faults must compile and fail behavioral assertions without
editing tracked production. Publication requires reviewed local verification;
hosted CI must be inspected at each published revision.

```text
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
cargo test --locked --test messaging_schedule_sequences
git diff --check
git show --format= --check --diff-merges=first-parent HEAD
```

Rustdoc uses RUSTDOCFLAGS=-D warnings. Unchanged host sample/adapters,
ADR-0018/0019 experiments and workflow do not trigger extra local commands.
Ignored review aids remain under target/review-2026-10-14-schedule.

## Limits and safe stopping point

The domain excludes Registered/Stopped/unknown identities, generated dispatch,
other agendas/timing steps/topics/capacities/callback policies, scheduled events,
configuration, panics/hangs/allocator failures, concurrency, performance and
whole-process bounds. Consumption-before-callback remains source inspection
evidence; tests observe completed outcomes. Failed remains terminal and no
retry/recovery follows. Existing event evidence remains separate.

If a mismatch appears, retain its smallest trace and resolve it within this
objective. Preserve unrelated changes. Stop after this one reviewed checkpoint.

## Completed local evidence

The focused model and focused warnings-denied Clippy pass. Initial/final full
locked baselines pass 145/146 workspace tests, formatting, all-target check,
Clippy, warnings-denied rustdoc and whitespace. Root Codex and parallel
gap_review/baseline_review/sensitivity accept the oracle and source form with
no blocking finding. Final test blob is
`fa4c467828c56f929fd5aeddb0ca522ae16be3d8`. There is no new lint exception.
Whole-tree review covers 38 Rust files/15,084 lines and 1,098 comment-only lines,
zero width findings and the same three fulfilled expectations.

Seven selected isolated faults compile then fail behavioral checks. Exact trials,
source hashes, diagnostics and limits are recorded in the focused review and
ignored aids. Root and parallel baseline_review accept the complete diff and
durable records. All 348 relative links across 67 Markdown files and 110 exact
traceability functions resolve. Six changed documents pass rendered HTML
structure and complete content inspection, with no pixel/human claim.
Publication and exact-revision hosted evidence remain separate steps.
