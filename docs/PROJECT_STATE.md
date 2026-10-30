# Project state

Last updated: **2026-10-31**

## Development branch

The user's 2026-10-31 direction selects `main` for development and verified
ordinary source pushes. The clean checkout fast-forwards the existing main
history to 74b1b2398accb1435416f89bd38b4c1dfee762ce without discarding commits.
Current guidance, automation and CI push trigger now use main; historical branch
evidence remains unchanged. Library source and consumer bytes are intact. Local
locked baseline (217 tests), reviewed actionlint and document/source checks pass;
ordinary main publication and exact hosted inspection remain pending.

## Current milestone

Reviewed experimental host v0.1 remains the regression baseline. Stage 5 advances
practical library use under ADR-0025. RFF-LIB-REQ-009/ADR-0034 add one selected
subscribed host publication so a consumer can retry rejected destinations after
partial fan-out. Package 0.0.0 remains unpublished and APIs provisional.

## Verified baseline

- Clean input 986ffd82a7a909d10bc2e6cbd50875c98410e2e4 matches refreshed origin.
  Live exact run 40694990149/job 122839265392 passes all 20 steps, 208 workspace
  tests, separate 14 adapter/five sample tests and the unchanged 222-line sample.
  Root raw logs and connector step inspection agree.
- Fresh initial/final locked format/check/Clippy/test/warnings-denied rustdoc
  and both whitespace checks pass, with 208/217 workspace tests. Nine focused
  tests and identical independent consumer check/Clippy/nine tests pass, with
  matching bytes/hash and sole path-dependency metadata. Complete root and
  independent Codex source/document/evidence/diff review passes with no findings.
- Published implementation f57819985c3f0320f992c9b237324edc83874733 passes exact
  hosted run 41061628585/job 124082536243: all 20 steps, 217 workspace/14 adapter/
  five sample tests and unchanged 222-line sample. Raw logs and connector steps
  agree; hosted tools/image match input.
- Local Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy 0.1.98 are unchanged.
  Input hosted tools are 1.99.1, rustfmt 1.10.0-stable/Clippy 0.1.99, runner
  2.338.0/windows-2025-vs2026 image 20261021.278.1. Local checks do not claim
  execution on hosted tools. Current evidence is retained in
  [the focused review](verification/SELECTIVE_PUBLICATION_REVIEW.md).

## Current architecture

One unpublished package owns synchronous LC1 lifecycle/work, bounded inbox
routing/dispatch, manual time, finite one-shot scheduling, bounded events and
optional configuration with immutable work visibility/one-use rollback. All work
shares selected identity/context construction. PeriodicWork supplies one
observed-time release for host delegation. EventFilter applies optional severity
admission; failure reports remain unfiltered. Host-owned examples coordinate
bounded work output and partial event writes. Selective publication adds one
subscription-checked attempt through the existing standalone/runtime message
owners, with no report allocation, retry history or callback-context expansion.

## Work in progress

RFF-LIB-REQ-009/ADR-0034 precede source changes. Implementation, public-only
consumer and identical independent-package verification pass. Complete root and
independent Codex source/document/evidence/diff review passes. Ordinary source
publication and exact implementation hosted inspection pass. This evidence-only
completion receives its own complete review, publication and exact hosted check.
No unfinished implementation or further capability remains in this work item.

## Highest risks and uncertainties

- Controlled consumers do not establish external adoption or broad usability.
  Retry flags represent enqueue acceptance only; later stop/failure can discard
  accepted copies. Repeated calls can duplicate. No fairness or drain deadline.
- Application/clock origin and slot pairing remain caller duties. Logical bounds
  do not bound process memory, allocation, blocking or callback time. Panics,
  hangs, allocator exhaustion and process failure remain uncontained.
- Finite models prove frozen domains. Hosted tools/images float; source/doc
  review and conditional probes remain outside the CI job.

## Important unresolved decisions

Major API freeze, unsafe/runtime/code-generation/dependency/security decisions,
platform commitments, identity/licences, tags/releases/crate publication and
deployment retain separate gates. MSRV, external I/O protocols, hardware, RTOS
and no_std remain undecided. Codex review is not human acceptance or qualification.

## Most likely next task

Assess concrete multi-producer coordination or callback-side feedback needs
before selecting another API or host policy. Do not add a generic retry service
or further model machinery without a demonstrated consumer need.

## Latest run

2026-10-31: selected one subscribed-destination publication over blind whole-topic
retry, separate destination topics and subset APIs. Input local/hosted baseline
passes. Final local baseline and identical retry-consumer verification pass;
complete review passes. Ordinary source publication and exact implementation
hosted verification pass; evidence-only completion receives its own hosted
inspection, then the run stops.
