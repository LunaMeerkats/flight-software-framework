# Stage 4 lifecycle construction review

Date: **2026-09-21**
Status: **Local checkpoint complete; publication pending**

## Objective and context

Verify the existing standalone lifecycle registry and unconfigured owned
runtime construction failure boundary. The starting revision is
`7cabe599d88fa5a7f3ce74b2e35fac33488aa711`, clean and equal to refreshed
`origin/codex/nightly`. Its hosted run 35466793755 passed; the initial locked
local baseline passes 122 tests on Rust/Cargo 1.98.0.

This is the recorded next Stage 4 gap. Source and test inspection found no
production defect. Zero-capacity rejection, logical registration saturation,
rejected-value ownership, and configured-runtime reservation failure are
already covered. Exact capacity-overflow diagnostics are not executed through
`LifecycleRegistry::new` or the unconfigured `Runtime::new` entry point.

## Acceptance criteria and files

- Add one public-API test in `tests/lifecycle_registry.rs` for exact
  `usize::MAX` registry reservation rejection.
- Add one public-API test in `tests/application_runtime.rs` for exact
  `usize::MAX` unconfigured runtime reservation rejection.
- Preserve production code, API, requirements, dependencies, and lint policy.
- Record source-inspected construction and registration properties separately
  from executed evidence in
  `docs/verification/LIFECYCLE_CONSTRUCTION_REVIEW.md`; update README, source
  register, project state, roadmap, and traceability.

## Verification

Run the two focused test targets, then the required locked Cargo baseline with
warnings-denied Clippy and rustdoc. Audit Rust physical/comment widths,
relative links, traceability names, changed rendered documents, and the
complete diff. Unchanged host adapters, workflow, and ADR-0018/0019 probes do
not trigger separate local commands.

After local acceptance, commit on `codex/nightly`, refresh origin, publish by
ordinary fast-forward, and inspect exact-revision hosted CI. Record completed
publication evidence separately without projecting a pass onto a later commit.

## Risks and safe stopping point

`usize::MAX` for these nonzero-sized records deterministically exercises
capacity overflow, not actual allocator exhaustion. Logical application limits
do not bound application-owned memory, callback allocations, stack use, or
execution time. Caller-scoped identity aliasing, panic/hang containment, and
human v0.1 acceptance remain open. If verification fails, repair or remove only
this run's changes. Stop after this one verified checkpoint and its publication
evidence.

## Completed local checks

Both new regressions pass: five lifecycle-registry, eleven application-runtime,
and 124 workspace tests. Formatting, all-target checking, warnings-denied
Clippy/rustdoc, and whitespace checks pass. No production change, dependency,
or lint exception was needed. The whole-tree audit passes 32 Rust files with
zero width findings and three unchanged fulfilled expectations, plus 44
Markdown files, 194 resolving relative links, and 88 exact traceability test
references. Complete source/diff review found no blocking defect. GitHub GFM
rendering of all seven changed documents preserved every heading and fenced
code block and produced nonempty linked HTML.
