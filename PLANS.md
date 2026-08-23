# Plan: apply source-formatting principles

Status: **Complete**
Date: **2026-08-23**

## Objective

Apply the newly recorded source-form principles to the existing runtime code
without changing public behavior. Improve progressive reading and naming,
make public callback failure documentation explicit, and retire only those
function-size expectations that clearer test structure makes unnecessary.

## Context

The pre-change baseline passed with 15 tests and all enforced formatting and
lint gates. The audit found no width, unsafe-code, module-layout, nesting, or
production function-size violation. It did find:

- private runtime record-lookup helpers before the public lifecycle methods;
- four public `Application` callbacks whose expected errors were described but
  lacked explicit `# Errors` sections;
- one restart-faulting fixture implementation separated from its type;
- a start-only fault fixture with a vague name; and
- two test expectations whose underlying tests could become clearer through a
  focused behavior split and a smaller identity fixture.

The other three long tests remain cohesive chronological lifecycle or
containment scenarios. Splitting them would add artificial helpers or scatter
the state trace.

## Acceptance criteria

- Present all public `Runtime` entry points before private record-lookup
  helpers.
- Give each fallible public `Application` callback an accurate `# Errors`
  section without implying cleanup, containment, or recovery.
- Co-locate the restart-faulting fixture and its implementation.
- Rename the start-only fault fixture and its observations to state the
  operation and peer roles precisely.
- Split restart rejection from successful retained-state restart because they
  are independently meaningful behaviors.
- Replace irrelevant dummy applications in the unknown-identity test with a
  bounded logical identity issuer and share the expected lifecycle error.
- Retire exactly the two expectations made unnecessary by those changes; keep
  the three cohesive expectations with their existing specific reasons.
- Preserve the public API, lifecycle behavior, ownership, capacity, error
  values, dependencies, and runtime execution model.
- Pass all repository checks and independent diff review before committing.

## Files and components

- `src/runtime.rs`: public callback documentation and public-first method
  ordering.
- `tests/application_runtime.rs`: fixture naming and locality, focused restart
  evidence, and simplified unknown-identity setup.
- `README.md`, `docs/PROJECT_STATE.md`, `docs/ROADMAP.md`,
  `docs/verification/SOURCE_QUALITY_BASELINE.md`, and
  `docs/verification/TRACEABILITY.md`: current counts and durable evidence.
- This plan: scope, acceptance criteria, verification, and stopping point.

## Verification approach

- Run `cargo fmt --all -- --check`.
- Run `cargo check --workspace --all-targets --all-features`.
- Run `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
- Probe `clippy::missing_errors_doc` separately and require zero findings.
- Run `cargo test --workspace --all-features`.
- Run warnings-denied all-feature rustdoc generation.
- Audit handwritten Rust physical and comment-only widths.
- Confirm exactly three fulfilled reasoned expectations remain.
- Resolve relative Markdown links, render changed documents, and check
  requirement/source identifiers and table shapes.
- Run `git diff --check` and review the complete diff.

## Known risks

- Source movement can hide an accidental semantic edit; the complete diff and
  behavior tests must show only equivalent ordering, naming, documentation, and
  test-evidence structure.
- Splitting a chronological test merely to satisfy a line threshold can make
  evidence harder to follow. Only the restart test has two independently named
  responsibilities; the remaining three expectations stay in place.
- Replacing the unknown-ID fixture must preserve a genuinely out-of-range
  identity and all four callback-suppression assertions.
- `missing_errors_doc` is probed but not enabled as a new workspace gate; lint
  adoption remains a separate policy decision.

## Safe rollback or stopping point

Stop after the runtime and integration-test source is progressively ordered,
the two avoidable expectations are retired, and all existing behavior remains
verified. Do not continue into messaging, another lint gate, module splitting,
generic test fixtures, or unrelated cleanup.

## Result

Implementation commit `c2aa32772c2fd32a0b87893f913e32ecacc5d051`
reached the code-side stopping point. All public runtime methods now precede
private lookup details; the four application callbacks have explicit error
sections; test fixture names and implementation locality are clearer; and the
restart and unknown-identity tests retain their evidence without line-count
exceptions.

Three reasoned expectations remain, down from five. The runtime integration
suite now contains ten focused tests, and the complete suite contains 16 tests.
No public API, runtime behavior, dependency, thread, executor, or message path
changed.
