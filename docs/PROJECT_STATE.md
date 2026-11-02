# Project state

Last updated: **2026-11-03**

## Current milestone

Stage 5 practical library use under ADR-0025. RFF-LIB-REQ-012/ADR-0037 add a
separate correlated-command consumer: delayed Cancel/Retry affect only matching
pending operations. Local and identical independent consumer checks pass;
root/independent complete review passes; publication remains pending. The original
callback consumer and its 12 tests are unchanged. Reviewed experimental host
v0.1 remains the regression baseline; package 0.0.0 is unpublished/provisional.

## Verified baseline

- Clean main input d56441235dc8670a14c8a54fadd4df4abeb6c9f3 matches fetched
  origin/main. Exact input run 41310258543/job 124858954952 passes all 20 steps:
  229 workspace/14 adapter/five sample tests and unchanged 222-line sample.
- Initial/final locked format/check/Clippy/test/warnings-denied rustdoc and both
  whitespace checks pass, with 229/236 workspace tests. Seven focused tests pass.
- Identical independent Cargo consumer check/Clippy/seven tests plus generated
  smoke test pass. Source bytes/SHA256 match; metadata confirms one independent
  workspace member and sole framework path dependency, with no library deps.
- Local Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy 0.1.98 are unchanged.
  Input hosted Rust/Cargo 1.99.1, rustfmt 1.10.0-stable/Clippy 0.1.99, runner
  2.338.2 and windows-2025-vs2026 image 20261021.278.1 remain separate evidence.

## Current architecture

One unpublished package owns synchronous LC1 lifecycle/work, bounded routing,
manual time, finite schedules, events and optional validated configuration.
Host-owned recurring releases delegate to existing work owners. Callback selective
publication uses checked routing and a pre-callback lifecycle snapshot. The new
consumer owns one operation/two acceptance flags and compares session IDs before
control effects; one host issuer rejects exhaustion after IDs 1..=255. No
framework API, service, dependency, workflow or policy configuration changes.

## Work in progress

Seven regressions cover stale controls with spare capacity, exact cancellation/
FIFO retention, busy resubmission, restart, all finite IDs and malformed commands.
Whole-tree width/waiver/link/test-reference and rendered content/structure checks
pass. Root and independent verification_support accept the complete 13-file diff
with no findings. Main publication/hosted inspection remain pending. See
[the focused record](verification/CORRELATED_COMMANDS_REVIEW.md).

## Highest risks and uncertainties

- One issuer must survive producer restart and all saved session requests;
  bypass/reset can alias IDs. No durable epoch, authentication, acknowledgement
  or duplicate-New suppression. Exhaustion requires stopping new issuance.
- Acceptance means enqueue, not processing; cancellation cannot retract copies.
  Ordinary command saturation/FIFO/clearing still apply. Controlled consumers
  do not establish external adoption, stability or reliable command delivery.
- Framework IDs/clocks remain caller-paired. Logical bounds do not bound process
  memory or callback time; panics/hangs/process failures remain uncontained.
  Floating hosted tools/images require exact-revision evidence.

## Important unresolved decisions

Major API freeze, unsafe/runtime/code-generation/dependency/security decisions,
platform commitments, identity/licences, tags/releases/crate publication and
deployment retain separate gates. MSRV and external I/O protocols remain undecided.
Codex review is not human acceptance or qualification.

## Most likely next task

Assess a concrete need for host command-result feedback before adding a bounded
acknowledgement path. Session persistence or multiple issuers need a separate
consumer contract; do not extend IDs or add a service without demonstrated need.

## Latest run

2026-11-03: seven focused/identical independent consumer tests and the final
236-test locked baseline pass. Complete root/independent review accepts the frozen
source for authorized main publication; no unrelated work is selected.
