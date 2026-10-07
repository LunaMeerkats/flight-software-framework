# Project state

Last updated: **2026-10-08**

## Current milestone

Stages 1 through 3 and the first source-quality checkpoint are complete.
Stage 4 has sample, hosted CI, dependency/scope, resource, failure, schedule,
identity, diagnostics, and ownership evidence. The user now delegates routine
host entry-point and architecture acceptance to documented autonomous review.
The actual reviews remain open; routine human acceptance no longer blocks work.

## Verified baseline

- Started clean at `95de6331fb04e7fc489bfcd39a1b98daaebece20`, equal to
  refreshed `origin/codex/nightly`; exact hosted run 37626703339 succeeded.
- Initial and final locked local baselines pass 141 tests. Formatting,
  all-target check, warnings-denied Clippy/rustdoc, and whitespace pass.
  This authority checkpoint changes documentation and automation instructions.
- Local Rust/Cargo: 1.98.0; rustfmt: 1.9.0-stable; Clippy: 0.1.98.
- Hosted tools: Rust/Cargo 1.99.0, rustfmt 1.10.0-stable,
  Clippy 0.1.99. The [source-policy re-audit](verification/CI_BASELINE.md)
  retains existing format, width, function-size, and waiver policy.
- Source audit retains 33 Rust files and 12,666 physical lines with zero width
  findings and three unchanged fulfilled expectations. All 304 relative links
  across 61 Markdown files and 105 traceability function references resolve.
  Eleven changed documents pass rendered HTML structure/content inspection;
  author/independent document/diff review passes; no pixel-level acceptance is
  claimed. Publication and exact-head hosted CI are pending.

## Current architecture

One unpublished package owns synchronous LC1 lifecycle/work, bounded inbox
routing/dispatch, manual time and one-shot scheduling, bounded events, and
optional runtime configuration with immutable work visibility and one-use
rollback. The private shared-source host sample composes these services.
Production behavior and public APIs remain unchanged. Review authority is
recorded separately in ADR-0024; actual host review is not yet complete.

## Work in progress

The [authority decision](adr/0024-autonomous-host-review-authority.md) removes
the routine human acceptance gate while retaining recorded review, evidence,
and actual reviewer provenance. The existing nightly automation now carries
that direction; its schedule, model, reasoning, target, and notification policy
are preserved and verified. No unfinished implementation remains. Local
acceptance passes; publication and exact-head hosted CI are pending.

## Highest risks and uncertainties

- Application identity and clock-origin pairing remain caller discipline:
  `ApplicationId` holds a record position; `FrameworkInstant` holds elapsed time.
- Copied generic topic/event identifiers can retain arbitrary shared referents;
  plain enums do not prove isolation of those referents.
- Logical capacities do not bound whole-process bytes. Allocator exhaustion
  and allocation/deallocation counts remain unverified; caller-retained
  configuration copies and observations lie outside runtime storage bounds.
- Callback/clock panics and hangs remain outside containment. Stop clearing
  does not prove application-internal/external cleanup; Failed is terminal.
- Scheduled errors are consumed without automatic retry, events, or rollback.
  Host output saturation is terminal.
- Stable Rust and runner images float. Source/document and conditional ADR
  checks remain outside CI.

## Important unresolved decisions

Autonomous v0.1 entry-point/architecture review remains to be completed,
including disposition of caller-scoped application and clock identity.
APIs and local grammar are unfrozen; a major API freeze still needs approval.
MSRV, message/lifecycle configuration access, broader events, external I/O,
hardware, RTOS, and no_std remain open; no scope expansion is approved here.

## Most likely next tasks

1. Complete the documented autonomous host entry-point/architecture review,
   record the exact revision and dispositions, then select the next host
   engineering increment. Consequential scope changes retain their own gates.
2. Use review findings to select a distinct bounded implementation or hardening
   objective; retain existing verified behavior and accurate limitations.

## Latest run

2026-10-08 interactive direction: routine host acceptance is delegated to
autonomous review. Current gates and nightly instructions are reconciled;
initial/final locked baselines pass 141 tests. Actual review is future
engineering work, not a request for human acceptance. Source/document/diff
review passes; publication and exact-head hosted CI are pending.
