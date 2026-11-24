# Message-callback failure-event review

Date: 2026-11-24. Reviewers: root Codex and an independent Codex agent.
Scope: RFF-LIB-REQ-020 revision 1 and ADR-0045.

## Input and objective

Clean main `151511aec980dffe41dc25ab5d406798437459ee` matches fetched
origin/main. The locked input baseline passes all 304 workspace tests,
format, all-target check/Clippy, warnings-denied rustdoc and whitespace.
Exact input hosted run 45135377234/job 137560284366 passes all 20 steps;
metadata and logged checkout identify that exact SHA.

Add one opt-in owner-preserving returned-message-error event operation.
The original dispatch and work-event consumers remain regression evidence.
No dependency, workflow, lint or permanent service ownership changes.

## Local commands and result

All commands below completed successfully on the final implementation:

```text
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --test message_dispatch_events
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
git diff --check
```

RUSTDOCFLAGS is `-D warnings` for rustdoc. The full suite passes 311 tests,
including seven new public-only histories. Focused target Clippy with
`--locked --test message_dispatch_events -- -D warnings` also passes.
Local Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy 0.1.98 remain
unchanged; no waiver or checker baseline is introduced.

Tests cover empty/success suppression, every rejected lifecycle state,
unknown IDs, exact original message errors/source chains and ownership
accessors. A failing callback accepts one self and one peer reply before
returning Requested(17). The selected clear count is exactly two: the older
queued tail plus self reply, excluding the attempted input. Peer FIFO
[11, 22, 77] survives and later peer work/dispatch succeed. Event fields and
one clock read are exact. Saturation keeps the older record and exposes
the rejected original event for explicit retry after time advances. Failed
redispatch emits no duplicate. Equal-slot foreign IDs retain their known
receiving-runtime alias limitation.

## Independent consumer

Executed the [guide recipe](../MESSAGE_FAILURE_EVENTS.md#verify-identical-source-independently)
at `C:/Users/drrsm/AppData/Local/Temp/rff-message-failure-events-059e4dfd7dd74704966a778ded76939f`.
After cargo new/add, these commands pass with that package's absolute
Cargo.toml supplied to every manifest-path argument:

```text
cargo check --manifest-path <consumer>/Cargo.toml --locked --offline --all-targets
cargo clippy --manifest-path <consumer>/Cargo.toml --locked --offline --all-targets -- -D warnings
cargo test --manifest-path <consumer>/Cargo.toml --locked --offline --test message_dispatch_events
cargo metadata --manifest-path <consumer>/Cargo.toml --locked --offline --format-version 1
```

All seven identical integration tests pass. Metadata confirms one independent
edition 2024 library workspace member, its sole framework path dependency
and two graph packages. Test source bytes match exactly: 18,793 bytes /
583 lines, SHA256
`7f60d287bf15f6a12606cccebd4b980615b1d0ae73f8d39818aae7d13d5dc93b`.
Normalized Git blob is `5710b95acdbd2db4d2d4291d396afa79504e9d65`.
The temporary Cargo package is evidence, not another maintained crate.

## Review and limits

Root and independent Codex source/test review accept the implementation.
Source review confirms the owner commits Failed and clears the selected
inbox before event construction reads the clock. Public tests observe the
post-return state and counts; they do not reborrow the mutably borrowed
owner from the injected clock. Private shared event construction preserves
the ordinary-work policy; its unchanged tests remain regression evidence.

The temporary inventory/link/test-reference and rendered HTML inspection
commands are retained under ignored
`target/review-2026-11-24-message-events/audit/`. They inspect handwritten
Rust physical/comment widths, unchanged expectations, relative paths and
fragments, exact test references and rendered content/structure. Audit passes
60 Rust files, 29,418 physical and 1,550 comment-only lines with zero width
findings and three unchanged fulfilled expectations. All 803 relative paths,
88 fragments and 271 unique exact test references across 130 Markdown files
resolve. All 12 changed Markdown documents pass rendered content/structure
inspection. Root and independent Codex accept the complete implementation
source/document/diff at staged tree
`ad3c08ebe1937656a3aca778de3830acd6a81665`.
These temporary aids do not establish new adopted gates or browser pixel QA.

The first rendered-document pass found an unquoted generic type interpreted
as HTML and a comma-qualified code fence unsupported by the review renderer.
Both source Markdown issues were corrected; the repeated rendered review
passes. A temporary hosted-log summary initially hit Windows console
encoding on a BOM; UTF-8 output fixes that reporting error. The GitHub
connector's log retrieval failed, so gh retrieves exact-run metadata/logs.
No implementation or required baseline check failed.

Callbacks and clocks must return. Reporting adds no rollback, implicit retry,
recovery, panic/hang containment, origin validation, reserved capacity or
guaranteed delivery. Queue-external event retention needs its own finite
policy. Logical bounds do not bound arbitrary application allocation or
execution time. Local/independent tests establish this controlled consumer,
without qualification, external-adoption or API-stability claims.

## Publication

Implementation `420d5e342a9096a878dfea57bc69be0d4830fa2a` has that exact
reviewed tree and reached origin/main by ordinary fast-forward push.
Local, tracking and live remote heads match after publication.

Exact implementation hosted
[run 45329058609](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/45329058609),
job 138225516658, completes all 20 steps successfully. Metadata and logged
checkout identify the exact implementation SHA. The job passes 311 workspace
tests, separate 14 adapter and five sample tests, warnings-denied rustdoc and
both whitespace checks. All 222 host-sample stdout lines match the input job.
Hosted Rust/Cargo 1.100.0, rustfmt 1.11.0-stable, Clippy 0.1.100, runner
2.338.2 and Windows image `20261116.315.1` match input. No toolchain/source-
policy change is selected; the local toolchain remains separately 1.98.0.

Codex accepts this bounded increment under ADR-0025. This record-only
completion receives separate document/diff review and exact hosted checking.
No implementation remains and no unrelated work follows. Raw local,
independent-consumer, audit and hosted evidence lives under ignored
`target/review-2026-11-24-message-events/`.
