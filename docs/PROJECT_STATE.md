# Project state

Last updated: **2026-09-21**

## Current milestone

Stages 1 through 3 and the first source-quality checkpoint are complete.
Stage 4 has sample, hosted CI, dependency/scope, messaging-constructor,
returned-message failure, schedule-construction, event-queue, and lifecycle-
construction evidence. The current checkpoint verifies exact reservation
failure through the standalone registry and unconfigured owned runtime.
Broader contract review and human v0.1 architecture acceptance remain open.

## Verified baseline

- Started clean at `7cabe599d88fa5a7f3ce74b2e35fac33488aa711`, equal to
  refreshed `origin/codex/nightly`; its hosted run 35466793755 succeeded.
  Initial locked local Cargo baseline: 122 tests.
- Final locked local baseline: 124 tests; focused lifecycle-registry and
  application-runtime targets: five and eleven. Formatting, all-target check,
  warnings-denied Clippy/rustdoc, and whitespace checks pass without new
  exceptions.
- Local Rust/Cargo: 1.98.0; rustfmt: 1.9.0-stable; Clippy: 0.1.98.
  Final source audit: 32 Rust files with zero width findings and three unchanged
  expectations; 44 Markdown files, 194 resolving links, and 88 exact
  traceability test references.
- Complete source/diff review found no blocking defect. GitHub GFM rendering of
  all seven changed documents preserved heading and fenced-code-block counts
  and produced nonempty linked HTML.
- The [lifecycle construction review](verification/LIFECYCLE_CONSTRUCTION_REVIEW.md)
  distinguishes executed deterministic capacity overflow from actual allocator
  exhaustion and source-inspected ownership/registration properties. Earlier
  [event](verification/EVENT_QUEUE_REVIEW.md),
  [schedule](verification/SCHEDULE_CONSTRUCTION_REVIEW.md),
  [dispatch](verification/MESSAGE_FAILURE_REVIEW.md),
  [messaging construction](verification/MESSAGING_CONSTRUCTION_REVIEW.md), and
  [dependency/scope](verification/DEPENDENCY_SCOPE_REVIEW.md) records retain
  their evidence.
- Published checkpoint `15e4166bbead2d824e732460fa1a67266386f648` has
  successful [hosted CI](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/35534767664):
  one job, all configured steps, 124 workspace tests, both focused host targets,
  and the sample executable. The lifecycle review records the actual hosted
  environment.

## Current architecture

One unpublished package owns synchronous LC1 lifecycle/work, bounded inbox
routing/dispatch, manual time and one-shot scheduling, bounded events, and
optional runtime configuration with immutable work visibility and one-use
rollback. The private shared-source host sample composes these services.
This increment changes two lifecycle-construction tests and supporting records
only; production code and API remain unchanged.

## Work in progress

No unfinished implementation remains. The lifecycle-construction checkpoint is
published with successful exact-revision CI. This documentation-only follow-up
records that result; later revisions require their own verification.

## Highest risks and uncertainties

- Impossible-capacity rejection is verified; actual allocator exhaustion and
  allocation counts are not. Logical limits do not bound whole-process bytes.
- Application identities remain caller-scoped; equal-position keys from
  different owners can alias and no origin is encoded.
- Callback/clock panics and hangs remain outside containment. Host saturation
  is terminal; no recovery, real-time, or physical delivery claim is made.
- Stable Rust and runner images float. Source/document and conditional ADR
  checks remain outside CI; an empty Cargo graph does not audit the toolchain.

## Important unresolved decisions

Human v0.1 acceptance remains pending. APIs and local grammar are unfrozen.
MSRV, message/lifecycle configuration access, broader events, external I/O,
hardware, RTOS, and no_std remain open; no scope expansion is approved here.

## Most likely next tasks

1. Select another bounded Stage 4 service resource, failure, or public-API
   contract gap after reconciling existing evidence.
2. Record human entry-point/architecture acceptance before broadening scope.

## Latest run

2026-09-21: five lifecycle-registry, eleven application-runtime, and 124
workspace tests pass. The two new regressions verify exact impossible-capacity
errors for both public lifecycle constructors. No production defect or API
change was needed; the published checkpoint's exact hosted CI passes.
