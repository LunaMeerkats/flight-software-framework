# Autonomous host review authority checkpoint

Date: **2026-10-08**
Status: **Complete: authority published and exact-revision hosted CI passed**

## Objective and context

Implement the user's direct instruction to continue development without
requesting routine human acceptance. Delegate host v0.1 entry-point and
architecture acceptance to documented autonomous review, retain actual review
evidence/provenance, and preserve separate consequential authorization gates.
Do not complete Stage 4 or a review merely by changing its reviewer policy.

Started clean at `95de6331fb04e7fc489bfcd39a1b98daaebece20`, equal to
refreshed `origin/codex/nightly`; exact hosted run 37626703339 succeeded.
The fresh locked baseline passes 141 tests on Rust/Cargo 1.98.0,
rustfmt 1.9.0-stable, and Clippy 0.1.98.

## Acceptance criteria

- ADR-0024 records the dated direct delegation and its limits.
- Active contributor, requirement, roadmap, state, entry-point, and
  traceability guidance no longer requires routine human acceptance. Actual
  autonomous reviews remain to be performed and cannot be invented.
- Historical checkpoints retain their original evidence; source behavior,
  API, dependency, licence, lint configuration, and workflow stay unchanged.
- The existing nightly automation carries the direction while retaining its
  schedule, model/reasoning, project target, execution, and notifications.
- Applicable checks, links, source/document/diff review, publication, and
  exact-revision hosted verification pass.

## Files and verification

Add `docs/adr/0024-autonomous-host-review-authority.md`; reconcile AGENTS,
README, architecture, requirements, roadmap, state, traceability, CI baseline,
and this plan. Update the existing automation through its application tool and
verify the persisted instruction and unchanged settings.

```text
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
git diff --check
```

Set `RUSTDOCFLAGS=-D warnings`. Audit relative links and physical/comment
widths; inspect changed rendered Markdown, review the complete diff, and
publish only to `codex/nightly` after local acceptance. Unchanged host adapters,
sample, workflow, and ADR-0018/0019 experiments do not trigger extra probes.

## Risks and stopping point

Delegation is authority, not proof that a human or Codex completed a review.
Record Codex provenance honestly. Tags, releases, crate publication,
deployment, identity/licence changes, major API/runtime/dependency decisions,
unsafe code, security, and hardware/RTOS/no_std commitments retain their own
authorization gates. Stop after this coherent authority checkpoint; the next
engineering task can perform the actual autonomous host review.

## Completed verification

Initial and final locked local baselines pass 141 tests. Formatting, all-target
check, warnings-denied Clippy/rustdoc, and whitespace pass. No Rust source or
function-size expectation changes. Whole-tree audit retains 33 Rust files and
12,666 physical lines with zero width findings and three fulfilled expectations.
All 304 relative links across 61 Markdown files and 105 exact traceability
function references resolve. Eleven changed documents pass rendered HTML
structure/content inspection; no pixel-level acceptance is claimed.

The existing automation was updated through the application tool. Its exact
saved prompt matches the requested delegation; all other persisted settings,
including daily midnight cadence, model/reasoning, local project target, and
failed-runs-only notifications, match the starting configuration. Author and
independent complete-diff/source/document reviews pass, including all eleven
rendered documents.

## Publication and hosted verification

Authority checkpoint `300363fb60343e99fd3ac2989581a3e78f5f65be` was
published by ordinary fast-forward; the remote head matched. Hosted push run
[37675094546](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/37675094546)
passes the exact logged checkout and all 20 reported steps (17 named workflow
steps plus job lifecycle). Logs confirm 141 workspace tests, separate 14
adapter/five sample tests, warnings-denied rustdoc, the executed sample, and
both whitespace checks. Focused reruns do not increase the workspace total.

Runner 2.337.0, `windows-2025-vs2026` image `20260925.250.1`, and hosted
Rust/Cargo 1.99.0, rustfmt 1.10.0-stable, and Clippy 0.1.99 match the prior
baseline. Local tools remain 1.98.0/1.9.0-stable/0.1.98; no policy adaptation
is required. This documentation follow-up records authority-checkpoint evidence;
later revisions need their own CI. Actual host milestone review remains the
next engineering task, with no routine human acceptance request.
