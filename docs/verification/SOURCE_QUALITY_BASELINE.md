# Source-quality baseline

Date: **2026-08-23**

This checkpoint measures and adopts the first repository-owned source-form
rules. It is a local maintenance policy, not evidence of flight qualification,
agency compliance, certification, real-time behavior, or operational
suitability. Source provenance and adopt/adapt/reject decisions are in
[the source register](../research/SOURCES.md).

## Tool baseline

- rustc 1.96.1 (`31fca3adb`, 2026-06-26)
- Cargo 1.96.1 (`356927216`, 2026-06-26)
- rustfmt 1.9.0-stable (`31fca3adb`, 2026-06-26)
- Clippy 0.1.96 (`31fca3adb`, 2026-06-26)

These versions record the tools used to establish the policy. They do not set
a minimum supported Rust version or pin the repository toolchain.

## Initial whole-tree audit

Before this checkpoint:

- `Cargo.toml` already forbade local unsafe code through inherited workspace
  lints, and `src/lib.rs` repeated the crate-level prohibition;
- no tracked rustfmt or Clippy configuration, workspace Clippy lint, CI
  workflow, custom source checker, dependency-policy configuration, or
  toolchain pin existed;
- no handwritten Rust physical line exceeded 100 columns;
- eight Rust comment-only lines were 81 to 83 columns;
- `clippy::too_many_lines` at the proposed 60-line threshold found no
  production function and five chronological integration tests, measuring 70,
  76, 71, 71, and 72 Clippy-counted lines;
- no `allow` or `expect` lint suppression existed; and
- the selected-lint audit found four public trait methods that would trigger
  `missing_errors_doc`, while `missing_panics_doc`, `mod_module_files`,
  `allow_attributes_without_reason`, and configured `excessive_nesting`
  produced no current finding.

Thirteen Markdown physical lines exceeded 100 columns, principally source URLs
and traceability table rows. This Rust source-form checkpoint does not impose a
strict Markdown physical-line limit.

## Adopted policy

- `rustfmt.toml` sets the stable `max_width = 100` option.
- `clippy.toml` sets `too-many-lines-threshold = 60`, and the workspace denies
  `clippy::too_many_lines` for all targets.
- The eight comment-only findings were reflowed to the 80-column review
  default.
- At adoption, the five linear integration tests retained their complete
  chronological state traces under item-level `expect` attributes. Each
  expectation carried a specific reason why extracting setup or assertions
  would make the tested invariant harder to review.
- The required baseline now includes `cargo check`, warnings-denied Clippy,
  all-feature tests, and warnings-denied all-feature rustdoc generation.
- Naming, progressive source ordering, module cohesion, abstraction level, and
  exceptional physical lines remain explicit review responsibilities.

The exact waiver and retirement rules are in [AGENTS.md](../../AGENTS.md).

## Follow-up source-order cleanup

Commit `c2aa32772c2fd32a0b87893f913e32ecacc5d051` applied the manual
progressive-reading and naming rules without enabling another lint or changing
runtime behavior. It moved private lookup helpers after all public runtime
operations, added explicit error sections to the four public application
callbacks, co-located one displaced fixture implementation, and clarified the
start-failure vocabulary.

The restart evidence was separated into independently meaningful rejection and
retained-state success tests. The unknown-identity test now uses the bounded
logical registry as its identity issuer and shares one expected lifecycle error
across all four operations. Those changes retired two expectations without
line compression or artificial helpers. Three reasoned chronological
expectations remain, `missing_errors_doc` has zero current findings, and the
suite contains 16 passing tests.

## Deferred and rejected gates

- A strict physical-line checker is deferred until a small checker can be
  tested with explicit URL, literal, fixture, and generated-source exclusions.
- `missing_errors_doc`, `missing_panics_doc`, `mod_module_files`,
  `allow_attributes_without_reason`, and `excessive_nesting` require separate
  whole-tree adoption increments rather than a lint-group enablement.
- Clippy's `cognitive_complexity` lint is rejected as evidence of true
  cognitive or cyclomatic complexity. A future metric must be parse-aware,
  validated against this Rust tree, and recorded with its selected meaning.
- Dependency-policy tooling, a CI workflow, and a toolchain pin remain separate
  decisions. None is implied by this checkpoint.

## Verification result

Implementation commit `c8f2f7af9232aa370a3ea9ded211eac1581dd375`
contains the configuration, selected lint, reasoned expectations, and comment
reflow. The following checks passed before that commit:

- `cargo fmt --all -- --check`
- `cargo check --workspace --all-targets --all-features`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-features` (15 passed)
- `cargo doc --workspace --all-features --no-deps` with
  `RUSTDOCFLAGS=-D warnings`
- `git diff --check`
- manual audit: zero handwritten Rust physical lines over 100 columns and zero
  comment-only Rust lines over 80 columns

Repository-document consistency and the same serial baseline are rechecked in
the documentation commit that records this checkpoint. All 37 relative links
resolve across 20 Markdown files, and all 20 documents render structurally with
one top-level heading.
