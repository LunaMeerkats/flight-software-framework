# ADR-0039: Runnable bounded command session

- Status: Implemented; local/independent checks and source review pass; publication pending
- Date: 2026-11-05
- Scope: Public-only executable consumer under RFF-LIB-REQ-014

## Context

ADR-0037/0038 demonstrate correlated commands and local results in integration
tests. Their fixture uses asserting setup/drain helpers and copied observations.
A developer still lacks an executable session connecting validated ingress,
result feedback and reply output without importing that fixture. Root Codex and
independent consumer_assessment select this specific integration gap.

## Decision

Independently author one standalone test-enabled Cargo example using public
framework APIs and std. One serial host owns the issuer and explicitly selects
every ingress, dispatch and drain. One producer retains a correlated reply with
two acceptance flags and emits one typed local result. Each of two subscribers
stages one host output. The host owns Cell slots borrowed by applications;
registering subscribers before the producer supplies destination IDs without
late-bound shared routing storage. Occupied host slots pause the corresponding
bounded turn before another dispatch. Retain full runtime errors and reject malformed
input explicitly. The fixed driver prints outside callbacks.

Keep the existing callback/correlated test sources unchanged. Verify the exact
new source as an independent path-dependent Cargo binary, including behavior,
execution, metadata and bytes/hash. One maintained package remains sufficient.

## Alternatives considered

- Extract the correlated fixture: defer; its test observations and asserting
  helpers are not operational policy, and extraction would churn frozen evidence.
- Import tests through path attributes: reject; developers should not depend on
  integration-test internals to run a public-API recipe.
- Independently author a standalone consumer: selected; it exposes setup/error
  handling and host decisions while preserving prior evidence.
- Add a generic runner, result API or bus acknowledgement service: defer; existing
  public composition has not demonstrated a missing library capability.
- Interactive/network command ingress: defer; external I/O framing, transport
  loss and retained-writer policy require their own concrete need and evidence.

## Evidence and consequences

The official Cargo targets reference informs example placement and test=true.
Official Rust Cell and checked_add documentation inform fixed copied slots and
explicit finite exhaustion (SRC-RUST-COMMAND-SESSION). Borrowed slots suffice
for this scoped session; Rc ownership, used by the earlier consumer, is not
needed here. Host turn order, payload grammar, capacities and scenario are local
choices. No upstream code is reused.
The [guide](../COMMAND_SESSION.md) and
[focused review](../verification/COMMAND_SESSION_REVIEW.md) record ten focused
and identical independent binary tests, matching source/output/metadata and the
253-test locked baseline. Root and independent consumer_assessment accept the
complete source without findings. Root and independent verification_support
also accept the complete document/diff evidence. Publication and exact hosted
acceptance remain pending until observed.

## Risks and revisit conditions

The example duplicates mission policy intentionally rather than sharing test
internals; its own behavioral tests must protect the demonstrated composition.
Serial host gates depend on trusted access and explicit draining. Session IDs
supply correlation only; results are local snapshots of command handling and
inbox acceptance. Printing is fallible and does not establish durable or physical
delivery. Logical storage bounds do not establish process-memory or timing bounds.
No persistence, duplicate suppression, concurrency or public API freeze follows.
Revisit extraction for demonstrated shared production responsibility, or library
APIs when an independently authored consumer cannot compose the required use.
Separate release, platform, security and other consequential gates remain.
