# Repository guidance

## Scope and safety

This repository develops an experimental, host-based Rust framework. It is not
flight-qualified, safety-certified, NASA-affiliated, operationally suitable, or
proven to a Technology Readiness Level. Never imply compatibility with cFS,
cFE, OSAL, PSP, CCSDS, or an RTOS. Do not claim real-time behavior, fault
tolerance, or determinism without a precise definition and supporting evidence.

Use only public, legally accessible sources. Do not seek or infer restricted,
proprietary, export-controlled, employer-owned, or government-only material.
Study upstream responsibilities and observable behavior; do not mechanically
translate NASA C source.

## Repository layout

- `docs/CHARTER.md`: purpose, boundaries, and success criteria.
- `docs/ARCHITECTURE.md`: current responsibility analysis and proposed shape.
- `docs/REQUIREMENTS.md`: versioned, testable behavioral requirements.
- `docs/ROADMAP.md`: staged delivery sequence.
- `docs/PROJECT_STATE.md`: concise current milestone, evidence, and risks.
- `docs/adr/`: consequential architecture decisions and revisit conditions.
- `docs/research/SOURCES.md`: primary-source provenance and adoption decisions.
- `docs/verification/`: requirement traceability and verification baselines.
- `PLANS.md`: the active or most recently completed bounded work plan.
- `src/`: the single unpublished library package; currently logical lifecycle
  records, a synchronous start/work/stop/in-place-restart owned runtime, a
  constructor-owned optional bounded configuration table with immutable
  ordinary-work visibility,
  bounded message-routing core, lifecycle-aware ownership of one inbox per
  application, caller-selected one-message application dispatch, and a
  standalone bounded structured-event queue, an injected manual clock, and a
  finite caller-driven one-shot work schedule, plus direct returned-work
  failure-event integration that borrows the clock and event queue.
- `tests/`: public-API lifecycle, runtime, routing, runtime-messaging, and
  application-dispatch, event-queue, manual-clock, and scheduled-work
  integration tests, including returned-work event reporting and saturation,
  plus standalone and runtime-integrated configuration validation, revision,
  visibility, retention, and consume-once rollback tests.
- `rustfmt.toml` and `clippy.toml`: stable formatting and selected Clippy
  configuration.
- `Cargo.toml` and `Cargo.lock`: root package/workspace configuration and
  locked dependency graph.
- `LICENSE-MIT` and `LICENSE-APACHE`: recipient-choice dual-licence terms;
  `.gitattributes` preserves their canonical LF bytes on every checkout.

The root contains one unpublished package and workspace. Add another crate only
when a demonstrated boundary cannot remain coherent in the existing package.
Follow standard Cargo placement for libraries, binaries, examples, benchmarks,
and integration tests. Use descriptive `foo.rs` modules with children under
`foo/`; do not introduce a mixed legacy `mod.rs` layout.

## Supported checks

The required baseline is:

```text
cargo fmt --all -- --check
cargo check --workspace --all-targets --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo doc --workspace --all-features --no-deps
git diff --check
```

Run the rustdoc command with `RUSTDOCFLAGS=-D warnings`. In PowerShell, set
`$env:RUSTDOCFLAGS = "-D warnings"` before invoking Cargo. Also verify that
relative Markdown links resolve, inspect changed rendered documents, audit
handwritten Rust physical and comment-only line widths, and review the complete
diff. Document any justified adaptation before treating it as the baseline.
Never report a check as passing unless it completed successfully.

When changing ADR-0018 or its borrowing experiment, also run the explicit
build and standalone Markdown `rustdoc --test` commands in
[the experiment](docs/verification/CONFIGURATION_CONTEXT_EXPERIMENT.md).
Cargo does not discover these probes; they are design evidence, not runtime
integration tests. Inspect negative diagnostics and source form separately.

When changing ADR-0019 or its mailbox experiment, also run the explicit build,
standalone Markdown `rustdoc --test`, and extracted-source rustfmt/Clippy
commands in [the experiment](docs/verification/HOST_MAILBOX_EXPERIMENT.md).
Cargo does not discover this probe; it demonstrates borrowed host-output
ownership and saturation, not a complete command/telemetry adapter pair.

The source-quality policy was established with rustc/cargo 1.96.1, rustfmt
1.9.0-stable, and Clippy 0.1.96. A whole-tree re-audit passed on 2026-08-26 with
rustc/cargo 1.98.0, rustfmt 1.9.0-stable, and Clippy 0.1.98. These versions
record evidence, not a minimum supported Rust version or a toolchain pin.
Re-audit the configuration and whole tree when the active toolchain changes.

## Engineering conventions

- Use stable Rust and safe Rust. Ordinary crates should forbid unsafe code.
- Keep public APIs small; model states and expected failures explicitly.
- Do not panic for expected library or runtime failures.
- Avoid global mutable state, hidden threads, hidden executors, and unbounded
  resource growth.
- Give operational queues finite capacity and specified full/disconnect policy.
- Inject time and external effects so tests can control them.
- Keep protocol parsing outside business logic and validate external input.
- Add dependencies only for a recorded need; review licences and enabled
  features before committing them.
- Treat deterministic tests as repeatability under controlled inputs, not as a
  real-time or scheduling guarantee.
- Explain invariants and reasoning in comments rather than restating syntax.

## Source form and organization

Stable rustfmt owns ordinary formatting, with `max_width = 100`. Comment-only
prose should normally fit within 80 columns. Rustfmt does not prove that every
comment, URL, literal, macro, generated line, or physical source line fits a
limit, so reviewers must inspect those cases until a narrowly designed checker
is separately adopted.

Functions and methods should normally contain at most 60 Clippy-counted lines.
The workspace denies `clippy::too_many_lines` at the threshold configured in
`clippy.toml` for library, test, example, benchmark, and binary targets. This
is a review trigger, not an incentive to compress statements or fragment a
cohesive low-complexity flow.

For an intentional exception, prefer an item-level expectation:

```rust
#[expect(
    clippy::too_many_lines,
    reason = "extraction would split one chronological state-machine scenario"
)]
```

The reason must identify the invariant, control-flow, or coupling cost that
makes extraction worse. Review every exception in the diff and keep it close to
the item. Do not raise the global threshold, add crate-wide allows, or create a
mass waiver list. An unfulfilled expectation must remain visible under the
warnings-denied baseline so stale exceptions are retired.

Give each module one coherent responsibility and a small intentional public
surface. Split only on independent reasons to change or independently testable
concepts; do not create shallow forwarding layers, catch-all `utils`,
`common`, `misc`, `helpers`, or `manager` modules, or arbitrary
directories to reduce counts.

Arrange non-trivial modules for progressive reading: module documentation and
invariants; imports and module declarations; constants, identifiers,
configuration, data, state, and error types; public traits and entry points;
core implementations; private helpers; tests. Keep a private helper near its
consumer only when that locality is clearer than putting incidental detail
after the main flow.

Use `snake_case` for modules, module files, functions, and variables;
`UpperCamelCase` for types and traits; and `SCREAMING_SNAKE_CASE` for
constants. Prefer names that state domain responsibility, ownership, failure,
blocking, units, or bounds accurately. Avoid vague verbs, unexplained
abbreviations, boolean mode flags, deeply nested control flow, hidden side
effects, and mixed abstraction levels.

Document public behavior, invariants, units, bounds, ownership, state
transitions, expected errors, and panic conditions where applicable. Keep unit
tests beside private implementation when they verify local invariants; keep
public-API and multi-service behavior under `tests/`. A chronological or
table-driven test may justify a narrow function-size expectation when helpers
would obscure the state trace.

Generated source must identify its generator and provenance, be reproducible,
and never be hand-edited. A future source-shape checker may exclude generated
files explicitly while compilation and tests continue to cover them.

## Lint and checker baseline policy

Adopt lints individually after a whole-tree audit. This checkpoint selects only
`clippy::too_many_lines` in addition to the existing Rust `unsafe_code`
policy. It does not enable Clippy's pedantic, restriction, or nursery groups.
`missing_errors_doc`, `missing_panics_doc`, `mod_module_files`,
`allow_attributes_without_reason`, and `excessive_nesting` remain candidates
for separate measured increments.

Do not use Clippy's `cognitive_complexity` lint as evidence of cognitive or
cyclomatic complexity. Evaluate and validate a parse-aware Rust metric in a
separate plan if complexity measurement becomes useful. Keep naming, source
ordering, module cohesion, and abstraction quality in review unless a narrow
parse-aware rule proves reliable; do not enforce architecture with regular
expressions.

A strict physical-line checker, dependency-policy tool, CI workflow, or
toolchain pin requires its own justified baseline and review. New and touched
code must not worsen known findings while a gate is deferred. Record any
environment-only failure precisely rather than weakening a threshold or hiding
warnings.

## Git expectations

- Autonomous work occurs only on `codex/nightly`.
- Preserve unexpected local changes and generated work; never reset or discard
  them to obtain a clean tree.
- Make small local commits only after the bounded increment is coherent and its
  applicable checks pass.
- Do not push unless this file is explicitly updated to authorize pushing.
- Never force-push, merge into the default branch, tag, release, publish a
  crate, deploy, or commit secrets.
- Repository content is licensed under `MIT OR Apache-2.0` with the confirmed
  notice `Copyright 2026 Daniel Smith`; do not change the licence or notice
  without explicit human approval.

## Completion

A work item is complete only when one coherent behavior or decision checkpoint
is implemented or recorded, applicable verification has actually passed, the
complete diff and source form are reviewed, traceability and project state are
truthful, and the next run can continue without relying on chat history.

## Prohibited progress substitutes

Do not add empty crates, placeholder modules, generic traits, broad TODO trees,
speculative portability layers, dependencies, suppressions, or checker
exclusions merely to make the repository look active. Do not widen scope into
hardware, no-std, RTOS, broad protocol, or deployment work before the host
framework has a reviewed, verified need.
