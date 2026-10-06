# Configuration work-view refresh review

Date: **2026-10-07**
Status: **Complete: checkpoint published and exact-revision hosted CI passed**

## Contract and coverage reconciliation

[ADR-0018](../adr/0018-configuration-aware-work-context.md) lends the currently
accepted revision and used bytes to each synchronous ordinary-work callback.
[ADR-0017](../adr/0017-bounded-configuration-snapshots.md) supplies immutable
bounded storage with consume-once rollback. This checkpoint retains both
decisions and RFF-REQ-006's existing meaning.

Standalone table tests already cover shrinking snapshots and owned inputs.
The starting runtime baseline covered an initial short prefix, fixed two-byte
transitions, configuration absence, and a configured zero-byte table. Host
observations use one-byte values. The [borrowing experiment](CONFIGURATION_CONTEXT_EXPERIMENT.md)
already rejects production callback-borrow escape; existing integration logs
retain copies across activation/rollback, and the host report outlives its
runtime. This review addresses the remaining explicit ordinary-work observation
of changing used lengths, rather than treating lifetime safety as untested.

## Executed observation

The existing `work_context_exposes_only_the_used_configuration_prefix` test in
`tests/configuration_runtime.rs` now runs one configured application through:

| Operation before work | Observed revision | Complete observed bytes |
| --- | --- | --- |
| Initial exact-bound configuration | 1 | `[4, 5, 6, 7]` |
| Replace with shorter content | 2 | `[8]` |
| Replace with empty content | 3 | `[]` |
| Roll back to the immediately previous snapshot | 2 | `[8]` |

All four work outcomes retain `Running`. After the runtime leaves scope, the
test checks exactly four callback invocations and four complete copied
observations. Each is `Some`, including revision three's empty bytes, so
configured presence is retained rather than confused with absence. No unused
inline capacity or stale suffix appears after shrinking or rollback; earlier
observations retain their original full content and revision.

The test reuses the existing observing application and assertion helper. It
strengthens one regression without adding a public API, production change,
dependency, lint expectation, or new test target. It is below the function-size
threshold. No external research or new architecture decision is needed.

## Evidence limits

This observes direct ordinary work on controlled in-memory input. Existing
composition tests remain the scheduled/event/messaging evidence; this test
does not add a changing-length scenario to every owner. Application-owned
copies/logs are outside the runtime's storage bound. Scope exit does not
instrument deallocation, bound retained bytes, or establish secure erasure.
Borrowing probes, broader configuration access, callback panic/hang handling,
identity provenance, and human v0.1 acceptance retain their existing status.

## Verification

The initial locked baseline passes 141 tests on Rust/Cargo 1.98.0,
rustfmt 1.9.0-stable, and Clippy 0.1.98. Stable rustfmt and the focused
configuration-runtime target pass all 11 tests, including the strengthened
regression. The final locked baseline passes 141 tests with formatting,
all-target check, warnings-denied Clippy/rustdoc, and whitespace passing.
Independent source review accepts the regression. The source audit covers 33
Rust files and 12,632 physical lines with zero width findings and three
unchanged fulfilled expectations. All 286 relative links across 59 Markdown
files and 105 exact traceability function references resolve. Author and
independent complete-diff/source reviews pass. Six changed documents pass
rendered HTML structure/content inspection; no pixel-level acceptance is claimed.

```text
cargo test --locked --test configuration_runtime
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
git diff --check
```

Rustdoc uses `RUSTDOCFLAGS=-D warnings`. Unchanged ADR-0018/0019 experiments,
host adapters/sample, and workflow do not trigger their separate local probes.
The full workspace suite includes both host test targets. Rendered Markdown
inspection covers structure/content; no pixel-level acceptance is claimed.
Temporary review aids/logs under `target/review-2026-10-07` remain ignored and
do not adopt a checker.

## Publication

Checkpoint `16fb457f6ee7a49109ef1e245c59ea5ec9588922` was published by
ordinary fast-forward and the remote head matched. Hosted push run
[37468611582](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/37468611582)
passes its exact logged checkout, single Windows job, and all 20 reported
steps. The workflow contains 17 named steps; GitHub adds job lifecycle steps.
Logs confirm the strengthened regression among 141 workspace tests, separate
14 adapter/five sample tests, warnings-denied rustdoc, the executed sample,
and both whitespace checks. Focused reruns do not add to the workspace total.

Runner 2.337.0 uses `windows-2025-vs2026` image `20260925.250.1`.
Rust/Cargo 1.99.0, rustfmt 1.10.0-stable, and Clippy 0.1.99 match the prior
checkpoint. Local tools remain 1.98.0/1.9.0-stable/0.1.98; no new policy
adaptation is required. Full metadata and logs remain in the ignored review
directory. This documentation follow-up records checkpoint evidence; later
revisions require their own CI result. Human v0.1 acceptance remains open.
