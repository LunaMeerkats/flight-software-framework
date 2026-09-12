# Host CI verification baseline

Date: **2026-09-12**
Status: **Configured locally; hosted execution unverified**

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
are recorded in [the completed plan](../../PLANS.md).

The committed whitespace command checks a root or tip commit, or a merge
against its first parent. A push containing multiple commits does not get
per-intermediate-commit whitespace coverage. The separate working-tree check
does not inspect unchanged committed content. No branch protection or required
check policy is configured by adding this file.

## Hosted acceptance still required

No workflow has been pushed or dispatched by this increment. To close the
hosted CI gate after authorized publication, record:

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
workflow does not claim to execute them. Human entry-point, dependency, scope,
and architecture review remain v0.1 release gates.
