# Configuration-operation diagnostics review

Date: **2026-10-01**
Scope: **Existing ADR-0017/0018 runtime configuration errors**

## Decision and inspected boundary

Retain the runtime-owned configuration operations selected by
[ADR-0017](../adr/0017-bounded-configuration-snapshots.md) and
[ADR-0018](../adr/0018-configuration-aware-work-context.md) while executing
their selected standard-error source shapes through public API tests. No
production defect was found, and this checkpoint changes no configuration
state transition, error type, public method, validation rule, or rollback
policy.

`RuntimeConfigurationError::NotConfigured` is a terminal structural cause and
therefore has no `std::error::Error::source`. The `Table` variant exposes its
exact `ConfigurationError`; semantic rejection then exposes the concrete
mission validation error, while `NoRollbackAvailable` is itself terminal.

## Executed public observations

`unconfigured_work_reports_absence_and_rejects_table_operations` now proves
that replacement and rollback both return the exact `NotConfigured` value and
terminate their standard source chains without a fabricated cause.

`work_observes_activation_rejection_rollback_and_revision_non_reuse` now
traverses both table-backed paths:

- semantic rejection exposes `RuntimeConfigurationError -> ConfigurationError
  -> MissionValidationError`; and
- a second rollback exposes `RuntimeConfigurationError -> ConfigurationError`
  and terminates at `NoRollbackAvailable`.

The unchanged remainder of that scenario proves that rejected validation
retains active revision 2, rollback restores revision 1, and the next accepted
replacement receives revision 3. The evidence observes diagnostics and state
together; it does not add automatic recovery or rollback.

## Evidence limits

These tests execute absence, one concrete mission-validation failure, and
consume-once rollback exhaustion. They do not exhaust every
`ConfigurationError` variant through the runtime owner, inject validator
panics, define logging policy, stabilize the public API, or establish human
architecture acceptance. The configuration remains an in-memory host
experiment with no selected schema, wire format, or loader.

## Verification

The initial and final locked baselines pass 134 tests on Rust/Cargo 1.98.0,
rustfmt 1.9.0-stable, and Clippy 0.1.98. The focused configuration-runtime
target passes 11 tests. Formatting, all-target checking, warnings-denied
Clippy/rustdoc, and whitespace checks pass without a new lint exception.
Rustdoc uses `RUSTDOCFLAGS=-D warnings`.

The whole-tree audit passes 32 Rust files with zero physical or comment-width
findings, no block comments, and three unchanged fulfilled expectations. All
54 Markdown files and 255 relative links pass the link audit; changed Markdown
passes PowerShell structural rendering. Pixel-level visual acceptance is not
claimed.

```text
cargo test --locked --test configuration_runtime
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
git diff --check
```
