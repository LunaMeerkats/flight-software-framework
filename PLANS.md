# Stage 4 scheduled-failure diagnostics review

Date: **2026-09-28**
Status: **Complete locally; publication and hosted CI pending**

## Objective and context

Close one bounded public-diagnostics evidence gap in the existing finite
scheduled-work API. Verify that both direct scheduled lifecycle rejection and
cooperative application failure expose the complete standard-library error
source chain while preserving the already-tested final item consumption and
later peer progress.

The starting revision is `bb3ec40cac4d72e9ca8b9d38c3a191fa55cdb070`,
clean and equal to `origin/codex/nightly`. Its exact hosted run 36276641746
passed. The initial locked local baseline passes 134 tests on Rust/Cargo 1.98.0,
rustfmt 1.9.0-stable, and Clippy 0.1.98.

This is a Stage 4 public-API and failure-diagnostics checkpoint. It does not add
scheduled event emission, retry, recurrence, panic or hang containment, an
executor, a dependency, or a new public API.

## Acceptance criteria

- Direct scheduled lifecycle rejection exposes
  `ScheduledWorkError -> RuntimeWorkError -> LifecycleError` through
  `std::error::Error::source`.
- Direct scheduled application failure exposes
  `ScheduledWorkError -> RuntimeWorkError -> application error` through the
  same standard interface.
- Both paths retain the consumed item, observed instant, exact typed error,
  terminal-state behavior where applicable, and later due-peer progress.
- Existing messaging-owned source-chain evidence remains passing.
- Required locked Cargo checks, source-form review, document checks, and
  complete diff review pass without a new lint exception.
- Durable state distinguishes this diagnostic evidence from scheduled event
  reporting or broader fault containment.

## Proposed files

- `tests/scheduled_work.rs`
- `docs/verification/SCHEDULE_FAILURE_DIAGNOSTICS_REVIEW.md`
- `README.md`
- `docs/ROADMAP.md`
- `docs/PROJECT_STATE.md`
- `docs/verification/TRACEABILITY.md`
- `PLANS.md`

## Verification approach

Run the direct and messaging-owned scheduled-work targets first, then the full
locked repository baseline:

```text
cargo test --locked --test scheduled_work
cargo test --locked --test messaging_scheduled_work
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
RUSTDOCFLAGS=-D warnings cargo doc --locked --workspace --all-features --no-deps
git diff --check
```

Also audit handwritten Rust physical widths and comment widths, relative
Markdown links, exact test references, changed rendered documents, and the
complete diff.

## Completed local checks

The focused direct and messaging-owned schedule targets pass 11 and nine tests,
and the initial and final locked workspace baselines pass 134 tests. Formatting,
all-target checking, warnings-denied Clippy/rustdoc, and whitespace checks pass
without a new lint exception. Production code, public API, dependencies,
licences, workflow, source register, and service policies are unchanged.

The whole-tree audit passes 32 Rust files with zero physical or comment-width
findings, no block comments, and three unchanged fulfilled expectations; 51
Markdown files, 237 resolving relative links, 35 source definitions, and 101
exact traceability test references. PowerShell Markdown rendering for the five
changed controlling documents passes structural inspection. Pixel-level visual
acceptance is not claimed.

## Risks and stopping point

The source chain is a diagnostic traversal contract, not ownership recovery or
automatic error handling. Downcasting in tests proves the concrete nested
errors at this API boundary; it does not promise a stable public API freeze.

Stop after the direct source-chain regressions and review record are verified.
Do not add scheduled events or change error types in this increment.
