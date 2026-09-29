# Stage 4 configured-runtime construction diagnostics review

Date: **2026-09-30**
Status: **Complete: checkpoint published and exact-revision hosted CI passed**

## Objective and context

Close one bounded public-diagnostics evidence gap in the existing configured
`Runtime` constructor. Verify that both deterministic runtime-storage failures
expose their exact `RuntimeCreateError` through `std::error::Error::source`
while preserving the already-tested complete validated configuration lineage.

The starting revision is `9cad622987f47f0ab14e9922a8ab2b4e29b20f6a`,
clean and equal to `origin/codex/nightly`. Its exact hosted run 36477422468
passed. The initial locked local baseline passes 134 tests on Rust/Cargo 1.98.0,
rustfmt 1.9.0-stable, and Clippy 0.1.98.

This is a Stage 4 constructor and public-diagnostics checkpoint. It does not
change construction order, allocation behavior, returned ownership,
configuration validation or revision semantics, public types, dependencies, or
runtime behavior.

## Acceptance criteria

- Zero-capacity and `usize::MAX` configured construction failures expose
  `RuntimeConfigurationCreateError -> RuntimeCreateError` through
  `std::error::Error::source`.
- Each traversed source exactly matches the existing typed `kind()` value.
- Each returned table still preserves active revision 2, rollback revision 1,
  and the next unused revision 3 after caller recovery.
- Required locked Cargo checks, source-form review, document checks, and
  complete diff review pass without a new lint exception.
- Durable state distinguishes deterministic capacity rejection from allocator
  exhaustion, automatic recovery, or a public API freeze.

## Proposed files

- `tests/configuration_runtime.rs`
- `docs/verification/CONFIGURATION_CONSTRUCTION_DIAGNOSTICS_REVIEW.md`
- `README.md`
- `docs/ROADMAP.md`
- `docs/PROJECT_STATE.md`
- `docs/verification/TRACEABILITY.md`
- `PLANS.md`

## Verification approach

Run the focused configured-owner target first, then the full locked repository
baseline:

```text
cargo test --locked --test configuration_runtime
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

## Risks and stopping point

The deterministic inputs exercise explicit zero capacity and an unrepresentable
nonzero-sized record reservation. They do not inject allocator exhaustion,
execute unrelated constructor wrappers, or add recovery policy.

Stop after the existing configured-construction regression proves the source
chain and unchanged lineage recovery. Do not change error types or add
allocation hooks in this increment.

## Completed local checks

The focused configuration-runtime target passes 11 tests, and the initial and
final locked workspace baselines pass 134 tests. Formatting, all-target
checking, warnings-denied Clippy/rustdoc, and whitespace checks pass without a
new lint exception.

The whole-tree audit passes 32 Rust files with zero physical or comment-width
findings, no block comments, and three unchanged fulfilled expectations; 53
Markdown files and 249 resolving relative links; 35 source identifiers and 107
exact traceability test-name references under the current audit method. Six
changed controlling documents pass PowerShell Markdown structural rendering.
Pixel-level visual acceptance is not claimed.

## Publication result

Checkpoint `887f9965bc8db0939dc161ffc3a58ad7387d2910` was published by
ordinary fast-forward; the remote head matched. Hosted run 36624191212 passed
that exact push and checkout: one Windows job, every configured step, the
strengthened regression, 134 workspace tests in aggregate, 14 focused adapter
tests, five focused sample tests, and the sample executable.

This documentation-only follow-up records completed evidence with unchanged
Rust, Cargo, workflow, and lint inputs. Its rendered content, links, and diff
are reviewed separately. A later revision requires its own hosted result.
