# Backpressure-aware recurring host coordination

Status: Local and independent-consumer verification passed — 2026-10-29

## Objective and context

Complete RFF-LIB-REQ-008/ADR-0033 in the existing public-only work_output example.
Clean input c899aef39a8df854d14b48484bd044ee14bba120 matches refreshed origin.
Initial locked format/check/Clippy/test/rustdoc and whitespace checks pass,
including 201 workspace tests. Exact input hosted run 40511047519 passes all
20 steps; raw logs and connector steps agree. No toolchain/configuration change.

## Acceptance and components

- One host turn gates dispatch on empty output/Running sink, attempts existing
  sole-subscriber transfer once, and polls/invokes recurrence only with empty
  pending storage. New output waits for a later turn; caller drains separately.
- Tests fill the three-stage pipeline, retain FIFO/configuration/timer while
  blocked, resume one overdue release, handle unavailable sinks and disable
  recurrence on work failure. Existing strict direct-call tests remain.
- Update the work-output guide/entry points, requirements, ADR, source register,
  traceability, focused verification, roadmap and current state. Verify identical
  source in an independent Cargo package; no library API/dependency changes.

## Verification and risks

Run the locked AGENTS.md baseline, focused work_output test/run and the guide's
independent-package check/Clippy/test/run/metadata/hash recipe. Inspect source
widths/waivers, relative links, changed rendered docs and complete diff with root
and independent Codex review. Raw aids/logs stay in ignored
target/review-2026-10-29-host-coordination. Refresh origin, publish by ordinary
fast-forward push and inspect exact hosted checkout/results.

Single-subscriber retry, serial caller pairing and nontransactional effects stay
explicit. A bounded operation count does not bound callback time. Stop after this
coherent checkpoint. If implementation fails, preserve evidence and remove only
current-run changes where safe; never discard unrelated work or weaken gates.

## Actual local evidence

Final locked baseline passes 208 workspace tests, 17 focused tests and both
runnable scenarios. The identical independent package passes check/Clippy/17
tests/run and normalized metadata/hash/byte inspection. Whole-tree source audit
finds zero width issues and three unchanged fulfilled expectations. Links,
traceability and changed rendered documents pass content/structure inspection.
Root and independent coordination_review accept the complete 13-file source,
document and diff with actual raw evidence and no remaining findings.
Exact hosted results remain separate and will be recorded after execution.
