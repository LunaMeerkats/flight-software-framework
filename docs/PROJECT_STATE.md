# Project state

Last updated: **2026-11-01**

## Current milestone

Stage 5 practical library use under ADR-0025. RFF-LIB-REQ-010/ADR-0035 extend
selective publication into the publish-only message callback context, enabling
an application-owned reply ledger across explicit retry commands. Implementation
and local verification are complete; publication/hosted inspection remains.
Reviewed experimental host v0.1 remains the regression baseline; package 0.0.0
is unpublished and APIs provisional.

## Verified baseline

- Clean main input 3d547b75cdae424b123927fde09a915299b940a8 matches fetched
  origin/main. The 2026-10-31 branch transition is complete; work stays on main.
- Fresh initial/final locked format/check/Clippy/test/warnings-denied rustdoc and both
  whitespace checks pass, with 217/224 workspace tests. The source audit finds 53
  Rust files, 23,070 physical/1,355 comment lines, no width findings and three
  existing expectations. All relative links and 188 exact test references resolve.
- Exact input hosted run 41136402063/job 124339823419 passes all 20 steps,
  217 workspace/14 adapter/five sample tests and the unchanged 222-line sample.
  Root raw logs and connector step summaries agree. Local Rust/Cargo 1.98.0,
  rustfmt 1.9.0-stable/Clippy 0.1.98 remain separate from hosted 1.99.1,
  rustfmt 1.10.0-stable/Clippy 0.1.99. Runner 2.338.2 uses image
  windows-2025-vs2026 20261030.294.1.

## Current architecture

One unpublished package owns synchronous LC1 lifecycle/work, bounded inbox
routing/dispatch, manual time, finite one-shot scheduling, bounded events and
optional configuration with immutable work visibility/one-use rollback. Periodic
releases remain host-owned. Event filtering and host output examples preserve
existing bounds. Selective publication reuses checked membership/subscription and
bounded enqueue; callbacks use an immutable pre-dispatch lifecycle snapshot.
There is no automatic retry, nested dispatch or lifecycle mutation in callbacks.

## Work in progress

The additive callback method, seven public-only consumer tests and identical
independent-package check/Clippy/test/metadata/hash verification pass. Source and
consumer review passes; complete document/diff and hosted inspection follow.
See [the focused record](verification/CALLBACK_PUBLICATION_REVIEW.md).

## Highest risks and uncertainties

- Controlled consumers do not establish external adoption. Retry flags record
  enqueue, not processing; later stop/failure may discard accepted copies.
  Repeated attempts can duplicate; callback effects are nontransactional.
- Application/clock origin and slot pairing remain caller duties. Logical bounds
  do not bound process memory, allocation, blocking or callback time. Panics,
  hangs, allocator exhaustion and process failure remain uncontained.
- Finite models cover frozen domains. Hosted tools/images float; source/document
  review and conditional probes remain outside the CI job.

## Important unresolved decisions

Major API freeze, unsafe/runtime/code-generation/dependency/security decisions,
platform commitments, identity/licences, tags/releases/crate publication and
deployment retain separate gates. MSRV, external I/O protocols, hardware, RTOS
and no_std remain undecided. Codex review is not human acceptance or qualification.

## Most likely next task

Reassess a concrete consumer friction after this reply-owned retry checkpoint;
consider bounded retry cancellation/lifecycle retention needs before adding a
coordinator or another service. Do not infer a generic retry system.

## Latest run

2026-11-01: baseline and live input hosted inspection pass. Recorded requirement,
ADR and plan before implementation; callback selective-publication increment
passes local and independent consumer checks on main. Publication follows review.
