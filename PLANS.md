# Stage 4 returned-stop failure and peer dispatch

Date: **2026-10-03**
Status: **Complete: checkpoint published and exact-revision hosted CI passed**

## Objective and context

Verify the complete retained peer FIFO and callback eligibility after a
cooperative stop error through the messaging owner. The existing stop test
checks one discarded message and peer counts, but does not consume the peer's
retained messages. ADR-0007 and ADR-0011 already define the expected behavior;
this checkpoint adds evidence without changing those contracts.

The starting revision is `2b138e11415695129e6162954600911f71ec2eaf`,
clean and equal to refreshed `origin/codex/nightly`. Its hosted run 36770717562
succeeded. The initial locked local baseline passes 134 tests on Rust/Cargo
1.98.0, rustfmt 1.9.0-stable, and Clippy 0.1.98.

## Acceptance criteria

- A concrete stop error commits only the selected application to `Failed` and
  clears its full two-message inbox with an exact discard count of two.
- The running peer retains its two older messages, accepts a third publication,
  and dispatches the exact three-message FIFO one message per call.
- Failed application stop, restart, ordinary work, and dispatch requests return
  lifecycle errors with zero discard and invoke no additional callbacks.
- The peer completes subsequent ordinary work; an empty dispatch invokes no
  callback.
- Focused and full locked checks, source-form review, rendered-document review,
  links, and complete-diff review pass without new lint exceptions.

## Proposed files and verification

Add `tests/messaging_stop_failure.rs` with a stop-specific callback trace.
Reconcile `docs/verification/STOP_FAILURE_REVIEW.md`, traceability, README,
roadmap, project state, and this plan. No production or dependency change is
planned.

```text
cargo test --locked --test messaging_stop_failure
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
git diff --check
```

Set `RUSTDOCFLAGS=-D warnings` for documentation. Audit handwritten Rust
physical/comment widths, expectations, relative Markdown links, exact test
references, changed rendered documents, and the complete diff. Publish only
after local acceptance, then inspect hosted CI at the exact published revision.

## Risks and safe stopping point

This returned-error scenario does not prove application-internal cleanup,
panic or hang containment, recovery, secure erasure, or human v0.1 acceptance.
Stop after the new public regression and its records are verified. If it fails,
investigate the existing stop boundary before changing production behavior;
preserve unrelated files and revert only this run's changes if needed.

## Completed local checks

`returned_stop_failure_preserves_peer_fifo_and_terminal_callback_gates`
passes in the focused target. The final locked workspace baseline passes 135
tests, with formatting, all-target check, warnings-denied Clippy/rustdoc, and
whitespace passing. No lint exception was added. Production, Cargo, workflow,
and licence inputs remain unchanged.

The whole-tree review finds no physical/comment-width violations or block
comments across 33 Rust files; three existing expectations remain fulfilled.
The final relative-link audit passes 55 Markdown files and 266 links. Eight
changed documents render successfully with PowerShell and receive structural review;
pixel-level visual acceptance is not claimed. The complete diff is reviewed
before committing. Exact publication and hosted results remain separate.

## Publication and required toolchain re-audit

Checkpoint `cd7318e0bfdcedb33610e887cfd794f90d6f16cf` was published by
ordinary fast-forward and its remote head matched. Hosted run
[37018451102](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/37018451102)
passed the exact logged checkout: one Windows job, every configured step, the
new regression, 135 workspace tests, 14 focused adapter tests, five focused
sample tests, and the sample executable.

The hosted stable toolchain advanced to Rust/Cargo 1.99.0, rustfmt
1.10.0-stable, and Clippy 0.1.99. The required configuration/whole-tree
re-audit is recorded in the [CI baseline](docs/verification/CI_BASELINE.md).
The unchanged format/lint configuration and source pass the new hosted gates;
manual source review and widths remain separate. No local toolchain was
installed or changed. This documentation-only follow-up records completed
evidence; later revisions require their own hosted result.
