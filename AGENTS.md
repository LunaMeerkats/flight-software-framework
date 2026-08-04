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
- `docs/verification/TRACEABILITY.md`: requirement-to-evidence status.
- `PLANS.md`: the active or most recently completed bounded work plan.

There is intentionally no Cargo workspace yet. When implementation begins, add
only the smallest crate layout needed for the selected vertical slice, then
document it here.

## Supported checks

The current documentation-only baseline uses:

```text
git diff --check
```

Also verify that relative Markdown links resolve and review the rendered
documents. Once a Cargo workspace exists, the default required checks are:

```text
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo doc --workspace --no-deps
```

Document any justified adaptation before treating it as the baseline. Never
report a check as passing unless it was run successfully.

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

## Git expectations

- Autonomous work occurs only on `codex/nightly`.
- Preserve unexpected local changes and generated work; never reset or discard
  them to obtain a clean tree.
- Make small local commits only after the bounded increment is coherent and its
  applicable checks pass.
- Do not push unless this file is explicitly updated to authorize pushing.
- Never force-push, merge into the default branch, tag, release, publish a
  crate, deploy, or commit secrets.
- Do not select or change the project licence without explicit human approval.

## Completion

A work item is complete only when one coherent behavior or decision checkpoint
is implemented or recorded, applicable verification has actually passed, the
diff is reviewed, traceability and project state are truthful, and the next run
can continue without relying on chat history.

## Prohibited progress substitutes

Do not add empty crates, placeholder modules, generic traits, broad TODO trees,
speculative portability layers, or dependencies merely to make the repository
look active. Do not widen scope into hardware, no-std, RTOS, broad protocol, or
deployment work before the host framework has a reviewed, verified need.
