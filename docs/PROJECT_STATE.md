# Project state

Last updated: **2026-09-13**

## Current milestone

Stages 1 through 3 and the first source-quality checkpoint are complete.
The combined sample and hosted CI pass at recorded revisions. Stage 4 now has
an autonomous dependency/features/licence review and scope inventory prepared
for human review. Resource/failure-path/public-API review and human v0.1
entry-point and architecture acceptance remain outstanding.

## Verified baseline

- Initial and final local baselines pass all 115 tests on rustc/Cargo 1.98.0,
  rustfmt 1.9.0-stable, and Clippy 0.1.98; rustdoc warnings are denied.
- Refreshed origin and local `codex/nightly` began equal at
  `c472c57c1aa241d821b891674ba25f42b25ae723`. Its
  [hosted run 34697866002](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/34697866002)
  was rechecked as completed/success. The original
  [CI baseline](verification/CI_BASELINE.md) records the first hosted run,
  Windows image, and Rust 1.98.1 toolchain. Neither proves a later commit.
- Full Cargo metadata, manifest/lockfile, and all-target dependency trees agree:
  one package, zero external Cargo dependencies, and zero features. Approved
  licence files and `publish = false` remain unchanged.
- The [dependency and scope record](verification/DEPENDENCY_SCOPE_REVIEW.md)
  distinguishes the Cargo graph from standard-library/host/CI tooling and
  records the autonomous scope inventory and notice/status corrections.
- The final whole-tree audit covers 32 Rust files, zero width findings,
  three fulfilled expectations, and 79 exact traceability test references.
  ADR-0018/0019 probes are unchanged and not separately rerun.

## Current architecture

One unpublished Cargo package provides finite LC1 lifecycle, synchronous work,
bounded lifecycle-owned inbox dispatch, injected manual time and one-shot
scheduling, bounded events, and constructor-owned configuration with immutable
ordinary-work visibility and consume-once rollback. The private shared-source
host sample and runtime behavior are unchanged by this review.

The Windows CI job requests stable Rust and records versions and checkout
identity. It uses a SHA-pinned checkout, read-only contents, no persisted
checkout credentials, and one native command per PowerShell step.

## Work in progress

The dependency and scope checkpoint has passed local verification and
autonomous source/document/diff review. Commit/publication and exact-revision
hosted verification are next. No runtime implementation is in progress.
Source pushes are authorized under AGENTS.md; human v0.1 review is separate.

## Highest risks and uncertainties

- Stable Rust and runner images float; later revisions need their own evidence.
- No external Cargo dependencies does not clear the host/toolchain/CI supply
  chain. This review is not a vulnerability or bundle-redistribution audit.
- Source/document and conditional ADR-probe checks remain outside CI.
- Callback/clock panics and hangs remain outside containment. Host saturation
  is terminal; no recovery, real-time, or physical delivery claim is made.
- IDs, revisions, and instants remain caller-scoped. Validators/callbacks need
  not terminate; logical retained bounds do not guarantee whole-process memory
  or stack usage.

## Important unresolved decisions

CI does not configure branch protection. MSRV, message/lifecycle configuration
access, application-authored or scheduled events, external I/O, hardware,
RTOS, and no_std remain open. Pre-v0.1 APIs and the local grammar remain unfrozen.

## Most likely next tasks

1. Audit resource bounds, failure paths, and public APIs against the existing
   contracts, producing a bounded input to human architecture review.
2. Record human v0.1 entry-point and architecture acceptance before broadening
   scope; no such approval has been inferred from source-push permission.

## Latest run

2026-09-13: selected the planned dependency and scope checkpoint. Independent
Codex audits found no blocking dependency/licence issue or positive unsupported
scope claim. Corrected the generated crate notice, stale sample CI wording,
and a historical CI evidence link. All local checks pass; publication results
will replace the remaining in-progress status.
