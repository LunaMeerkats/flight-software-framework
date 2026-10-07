# Project state

Last updated: **2026-10-08**

## Current milestone

Stages 1 through 3 and the first source-quality checkpoint are complete.
Stage 4 has sample, hosted CI, dependency/scope, resource, failure, schedule,
identity, diagnostics, and ownership evidence. This run strengthens callback
publication's availability refresh after a stopped peer restarts.
Broader contract review and human v0.1 architecture acceptance remain open.

## Verified baseline

- Started clean at `b53f680ad938da6913cb774b204816902074322a`, equal to
  refreshed `origin/codex/nightly`; exact hosted run 37469088813 succeeded.
- Initial and final locked local baselines pass 141 tests. Formatting,
  all-target check, warnings-denied Clippy/rustdoc, and whitespace pass.
  Focused message-dispatch target passes six tests. One existing test is
  strengthened; the test count does not increase.
- Local Rust/Cargo: 1.98.0; rustfmt: 1.9.0-stable; Clippy: 0.1.98.
- Hosted tools: Rust/Cargo 1.99.0, rustfmt 1.10.0-stable,
  Clippy 0.1.99. The [source-policy re-audit](verification/CI_BASELINE.md)
  retains existing format, width, function-size, and waiver policy.
- Final audit passes 33 Rust files and 12,666 physical lines with zero width
  findings and three unchanged fulfilled expectations. All 291 relative links
  across 60 Markdown files and 105 traceability function references resolve.
  Six changed documents pass rendered HTML structure/content inspection;
  author/independent source/diff review pass. No pixel-level acceptance is claimed.
- Checkpoint `44709878005d8b91df8411f9e2ccc8179d21ddde` is published.
  [Hosted run 37626284281](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/37626284281)
  passes the exact logged checkout and all 20 reported steps, including 141
  workspace tests, focused adapters/sample, documentation, and the executed
  sample. Tools/runner match the prior checkpoint; no policy adaptation.

## Current architecture

One unpublished package owns synchronous LC1 lifecycle/work, bounded inbox
routing/dispatch, manual time and one-shot scheduling, bounded events, and
optional runtime configuration with immutable work visibility and one-use
rollback. The private shared-source host sample composes these services.
This checkpoint changes tests and records only; production behavior and public
API remain as accepted in ADR-0011/0012.

## Work in progress

The [dispatch availability review](verification/DISPATCH_AVAILABILITY_REFRESH_REVIEW.md)
extends the existing stopped-peer dispatch scenario through successful restart
and a third publisher callback. It checks exact delivered/unavailable/delivered
reports, new-only peer reply consumption, and no empty-inbox callback.
No unfinished implementation remains. Local acceptance, publication, and
exact-checkpoint hosted verification pass. This documentation follow-up
records that result; later revisions need their own CI.

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

Human v0.1 acceptance remains pending, including caller-scoped application and
clock identity before API stabilization. APIs and local grammar are unfrozen.
MSRV, message/lifecycle configuration access, broader events, external I/O,
hardware, RTOS, and no_std remain open; no scope expansion is approved here.

## Most likely next tasks

1. Reconcile remaining Stage 4 contracts against existing tests and probes,
   choosing another distinct bounded gap only where evidence warrants it.
2. Record human entry-point/architecture acceptance before broadening scope.

## Latest run

2026-10-08: the strengthened callback-availability regression observes a peer
as running, stopped, and restarted, with only the new reply consumed after
restart. Focused six-test and full 141-test locked baselines pass without a new
lint exception or production change. Source/document/diff review, publication,
and exact-checkpoint hosted verification pass.
Human v0.1 acceptance remains open.
