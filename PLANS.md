# Autonomous host v0.1 milestone review

Date: **2026-10-09**
Status: **Complete: review published and exact-checkpoint hosted CI passed**

## Objective and context

Complete the actual host entry-point and architecture review delegated in
[ADR-0024](docs/adr/0024-autonomous-host-review-authority.md). This is the
recorded next task and closes a milestone uncertainty before further hardening.
Assess the existing implementation; do not add a feature or freeze public APIs.

Reviewed input: `ba42f800d0b74637b28c833a55192fc541912fe5`, clean on
`codex/nightly` and equal to refreshed origin. Fresh locked local checks pass
141 workspace tests on Rust/Cargo 1.98.0, rustfmt 1.9.0-stable, Clippy 0.1.98.
Focused adapter/sample tests pass 14/five; the executable reproduces the
controlled host trace. Prior exact-head hosted run 37675969512 is confirmed
successful, separately from this local evidence.

## Acceptance criteria

- Record exact reviewed input, Codex reviewer identities, review scope,
  requirement/test evidence, findings, limitations, and explicit dispositions.
- Inspect every current user-facing entry point, including generated public
  rustdoc and executed sample output, against RFF-REQ-001.
- Review resource/failure contracts, API shape, dependencies/features, licences,
  provenance, and all Stage 4 host evidence gates without widening their scope.
- Reconcile any current documentation finding and requirement/roadmap/state
  status only after the actual review is complete. Preserve dated history.
- Applicable checks, source form, relative links, exact traceability references,
  rendered-document and complete-diff review pass before publication.
- Publish by ordinary fast-forward to the existing `origin/codex/nightly` and
  inspect exact-head hosted CI for each published revision.

## Components and verification

Add a durable review in `docs/verification/HOST_V0_1_REVIEW.md`. Reconcile
README, requirements, architecture, roadmap, state, traceability, sample guide,
CI baseline, source register as needed, and this plan. Runtime/source, manifests,
licences, lint policy, workflow, and historical checkpoints are review inputs.

```text
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
cargo test --locked --test host_adapters
cargo test --locked --test host_sample
cargo run --locked --example host-echo
cargo metadata --locked --format-version 1 --all-features
cargo tree --locked --workspace --all-features --target all --edges all
cargo tree --locked --workspace --no-default-features --target all --edges all
git check-attr text eol -- LICENSE-MIT LICENSE-APACHE
git diff --check
git show --format= --check --diff-merges=first-parent HEAD
```

Rustdoc runs with `RUSTDOCFLAGS=-D warnings`. Root executes Cargo serially;
parallel Codex reviewers inspect entry points, architecture, and scope.
Unchanged ADR-0018/0019 experiments and workflow do not trigger standalone
probes or actionlint. Temporary audits/rendering aids remain ignored in
`target/review-2026-10-09-host`; they are not new repository gates.

## Risks and stopping point

Accept only the experimental serial host milestone supported by evidence.
Caller-scoped identity/clock pairing, cooperative-only containment, logical
capacity limits, and floating toolchain remain explicit limitations. Codex
review does not establish human review, an API freeze, operational suitability,
or authorization to release/deploy. If a finding invalidates a host requirement,
record it and leave that gate open. Stop after this coherent review checkpoint;
use its findings to select the next separate hardening increment.

## Completed local review and verification

Codex entry-point, architecture, and scope reviewers report no blocking finding
against the current host requirements. The durable review records all gate
dispositions and accepted limits; RFF-REQ-001 and Stage 4 are accepted for the
experimental serial host target. Runtime, API, dependency, licence, lint, and
workflow inputs are unchanged. The source-register push-filter wording is
corrected from default branch to the actual codex/nightly filter.

Initial and final locked baselines pass 141 workspace tests, formatting,
all-target check, warnings-denied Clippy/rustdoc and whitespace. Initial focused
adapters/sample pass 14/five and the executable matches the documented trace.
Fresh complete metadata and both all-target trees confirm the empty external
Cargo dependency/feature graph. The whole-tree audit finds no width debt across
33 Rust files/12,666 physical lines and retains three fulfilled expectations.
All 320 relative links across 62 Markdown files and 105 exact traceability
function references resolve. Eleven changed documents pass rendered HTML
structure/content inspection; no pixel-level acceptance is claimed. Primary
and parallel Codex source, complete-diff, evidence, and rendered-content reviews
accept the reconciliation. Publication and exact-checkpoint hosted CI pass below.

## Publication and hosted verification

Review checkpoint `44f85edd4ae8fdcfb6fbcb34b59cea7478d496cb` was published
by ordinary fast-forward on codex/nightly; local and remote heads matched.
[Hosted push run 37782350928](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/37782350928)
passes its exact logged checkout, one Windows job, and all 20 reported steps.
Logs confirm 141 workspace tests, 14 focused adapters, five sample tests,
warnings-denied rustdoc, executed sample and both whitespace checks.

Runner 2.337.0, windows-2025-vs2026 image 20260925.250.1, Rust/Cargo 1.99.0,
rustfmt 1.10.0-stable and Clippy 0.1.99 match the reviewed baseline. Local
toolchain stays 1.98.0; source policy is unchanged. This follow-up records that
checkpoint's completed evidence; later revisions require their own CI.
Stop after the host review checkpoint; the next candidate is bounded model
testing of publication/dequeue sequences.
