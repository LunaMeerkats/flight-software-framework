# Lifecycle construction and registration review

Date: **2026-09-21**
Scope: **Existing ADR-0003 and ADR-0006 construction and registration bounds**

## Decision and inspected boundary

Retain the current standalone lifecycle registry and owned runtime construction
design while strengthening their public observations. No production defect was
found in `src/lifecycle.rs` or `src/runtime.rs` at starting revision
`7cabe599d88fa5a7f3ce74b2e35fac33488aa711`. This checkpoint supplements
RFF-REQ-002, [ADR-0003](../adr/0003-stop-gated-lifecycle-and-runtime-local-identity.md),
and [ADR-0006](../adr/0006-static-application-ownership-for-initial-runtime.md);
it changes none of their behavior or identity policy.

Both constructors reject zero before calling `Vec::try_reserve_exact` for the
complete logical application limit. Reservation failure maps to a typed error
that retains the requested count. Successful construction returns an empty
owner with its fixed logical capacity. These are source-inspected facts.

Registration checks the logical limit before pushing a record. The standalone
registry changes no retained state when full. The owned runtime returns the
rejected application value without invoking it or changing existing records.
Successful identities use the next record position, remain stable because
there is no removal, and are scoped by caller contract to their issuing owner.
Equal-position keys from different owners can still alias; this review does not
change or strengthen that documented policy.

The Rust 1.98 standard-library documentation under `SRC-RUST-VEC-RESERVATION`
in the [source register](../research/SOURCES.md) states that
`Vec::try_reserve_exact` returns an error for capacity overflow or allocator
failure and that an allocator may provide more capacity than requested. Keep
the explicit logical record limit independent of actual allocation capacity.
No custom allocator, unsafe oversized fixture, or API change is introduced.

## Executed public observations

The two regressions are:

- `oversized_registry_capacity_returns_exact_reservation_error`: requesting
  `usize::MAX` logical lifecycle records returns exactly
  `RegistryCreateError::CapacityAllocationFailed { requested: usize::MAX }`.
- `oversized_runtime_capacity_returns_exact_reservation_error`: requesting
  `usize::MAX` owned runtime records returns exactly
  `RuntimeCreateError::CapacityAllocationFailed { requested: usize::MAX }`
  through the unconfigured public constructor.

These impossible requests exercise deterministic capacity overflow for the
nonzero-sized record types. They do not exhaust available heap memory or
distinguish allocator failure from capacity overflow after the framework maps
the standard-library error.

Existing tests remain the ownership and mutation evidence. They cover zero
capacity, exact logical saturation, unchanged retained lifecycle records,
returned application ownership, complete LC1 transitions, configured-runtime
construction failure with unchanged table lineage, and unknown identities.
The configured and unconfigured runtime constructors share one private record
reservation helper; both public paths now execute its typed oversized-capacity
result.

## Resource and evidence limits

The registry and runtime retain at most the configured number of framework
records after successful construction. That logical count does not bound
application-owned memory, callback or error allocations, caller-retained
values, stack use, or execution duration. This run does not instrument actual
allocation bytes, deallocation, or an allocator-reported out-of-memory event.

No origin-bearing identity, deregistration, slot reuse, dynamic application
loading, panic/hang containment, concurrency, real-time guarantee, recovery
from `Failed`, or production-space suitability is added or established. Human
v0.1 acceptance and the wider Stage 4 review remain open.

## Verification

The initial locked baseline passes 122 tests on Rust/Cargo 1.98.0, rustfmt
1.9.0-stable, and Clippy 0.1.98. The focused command
`cargo test --locked --test lifecycle_registry --test application_runtime`
passes five and eleven tests respectively. The final locked baseline passes 124
tests and every command below.

```text
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
git diff --check
```

Rustdoc uses `RUSTDOCFLAGS=-D warnings`. Host adapters, sample, workflow, and
ADR-0018/0019 probes are unchanged, so their separate local commands are not
triggered. The full suite includes both host test targets. The whole-tree audit
passes 32 Rust files with zero physical/comment width findings and three
unchanged fulfilled expectations, plus 44 Markdown files, 194 resolving
relative links, and 88 exact traceability test references. Complete source and
diff review found no blocking defect. GitHub GFM rendering of all seven changed
documents produced nonempty HTML with matching heading and fenced-code-block
counts. The following section records the subsequently completed publication
and hosted CI evidence.

## Published checkpoint and continuation

Commit `15e4166bbead2d824e732460fa1a67266386f648` contains both regressions
and the reviewed checkpoint. Ordinary fast-forward publication succeeded and
the remote head matched. On 2026-09-20 UTC (2026-09-21 Sydney),
[hosted run 35534767664](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/35534767664)
passed that exact push and checkout: one Windows job and all configured steps.
Logs confirm 124 workspace tests including both new regression names, 14
focused adapter tests, five focused sample tests, the sample executable, and
every configured baseline command.

Actual runner: 2.337.0; image: `windows-2025-vs2026` version
`20260907.229.1` (requested label `windows-2025`). Actual Rust/Cargo: 1.98.1;
rustfmt: 1.9.0-stable; Clippy: 0.1.98. This matches the previously reviewed
hosted environment. The all-target hosted baseline and companion whole-tree
source review preserve existing policy without a new waiver. Local Rust/Cargo
remain 1.98.0.

This documentation-only follow-up records the completed result with unchanged
Rust, Cargo, workflow, and lint inputs. It does not establish a hosted pass for
its own later revision or human v0.1 acceptance. The next bounded task should be
selected from a reconciled service resource, failure, or public-API contract
gap. Broader scope remains unapproved.
