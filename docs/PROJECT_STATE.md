# Project state

Last updated: **2026-11-04**

## Current milestone

Stage 5 practical library use under ADR-0025. RFF-LIB-REQ-013/ADR-0038 add
bounded host command-result feedback to the correlated consumer. The serial host
distinguishes local command effects with one result slot and gated dispatch.
Reviewed host v0.1 remains the regression baseline; package 0.0.0 stays unpublished
and provisional. The original callback consumer remains unchanged.

## Verified baseline

- Clean main input 4f64a4e70037609c6f382b20ff6c13967a54e05e matches fetched
  origin/main. Initial locked format/check/Clippy/236 workspace tests,
  warnings-denied rustdoc and working/committed whitespace checks pass.
- Exact input run 41451380499/job 125297152054 passes all 20 steps:
  236 workspace/14 adapter/five sample tests and unchanged 222-line sample.
  Independent verification_review inspects raw metadata/logs and exact checkout.
- Local Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy 0.1.98 are unchanged.
  Hosted Rust/Cargo 1.99.1, rustfmt 1.10.0-stable/Clippy 0.1.99, runner
  2.338.2 and windows-2025-vs2026 image 20261021.278.1 remain separate evidence.
- Final locked baseline passes 243 workspace tests and warnings-denied rustdoc;
  14 focused tests/Clippy pass. All seven original correlation test bodies remain.
- Identical independent consumer check/Clippy/14 tests plus generated smoke pass;
  independent workspace metadata, exact bytes and SHA256 are verified.

## Current architecture

One unpublished package owns synchronous LC1 lifecycle/work, bounded routing,
manual time, finite schedules, events and optional validated configuration.
Host-owned recurring releases delegate to existing work owners. Callback selected
publication uses checked routing and a pre-callback lifecycle snapshot. The
correlated consumer retains one operation/two acceptance flags and one session
issuer. Its result mailbox adds one fixed shared allocation at construction;
producer turns pause until explicit host draining. No library API, service,
dependency, workflow or policy-configuration change is selected.

## Work in progress

Consumer implementation and local/independent package verification pass. Root
and independent verification_review accept the complete source/document diff
without findings. Whole-tree width/waiver/link/exact-test-reference and changed
rendered content/structure checks pass. Main publication/exact hosted evidence
remain pending; no unfinished source work remains.
See [the focused record](verification/COMMAND_RESULTS_REVIEW.md).

## Highest risks and uncertainties

- The result gate is serial host policy; ignoring drain blocks producer dispatch.
  Direct bypass returns a callback failure and clears queued selected commands.
  Results are local snapshots; accepted peer copies may later be cleared or fail.
- Session issuer reset/bypass can alias IDs. No durable epoch, authentication,
  duplicate suppression or reliable transport. Acceptance remains enqueue only.
- Framework identities/clocks remain caller-paired. Logical limits do not bound
  process memory or callback time; panics/hangs/process failures are uncontained.
  Floating hosted tools/images require exact-revision evidence.

## Important unresolved decisions

Major API freeze, unsafe/runtime/code-generation/dependency/security decisions,
platform commitments, identity/licences, tags/releases/crate publication and
deployment retain separate gates. MSRV and external I/O protocols remain undecided.
Codex review is not human acceptance or qualification.

## Most likely next task

Assess a runnable public-API command-session example joining ingress, explicit
result draining and reply output. A bus-result transport or persistence still
requires a concrete need; retain one bounded increment and the host baseline.

## Latest run

2026-11-04: initial/final 236/243-test locked baselines, 14 focused and identical
independent consumer tests pass. Original histories remain. Complete root and
independent review passes; publication/hosted inspection remains for this increment.
