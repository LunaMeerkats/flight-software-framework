# Bounded startup and cleanup consumer

## Objective and selection

Implement RFF-LIB-REQ-023 revision 1 and ADR-0048: a fixed three-application
public-only startup/cleanup recipe. Clean input main
82ed58a04aba1bcc0660c3bc386902ff3e0654ac matches refreshed/live origin.
The locked input baseline passes 332 tests and all static/documentation checks.
Composition currently demonstrates infallible lifecycle callbacks; propagating
the first start error leaves partial-start cleanup policy unspecified.

## Acceptance and components

- Start three distinct initially Registered applications in fixed order.
  Stop starting at the first error; preserve the original typed error.
- Stop only the successful prefix in reverse order. Continue after stop
  errors; retain every result in at most three inline report slots.
- Complete startup leaves all Running until a separate explicit shutdown.
- Add one runnable test-enabled public-only example and guide. No library
  API, dependencies, retry, recovery or dependency graph.
- Test first/middle/last start failure, reverse order, multiple stop failures,
  exact non-Copy error/source ownership, lifecycle rejection and stdout.
  Verify identical source independently with metadata, bytes and SHA256.

## Verification and stopping point

Run focused tests/run and the full locked baseline. Independently check/lint/
test/run the source as a sole-path-dependency binary. Review whole-tree widths,
narrow expectations, Markdown links, rendered changed documents and complete
diff with independent Codex review. Commit on main, fast-forward push, inspect
exact hosted CI and record results. Stop after one increment. If it cannot
pass, preserve useful research and revert only current-run implementation.

## Risks

Callbacks must return. Stop success records callback success, not independent
proof of application resource cleanup. Failed startup may already have effects;
the framework cannot stop/restart that terminal Failed record. Distinct
same-owner IDs, initial states and order are fixed consumer setup invariants.
General orchestration and compensation remain separate decisions.

## Local result

All final locked commands pass, including 339 workspace and seven focused
example tests. The identical independent binary passes check/Clippy/seven
tests/run, metadata, bytes/SHA256 and exact ten-line stdout comparison.
Root and independent Codex accept the full 14-file increment without findings.
Whole-tree source-form/link/test-reference review and all 12 rendered-document
content/structure reviews pass; no new waiver, policy or dependency is added.
The focused record is docs/verification/STARTUP_CLEANUP_REVIEW.md.
Accepted staged tree `f097cb12491567dc864833d7f15dea1936dbf184` matches
implementation `df30c44b9201e74f56904da4d4f23cadbd2fb156`, published by
ordinary fast-forward. Exact run 46005118740/job 140526235236 passes all
20 steps, 339 workspace tests and separate host checks. All 222 sample
stdout lines match input; hosted tools, runner and image match input.
Evidence-only completion receives separate review and exact hosted inspection.
Stop here; no implementation remains.
