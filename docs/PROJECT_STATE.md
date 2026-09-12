# Project state

Last updated: **2026-09-12**

## Current milestone

Stages 1 through 3 and the first source-quality checkpoint are complete.
The combined host sample and its first hosted CI run now pass. Source
publication is authorized without per-push human review. Separate human v0.1
entry-point, dependency/scope, and architecture acceptance remain outstanding.

## Verified baseline

- Local required baseline passes all 115 tests on rustc/cargo 1.98.0,
  rustfmt 1.9.0-stable, and Clippy 0.1.98; rustdoc warnings are denied.
- Policy commit `abb1293790136128d5f27d8c48c1e3d98a355540` and 31 accumulated
  commits were published by ordinary fast-forward push; remote HEAD matched.
- [Hosted run 34697541024](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/34697541024)
  passed one job and all 20 steps at that exact revision: 115 workspace tests,
  14 adapter tests, five sample tests, format/check/Clippy, rustdoc, the sample,
  and both whitespace checks, including hosted checkout/bootstrap and cleanup.
- Actual image: `windows-2025-vs2026` version `20260907.229.1`, selected via
  `windows-2025`. Hosted rustc/cargo 1.98.1, rustfmt 1.9.0-stable, Clippy
  0.1.98. The selected all-target configuration passes on this newer toolchain;
  companion whole-tree review covers 32 Rust files, zero widths, and three
  unchanged fulfilled expectations. These versions are evidence, not a pin.
- No Rust, Cargo file, runtime API, dependency, unsafe code, lint threshold,
  expectation, or workflow changed during the publication follow-up.
  Source/document and complete-diff review pass; exact run evidence is in
  [the CI baseline](verification/CI_BASELINE.md).
- ADR-0018/0019 experiments are unchanged and not separately rerun. Local
  rustup installation was not run; the hosted installation step passed.

## Current architecture

One unpublished Cargo package provides finite LC1 lifecycle, synchronous work,
bounded lifecycle-owned inbox dispatch, injected manual time and one-shot
scheduling, bounded events, and constructor-owned configuration with immutable
ordinary-work visibility and consume-once rollback. The private shared-source
host sample and all runtime behavior are unchanged.

The single Windows CI job requests stable Rust and records actual versions
and checkout identity. It uses a SHA-pinned checkout, read-only contents, and
no persisted checkout credentials. Each native check has its own PowerShell step.

## Work in progress

No unfinished implementation remains. The source is published and the first
hosted baseline passed. This evidence-only follow-up records that result;
later commits and runs require separate verification.
The nightly automation now uses the standing source-publication permission
with its schedule/model/reasoning/target unchanged.

## Highest risks and uncertainties

- Stable Rust and the runner image float. Later revisions and changed toolchains
  need their own verification; earlier CI success is not transferable evidence.
- Source/document and conditional ADR-probe checks remain outside the CI job.
  Human v0.1 release/scope-expansion gates remain separate from source pushes.
- Callback/clock panics and hangs remain outside containment. Host saturation
  is terminal; there is no recovery, real-time, or physical delivery claim.
- IDs, revisions, and instants remain caller-scoped. Validators/callbacks need
  not terminate; large inline bounds can exhaust stack resources.

## Important unresolved decisions

CI does not configure branch protection. MSRV, message/lifecycle configuration
access, application-authored or scheduled events, external I/O, hardware,
RTOS, and no_std remain open. Pre-v0.1 APIs and the local grammar remain unfrozen.

## Most likely next tasks

1. Review dependency features/licences and user-facing scope claims.
2. Record human v0.1 architecture review before broadening scope.
3. Publish subsequent verified source increments and inspect their exact CI runs.

## Latest run

2026-09-12 follow-up: recorded the user's source-publication authorization,
updated the nightly automation in place, published the accumulated source,
and verified the first hosted baseline. All work and reviews were performed
by Codex; standing user authorization does not imply human code review.
