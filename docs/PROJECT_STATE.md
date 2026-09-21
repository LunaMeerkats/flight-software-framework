# Project state

Last updated: **2026-09-22**

## Current milestone

Stages 1 through 3 and the first source-quality checkpoint are complete.
Stage 4 has sample, hosted CI, dependency/scope, constructor/resource, returned-
message failure, and schedule evidence. The current checkpoint makes the
caller-scoped `ApplicationId` boundary executable: a same-position key from a
different owner selects the receiving owner's local record. Broader contract
review and human v0.1 architecture acceptance remain open.

## Verified baseline

- Started clean at `a46ad83078a52609b4a75eabcdbec68ff69d0351`, equal to
  refreshed `origin/codex/nightly`; its hosted run 35534913845 succeeded.
  Initial locked local Cargo baseline: 124 tests.
- Final locked local baseline: 126 tests; focused lifecycle-registry and
  application-runtime targets: six and twelve. Formatting, all-target check,
  warnings-denied Clippy/rustdoc, and whitespace checks pass without new
  exceptions.
- Local Rust/Cargo: 1.98.0; rustfmt: 1.9.0-stable; Clippy: 0.1.98.
  Final source audit: 32 Rust files with zero width findings, no block comments,
  and three unchanged expectations; 45 Markdown files, 194 resolving relative
  links, and 90 exact traceability test references.
- Rendered HTML for all Markdown files passes heading, table, link, and source-
  content checks. The changed-document comparison preserves exact text,
  headings, lists, inline/fenced code, and tables. This structural inspection is
  not pixel-level visual acceptance.
- Complete source and diff review found no blocking defect. The
  [identity scope review](verification/APPLICATION_ID_SCOPE_REVIEW.md)
  distinguishes executed alias behavior from caller authorization and from an
  origin-bearing identity design.

## Current architecture

One unpublished package owns synchronous LC1 lifecycle/work, bounded inbox
routing/dispatch, manual time and one-shot scheduling, bounded events, and
optional runtime configuration with immutable work visibility and one-use
rollback. The private shared-source host sample composes these services.
This increment changes two public integration tests and supporting records
only; production code and API remain unchanged.

## Work in progress

The application-identity scope checkpoint is locally complete and reviewed.
Commit, ordinary fast-forward publication, and exact-revision hosted CI evidence
remain to be completed and recorded.

## Highest risks and uncertainties

- `ApplicationId` encodes only a record position. Equal-position keys from
  separate live owners compare equal and can invoke or transition the receiving
  owner's local application. Correct issuer pairing remains caller discipline.
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

1. Complete and record exact-revision publication evidence for this checkpoint.
2. Then select another bounded Stage 4 service or public-API gap, or record
   human entry-point/architecture acceptance before broadening scope.

## Latest run

2026-09-22: six lifecycle-registry, twelve application-runtime, and 126
workspace tests pass. The two new regressions execute same-position identity
aliasing through the logical and owned callback boundaries. No production
defect, API change, dependency, or lint-policy change was needed.
