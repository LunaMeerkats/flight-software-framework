# Project state

Last updated: **2026-11-25**

## Current milestone

Stage 5 practical library use under ADR-0025. RFF-LIB-REQ-014 revision 7
integrates structured message-failure diagnostics into the runnable command
session. The reviewed host v0.1 milestone remains the regression baseline.
Package 0.0.0 remains unpublished with provisional APIs.

## Verified baseline

- Clean input `d252d50d77230de7299ed3ed20d6d4fcda5f0d27` matches fetched
  origin/main. Exact input hosted run 45329451567 passes all 20 steps.
- Pre-edit locked format/check/Clippy, 311 workspace tests, warnings-denied
  rustdoc and whitespace pass; command session and combined sample run.
- Final locked baseline passes 316 tests and all static/documentation
  checks. All 24 focused command tests also pass in the identical independent
  binary; metadata, source bytes/hash and stdout match. The exact prior
  51-line transcript is preserved as a prefix. Combined sample is unchanged.
- Implementation `04521873273b581a3e6b529ff3e5a14f8e4aff34` reached
  origin/main by fast-forward. Exact hosted run 45555614969 passes all 20
  steps, 316 tests and separate adapter/sample checks at that checkout.
  The combined sample matches all 222 input stdout lines.
- Local tools remain Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy
  0.1.98. Input hosted tools remain Rust/Cargo 1.100.0, rustfmt
  1.11.0-stable, Clippy 0.1.100, runner 2.338.2 and Windows image
  `20261116.315.1`. No toolchain or policy change is selected.

## Current architecture

One unpublished package owns synchronous lifecycle/work, bounded messaging,
manual time, finite schedules, events and optional validated configuration.
Hosts select callback and I/O turns. The command consumer now composes
opt-in message-error reporting with one event FIFO slot, one rejected
terminal producer event and one diagnostic output slot. Explicit retry,
staging and bounded writes leave healthy subscribers selectable. Library
APIs, ownership and cleanup policies remain unchanged by this consumer.

## Work in progress

No implementation remains. Root and independent Codex accept the complete
source/document/diff review and exact implementation hosted evidence. The
[focused follow-up](verification/COMMAND_SESSION_REVIEW.md#message-failure-diagnostics-follow-up)
records actual evidence, including passing source/link/width and all 12
rendered-document reviews. Evidence-only completion receives separate
review and exact hosted inspection; its final result is retained in
automation memory.

## Highest risks and uncertainties

- One terminal producer failure bounds diagnostic retention. More reporting
  roles or recovery need a new bound and admission policy.
- Callback/clock/writer turns must return. Logical bounds do not bound
  arbitrary allocation or execution time; acceptance does not prove delivery.
- Failed command callbacks clear selected queued work while accepted peer
  replies persist; no transaction rollback or recovery is supplied.
- IDs remain runtime-local, with equal-slot cross-owner aliasing possible.

## Important unresolved decisions

MSRV and external I/O protocols remain undecided. Release, platform,
security, major API freeze and other consequential decisions retain gates.

## Most likely next task

Assess the next concrete consumer integration need, including whether
lifecycle startup/shutdown diagnostics merit a small opt-in owner path.
Avoid extending retry machinery without a demonstrated use.

## Latest run

2026-11-25. Command-session diagnostics are implemented, locally and
independently verified, reviewed and published. Exact implementation CI
passes all 20 steps. Stop at this bounded checkpoint.
