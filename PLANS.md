# Runtime ordinary-work failure sequence checkpoint

Date: **2026-10-12**
Status: **Local implementation verified; publication pending**

## Objective and authority

Strengthen RFF-REQ-002/003 and the peer-operability aspect of RFF-REQ-008 with
one separate reduced model of returned ordinary-work failure. The previous
successful-callback model excludes Failed and work. Input
`8705408ee9810470b19c7eef796557b5d2b7ee38` starts clean on codex/nightly, equal
to refreshed origin; exact input hosted run 38054936905 is freshly successful.
The initial locked local baseline passes 143 workspace tests on Rust/Cargo
1.98.0, rustfmt 1.9.0-stable and Clippy 0.1.98. Existing policies are encoded;
this run needs no policy adoption, production change or external research.

Repository ADR-0003/0009/0011 supply the oracle contract. Work succeeds only
while Running; a returned concrete error commits terminal Failed, clears exactly
that inbox and preserves peers. Rejected operations invoke no callback and
clear nothing. Routine autonomous review follows ADR-0024.

## Frozen finite domain

- Two initially Running applications, one shared topic, capacities [1,2]. Run
  the same domain twice, swapping which application always returns a work error.
  Start, stop, restart and message callbacks succeed; healthy work succeeds.
- Five generated operations: Publish with the operation-position byte,
  Work(fault), Work(peer), Stop(fault), Restart(fault). Enumerate lengths zero
  through six: 39,062 traces and 224,610 generated operations across both roles.
- Independent Active/Paused/Terminal states, append-only accepted payload
  histories and discard watermarks predict ordered publication reports,
  classifications, typed errors, exact clearing, and lifecycle/work counters.
  Compare both states, capacities and pending counts after every operation.
- Supplemental final passive dispatch observes complete retained topic/payload
  FIFO suffixes and empty/rejected callback suppression. A final healthy work
  attempt directly observes peer progress at every prefix. These operations are
  excluded from generated counts; every prefix is separately enumerated.

Both fault roles avoid a first-slot or capacity-one blind spot. Generating peer
stop/restart would repeat the accepted lifecycle model and expand the alphabet;
events, schedules and configuration would broaden the selected responsibility.
Shorter traces omit six-step stop/restart/failure/publication witnesses.

## Components and acceptance

Add tests/runtime_work_failure_sequences.rs and a focused verification review.
Update AGENTS layout, this plan, traceability, roadmap and concise project state.
Root Codex owns durable records; parallel Codex scope_review implements the test,
sensitivity reviews selected isolated faults, and an independent reviewer checks
oracle/source/diff/document evidence. No new API, dependency, lint exception,
requirement or architecture decision is planned.

Acceptance requires asserted frozen counts and exact outcomes, focused and full
locked checks, whole-tree source width/waiver inspection, complete-diff review,
resolving relative links and exact traceability references, and changed rendered
document structure/content inspection. Selected isolated source fault trials
must compile and fail behavioral assertions without modifying production.

```text
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
cargo test --locked --test runtime_work_failure_sequences
git diff --check
git show --format= --check --diff-merges=first-parent HEAD
```

Rustdoc uses RUSTDOCFLAGS=-D warnings. Unchanged host adapters/sample,
ADR-0018/0019 experiments and workflow do not trigger extra local commands.
Ignored review aids stay under target/review-2026-10-12-failure. Commit and push
only after applicable checks and actual Codex review pass; inspect exact-head
hosted CI separately and record completed publication evidence.

## Limits and safe stopping point

The model excludes generated start/dispatch, Registered, start/stop/restart/
message errors, callback publication, event/schedule/configuration composition,
arbitrary topologies/capacities, allocator failures, concurrency, timing and
whole-process bounds. Fixed always-failing work is one cooperative fault policy.
It does not establish the event aspect of RFF-REQ-008 or recovery from Failed.
If a mismatch appears, retain the smallest trace and resolve it within this
objective. Preserve unrelated changes; revert only current-run implementation
if the baseline cannot be restored. Stop after this one coherent checkpoint.

## Completed local verification

The focused model passes one test over all asserted frozen traces/operations.
Initial/final full locked baselines pass 143/144 workspace tests, formatting,
all-target check, warnings-denied Clippy/rustdoc and whitespace. A planning
arithmetic typo (225,610) was caught by the final aggregate assertion and corrected
to 224,610 before acceptance; the domain and behavioral assertions are unchanged.
Root review caught one 81-column comment, now 72 columns. No new lint exception
or production change is added. Selected isolated faults and final document/diff
reviews are recorded in the [review](docs/verification/RUNTIME_WORK_FAILURE_SEQUENCE_REVIEW.md).
The final test Git blob is `caf375400917899de01ff7615a411b4555ca1b11`.
Publication and exact-head hosted execution remain distinct evidence.

Six final-copy isolated faults compile and fail behavioral assertions; exact
substitutions, small traces and limits are recorded in the review. Root and
parallel Codex source/oracle review find no blocking issue. The whole-tree audit
covers 36 Rust files/13,935 physical lines, zero widths and three unchanged
fulfilled expectations. All 336 relative links across 65 Markdown files and
108 exact traceability functions resolve. Changed rendered HTML structure and


diff review are completed before the commit/publication boundary.
