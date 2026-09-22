# Project state

Last updated: **2026-09-23**

## Current milestone

Stages 1 through 3 and the first source-quality checkpoint are complete.
Stage 4 has sample, hosted CI, dependency/scope, constructor/resource, returned-
message failure, schedule, and application-identity evidence. The current
checkpoint executes the detached message-topology consequence: a same-position
key from a different issuer passes positional validation and addresses the
configured inbox. Broader contract review and human v0.1 architecture
acceptance remain open.

## Verified baseline

- Started clean at `3041407580c09236adbc46ad05819c66984b2c9e`, equal to
  refreshed `origin/codex/nightly`; its hosted run 35658357984 succeeded.
  Initial locked local Cargo baseline: 126 tests.
- Final locked local baseline: 127 tests; focused message-bus target: nine.
  Formatting, all-target check, warnings-denied Clippy/rustdoc, and whitespace
  checks pass without new exceptions.
- Local Rust/Cargo: 1.98.0; rustfmt: 1.9.0-stable; Clippy: 0.1.98.
  Final source audit: 32 Rust files with zero width findings, no block comments,
  and three unchanged expectations; 46 Markdown files, 200 resolving relative
  links, and 91 exact traceability test references.
- Rendered HTML for all Markdown files passes heading, table, link, and source-
  content checks. The changed-document comparison preserves exact text,
  headings, lists, inline/fenced code, and tables. This structural inspection is
  not pixel-level visual acceptance.
- Complete source and diff review found no blocking defect. The
  [detached-message identity review](verification/MESSAGE_IDENTITY_SCOPE_REVIEW.md)
  distinguishes executed alias behavior from caller authorization and records
  the integrated owner's topology-assignment mitigation.
- Publication and exact-revision hosted CI for this checkpoint remain to be
  completed. The starting revision's exact hosted evidence remains valid only
  for that earlier revision.

## Current architecture

One unpublished package owns synchronous LC1 lifecycle/work, bounded inbox
routing/dispatch, manual time and one-shot scheduling, bounded events, and
optional runtime configuration with immutable work visibility and one-use
rollback. The private shared-source host sample composes these services.
This increment adds one public detached-message regression and supporting
records only; production code and API remain unchanged.

## Work in progress

No unfinished implementation remains. The detached-message identity checkpoint
is locally verified and awaiting ordinary fast-forward publication plus exact-
revision hosted CI.

## Highest risks and uncertainties

- `ApplicationId` encodes only a record position. Equal-position keys from
  separate live owners compare equal and can invoke or transition the receiving
  owner's local application or address a detached inbox. Correct issuer pairing
  remains caller discipline.
- Adding issuer provenance needs a bounded origin source and explicit equality,
  exhaustion, persistence, and public-API decisions; no redesign is approved by
  this evidence checkpoint.
- Impossible-capacity rejection is verified; actual allocator exhaustion and
  allocation counts are not. Logical limits do not bound whole-process bytes.
- Callback/clock panics and hangs remain outside containment. Host saturation
  is terminal; no recovery, real-time, or physical delivery claim is made.
- Stable Rust and runner images float. Source/document and conditional ADR
  checks remain outside CI; an empty Cargo graph does not audit the toolchain.

## Important unresolved decisions

Human v0.1 acceptance remains pending, including whether caller-scoped identity
is sufficient before API stabilization. APIs and local grammar are unfrozen.
MSRV, message/lifecycle configuration access, broader events, external I/O,
hardware, RTOS, and no_std remain open; no scope expansion is approved here.

## Most likely next tasks

1. Select another bounded Stage 4 service or public-API gap after reconciling
   existing evidence.
2. Record human entry-point/architecture acceptance before broadening scope.

## Latest run

2026-09-23: nine message-bus and 127 workspace tests pass. The new regression
executes same-position foreign-identity acceptance and inbox access through the
detached topology. No production defect, API change, dependency, or lint-policy
change was needed. Publication and hosted evidence remain pending.
