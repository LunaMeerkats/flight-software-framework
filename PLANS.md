# Plan: initial source-quality policy checkpoint

Status: **Complete**
Date: **2026-08-23**

## Objective

Measure the existing Rust source against the newly approved source-quality
rules and adopt the smallest coherent machine-enforced checkpoint before Stage
2 feature growth. Track stable 100-column formatting, activate one audited
60-line function review gate, resolve or justify every initial finding, and
record exact public-source provenance and waiver policy.

## Context

The pre-change baseline passed with 15 tests, but the repository encoded only
the workspace `unsafe_code = "forbid"` lint. It had no tracked rustfmt or Clippy
configuration, source-shape checker, CI workflow, dependency policy, or
toolchain pin. Contributor guidance omitted `cargo check`, warnings-denied
rustdoc, source ordering, module layout, naming, and lint-waiver rules.

The whole-tree audit found no handwritten Rust physical line over 100 columns,
eight comment-only lines between 81 and 83 columns, and five integration tests
between 70 and 76 Clippy-counted lines at a proposed threshold of 60. Production
functions were below the threshold. The five tests are linear lifecycle and
failure traces whose complete order is part of the evidence.

Official Rust tooling and style sources define the mechanisms. JPL, NASA, ECSS,
and JAXA sources inform the value of local coding rules, review, analysis, and
recorded evidence, but do not prescribe this Rust policy or apply to this
experimental repository. A bounded Australian public-source search found no
independent agency Rust or code-style standard; the absence is recorded rather
than filled by inference.

## Acceptance criteria

- Track stable rustfmt with `max_width = 100`.
- Track Clippy with `too-many-lines-threshold = 60` and deny only
  `clippy::too_many_lines` through inherited workspace lints.
- Reflow all eight initial comment-only width findings.
- Review all five test findings individually. Use item-level expectations with
  specific reasons only where extracting the chronological state trace would
  make the test harder to understand.
- Leave no unexplained or broad lint suppression, threshold increase, lint-group
  enablement, generated waiver list, new dependency, or runtime behavior change.
- Record naming, module layout, progressive ordering, documentation, generated
  code, manual-review boundaries, tool versions, waiver retirement, and
  checker-baseline rules in `AGENTS.md`.
- Record exact public sources and adopt/adapt/reject decisions in the source
  register.
- Update the release gate, roadmap, project state, README, and a durable
  source-quality baseline without inventing a behavioral requirement or ADR.
- Pass the expanded serial baseline and repository consistency checks before
  the documentation commit.

## Proposed files and components

- `rustfmt.toml`, `clippy.toml`, and `Cargo.toml` for selected machine
  configuration.
- Existing Rust comments and the five public integration tests for the complete
  audited finding set.
- `AGENTS.md`, `README.md`, `docs/research/SOURCES.md`,
  `docs/verification/SOURCE_QUALITY_BASELINE.md`, `docs/REQUIREMENTS.md`,
  `docs/ROADMAP.md`, and `docs/PROJECT_STATE.md` for durable policy and state.
- This plan for scope, evidence, risks, and the stopping point.

## Verification approach

- Run `cargo fmt --all -- --check`.
- Run `cargo check --workspace --all-targets --all-features`.
- Run
  `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
- Run `cargo test --workspace --all-features`.
- Run `cargo doc --workspace --all-features --no-deps` with
  `RUSTDOCFLAGS=-D warnings`.
- Audit handwritten Rust lines over 100 columns and comment-only lines over 80.
- Resolve relative Markdown links and cross-check requirement/source
  identifiers and Markdown table shapes.
- Render changed Markdown to HTML and inspect its document structure.
- Run `git diff --check`, inspect the complete diff, and confirm branch/status
  before local commits.

## Known risks

- Function length is a coarse review trigger and does not measure correctness,
  coupling, nesting, or complexity.
- Suppressing the five tests without precise reasons would hide debt; splitting
  them mechanically would obscure chronological evidence. Each expectation must
  remain narrow and become unfulfilled when the finding disappears.
- Stable rustfmt does not wrap every comment, URL, literal, macro, or generated
  line, so a formatting pass is not a complete physical-line proof.
- Enabling additional lints in this increment would mix separate audits and
  could create warning debt or cosmetic churn.
- Agency material can be misrepresented as an applicable Rust standard. Every
  source decision must distinguish local adaptation from compliance.

## Safe rollback or stopping point

Stop after the tracked configuration, complete initial finding disposition,
durable policy/provenance, and expanded serial baseline are coherent. If the
60-line gate cannot be adopted without artificial fragmentation or broad
suppression, retain the measured baseline and rustfmt policy but defer the lint.
Do not continue into messaging, events, time, configuration, CI, dependency
tooling, complexity tooling, or another lint in this increment.

## Result

Implementation commit `c8f2f7af9232aa370a3ea9ded211eac1581dd375`
reached the code-side stopping point. It adds the two stable configuration
files, activates the individual workspace lint, reflows all eight comment
findings, and adds five item-level expectations with review-specific reasons.
No production function requires an exception.

The initial audit and final configuration use rustc/cargo 1.96.1, rustfmt
1.9.0-stable, and Clippy 0.1.96. The implementation-side expanded baseline
passed: formatting, all-target checking, warnings-denied Clippy, 15 tests,
warnings-denied all-feature rustdoc, Git whitespace, zero Rust physical lines
over 100 columns, and zero comment-only Rust lines over 80 columns.

The documentation commit records the exact research, adopt/adapt/reject
decisions, contributor and waiver policy, release gate, roadmap, project state,
and repository-consistency verification. All 37 relative links resolve across
20 Markdown files; eight requirements match traceability; eight referenced
source identifiers resolve in the 21-entry register; both tables have
consistent row shapes; and all 20 documents render structurally with one
top-level heading. The run stops before unrelated Stage 2 behavior or another
enforcement mechanism.
