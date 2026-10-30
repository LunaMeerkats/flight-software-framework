# Subscription-respecting selective publication

Status: Local verification and Codex review passed — 2026-10-31

## Objective and context

Complete RFF-LIB-REQ-009/ADR-0034: enable a host to retry one rejected subscriber
without duplicating an earlier fan-out acceptance. Clean input
986ffd82a7a909d10bc2e6cbd50875c98410e2e4 matches refreshed origin. Initial locked
format/check/Clippy/test/warnings-denied rustdoc and both whitespace checks pass
with 208 workspace tests. Exact input hosted raw logs/connector steps pass all
20 steps, 208 workspace/14 adapter/five sample tests and unchanged sample output.

## Acceptance and components

- Add MessageBus/MessagingRuntime publish_to and export typed PublishToError.
  Reject unknown/unsubscribed destinations before lifecycle/capacity checks;
  reuse bounded enqueue with no report allocation or peer mutation.
- Add public-only tests and a fixed two-subscriber retry consumer. Cover
  lifecycle, FIFO/saturation, nonmutating errors, no duplicate accepted copies
  and acceptance-versus-processing. Keep broadcast/context contracts unchanged.
- Document use/errors/resources and independent identical-source verification.
  Update requirements, ADR, traceability, roadmap, current state and entry points.

## Verification and stopping point

Run the locked AGENTS.md baseline plus focused selective-publication tests.
Check/lint/test an independent path-dependent package with the identical tests;
inspect metadata and source hashes. Inspect widths/waivers, relative links,
changed rendered documents and complete diff with independent Codex review.
Publish coherent verified commits by ordinary fast-forward push and inspect
exact hosted checkout/results. Logs/aids stay in ignored
target/review-2026-10-30-selective-retry (run's UTC date).

Retry ledgers represent enqueue acceptance only. No automatic deduplication,
drain, fairness, authentication or timing guarantee. Stop after this checkpoint;
if implementation cannot pass, remove only current-run changes where safe and
retain useful evidence without discarding unrelated state or weakening gates.

## Actual evidence

Final locked baseline passes 217 workspace tests; focused nine tests pass.
Identical external package check/Clippy/nine tests, bytes/SHA256 and normalized
metadata pass. Whole-tree source-width/waiver audit, links and traceability
inspection pass; generated API docs and changed rendered documents are reviewed
for content/structure. Root and independent source review find no source issues;
one guide capacity mismatch is corrected. Complete root/independent evidence/diff
review passes with no findings. Ordinary publication and exact hosted inspection
remain before this checkpoint is complete.
