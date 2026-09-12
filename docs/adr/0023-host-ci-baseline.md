# ADR-0023: One host CI baseline

- Status: Accepted and configured; hosted execution unverified
- Date: 2026-09-12
- Scope: Stage 4 execution of existing checks and the combined sample

## Context

ADR-0022 completes the controlled host sample. Its 115-test local baseline
does not fulfill the separate requirement to pass checks in CI. The repository
has a GitHub origin and its live default branch is `codex/nightly`. No push is
authorized. A locally reviewed workflow is a useful stopping point before
external activation, without claiming a hosted result.

## Decision

Use one [GitHub Actions workflow](../../.github/workflows/ci.yml) on the named
`windows-2025` hosted image, with PowerShell and stable Rust. This matches the
OS/toolchain family of the existing local evidence. Install rustfmt and Clippy
explicitly with rustup, select stable through `RUSTUP_TOOLCHAIN`, and log the
checkout SHA and actual compiler, Cargo, rustfmt, and Clippy versions.

Run formatting, all-target/all-feature checking and linting, workspace tests,
the two focused shared-source host targets, warnings-denied rustdoc, and the
sample binary. Add `--locked` to commands that resolve the dependency graph so
unexpected lockfile changes fail. Keep the existing local command baseline
valid. The focused targets intentionally duplicate tests within the workspace
suite so the mission boundary has explicit named job steps.

Run on pushes to `codex/nightly`, ordinary pull requests with default activity
types, and manual dispatch. Keep checkout's default event revision: a PR tests
GitHub's merge result, not an overridden contributor head. Record that exact
SHA when assessing results. There are no path filters that silently skip docs
or configuration changes. Manual dispatch requires the workflow on the default
branch; PR conflicts, fork approvals, and repository settings can prevent runs.

Set `contents: read`, pin checkout v7.0.1 to upstream commit
`3d3c42e5aac5ba805825da76410c181273ba90b1`, and disable persisted checkout
credentials. Use no secrets, self-hosted runners, caches, artifact uploads,
publication, dispatch to privileged workflows, or write permission. Pinning
fixes this action revision; it does not audit or eliminate upstream trust.

Each `run` step contains one native command. GitHub's built-in `pwsh` wrapper
propagates command failure; later checks cannot hide a failed earlier command.
A 20-minute job timeout bounds a hung check at the CI layer. It is unrelated
to application deadline or hang containment. No failure is allowed to pass
through `continue-on-error`.

Retain `git diff --check` for working-tree changes, and check committed tip
whitespace with `git show --format= --check --diff-merges=first-parent HEAD`.
Fetch depth two provides the first parent. On a PR this inspects the merge
result against its first parent; on a push it covers the final commit, not
every intermediate commit in a multiple-commit push. It handles root commits.
Do not interpret a clean working-tree diff as committed-content evidence.

## Alternatives considered

- **Linux or a multi-OS matrix:** useful later host portability evidence, but
  adds an environment before the existing Windows baseline is hosted. Start
  with one job; no cross-platform support claim is made.
- **Pinned compiler and fully frozen environment:** could support a separately
  justified reproducibility policy. The current policy uses stable and logs
  evidence versions. Keep that policy; a named runner image still changes.
- **Preinstalled toolchain only:** simpler, but leaves component/channel
  selection implicit. Use the runner's rustup to request stable explicitly.
- **Additional setup/cache actions or shared orchestration scripts:** no
  demonstrated need in this small dependency-free workspace. Direct commands
  make baseline coverage and failure propagation reviewable.
- **Privileged PR trigger or head override:** unnecessary for verification.
  Keep ordinary `pull_request` and the tested merge result.
- **Wait for publication permission before preparing CI:** delays a reversible
  reviewable configuration. Complete local work while retaining the push gate.

## Evidence and provenance

The [CI verification record](../verification/CI_BASELINE.md) separates local
checks from hosted acceptance. The [source register](../research/SOURCES.md)
records GitHub workflow/event/runner documentation, checkout provenance,
rustup/Cargo behavior, Git diff behavior, and the temporary actionlint validator.
No upstream NASA source or runtime implementation is reused.

## Risks and revisit conditions

Stable Rust and the hosted image float; logs identify what actually ran, not a
reproducible compiler/image pin or an MSRV. A changed toolchain needs the
existing whole-tree source-policy re-audit. Static validation and local command
replay cannot prove hosted checkout, bootstrap, permissions, or runner behavior.

Keep the CI release gate open until a successful exact-revision hosted run is
recorded. Human source/document review, conditional ADR probes, dependency and
scope review, and v0.1 architecture review remain separate. Revisit for an
observed hosted failure, a default-branch change, a concrete additional host,
required check policy, dependency growth, or evidence that version drift is
obstructing review. No push authorization or release decision is changed.
