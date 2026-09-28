# Stage 4 messaging-construction diagnostics review

Date: **2026-09-29**
Status: **Complete locally; publication and hosted verification pending**

## Objective and context

Close one bounded public-diagnostics evidence gap in the existing
`MessagingRuntime` attachment API. Verify that a later-inbox capacity-overflow
failure exposes its complete standard-library error source chain while
preserving the already-tested unchanged runtime ownership and corrected
attachment path.

The starting revision is `f4ffabbef7787b3bbbdede28f1e4bcf50a044dcb`,
clean and equal to `origin/codex/nightly`. Its exact hosted run 36347110418
passed. The initial locked local baseline passes 134 tests on Rust/Cargo 1.98.0,
rustfmt 1.9.0-stable, and Clippy 0.1.98.

This is a Stage 4 constructor and public-diagnostics checkpoint. It does not
change construction order, allocation behavior, returned ownership, public
types, message routing, dependencies, or configuration semantics.

## Acceptance criteria

- The failed attachment exposes
  `MessagingRuntimeCreateError -> MessagingRuntimeCreateErrorKind ->
  MessageBusCreateError` through `std::error::Error::source`.
- The source-chain values match the existing typed `kind()` result exactly.
- The returned runtime still retains both registered applications and supports
  corrected attachment, preserved application behavior, peer publication, and
  peer work.
- Required locked Cargo checks, source-form review, document checks, and
  complete diff review pass without a new lint exception.
- Durable state distinguishes this diagnostic evidence from allocator
  exhaustion, every constructor failure, or a public API freeze.

## Proposed files

- `tests/runtime_messaging.rs`
- `docs/verification/MESSAGING_CONSTRUCTION_DIAGNOSTICS_REVIEW.md`
- `README.md`
- `docs/ROADMAP.md`
- `docs/PROJECT_STATE.md`
- `docs/verification/TRACEABILITY.md`
- `PLANS.md`

## Verification approach

Run the focused owner target first, then the full locked repository baseline:

```text
cargo test --locked --test runtime_messaging
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

The source chain is diagnostic traversal, not automatic recovery. The
deterministic `usize::MAX` input exercises an unrepresentable nonzero-sized
inbox reservation; it does not inject allocator exhaustion or execute every
construction failure variant.

Stop after the existing owner regression proves the nested source chain and
recovery behavior. Do not change error types or add allocation hooks in this
increment.

## Completed local checks

The focused runtime-messaging target passes 11 tests, and the initial and final
locked workspace baselines pass 134 tests. Formatting, all-target checking,
warnings-denied Clippy/rustdoc, and whitespace checks pass without a new lint
exception. The first full Clippy attempt correctly rejected a 69-line expanded
test; extracting the independently meaningful source-chain assertion into a
narrow helper restored the 60-line policy without a waiver.

The whole-tree audit passes 32 Rust files with zero physical or comment-width
findings, no block comments, and three unchanged fulfilled expectations; 52
Markdown files and 244 resolving relative links; 35 source definitions and 104
exact traceability test-name references under the current audit method. Six
changed controlling documents pass PowerShell Markdown structural rendering.
Pixel-level visual acceptance is not claimed.
