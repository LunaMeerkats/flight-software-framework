# Callback-owned selective reply retry

Status: Complete implementation checkpoint — 2026-11-01

## Objective and selection

Close the callback ownership gap named by ADR-0034: a handler can inspect partial
broadcast acceptance but cannot retry one rejected subscriber without moving
reply state to the host or changing topic vocabulary. Root Codex and independent
consumer_assessment select one context method over speculative coordination.
RFF-LIB-REQ-010 and ADR-0035 precede implementation; retain existing contracts.

## Acceptance and components

- Add ApplicationMessageContext::publish_to using the existing selective primitive
  and dispatch-state snapshot. No new report, queue, dependency or retry service.
- Add public-only tests/callback_publication.rs: application-owned reply and fixed
  ledger; explicit rejected-only retry, new-reply gating, FIFO/peer exclusion,
  repeated rejection, lifecycle refresh, routing errors, self-publication and
  nontransactional callback-failure clearing.
- Add guidance/independent-package recipe and update traceability, architecture,
  roadmap, source register, AGENTS, README and concise current state.
- Run locked baseline, focused test, identical independent check/Clippy/test and
  metadata/hash checks; audit widths/waivers, links, rendered documents and full
  diff. Obtain independent Codex review before committing and pushing main.
- Refresh origin, ordinary fast-forward push, inspect exact hosted revision and
  record actual results. Stop after this one coherent increment.

## Baseline and risks

Clean input main 3d547b75cdae424b123927fde09a915299b940a8 matches fetched origin.
Initial locked format/check/Clippy/test/warnings-denied rustdoc and both whitespace
checks pass (217 workspace tests). Local Rust/Cargo 1.98.0, rustfmt 1.9.0-stable
and Clippy 0.1.98 are unchanged. Live hosted run 41136402063 passes all 20 steps;
root raw logs and connector summaries confirm its exact checkout and results.

The ledger records enqueue only; later stop/failure may discard accepted copies.
Retry initiation/cancellation and foreign-ID pairing remain mission duties.
Effects are nontransactional; logical bounds do not bound process memory/time.

## Safe stopping point

One verified additive context method and controlled consumer. If checks/review
cannot pass, remove only this run's implementation and retain actual decision
and failure evidence. No release, tag, crate publication or deployment.

## Actual local results

The locked baseline passes 224 workspace tests, focused seven tests/Clippy pass,
and the independent package passes check/Clippy/seven identical tests plus its
separate generated smoke test. Metadata/bytes/SHA256 match the focused record.
Root and independent callback_review accept source and consumer with no findings.
Whole-tree widths/waivers, relative links and 188 exact test references pass.
All 12 implementation-changed Markdown documents pass rendered content/structure
review; generated API docs and complete root/independent Codex diff review pass.
Implementation e2ad3484530091e9b51e5f61ead43318187e802e equals the accepted
staged patch and is published by ordinary fast-forward to origin/main. Exact
run 41204737701/job 124547413239 passes all 20 steps, 224 workspace/14 adapter/
five sample tests and unchanged 222-line sample. Root raw logs and connector
summaries agree; hosted tools/runner match input and image is 20261021.278.1.
The record-only completion retains source bytes and receives separate review,
ordinary main publication and exact hosted inspection in automation memory/report.
No further implementation or behavior is selected.
