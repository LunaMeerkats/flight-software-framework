# Project state

Last updated: **2026-10-25**

## Current milestone

Reviewed experimental serial host v0.1 remains the regression baseline. Stage 5
advances practical library use under ADR-0025. Three public-only examples and
runtime-issued ordinary-work identity remain verified. RFF-LIB-REQ-005 now adds
host-owned periodic releases with ten public-only consumer regressions.
Package 0.0.0 remains unpublished and APIs provisional.

## Verified baseline

- Clean input 86d82c9b336ba013f7e460fc309a6fb8fcfe08b6 matches refreshed
  origin/codex/nightly. Exact input run 39959863270/job 120434842528 passes
  all 20 steps; live metadata/steps/logs are inspected again.
- Initial locked format/check/Clippy/test and warnings-denied rustdoc pass all
  176 workspace tests; working-tree and committed-tip whitespace checks pass.
- Final locked baseline passes 186 workspace tests. Ten focused and ten identical
  independent-package tests pass, along with check/Clippy, actual metadata and
  matching source hashes. Full-tree widths, links and rendered-document review
  pass; root and independent Codex complete review accepts with no findings.
- Local Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy 0.1.98 are unchanged.
  Input hosted tools are Rust/Cargo 1.99.1, rustfmt 1.10.0-stable/Clippy 0.1.99,
  runner 2.338.0/windows-2025-vs2026 image 20261021.278.1. Local results do not
  claim execution on that hosted toolchain. Actual results and limitations are
  in [the focused record](verification/PERIODIC_WORK_REVIEW.md).

## Current architecture

One unpublished package owns synchronous LC1 lifecycle/work, bounded inbox
routing/dispatch, manual time, finite one-shot scheduling, bounded events and
optional runtime configuration with immutable work visibility/one-use rollback.
All ordinary work shares one selected-identity/context construction path.
PeriodicWork adds a separate inline host-polled release cursor: one due item,
observed-time re-arm, coalesced late polls and explicit overflow exhaustion.
It executes no callback; the host delegates to the existing work owner.

## Work in progress

RFF-LIB-REQ-005/ADR-0030 precede source changes. Production API and public-only
consumer tests, final locked baseline and actual independent Cargo verification
are complete. Root Codex and independent repeated_work_assessment accept the
complete source/document/diff and actual evidence with no remaining findings.
Authorized source publication and exact hosted inspection remain pending.

## Highest risks and uncertainties

- Controlled consumers do not prove external adoption, broad usability or API
  stability. Immediate ordinary-work publication remains mission composition.
- Application/clock origins and positional inbox pairing remain caller duties.
  Identity is not authenticated, persistent or origin-checked. Logical bounds
  do not bound process memory, callback allocation or time.
- Periodic release is not work execution. Future independent releases remain
  after work errors until host disposal; phase/drift, polling order, output
  drain and error policy are explicit host responsibilities. No fixed phase,
  catch-up backlog, fairness or completion-time measurement is provided.
- Panics/hangs and allocator exhaustion are uncontained. Cooperative cleanup
  does not prove external cleanup. Event saturation can omit records; retries
  remain caller policy. In-process output is not physical delivery. Finite
  models prove their frozen domains. Hosted tools/images float; manual reviews
  and conditional probes remain outside the CI job.

## Important unresolved decisions

Major API freeze, unsafe/runtime/code-generation/dependency/security decisions,
platform commitments, identity/licences, tags/releases/crate publication and
deployment retain separate gates. MSRV, broader service contexts, external I/O,
hardware, RTOS and no_std remain undecided. Codex review does not establish
human review or operational suitability.

## Most likely next task

Reassess concrete consumer friction after this release-policy increment. A
consumer needing safe explicit dispatch/recovery policy or bounded event
filtering is a candidate; establish need before adding another abstraction or
model. Fixed-phase/multi-timer execution needs separate consumer evidence.

## Latest run

2026-10-25: selected a host-owned recurring release cursor after comparing host
arithmetic, rebuilt agendas and execution wrappers. Requirement/ADR established
before implementation; source and ten consumer regressions implemented. Full
local/independent verification and complete autonomous review pass. Source
publication and exact hosted inspection remain in progress.
