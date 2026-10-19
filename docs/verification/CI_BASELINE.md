# Host CI verification baseline

Date: **2026-09-12**
Status: **Hosted baseline passed at the exact revision below**

## Execution contract

[ADR-0023](../adr/0023-host-ci-baseline.md) selects one
[workflow](../../.github/workflows/ci.yml) for the existing Windows host
baseline. The job uses stable Rust, explicit rustfmt/Clippy installation,
warnings-denied rustdoc, and one native command per PowerShell step.

The initial unlocked AGENTS.md baseline passed before changes. These workflow
command bodies also passed locally with `RUSTUP_TOOLCHAIN=stable` and
`RUSTDOCFLAGS=-D warnings`:

```text
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo test --locked --test host_adapters
cargo test --locked --test host_sample
cargo doc --locked --workspace --all-features --no-deps
cargo run --locked --example host-echo
git show --format= --check --diff-merges=first-parent HEAD
git diff --check
```

The suite has 115 passing tests, including 14 adapter and five sample tests;
the focused reruns do not increase that total. The executable prints the three
documented command/telemetry records and its structured scenario report.
`--locked` checks the tracked dependency resolution without selecting an MSRV
or freezing the compiler. Cargo.toml, Cargo.lock, and all Rust source are
unchanged from `634941453be2503371d962f5d80ec865448a730a`.

`git rev-parse HEAD`, `rustc --version --verbose`, `cargo --version`,
`cargo fmt --version`, and `cargo clippy --version` passed. Local replay uses
the already-installed stable rustc/cargo 1.98.0, rustfmt 1.9.0-stable, and
Clippy 0.1.98. The hosted rustup installation command is syntax/help-reviewed,
not executed locally; this run does not upgrade the user's toolchain.

## Workflow validation

Use the upstream actionlint 1.7.12 standalone executable to check this workflow:

```text
actionlint -color .github/workflows/ci.yml
```

The temporary Windows amd64 release archive is verified against the upstream
SHA256 checksum
`6e7241b51e6817ea6a047693d8e6fed13b31819c9a0dd6c5a726e1592d22f6e9`.
Its source/version provenance is recorded in
[the source register](../research/SOURCES.md). This is a local workflow
maintenance check, not a new CI action, runtime dependency, Rust source-shape
gate, or proof that GitHub will execute a job. PowerShell parsing and bounded
failure/whitespace probes complement static workflow validation; exact results
are recorded in the
[completed CI plan at c472c57](https://github.com/LunaMeerkats/flight-software-framework/blob/c472c57c1aa241d821b891674ba25f42b25ae723/PLANS.md).
This immutable reference preserves the evidence as the active plan advances.

The committed whitespace command checks a root or tip commit, or a merge
against its first parent. A push containing multiple commits does not get
per-intermediate-commit whitespace coverage. The separate working-tree check
does not inspect unchanged committed content. No branch protection or required
check policy is configured by adding this file.

## First hosted acceptance — 2026-09-12

The source and CI workflow were published by ordinary fast-forward push under
the user's standing permission. The first
[hosted run](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/34697541024)
completed successfully with one Windows job and all 20 steps successful.

- Event: `push`; workflow and logged checkout revision:
  `abb1293790136128d5f27d8c48c1e3d98a355540`.
- Workflow label: `windows-2025`. Actual setup log image:
  `windows-2025-vs2026`, image version `20260907.229.1`, runner `2.337.0`.
- rustc 1.98.1 (`48a229cea`, 2026-09-01), Cargo 1.98.1
  (`797e8a9bc`), rustfmt 1.9.0-stable, Clippy 0.1.98.
- Checkout, stable installation, version logging, format, check, Clippy,
  workspace tests (115), adapter tests (14), sample tests (five), rustdoc,
  executable sample, both whitespace checks, and runner cleanup all passed.
- The sample log has the exact documented 0, 42, and 100 command/telemetry
  records. The shared-driver tests establish the structured scenario outcomes.

The hosted toolchain is newer than the unchanged local 1.98.0 installation.
The all-target hosted fmt/Clippy/check baseline revalidates the selected
configuration. Companion whole-tree review covers all 32 Rust files, no physical
or comment-only width findings, and three unchanged fulfilled expectations.
No new source-form debt, suppression, dependency, or toolchain pin is introduced.
Conditional ADR-0018/0019 probes are unchanged and not separately rerun.

This is exact-revision CI evidence, not human code review or a v0.1 release.
The later documentation commit records the result without changing Rust,
Cargo files, lint configuration, or the workflow. Its CI result must still be
assessed separately rather than inherited from this run.

## Hosted toolchain re-audit — 2026-10-03

The returned-stop checkpoint at
`cd7318e0bfdcedb33610e887cfd794f90d6f16cf` passes
[push run 37018451102](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/37018451102).
The workflow is the unchanged tracked `.github/workflows/ci.yml` at that
revision, and the `Record checkout` log contains that exact SHA. Its single
Windows job and every configured step complete successfully: 135 workspace
tests including the new regression, 14 focused adapter tests, five focused
sample tests, warnings-denied documentation, the sample executable, and both
whitespace steps. Focused reruns do not increase the workspace total.

Runner 2.337.0 uses `windows-2025-vs2026` image `20260925.250.1`. Stable now
resolves to rustc 1.99.0 (`b940084d7`, 2026-09-28), cargo 1.99.0
(`5f94df478`, 2026-08-27), rustfmt 1.10.0-stable, and Clippy 0.1.99. Local
execution remains on Rust/Cargo 1.98.0, rustfmt 1.9.0-stable, and Clippy
0.1.98; no local 1.99.0 execution or toolchain installation is claimed.

AGENTS.md requires a configuration and whole-tree source re-audit when the
active toolchain changes. The re-audit retains the existing policy:

- `rustfmt.toml` still contains stable `max_width = 100`; the exact hosted
  `cargo fmt --all -- --check` accepts every existing target's formatting.
- `clippy.toml` still sets the local 60-line threshold. All targets inherit
  the workspace `too_many_lines = "deny"` policy; hosted Clippy with warnings
  denied passes, including all three unchanged item-level expectations.
- The separate manual review covers all 33 handwritten Rust files and their
  module organization, names, progressive reading order, and invariants.
  Physical/comment-width review finds zero violations and no block comments.
  No `allow` attribute, broad waiver, threshold change, or exclusion is added.
- Safe-Rust policy remains `unsafe_code = "forbid"`. No production, manifest,
  lockfile, lint configuration, or workflow change is needed for this re-audit.

The [stop-failure review](STOP_FAILURE_REVIEW.md) records behavior and limits.
This is exact-revision hosted and autonomous source-review evidence, not an
MSRV, toolchain pin, human v0.1 acceptance, or operational suitability claim.

## Evidence for subsequent hosted runs

The initial configuration checkpoint was local only. The user's subsequent
2026-09-12 approval authorizes verified source pushes without per-push human
review, as recorded in AGENTS.md. To close the hosted CI gate, record:

1. The run URL, event, workflow revision, and logged checkout SHA. For a PR,
   identify the tested merge SHA and its base/head; do not project that result
   onto another head or a later merge.
2. Runner image identity from the setup log, compiler/tool versions, and the
   conclusion of every required step, including the full test count and sample.
3. Any toolchain drift re-audit of the whole source tree required by AGENTS.md.
4. Failures, skipped/cancelled checks, fork approval holds, or absent jobs as
   incomplete evidence. A successful local command replay is not a hosted run.

The CI job automates Cargo checks, the sample, and whitespace only. Relative
Markdown links, rendered-document inspection, physical/comment-only Rust
widths, naming/cohesion, reasoned expectations, and complete-diff review remain
local review responsibilities. ADR-0018/0019 standalone probes remain required
when those decisions or probes change; Cargo does not discover them, and this
workflow does not claim to execute them. Recorded entry-point, dependency,
scope, and architecture reviews remain host v0.1 evidence gates. The user's
2026-10-08 [delegation](../adr/0024-autonomous-host-review-authority.md) permits
routine entry-point/architecture acceptance through documented autonomous
review; it does not establish human review or authorize releases/deployment.

## Autonomous host milestone review — 2026-10-09

The [completed host review](HOST_V0_1_REVIEW.md) uses input
`ba42f800d0b74637b28c833a55192fc541912fe5`. Fresh local locked checks pass
141 workspace tests, 14 focused adapters, five focused sample tests, the sample
executable, and warnings-denied documentation. Codex reviews source form,
resource/failure/API contracts, provenance, dependencies, and entry-point scope.
These local/manual reviews remain distinct from hosted execution.

[Input run 37675969512](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/37675969512)
is freshly confirmed successful at its exact logged checkout: one Windows job
and all 20 reported steps. Runner 2.337.0, image `windows-2025-vs2026`
`20260925.250.1`, Rust/Cargo 1.99.0, rustfmt 1.10.0-stable, and Clippy 0.1.99
match the prior policy re-audit. Local tools remain on 1.98.0. The actual review
closes routine host milestone gates; it does not create human-reviewed or
release-ready status. The review's documentation revisions need their own CI.

Review checkpoint `44f85edd4ae8fdcfb6fbcb34b59cea7478d496cb` is published.
[Push run 37782350928](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/37782350928)
passes its exact logged checkout, all 20 reported steps, 141 workspace tests,
14 focused adapters, five sample tests, warnings-denied documentation, the
executed sample and whitespace. Runner/image/tools match the reviewed input.
This follow-up records that exact checkpoint; its own later revision must be
assessed separately, rather than inheriting the checkpoint's pass.

## Finite message-sequence checkpoint — 2026-10-10

The [sequence review](MESSAGE_SEQUENCE_REVIEW.md) records checkpoint
`82078b3656481ec9527aaa059e159ca446c54aec` and its finite-domain limits.
[Push run 37935055379](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/37935055379)
passes the exact logged checkout, one Windows job 113834907623 and all 20
reported steps. Logs confirm 142 workspace tests, including the new
publication/dequeue model, separate 14 adapter/five sample tests,
warnings-denied rustdoc, executed sample and both whitespace checks.

Runner 2.337.0, windows-2025-vs2026 image 20260925.250.1, Rust/Cargo 1.99.0,
rustfmt 1.10.0-stable and Clippy 0.1.99 match the existing reviewed source
baseline. Fresh local checks remain on 1.98.0, with zero source-policy drift.
The workflow is unchanged. The dated ADR-0023 default-branch revisit confirms
main contains the same workflow while nightly push/PR triggers remain intact.
No manual dispatch or settings change was performed in this run. This evidence
follow-up needs its own exact-head CI; it does not inherit this checkpoint pass.

## Runtime lifecycle sequence checkpoint — 2026-10-11

The [runtime sequence review](RUNTIME_MESSAGE_SEQUENCE_REVIEW.md) records
checkpoint `c3eaa10cb013d1d4f5dbd11f6f0c4a3d4d98d37b` and its successful-callback
finite domain. It adds one integration target without changing production or CI.
[Push run 38054603275](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/38054603275)
passes the exact logged checkout, one Windows job 114220376706 and all 20
reported steps. Logs confirm 143 workspace tests including the new lifecycle
model, separate 14 adapters/five sample tests, warnings-denied rustdoc, the
executed sample and both committed-tip/working-tree whitespace checks.

Runner 2.337.0, windows-2025-vs2026 image 20260925.250.1, Rust/Cargo 1.99.0,
rustfmt 1.10.0-stable and Clippy 0.1.99 match the prior reviewed hosted baseline.
Fresh locked local checks remain on 1.98.0. Source review finds no width,
configuration or waiver drift. Conditional ADR probes and actionlint are
unchanged and not triggered; the hosted job still runs both focused host
targets and the sample. This evidence follow-up needs separate exact-head CI,
and does not inherit the checkpoint's pass or establish human review.

## Runtime ordinary-work failure sequence checkpoint — 2026-10-12

The [failure sequence review](RUNTIME_WORK_FAILURE_SEQUENCE_REVIEW.md) records
checkpoint `d40cd8f3810e41dc5b97e895c21df5bb9635de57` and both-role finite limits.
[Push run 38141934356](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/38141934356)
passes its exact logged checkout, Windows job 114477390027 and all 20 steps.
Logs confirm 144 workspace tests including the new model, separate 14 adapters/
five sample tests, warnings-denied rustdoc, the complete executed sample report
and both whitespace checks. Root and parallel Codex final_review inspect raw
logs and job JSON separately from local verification; source/document/diff
review remains a local obligation.

Runner 2.337.0, windows-2025-vs2026 image 20260925.250.1, Rust/Cargo 1.99.0,
rustfmt 1.10.0-stable and Clippy 0.1.99 match the existing hosted baseline.
The companion local source review covers 36 Rust files/13,935 lines with no width,
configuration or waiver changes. Local execution remains on 1.98.0. Production,
dependencies and workflow are unchanged. Conditional ADR probes and actionlint
are not triggered; the hosted job still executes both focused host targets and
the sample. This later evidence follow-up requires its own exact-head hosted
result rather than inheriting the checkpoint's pass.

## Runtime message-callback failure sequence checkpoint — 2026-10-13

The [message-failure sequence review](RUNTIME_MESSAGE_FAILURE_SEQUENCE_REVIEW.md)
records checkpoint `fe40d592eb434a0730eac8c14744361e9d685200`, both fault roles
and its separate reduced dispatch domain. It adds one integration target without
changing production, dependencies or workflow.
[Push run 38253819562](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/38253819562)
passes exact logged checkout, Windows job 114826598201 and all 20 reported steps.
Logs confirm 145 workspace tests including the new model, separate 14 adapters/
five sample tests, warnings-denied rustdoc, the exact unchanged full sample report
and both whitespace checks. Root and parallel Codex evidence_review inspect
raw logs/job JSON separately from local verification.

Runner 2.337.0, windows-2025-vs2026 image 20260925.250.1, Rust/Cargo 1.99.0,
rustfmt 1.10.0-stable and Clippy 0.1.99 match the existing hosted baseline.
The companion whole-tree review covers 37 Rust files/14,480 physical lines, no
width/configuration/waiver drift and three unchanged fulfilled expectations.
Local execution remains on 1.98.0. Conditional ADR probes and actionlint are
unchanged and not triggered; hosted CI still executes both focused host targets
and the sample. This later evidence follow-up requires its own exact-head hosted
result rather than inheriting the checkpoint's pass.

## Messaging-owned schedule sequence checkpoint — 2026-10-14

The [schedule sequence review](MESSAGING_SCHEDULE_SEQUENCE_REVIEW.md) records
checkpoint `0ca95560041f249911eb9d107cedd06f731ea159`: both fault positions,
fixed physical agenda, manual-time/publication/attempt histories through length
eight and supplemental healthy work/FIFO observations. Production, dependencies
and workflow are unchanged.
[Push run 38403821340](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/38403821340)
passes its exact logged checkout, Windows job 115318053007 and all 20 steps.
Raw logs confirm 146 workspace tests including the new model, separate
14 adapters/five sample tests, warnings-denied rustdoc, the exact unchanged
full sample report and both whitespace checks. Root and parallel Codex
baseline_review inspect raw JSON/logs separately from local verification.

Runner 2.337.0, windows-2025-vs2026 image 20260925.250.1, Rust/Cargo 1.99.0,
rustfmt 1.10.0-stable and Clippy 0.1.99 match the reviewed input. Companion
whole-tree source/configuration review covers 38 Rust files/15,084 lines, zero
width findings and three unchanged fulfilled expectations. Local execution
remains on 1.98.0. Conditional ADR probes/actionlint are unchanged and not
triggered; hosted CI still executes both focused host targets and the sample.
Any later documentation revision requires its own completed exact-head hosted
result; inspect the current branch head rather than inheriting this checkpoint.

## Standalone configuration sequence checkpoint — 2026-10-15

The [configuration sequence review](CONFIGURATION_SEQUENCE_REVIEW.md) records
checkpoint `56a68a0c7c860dee02ac42e441f9857421aad164`: one table with bound two,
accepted/rejected replacement and consume-once rollback histories through length
six, plus distinct retained-history/revision probes. Production, dependencies
and workflow are unchanged.
[Push run 38569938802](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/38569938802)
passes exact logged checkout, Windows job 115872183297 and all 20 steps.
Raw logs confirm 147 workspace tests including the model, separate 14 adapters/
five sample tests, warnings-denied rustdoc, all 222 unchanged sample report lines
and both whitespace checks. Root and parallel Codex baseline_review inspect
raw JSON/logs separately from local checks and source/document/diff review.

Runner 2.337.0, windows-2025-vs2026 image 20260925.250.1, Rust/Cargo 1.99.0,
rustfmt 1.10.0-stable and Clippy 0.1.99 match the reviewed input. Companion
whole-tree source/configuration review covers 39 Rust files/15,333 lines, zero
width findings and three unchanged fulfilled expectations. Local execution
remains on 1.98.0. Conditional ADR probes/actionlint are unchanged and not
triggered; hosted CI still executes both focused host targets and the sample.
This later evidence follow-up preserves source and requires its own completed
exact-head hosted result rather than inheriting the checkpoint's pass.

## Direct-runtime configuration/work sequence checkpoint — 2026-10-16

The [runtime configuration sequence review](RUNTIME_CONFIGURATION_SEQUENCE_REVIEW.md)
records checkpoint `283b07c280746dd6ac7c26b60bdaf0ab14e5bad3`: two initially
Running applications, bound-two configuration, selected/peer work and selected
stop/restart through length five for both selected indices, plus separate
terminal probes. Production, dependencies and workflow are unchanged.
[Push run 38739984030](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/38739984030)
passes exact logged checkout, Windows job 116438836697 and all 20 steps.
Raw logs confirm 148 workspace tests including the model, separate 14 adapters/
five sample tests, warnings-denied rustdoc, all 222 unchanged sample report lines
and both whitespace checks. Root and parallel Codex baseline_review inspect
raw JSON/logs separately from local checks and source/document/diff review.

Runner 2.337.0, windows-2025-vs2026 image 20260925.250.1, Rust/Cargo 1.99.0,
rustfmt 1.10.0-stable and Clippy 0.1.99 match the reviewed input. Companion
whole-tree review covers 40 Rust files/15,912 lines, zero width findings and
three unchanged fulfilled expectations. Local execution remains on 1.98.0.
Conditional ADR probes/actionlint are unchanged and not triggered; hosted CI
still executes both focused host targets and the sample. This later evidence
follow-up preserves source and requires its own completed exact-head hosted
result rather than inheriting the checkpoint pass.

## Standalone event history and toolchain checkpoint — 2026-10-17

The [event sequence review](EVENT_QUEUE_SEQUENCE_REVIEW.md) records checkpoint
`7137e924819b8cc35f0c4f3464f80c3d8df1b149`: capacities one, two and three,
mixed emission/dequeue histories through length eight, separate terminal drains
and selected compiling fault sensitivity. Production, dependencies, lint and
workflow remain unchanged.
[Push run 38905783350](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/38905783350)
passes exact logged checkout, Windows job 116988877022 and all 20 reported steps.
Raw logs confirm 149 workspace tests including the new model, separate 14 adapter/
five sample tests, warnings-denied rustdoc, all 222 unchanged sample report lines
and both whitespace checks. Root and independent Codex source_review inspect
raw JSON/logs separately from local checks and source/document/diff review.

Hosted Rust/Cargo changes from 1.99.0 to 1.99.1; compiler commit 58cf8f9c3 is
dated 2026-10-13. Rustfmt is 1.10.0-stable and Clippy 0.1.99 at that compiler
commit. Runner 2.337.0 uses windows-2025-vs2026 image 20261007.264.1 instead of
20260925.250.1. The required configuration and whole-tree source re-audit accepts
stable max_width=100, comment-only review at 80, too-many-lines-threshold=60,
unsafe_code=forbid and the individually selected too_many_lines deny lint.
All-target hosted Clippy completes with warnings denied and three fulfilled
existing expectations. No new lint group, waiver, exclusion or source change is
needed. The companion source audit covers 41 Rust files/16,117 physical lines/
1,120 comment-only lines with zero width findings. This retains the current
policy without claiming toolchain pinning or a minimum supported Rust version.

Local tools remain Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy 0.1.98;
there is no local 1.99.1 execution claim. Conditional ADR probes/actionlint are
unchanged and not triggered; hosted CI executes its configured host/sample
checks. Ignored raw evidence remains under target/review-2026-10-17-events.
This documentation follow-up preserves Rust/configuration/workflow blobs and
requires its own exact-head hosted result rather than inheriting this checkpoint.

## Direct-runtime failure-event history checkpoint — 2026-10-18

The [runtime event sequence review](RUNTIME_EVENT_SEQUENCE_REVIEW.md) records
checkpoint `9fc7e0240b36f7eca76f81c5b48030ff8904a338`: two distinct first-work
failures, a healthy peer, event capacities one/two with an older diagnostic, and
mixed work/time/dequeue/explicit rejected-event retry histories through length
five. Production, dependencies, lint and workflow remain unchanged.
[Push run 39035017652](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/39035017652)
passes exact logged checkout, Windows job 117402970574 and all 20 steps. Raw logs
confirm 150 workspace tests including the model, separate 14 adapter/five sample
tests, warnings-denied rustdoc, all 222 unchanged sample report lines and both
whitespace checks. Root and independent Codex records_review inspect raw JSON/logs
separately from local checks and complete source/document/diff acceptance.

Rust/Cargo 1.99.1, rustfmt 1.10.0-stable, Clippy 0.1.99, runner 2.337.0 and
windows-2025-vs2026 image 20261007.264.1 match the reviewed input. Companion
whole-tree review covers 42 Rust files/16,520 physical lines/1,129 comment-only
lines, zero width findings and three unchanged fulfilled expectations. Local
execution remains on 1.98.0. Conditional ADR probes/actionlint are unchanged and
not triggered; hosted CI executes its configured host/sample checks. Ignored raw
evidence remains under target/review-2026-10-18-work-events. This documentation
follow-up preserves source/configuration/workflow blobs and requires its own
exact-head hosted result rather than inheriting the checkpoint pass.

## Messaging-owned event history checkpoint — 2026-10-19

The [messaging event history review](MESSAGING_EVENT_SEQUENCE_REVIEW.md) records
checkpoint `27194287b8670b75efac05f3a3f0a7de38a312d5`: two initially Running
applications, inbox capacities [1,2], both fault roles, event capacities one/two
with an older diagnostic, and mixed publication/work/peer-dispatch/time/event-
dequeue/explicit retry histories through length five. Production, dependencies,
lint and workflow are unchanged.
[Push run 39130580285](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/39130580285)
passes exact logged checkout, Windows job 117684735602 and all 20 steps. Raw logs
confirm 151 workspace tests including the model, separate 14 adapter/five sample
tests, warnings-denied rustdoc, all 222 unchanged sample report lines and both
whitespace checks. Root and independent Codex records_review inspect raw metadata/
logs separately from local source/document/diff acceptance.

Rust/Cargo 1.99.1, rustfmt 1.10.0-stable, Clippy 0.1.99, runner 2.337.0 and
windows-2025-vs2026 image 20261007.264.1 match the reviewed input. Companion
whole-tree review covers 43 Rust files/17,101 physical lines/1,138 comment-only
lines, zero width findings and three unchanged fulfilled expectations. Local
execution remains on 1.98.0. Conditional ADR probes/actionlint are unchanged and
not triggered; hosted CI executes its configured host/sample checks. Ignored raw
evidence remains under target/review-2026-10-19-messaging-events. This later
documentation follow-up preserves source/configuration/workflow blobs and requires
its own exact-head hosted result rather than inheriting the checkpoint pass.

## Practical-library direction checkpoint — 2026-10-19

[ADR-0025](../adr/0025-practical-library-development.md) records the practical
reusable-library objective and autonomous progression beyond the v0.1 baseline.
Checkpoint `1e1dd96043494872dbf463642ad8e8d2becca61f` reconciles active guidance,
charter, roadmap and crate documentation; runtime behavior and APIs are unchanged.
The existing nightly prompt is updated and exactly read back with other settings
preserved. This is direction and documentation evidence, not consumer readiness.

[Push run 39247706648](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/39247706648)
passes its exact logged checkout, Windows job 118043591010 and all 20 steps. Raw
logs confirm 151 workspace tests, separate 14 adapter/five sample tests,
warnings-denied rustdoc, the unchanged 222-line sample and both whitespace checks.
Root and independent Codex library_direction_review inspect raw hosted metadata/
logs separately from local source/diff review.

Hosted Rust/Cargo 1.99.1, rustfmt 1.10.0-stable, Clippy 0.1.99, runner 2.337.0
and windows-2025-vs2026 image 20261007.264.1 match input. Local checks remain on
Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy 0.1.98. Whole-tree review
covers 43 Rust files/17,104 physical lines/1,138 comment-only lines, zero width
findings and three unchanged fulfilled expectations. Unchanged host/experiment/
workflow sources do not trigger extra conditional local probes; hosted CI runs
its configured host targets/sample. This documentation follow-up preserves source
and requires its own complete review and exact-head hosted result.
