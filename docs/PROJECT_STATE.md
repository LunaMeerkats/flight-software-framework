# Project state

Last updated: **2026-11-02**

## Current milestone

Stage 5 practical library use under ADR-0025. RFF-LIB-REQ-011/ADR-0036 add an
explicit cancellation command to the public callback consumer so a permanently
Failed subscriber need not occupy its reply slot indefinitely. Local and
independent consumer verification and complete Codex review pass; hosted checks
follow.
Reviewed experimental host v0.1 remains the regression baseline. Package 0.0.0
is unpublished and APIs provisional.

## Verified baseline

- Clean main input 1972d66f20d08ab3590e84035073bc4d015b35b9 matches fetched
  origin/main. Exact input run 41204977193/job 124548108020 passes all 20 steps,
  224 workspace/14 adapter/five sample tests and the 222-line host sample.
- Initial/final locked format/check/Clippy/test/warnings-denied rustdoc and both
  whitespace checks pass, with 224/229 workspace tests. The focused consumer
  passes all 12 tests, including five new cancellation regressions.
- Identical independent Cargo consumer check/Clippy/test pass; metadata confirms
  one workspace member and sole framework path dependency, with matching bytes
  and SHA256. The library still has no dependencies. See the focused record.
- Local Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy 0.1.98 are unchanged.
  Input hosted Rust/Cargo 1.99.1, rustfmt 1.10.0-stable/Clippy 0.1.99 and runner
  2.338.2/image windows-2025-vs2026 20261021.278.1 remain separate evidence.

## Current architecture

One unpublished package owns synchronous LC1 lifecycle/work, bounded inbox
routing/dispatch, manual time, finite one-shot scheduling, bounded events and
optional configuration with immutable work visibility/one-use rollback. Periodic
releases remain host-owned. Selective callback publication uses checked routing
and the pre-callback lifecycle snapshot. Consumer Cancel clears only its current
pending reply; already accepted copies remain owned by their destinations.
No library service, public API, dependency or workflow changes in this increment.

## Work in progress

Five new tests cover cancellation, peer retention, failed destinations, command
backpressure and producer restart. Existing seven callback test bodies remain
unchanged. Root and independent cancellation_review accept the complete 13-file
source/document diff with no findings. Width/waiver/link/test-reference audits and
all 12 rendered documents pass content/structure inspection. Publication and
exact hosted checks follow.
See [the focused record](verification/CALLBACK_CANCELLATION_REVIEW.md).

## Highest risks and uncertainties

- Cancel has no operation ID or acknowledgement; stale commands can cancel a
  newer reply. Enqueue does not prove dispatch. Stop may discard queued Cancel
  while retaining application-owned reply state; terminal Failed owners cannot
  process commands. Host command ordering and resubmission remain explicit.
- Retry flags record enqueue, not processing. Cancellation cannot retract
  copies, and later lifecycle clearing may discard them. Effects are
  nontransactional. Controlled consumers do not establish external adoption.
- IDs/clocks remain caller-paired. Logical bounds do not bound process memory,
  blocking or callback time. Panics, hangs and allocator/process failures remain
  uncontained. Floating hosted tools/images require exact-revision evidence.

## Important unresolved decisions

Major API freeze, unsafe/runtime/code-generation/dependency/security decisions,
platform commitments, identity/licences, tags/releases/crate publication and
deployment retain separate gates. MSRV, external I/O protocols, hardware, RTOS
and no_std remain undecided. Codex review is not human acceptance or qualification.

## Most likely next task

Assess a concrete consumer need for command-result feedback or correlation
before adding another protocol/service. Preserve this fixed consumer's explicit
cancellation scope; do not infer general reliable command delivery.

## Latest run

2026-11-02: initial/final baseline, 12 focused/identical independent consumer
checks and complete root/independent Codex review pass. Exact hosted results are
recorded as they finish; no later behavior selected.
