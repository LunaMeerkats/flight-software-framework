# Bounded configuration snapshot lifecycle

Date: **2026-08-31**
Status: **Complete**

## Objective and context

Implement the standalone storage and transition core of ADR-0005. Stage 2 is
complete and the clean starting commit is `782149b`. The required baseline
passed with 58 tests on rustc/cargo 1.98.0, rustfmt 1.9.0-stable, and Clippy
0.1.98. Configuration is the next roadmap slice; runtime ownership and
application visibility need a separate integration because work currently has
no configuration context.

## Selected boundary

Use inline const-bounded byte snapshots with immutable public views and one
retained mission validation function. This keeps both retained content and
history bounded without assuming that arbitrary generic values are immutable
or allocation-free. Bytes are in-memory data, not a selected wire format.
ADR-0017 records alternatives and error precedence. ADR-0005 remains only
partially implemented; RFF-REQ-006 is not yet fully verified.

## Acceptance criteria

- Construction accepts revision 1 only after length and semantic validation.
- Each replacement validates with the same function, then checks revision
  exhaustion before mutating active content, rollback content, or high-water.
- Rejection preserves the concrete validation error and all retained state.
- Each accepted replacement receives the next unused revision, including equal
  content, and retains only the former active snapshot for rollback.
- Rollback consumes the sole slot, restores its original revision, never
  decreases high-water, and does not revalidate previously accepted bytes.
- A second rollback returns `NoRollbackAvailable` without mutation.
- The sequence `1 -> 2 -> rollback 1 -> replacement 3` is observed publicly.
- Private unit setup proves checked `u64` exhaustion without a public revision
  override, including rejection precedence and exhaustion after rollback.
- Exact byte capacity, oversize input, empty input, copied ownership, retained
  validation policy, and replacement of old history have explicit tests.
- No mutable snapshot access, generic service trait, dependency, file/parser,
  schema, runtime context, event path, persistence, or automatic rollback is
  introduced. Runtime safe-point, restart, and failure behavior remain pending.

## Files and verification

Change `src/configuration.rs`, `src/lib.rs`, and `tests/configuration_table.rs`;
record ADR-0017 and update ADR-0005 status, README, contributor layout,
architecture, requirements, roadmap, project state, and traceability.

Run focused configuration tests and the complete documented baseline:
`cargo fmt --all -- --check`,
`cargo check --workspace --all-targets --all-features`,
`cargo clippy --workspace --all-targets --all-features -- -D warnings`,
`cargo test --workspace --all-features`, and
`cargo doc --workspace --all-features --no-deps` with
`RUSTDOCFLAGS=-D warnings`, then `git diff --check`.

Review physical/comment-only widths, narrow expectations, source order and
cohesion, relative links, requirement/source/test references, rendered changed
documents, and the complete diff. No new checker gate is being adopted.

Document-review adaptation: the in-app browser rejects local-file URLs under
its security policy. No alternate serving or browser workaround is permitted.
Inspect generated HTML content and structural audit results instead; record
browser visual inspection as blocked, not passed. This adapts only document
presentation review, not any Cargo, source-quality, link, or diff gate.

## Risks and safe stopping point

The const bound is selected by mission composition; very large inline values
can exhaust host stack space. Active, rollback, and a validated candidate can
coexist during replacement. Caller copies, input buffers, and validation
effects/errors are outside retained snapshot storage. A function pointer fixes
which validator is used but cannot prove purity, termination, or stable external
state. No callback panic or host failure containment is claimed. Revisions
are table-local and do not identify their origin.

Stop after this core is tested and documented, with runtime integration
explicitly pending. If verification cannot pass, preserve the decision and
remove only this run's incomplete implementation. Commit only on
`codex/nightly`; do not push.

## Result

Implemented the standalone core with immutable inline byte snapshots, one
retained validator, exact rejection errors, checked revisions, and consume-once
rollback. Twelve public tests and three private boundary tests cover the
selected contract, including exhaustion and validation suppression.

The complete documented format/check/Clippy/test/rustdoc/whitespace baseline
passes with 73 tests and warnings denied. No new waiver or dependency was
needed. RFF-REQ-006 remains partial; runtime integration is the next likely
bounded objective.

Source review found 21 handwritten Rust files with no physical line over 100
columns and no comment-only line over 80. The three existing reasoned
expectations remain fulfilled; no production function needs a waiver.
Document review found 28 Markdown files, 75 resolving relative links, eight
matching requirement/traceability rows, 21 defined source identifiers, and 54
exact traceability test references. All documents rendered structurally; the
ten changed documents retained their headings, lists, code, table cells, and
content. Independent code and documentation review found no remaining defect.
Browser visual QA remains blocked as documented above. The complete diff was
reviewed; the next evidence checkpoint records the local implementation hash.
