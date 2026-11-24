# Project state

Last updated: **2026-11-24**

## Current milestone

Stage 5 practical library use under ADR-0025. RFF-LIB-REQ-020 revision 1
adds opt-in structured diagnostics for returned message-callback errors.
The reviewed host v0.1 milestone remains the regression baseline. Package
0.0.0 remains unpublished with provisional APIs.

## Verified baseline

- Clean input `151511aec980dffe41dc25ab5d406798437459ee` matches fetched
  origin/main. Exact input hosted run 45135377234 passes all 20 steps.
- Pre-edit locked format/check/Clippy, 304 workspace tests, warnings-denied
  rustdoc and whitespace pass. Final locked baseline passes 311 tests and
  all corresponding static/documentation checks.
- Seven focused public-API histories also pass in an independent library
  consumer with exact source bytes/SHA256 and verified metadata.
- Implementation `420d5e342a9096a878dfea57bc69be0d4830fa2a` reached
  origin/main by fast-forward. Exact hosted run 45329058609 passes all 20
  steps, 311 workspace tests and separate adapter/sample checks. All 222
  combined sample stdout lines match input.
- Local tools remain Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy
  0.1.98. Input hosted tools remain Rust/Cargo 1.100.0, rustfmt
  1.11.0-stable, Clippy 0.1.100, runner 2.338.2 and Windows image
  `20261116.315.1`. No toolchain or policy change is selected.

## Current architecture

One unpublished package owns synchronous lifecycle/work, bounded messaging,
manual time, finite schedules, events and optional validated configuration.
Hosts select callback and I/O turns. Opt-in work and message-error reporting
borrows time/event services after owner-specific lifecycle/inbox cleanup.
Shared application lookup supports observations and application-defined
effects; application-owned output handoff remains consumer policy.

## Work in progress

Implementation and local/independent-consumer verification are complete.
Root and independent Codex accept the complete source/document/diff and
the exact implementation hosted result passes. The
[focused record](verification/MESSAGE_FAILURE_EVENTS_REVIEW.md) preserves
actual evidence and limits. This record-only completion receives its own
review and exact hosted check.

## Highest risks and uncertainties

- Events can saturate; caller-retained retries need a separate finite bound.
- Callback/clock/sink turns must return. Logical bounds do not bound arbitrary
  allocation or execution time, and acceptance does not prove delivery.
- Message failure clears self/tail deliveries while accepted peer effects
  persist; it supplies no transaction rollback or recovery.
- IDs remain runtime-local, with equal-slot cross-owner aliasing possible.

## Important unresolved decisions

MSRV and external I/O protocols remain undecided. Release, platform,
security, major API freeze and other consequential decisions retain gates.

## Most likely next task

Assess a runnable command consumer using message-error diagnostics and
bounded host output before adding more callback wrappers or services.

## Latest run

2026-11-24. Message-callback failure diagnostics are implemented, locally
and independently verified, reviewed and published. Exact implementation
CI passes all 20 steps. Stop at this bounded checkpoint.
