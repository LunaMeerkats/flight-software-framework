# Project state

Last updated: **2026-09-12**

## Current milestone

Stages 1 through 3 and the first source-quality checkpoint are complete.
ADR-0022 combines the existing services in one tested executable sample.
ADR-0023 now configures its Windows CI job. Hosted CI execution and human
v0.1 release reviews remain outstanding.

## Verified baseline

- Started clean on `codex/nightly` at
  `634941453be2503371d962f5d80ec865448a730a`. Initial required baseline
  passed 115 tests on rustc/cargo 1.98.0, rustfmt 1.9.0-stable, and
  Clippy 0.1.98. These are evidence, not an MSRV or pin.
- The workflow's locked command bodies pass locally: format, all-target/
  all-feature check, warnings-denied Clippy, 115 tests, warnings-denied
  rustdoc, both focused host targets (14/five tests), and the sample binary.
  Git working-tree and committed-tip whitespace commands pass.
- No Rust, Cargo manifest/lockfile, runtime API, dependency, unsafe code,
  lint threshold, or expectation changed. The initial whole-tree audit found
  zero width findings in 32 Rust files and three fulfilled expectations.
- Actionlint 1.7.12, all 16 PowerShell command parses, native failure probes,
  and root/ordinary/merge/depth-two whitespace fixtures pass. The corrected
  exit-code probe preserves failure rather than exact numeric status.
- Audits pass for 38 Markdown files, 142 relative links, 32 sources, and
  79 exact test references. Nine changed rendered documents match source;
  browser layout review has no overflow or console warnings. CI/ADR opening
  screenshots and independent complete-diff review are complete.
- ADR-0018/0019 experiments are unchanged and not separately rerun.
  Local rustup installation is not run; hosted bootstrap remains unverified.

## Current architecture

One unpublished, dependency-free safe-Rust library provides finite LC1
lifecycle, synchronous work, bounded lifecycle-owned inbox dispatch, manual
injected time and one-shot scheduling, bounded events, and constructor-owned
configuration with immutable ordinary-work visibility and consume-once rollback.
The private shared-source host sample and all runtime behavior are unchanged.

One GitHub Actions job requests stable Rust on Windows 2025, uses an upstream
SHA-pinned checkout with read-only contents and persisted credentials disabled,
and runs each native check in its own PowerShell step. It logs the checkout
and tool versions. No external action has been dispatched by this increment.

## Work in progress

No unfinished implementation remains. The CI configuration checkpoint is
locally validated and reviewed. Hosted
acceptance requires an authorized push and an exact-revision successful run;
no hosted result is inferred from the workflow file or local command replay.

## Highest risks and uncertainties

- Stable Rust and the named runner image float. Changed toolchain versions
  require the existing source-quality re-audit; no reproducibility pin exists.
- CI installation, checkout, event/permission behavior, and repository settings
  remain unverified on GitHub. Pushes remain unauthorized.
- Manual source/document and conditional ADR-probe checks are outside the job.
  Human scope, dependency, and architecture reviews remain release gates.
- Callback/clock panics and hangs remain outside containment. Host saturation
  remains terminal; there is no recovery, real-time, or physical delivery claim.
- IDs, revisions, and instants remain caller-scoped. Validators/callbacks need
  not terminate; large inline bounds can exhaust stack resources.

## Important unresolved decisions

Hosted acceptance is open under the existing publication gate. CI does not
configure branch protection. MSRV, message/lifecycle configuration access,
application-authored or scheduled events, external I/O, hardware, RTOS, and
no_std remain open. Pre-v0.1 APIs and the local grammar remain unfrozen.

## Most likely next tasks

1. Review dependency features/licences and user-facing scope claims locally.
2. After explicit push authorization, run CI and record exact hosted evidence.
3. Record human v0.1 architecture review before broadening scope.

## Latest run

2026-09-12: Prepared the existing baseline/sample CI configuration as a bounded
Stage 4 checkpoint. Locked local command replay passes with 115 tests; hosted
execution remains unverified. The current plan records final review evidence.
No push is authorized or performed.
