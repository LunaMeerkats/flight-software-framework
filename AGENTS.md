# Repository guidance

## Product direction and autonomous progression

Build Rust Flight Framework into a practical, reusable Rust library that other
developers can integrate into real projects. Its current experimental maturity
describes available evidence; an experiment or sample alone is not the product
goal. Prioritise usable APIs, clear integration/error/resource documentation,
realistic examples and verification through downstream public-API use.

The user's 2026-10-19 direction in
[ADR-0025](docs/adr/0025-practical-library-development.md) makes v0.1 a verified
baseline, not a scope ceiling. Research, select and implement justified later
milestones autonomously. Ordinary API, service, adapter and architecture evolution
does not need renewed permission merely because it exceeds v0.1. Define the
consumer need, compare consequential alternatives, update requirements/ADRs and
traceability, and complete one coherent verified increment at a time. Balance
hardening with functionality and usability; select tests and documentation for
the concrete risk or adoption gap they close.

The separately listed consequential decisions retain their authorization gates.
Practical usefulness does not establish qualification, operational suitability,
compatibility, API stability or permission to release/publish/deploy.

## Scope and safety

The current implementation is an experimental, host-based Rust framework. It is
not flight-qualified, safety-certified, NASA-affiliated, operationally suitable, or
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
- `.github/workflows/ci.yml`: one Windows host baseline job; its commands and
  hosted-evidence boundary are recorded in `docs/verification/CI_BASELINE.md`.
- `src/`: the single unpublished library package; currently logical lifecycle
  records, a synchronous start/work/stop/in-place-restart owned runtime, a
  constructor-owned optional bounded configuration table with immutable
  ordinary-work visibility,
  bounded message-routing core, lifecycle-aware ownership of one inbox per
  application, caller-selected one-message application dispatch, and a
  standalone bounded structured-event queue, an injected manual clock, and a
  finite caller-driven one-shot work schedule, plus direct and messaging-owned
  ordinary-work failure-event integration that borrows the clock and event
  queue, reporting after owner-specific lifecycle commitment and inbox cleanup.
  Direct and messaging-owned schedules share one private clock/consumption
  decision and delegate ordinary work through their respective owners.
  `src/scheduling/periodic.rs` owns one inline host-polled recurring release
  cursor; it supplies items without invoking or wrapping either work owner.
- `tests/`: public-API lifecycle, runtime, routing, runtime-messaging and
  finite standalone publication/dequeue reference-model sequence tests; a
  reduced runtime-owned publication/stop/restart history model with passive
  final-dispatch observations; separate reduced ordinary-work and message-
  callback failure models with both fault roles, selected clearing and peer
  retention; a reduced messaging-owned one-shot schedule model combines manual
  time, publication, final item consumption and returned-work failure.
  Integration tests also cover application dispatch, event queues,
  manual clocks and scheduled work, including ordinary-work event
  reporting, exact inbox clearing, event saturation, and messaging-owned
  one-shot schedule consumption with preserved peer queues,
  plus standalone and runtime-integrated configuration validation, revision,
  visibility, retention, and consume-once rollback tests; a separate finite
  standalone configuration history model composes accepted/rejected replacement
  and consume-once rollback without reproducing the table's storage layout.
  A separate direct-runtime history model composes configuration changes,
  ordinary-work observations, selected stop/restart and returned-work failure,
  with a healthy peer observing retained configuration lineage.
  A separate finite standalone event history model checks mixed emission,
  rejection and dequeue histories with an accepted-record ledger and cursor.
  A separate direct-runtime event history model composes two distinct returned
  work failures, healthy peer work, advancing time, event dequeue and explicit
  rejected-event retry using symbolic records and independent failure history.
  A separate messaging-owned event history model joins publication, selected
  work failure, peer work/dispatch, time, event dequeue and explicit retry with
  independent message and event acceptance histories.
- `examples/host-echo/`: a private multi-file Cargo command/telemetry example.
  `mission.rs` owns composition/ingress; its `codec.rs` and `applications.rs`
  children own validation and application/output behavior. `work.rs` owns
  bounded configuration observations and cooperative fault injection.
  `sample.rs` owns the fixed combined scenario and report; `main.rs` prints
  diagnostics outside callbacks. `tests/host_sample.rs` loads the exact mission
  and driver source to verify lifecycle, time, configuration, and fault traces.
  The adapter integration
  target `tests/host_adapters.rs` loads that same mission through one explicit
  `#[path]`; explicit child paths keep both target forms identical, avoiding
  copied adapters or new library exports.
- `examples/quickstart.rs`: one independently authored public-API consumer.
  Its `test = true` target executes behavioral tests in the workspace baseline;
  `docs/GETTING_STARTED.md` shows the same source as an independent path-dependent
  Cargo binary. Temporary consumer verification adds no maintained crate.
- `examples/composition.rs`: a second public-only consumer with two independently
  implemented application types, mission enum delegation and typed message-error
  mapping. Its test-enabled target and independent-package verification retain
  the same ownership and evidence boundary as the quickstart.
- `examples/work_output.rs`: a public-only configured ordinary-work producer
  with a capacity-one pending slot and explicit host transfer to one subscriber.
  Its test-enabled target and independent-package recipe verify this mission
  policy, including saturation retry and bounded recurring host turns that gate
  work/dispatch on downstream capacity; it adds no ordinary-work service context.
- `tests/work_identity.rs`: public-only diagnostic consumer and context-identity
  integration evidence across every ordinary-work entry path. Its guide verifies
  identical tests in an independent Cargo package without another example target.
- `tests/periodic_work.rs`: public-only repeated-work consumer and release-policy
  evidence. Its guide verifies identical tests in an independent Cargo package;
  host delegation preserves each existing owner's lifecycle and cleanup boundary.
- `tests/selective_publication.rs`: public-only targeted publication and fixed
  two-subscriber retry consumer. Its guide verifies identical tests in an
  independent Cargo package. The acceptance ledger records inbox enqueue only;
  later stop/failure may clear accepted copies without proving processing.
- `tests/event_filter.rs`: public-only optional-diagnostic consumer and explicit
  severity-filter evidence. Its guide verifies identical tests in an independent
  Cargo package; unfiltered emission and failure-event wrappers retain their
  existing contracts.
- `examples/event_output.rs`: a public-only host event presenter that owns one
  writer, pending event, fixed rendered line and accepted-byte cursor. Its
  test-enabled target and independent-package recipe verify partial writes,
  explicit errors and host flush without adding a library sink service.
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

[ADR-0023](docs/adr/0023-host-ci-baseline.md) configures one Windows CI job.
Its Cargo resolving commands add `--locked`; this is also a supported local
form and must fail instead of changing dependency resolution. CI separately
runs both focused host tests and the sample. It adds
`git show --format= --check --diff-merges=first-parent HEAD` for committed-tip
whitespace (first-parent merge diff on PRs), with checkout depth two. This
does not inspect every intermediate commit in a multiple-commit push.
Keep source/document/diff review and conditional ADR probes outside this job
explicit. When changing the workflow, run `actionlint -color .github/workflows/ci.yml`
with reviewed actionlint 1.7.12 and inspect PowerShell failure propagation.
The [CI baseline](docs/verification/CI_BASELINE.md) records provenance and
hosted acceptance; a local replay does not establish a hosted CI pass.

When changing the host adapters or combined sample, also run
`cargo test --test host_adapters`, `cargo test --test host_sample`, and
`cargo run --example host-echo`. These exercise the exact shared mission/driver
source and the host entry point. The combined sample demonstrates controlled
host behavior; it does not complete CI or a recorded v0.1 review or guarantee
physical delivery.

When changing the quickstart or its setup guidance, also run
`cargo test --locked --example quickstart` and
`cargo run --locked --example quickstart`. Follow the independent-package commands
in [the getting-started guide](docs/GETTING_STARTED.md), then check/lint/test/run
that package and compare its main-source hash with the example. Inspect Cargo
metadata for its independent workspace and sole library path dependency. Record
the external package path and exact commands/results in the focused verification
record. This demonstrates that consumer only, not general adoption or stability.

When changing the composition example or its guidance, likewise run
`cargo test --locked --example composition` and
`cargo run --locked --example composition`, then follow the independent-package
commands in [the composition guide](docs/COMPOSING_APPLICATIONS.md). Check, lint,
test and run that package, inspect metadata and compare source hashes. Record
its path and exact results in the focused composition verification record.

When changing the ordinary-work output example or its guidance, run
`cargo test --locked --example work_output` and
`cargo run --locked --example work_output`, then execute the independent-package
recipe in [its guide](docs/ORDINARY_WORK_OUTPUT.md). Check, lint, test and run the
identical source, inspect metadata and compare hashes. Record the package path
and exact results in the focused work-output record. Its explicit retry policy
requires exactly one subscriber and does not support partial fan-out retries.

When changing ordinary-work identity or its consumer guidance, run
`cargo test --locked --test work_identity`, then follow the independent-package
recipe in [the identity guide](docs/APPLICATION_WORK_IDENTITY.md). Check, lint
and test the identical test source; inspect metadata and compare SHA256 hashes.
Record the actual package path and results in the focused identity review.
This verifies the controlled consumer, not general adoption or identity origin.

When changing periodic releases or their consumer guidance, run
`cargo test --locked --test periodic_work`, then follow the independent-package
recipe in [the periodic-work guide](docs/PERIODIC_WORK.md). Check, lint and test
the identical test source, inspect metadata and compare SHA256 hashes. Record
the actual package path/results in the focused periodic-work review. A released
item is not evidence that the host invoked or completed its callback.

When changing selective publication or its consumer guidance, run
`cargo test --locked --test selective_publication`, then follow the independent
package recipe in [the selective-publication guide](docs/SELECTIVE_PUBLICATION.md).
Check, lint and test identical test source; inspect metadata and SHA256 hashes.
Record the actual package path and results in the focused review. Targeted
publication respects subscriptions; acceptance does not guarantee processing,
retention after stop/failure or duplicate suppression on repeated calls.

When changing event filtering or its consumer guidance, run
`cargo test --locked --test event_filter`, then follow the independent-package
recipe in [the filtering guide](docs/EVENT_FILTERING.md). Check, lint and test the
identical test source; inspect metadata and compare SHA256 hashes. Record the
actual package path and results in the focused filtering review. Filtering is
optional admission policy, not reserved capacity or guaranteed event delivery.

When changing the event-output example or its guidance, run
`cargo test --locked --example event_output` and
`cargo run --locked --example event_output`, then execute the independent-package
recipe in [the event-output guide](docs/EVENT_OUTPUT.md). Check, lint, test and
run the identical source; inspect metadata and compare SHA256 hashes. Record
the actual package path and results in the focused event-output review. Writer
acceptance and explicit flush do not establish physical or durable delivery.

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

The 2026-09-12 hosted run passes the all-target baseline on rustc/cargo 1.98.1,
rustfmt 1.9.0-stable, and Clippy 0.1.98. The companion whole-tree source review
finds no width or policy changes; exact evidence is in the CI baseline.

The 2026-10-03 hosted checkpoint passes on rustc/cargo 1.99.0, rustfmt
1.10.0-stable, and Clippy 0.1.99. The required configuration and whole-tree
re-audit retains the same policy without source or waiver changes; exact
revision, runner, and review evidence are in the CI baseline. Local checks
remain on rustc/cargo 1.98.0 and do not claim a local 1.99.0 execution.

The 2026-10-17 hosted checkpoint passes on rustc/cargo 1.99.1, rustfmt
1.10.0-stable, and Clippy 0.1.99. The required configuration and whole-tree
re-audit retains the same policy without source or waiver changes; exact
revision, updated runner image, and review evidence are in the CI baseline.
Local checks remain on 1.98.0 and do not claim local 1.99.1 execution.

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

A strict physical-line checker, dependency-policy tool, or toolchain pin
requires its own justified baseline and review. ADR-0023 separately records
the adopted CI configuration and its exact-revision hosted evidence. New and
touched code must not worsen known findings while a gate is deferred. Record any
environment-only failure precisely rather than weakening a threshold or hiding
warnings.

## Git expectations

- Autonomous work occurs only on `main`. The user's 2026-10-31 instruction
  replaces the earlier `codex/nightly` development and publication policy.
- Preserve unexpected local changes and generated work; never reset or discard
  them to obtain a clean tree.
- Make small local commits only after the bounded increment is coherent and its
  applicable checks pass.
- The user authorized source publication without per-push human review on
  2026-09-12. After applicable local verification and autonomous source/diff
  review pass, push `main` to the existing `origin` repository
  (`https://github.com/LunaMeerkats/flight-software-framework.git`) using an
  ordinary fast-forward push. The 2026-10-31 branch direction extends this
  standing permission to `main`, including the accumulated verified commits.
- Refresh remote state before pushing; preserve remote changes and stop on
  non-fast-forward rejection. Verify the remote head and inspect hosted CI
  after publication. Local checks permit the first CI-enabling push; a local
  pass never establishes a hosted pass.
- Source publication does not require a human code review or completed v0.1
  release review. Preserve truthful Codex provenance and actual check results;
  standing permission is not evidence that a human reviewed the code.
- Never force-push, tag, release, publish a crate, deploy, or commit secrets.
- Repository content is licensed under `MIT OR Apache-2.0` with the confirmed
  notice `Copyright 2026 Daniel Smith`; do not change the licence or notice
  without explicit human approval.

This standing permission covers source pushes only. Tag, release,
crate-publication, deployment, licence and the specifically listed consequential
decision gates remain. Routine development beyond v0.1 follows ADR-0025.

## Autonomous milestone review authority

On 2026-10-08 the user delegated routine host v0.1 entry-point and architecture
acceptance to autonomous review in
[ADR-0024](docs/adr/0024-autonomous-host-review-authority.md). Do not solicit
routine human acceptance or stop bounded development because a human has not
reviewed it. Perform the actual review, record the exact revision, reviewer,
evidence, findings, and limitations, and continue verified engineering work.

This delegation can satisfy routine host milestone review gates after their
documented review completes. Permission itself does not complete a review or
establish human-reviewed status. Preserve truthful Codex provenance and the
separate consequential authorization gates recorded in ADR-0024.

ADR-0025 extends routine autonomous engineering and documented milestone review
to the practical-library programme beyond v0.1. Record actual evidence and Codex
review provenance for each later milestone; do not treat direction as completed
review or a blanket approval of the separate consequential decisions.

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
