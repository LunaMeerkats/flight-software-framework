# Project state

Last updated: **2026-10-27**

## Current milestone

Reviewed experimental serial host v0.1 remains the regression baseline. Stage 5
advances practical library use under ADR-0025. RFF-LIB-REQ-006 adds explicit
minimum-severity filtering for optional events, retaining unfiltered emission
and runtime failure reporting. Package 0.0.0 stays unpublished; APIs provisional.

## Verified baseline

- Clean input 1759cbe2139220126fe4030c9b1d2b39f7745883 matches refreshed
  origin/codex/nightly. Live exact input run 40195931334/job 121176634812 passes
  all 20 steps, 186 workspace tests, separate 14 adapter/five sample tests and
  the unchanged 222-line sample. Root raw-log and connector inspection agree.
- Initial/final locked format/check/Clippy/test and warnings-denied rustdoc pass
  186/193 tests. Seven focused and seven identical independent tests pass, as
  do external check/Clippy, actual metadata and matching source hashes. The
  guide snippet passes one standalone Markdown rustdoc test.
- Published implementation db994e776c68fb5432e646afb2ae5344cb2444e5 passes
  exact hosted run 40329281385/job 121594066549: all 20 steps, 193 workspace
  tests, separate 14 adapter/five sample tests and the unchanged 222-line sample.
  Hosted tools/runner/image match input. Complete Codex source/diff review passes.
- Local Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy 0.1.98 are unchanged.
  Input hosted tools are Rust/Cargo 1.99.1, rustfmt 1.10.0-stable/Clippy 0.1.99,
  runner 2.338.0/windows-2025-vs2026 image 20261021.278.1. Local results do not
  claim execution on that hosted toolchain. Actual results and limitations are
  in [the focused record](verification/EVENT_FILTERING_REVIEW.md).

## Current architecture

One unpublished package owns synchronous LC1 lifecycle/work, bounded inbox
routing/dispatch, manual time, finite one-shot scheduling, bounded events and
optional runtime configuration with immutable work visibility/one-use rollback.
All ordinary work shares selected identity/context construction. PeriodicWork
supplies at most one observed-time release for explicit host work delegation.
EventFilter retains one copied severity; explicit filtered emission omits
excluded records before capacity inspection and delegates admitted records to
existing bounded FIFO emission. The queue retains no filter or admission history.

## Work in progress

RFF-LIB-REQ-006/ADR-0031 precede source changes. Production API, public-only
consumer tests, final locked baseline and actual independent Cargo verification
are complete. Root and independent filter_review accept the complete source,
documents, full diff and actual evidence with no remaining findings. Source
publication and exact implementation hosted inspection pass. This evidence-only
completion receives its own review, source publication and exact hosted check;
no unfinished implementation or further capability remains in this work item.

## Highest risks and uncertainties

- Controlled consumers do not prove adoption, broad usability or API stability.
  Optional omission differs from acceptance-before-effect; mission policy must
  choose deliberately. Admitted events can still saturate and omit later errors.
- Application/clock origins and positional inbox pairing remain caller duties.
  Identity is not authenticated, persistent or origin-checked. Logical bounds
  do not bound process memory, callback allocation or time.
- Periodic release is not execution. Hosts own work/error/timer policy, polling
  order and output drain; there is no fixed phase, backlog, fairness or deadline
  guarantee. Unfiltered failure reporting bypasses the optional admission policy.
- Panics/hangs and allocator exhaustion are uncontained. Cooperative cleanup
  does not prove external cleanup. In-process output is not physical delivery.
  Finite models prove their frozen domains. Hosted tools/images float; manual
  reviews and conditional probes remain outside the CI job.

## Important unresolved decisions

Major API freeze, unsafe/runtime/code-generation/dependency/security decisions,
platform commitments, identity/licences, tags/releases/crate publication and
deployment retain separate gates. MSRV, broader service contexts, external I/O,
hardware, RTOS and no_std remain undecided. Codex review does not establish
human review or operational suitability.

## Most likely next task

Reassess host integration friction after optional-event admission. A consumer
that needs a bounded event-drain/diagnostic presentation boundary or explicit
host work/dispatch coordination is a candidate. Establish need and compare
mission composition before adding policy, queues or service contexts.

## Latest run

2026-10-27: selected and implemented one explicit event-admission increment.
Initial/final locked baselines pass 186/193; focused/identical external tests
pass seven each, with metadata/hash inspection and one guide doctest. Existing
unfiltered contracts and prior tests are retained. Complete autonomous review,
ordinary publication and exact implementation hosted inspection pass. The
documentation completion receives its own exact hosted check, then the run
stops without beginning another capability.
