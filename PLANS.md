# Correlated cancellation of a callback-owned operation

Status: Complete implementation checkpoint — 2026-11-03 Australia/Sydney

## Objective and selection

Prevent a delayed cancellation from erasing a newer pending reply. Root Codex
and independent consumer_assessment select a separate public-only consumer using
existing APIs over a larger acknowledgement protocol or automatic lifecycle
abandonment. RFF-LIB-REQ-012 revision 1 precedes implementation. Preserve the
original callback consumer and its 12 regressions unchanged.

## Acceptance and components

- Add tests/correlated_commands.rs, independently composed with public APIs.
  One pending operation and two acceptance flags; matching Cancel/Retry only.
- Issue finite session-local IDs with checked exhaustion; demonstrate stale
  commands, exact FIFO/peer retention, busy admission, saturation and restart.
- Record policy/alternatives in ADR-0037, guide, source register, traceability,
  focused review, roadmap and current state. Keep docs proportionate.
- Run focused checks and locked baseline, identical independent Cargo consumer,
  metadata/byte/SHA256 comparison, width/waiver/link/test-reference audits and
  rendered-document content/structure inspection. Obtain independent Codex
  source/full-diff review before committing main.
- Refresh origin, ordinary fast-forward push main, inspect exact hosted CI.

## Baseline, risks and safe stopping point

Clean main input d56441235dc8670a14c8a54fadd4df4abeb6c9f3 matches fetched
origin/main. Initial locked baseline passes 229 workspace tests and all checks.
Local Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy 0.1.98 are unchanged.

IDs correlate equality within one host session only. No acknowledgement,
authentication, deduplication, persistent epoch, new library service, dependency
or workflow is introduced. Command acceptance remains enqueue only; accepted
copies cannot be retracted. Stop/restart retains application state and must not
reset the host issuer. If verification fails, remove only current-run source
changes where safe and retain useful decision/failure evidence. Stop after this
one coherent verified consumer increment.

## Actual local results

Seven focused tests/Clippy pass. Final locked baseline passes 236 workspace tests
and warnings-denied rustdoc. Identical independent consumer check/Clippy/seven
tests plus generated smoke test pass; metadata, bytes and SHA256 match the
focused record. Whole-tree audit finds 54 Rust files, 23,848 physical/1,361
comment-only lines, zero width findings and three unchanged expectations.
Relative links and all 200 exact test references resolve. Changed documents pass
rendered content/structure inspection, without a pixel-QA claim. Root and
independent verification_support accept the complete 13-file diff with no
findings. All 12 changed documents pass rendered content/structure review.
Implementation 34514bd98bb671fadb2863f5d12492b54297e37c exactly matches the
accepted staged patch and is published by ordinary fast-forward to origin/main.
Exact run 41450862997/job 125295426494 passes all 20 steps, 236 workspace/14
adapter/five sample tests and unchanged 222-line sample. Hosted tools/runner/image
match input. This record-only completion receives separate review/publication
and exact hosted inspection in automation memory/report. No further behavior is
selected.
