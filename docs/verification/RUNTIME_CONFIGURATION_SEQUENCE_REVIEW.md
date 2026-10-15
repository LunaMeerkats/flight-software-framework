# Runtime configuration/work sequence review

Date: **2026-10-16**
Reviewer: **Codex**, with parallel gap, baseline and sensitivity review
Status: **Complete: checkpoint published and exact-revision hosted CI passed**

## Gap and decision

RFF-REQ-006 and ADR-0005/0017/0018 define configuration replacement, rejection,
consume-once rollback, revision non-reuse, immutable ordinary-work visibility,
lifecycle retention and no automatic rollback after application errors. The
[standalone model](CONFIGURATION_SEQUENCE_REVIEW.md) composes table operations,
but never invokes an application. Existing configuration runtime scenarios cover
individual rules and selected histories. In particular,
`lifecycle_rejection_suppresses_work_and_restart_retains_configuration_history`
never replaces or rolls back configuration while the application is stopped.

Select generated direct-runtime histories to compose that stopped-change path
with fresh callback visibility, retained application state, failure and peer
observations. A six-operation configuration/work-only alphabet would omit the
identified lifecycle path. Extending the standalone test would mix table and
runtime responsibilities. A separate integration target keeps their evidence
distinct. No production defect or new behavior is presumed.

## Frozen domain

- One Runtime with two initially Running applications, one constructor-owned
  bound-two configuration table and initial [1] at revision one.
- Accepted candidates [1] and [2,3], rejected candidate [0], and Rollback.
  The pure fixture validator accepts nonempty content with a nonzero first
  byte. The short accepted candidate can equal current content.
- WorkSelected, WorkPeer, StopSelected and RestartSelected complete the eight
  operations. The selected application fails on its second invoked work;
  the peer always succeeds. Start/stop/restart callbacks succeed.
- Every sequence of length zero through five for both selected indices gives
  74,898 traces and 363,792 generated operations, checked by aggregate counters.
- Separate final WorkPeer/Rollback/WorkPeer/ReplaceShort/WorkPeer/Rollback/
  WorkPeer probes give 524,286 supplemental operations. They expose terminal
  prefix content, rollback history and the next unused revision without
  invoking the selected application or adding to generated trace lengths.

Length five includes WorkSelected/StopSelected/ReplaceExact/RestartSelected/
WorkSelected. The final callback observes the replacement, retains the first
call across restart and returns the second-call error. Other generated orders
combine rejection and rollback with terminal suppression and peer progress.

## Independent reference and observations

The test uses an append-only acceptance/predecessor ledger and movement history
to derive active identity and rollback eligibility. Acceptance count assigns
revisions. It does not reproduce the production active/rollback/high-water
storage triple or invoke production transition logic. Independent lifecycle and
callback histories predict each request's exact result and callback effects.

The fixture copies the callback's optional view into an owned revision/byte
record, including when work returns its concrete error. Compare complete callback
logs and both lifecycle states initially and after every operation. Stop/restart
callback counts distinguish rejection from invocation. Supplemental healthy-peer
work makes configuration state visible without creating a new selected failure.
Inspect the runtime/table/mission error-source relationships separately.

## Acceptance and commands

Run focused model and warnings-denied Clippy, then the full locked baseline:

```text
cargo test --locked --test runtime_configuration_sequences
cargo clippy --locked --test runtime_configuration_sequences -- -D warnings
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
git diff --check
git show --format= --check --diff-merges=first-parent HEAD
```

Rustdoc uses RUSTDOCFLAGS=-D warnings. Review all handwritten Rust widths and
reasoned expectations, relative links, exact traceability functions, changed
rendered documents and the complete source/diff. Selected isolated faults must
compile before behavioral rejection counts as sensitivity evidence. Retain aids,
hashes and raw logs under target/review-2026-10-16-runtime-config. These are
temporary review aids, not a newly adopted source or mutation-testing gate.

Unchanged ADR experiments, host adapters/sample and workflow do not trigger their
extra local commands. Hosted CI still runs its configured focused host targets
and sample. Local review and local checks remain separate from hosted execution.

## Local evidence and sensitivity

The focused model and focused warnings-denied Clippy pass. Initial/final full
locked baselines pass 147/148 workspace tests, format, all-target check, Clippy,
warnings-denied rustdoc and whitespace. The initial focused Clippy run found
one collapsible fixture condition; it was corrected before final checks and
all sensitivity trials. No suppression or waiver was added.

Root Codex and parallel baseline_review independently accept the full oracle
and source at blob `ca64c5a09f1ab93e046e612fd869937b8b3ecff6`.
Whole-tree audit covers 40 Rust files/15,912 physical lines and 1,113 comment-only
lines, zero width findings and three unchanged fulfilled expectations.
Relative links and 112 exact traceability function references resolve.
All 357 relative links across 69 Markdown files resolve. Root and parallel
baseline_review accept the complete source/document/diff; all six changed
rendered documents preserve full content and HTML structure. This is not pixel
inspection or human acceptance. Publication and exact-revision CI are below.

The isolated unmodified control passes this same final test. Six selected
production faults compile and fail callback-log equality assertions:

| Isolated fault | First generated witness and failing observation |
| --- | --- |
| Stale callback revision | Empty trace; supplemental step 5 |
| Truncated callback bytes | ReplaceExact; supplemental step 2 |
| Automatic rollback after work failure | WorkSelected/ReplaceShort/WorkSelected; supplemental step 4 |
| Automatic rollback after restart | StopSelected/ReplaceShort/RestartSelected; supplemental step 4 |
| Invoke rejected work before returning its correct error | StopSelected/WorkSelected; generated step 2 |
| Rejected replacement consumes configuration history | ReplaceShort/Reject; supplemental step 3 |

The final fault changes active content as well as consuming history; this trial
does not isolate slot loss from active mutation. All first witnesses occur with
selected index zero; the control exercises both indices. Exact source patches,
commands, runtime/test/log hashes and results are retained in
target/review-2026-10-16-runtime-config/mutations/manifest.json.
Root separately inspects the raw logs, source patches and matching hashes.

Each isolated trial runs the following with its manifest's absolute copy path:

```text
cargo test --locked --manifest-path <copy>/Cargo.toml --test runtime_configuration_sequences -- --nocapture
```

Control exits zero; all six faults exit 101 after successful compilation and
behavioral assertion failure. These selected faults demonstrate sensitivity,
not exhaustive mutation coverage. They do not alter tracked production.

## Limits and disposition

This finite model excludes arbitrary content, validators, bounds, initial
rejection, exhaustion, unknown/Registered applications, other callback failures,
messaging, scheduling, events, allocation failures, panics/hangs and concurrency.
Existing tests and borrowing probes retain their separate evidence. The pure
fixture does not establish arbitrary mission validator purity or termination.
Failed remains terminal; no new recovery, operational, qualification or human
review claim follows. Production, public APIs, requirements, dependencies,
licence, lint policy and workflow are unchanged. Existing local contracts supply
the design; no external research or source reuse is introduced.

## Publication evidence

Checkpoint `283b07c280746dd6ac7c26b60bdaf0ab14e5bad3` is published by ordinary
fast-forward on codex/nightly. Local and remote heads match with a clean tree
at publication. [Push run 38739984030](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/38739984030)
passes that exact logged checkout, Windows job 116438836697 and all 20 steps.
Root inspects raw JSON/logs: 148 workspace tests including the new model,
separate 14 adapters/five sample tests, warnings-denied rustdoc, the complete
unchanged 222-line sample report and both whitespace checks. Parallel
baseline_review independently reviews this hosted evidence.

Hosted runner 2.337.0, windows-2025-vs2026 image 20260925.250.1,
Rust/Cargo 1.99.0, rustfmt 1.10.0-stable and Clippy 0.1.99 match the input.
Local execution remains on 1.98.0. This later documentation revision preserves
source, receives full review and requires its own exact-head CI result. The
checkpoint pass does not establish a later revision's result or human review.
