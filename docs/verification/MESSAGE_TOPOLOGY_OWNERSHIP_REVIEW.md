# Copied message-topology ownership review

Date: **2026-10-04**
Scope: **ADR-0010/0011 caller-storage independence at both constructors**

## Inspected boundary and decision

[ADR-0010](../adr/0010-bounded-message-routing-core.md) selects copied,
immutable capacities and topic sets in registration order.
[ADR-0011](../adr/0011-runtime-owned-message-availability.md) assigns each
runtime configuration position to its registered application and constructs a
fresh owned bus. Both paths use `create_endpoint` in `src/messaging.rs`, which
stores capacities and copies topic values into endpoint-owned storage.

Existing constructor helpers let configuration arrays go out of scope, but
their topic slices are static or remain unchanged. This checkpoint adds public
tests of topic mutation and destruction together with configuration replacement
and reordering. The existing implementation needs no change. This is evidence
for RFF-REQ-003, not a new requirement or architecture decision. No external
research or source-register amendment is needed for the recorded contract.

## Executed public observations

`copied_topology_survives_caller_configuration_and_topic_changes` constructs
the standalone bus with capacities one and two. Both applications subscribe
to Command; only the peer subscribes to Telemetry. After construction it
reorders, replaces, clears, and drops the configuration vector, changes both
topic vectors to Unrouted, clears them, and lets them drop.

The bus still exposes capacities one and two, routes Command to the original
identities in registration order, routes Telemetry only to the peer, and has
no Unrouted subscribers. Dequeue returns exactly the Command for the first
application and Command then Telemetry for the peer, followed by empty inboxes.

`copied_runtime_topology_survives_caller_storage_changes_and_drop` constructs
two registered applications with Command subscriptions and capacities one
and two. Its fixture reorders and replaces configuration entries with capacity
nine and empty topics, drops the configuration vector, replaces the original
topic with Unrouted, and returns after caller topic storage has dropped.

The runtime owner retains original capacities and registration-order
Unavailable outcomes before start. Unrouted has no subscribers. After start,
the first Command reaches both applications; the second fills the peer while
the first rejects it; the third is rejected by both. Stopping the first clears
exactly one delivery and leaves the peer's two queued records. Restart restores
the first endpoint's original subscription and capacity: another Command is
Delivered there and InboxFull at the peer. This runtime test observes queue
counts and lifecycle effects; exact FIFO contents are observed in the
standalone test and existing dispatch regressions.

## Evidence limits

The copied topic is a plain mission enum. `Copy + Eq` does not establish deep
isolation of arbitrary referents or equality depending on shared mutable state.
This checkpoint does not add dynamic subscriptions, validate identity issuers,
measure allocator capacity or whole-process memory, stabilize APIs, or complete
human v0.1 acceptance. Mission composition still selects intended positions.

## Verification

The initial locked baseline passes 135 tests at
`565c9d44a0cc18bf9717c64416e5f265b5dda9e2`. Focused targets pass 10
standalone and 12 runtime tests. The first runtime compile attempt used an
incorrect test accessor; it was corrected to `discarded_deliveries`. Clippy
then reported 83/60 lines, reduced to 65/60 by extracting fixture construction.
A shorter local owner binding removes redundant line wrapping; final all-target
Clippy passes without a new exception or threshold change.

```text
cargo test --locked --test message_bus --test runtime_messaging
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
git diff --check
```

Rustdoc uses `RUSTDOCFLAGS=-D warnings`. The final locked workspace passes 137
tests, formatting, all-target check, warnings-denied Clippy/rustdoc, and
whitespace. Final source review passes 33 Rust files and 12,483 physical lines
with zero physical/comment-width findings, no block comments, and three
unchanged fulfilled expectations. All 270 relative links across 56 Markdown
files and 101 exact traceability test references resolve. Six changed documents
render with PowerShell and receive HTML structural/content review; pixel-level
acceptance is not claimed. Complete source/diff review passes.

Production, public API, dependency, licence, lint configuration, and workflow
inputs are unchanged. Conditional ADR-0018/0019 probes are unchanged and were
not rerun.

## Publication and hosted result

Checkpoint `ab9914618a739d4b7c8a46f0e14d7e12c55b6649` was published by
ordinary fast-forward and the remote head matched. Hosted
[push run 37128965033](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/37128965033)
uses the unchanged `.github/workflows/ci.yml` at that exact revision. Its
`Record checkout` log matches the checkpoint SHA. The single Windows job and
every configured step pass, including both new regressions, 137 workspace
tests in aggregate, 14 focused adapter tests, five focused sample tests,
warnings-denied rustdoc, the executed sample, and both whitespace checks.

Runner 2.337.0 uses `windows-2025-vs2026` image `20260925.250.1`. Compiler
1.99.0 (`b940084d7`, 2026-09-28), Cargo 1.99.0 (`5f94df478`, 2026-08-27),
rustfmt 1.10.0-stable, and Clippy 0.1.99 match the previous hosted checkpoint;
the existing source-policy re-audit remains recorded in the CI baseline.
Local tools remain Rust/Cargo 1.98.0, rustfmt 1.9.0-stable, and Clippy 0.1.98.
This documentation follow-up records completed exact-revision evidence;
later revisions need their own CI result. Human v0.1 acceptance remains open.
