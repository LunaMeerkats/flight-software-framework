# Returned-stop failure-event review

Date: 2026-11-26. Reviewers: root Codex and an independent Codex agent.
Scope: RFF-LIB-REQ-021 revision 1 and ADR-0046.

## Input and objective

Clean main `8fca981837327828b3caf6245984f5fa639e4fd6` matches refreshed
origin/main. The locked input baseline passes 316 workspace tests, format,
all-target check/Clippy, warnings-denied rustdoc and whitespace. Exact input
[hosted run 45556097802](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/45556097802),
job 139008394972, passes all 20 steps with that exact logged checkout.
Local Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy 0.1.98 are unchanged.
Hosted Rust/Cargo 1.100.0, rustfmt 1.11.0-stable, Clippy 0.1.100, runner
2.338.2 and Windows image `20261116.315.1` match the preceding checkpoint.

Add opt-in stop diagnostics through each existing owner while preserving
ordinary stop and operation-specific success/error types. The concrete
consumer need is reporting shutdown failure after nonempty inbox cleanup.
No dependency, workflow, lint or permanent service ownership changes.

## Local commands and result

All commands below complete successfully on the implementation:

```text
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --test stop_failure_events
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
git diff --check
```

RUSTDOCFLAGS is `-D warnings` for rustdoc. The full suite passes 324 tests;
all eight focused public-only histories pass, as does focused target Clippy
with `--locked --test stop_failure_events -- -D warnings`. No new waiver,
checker baseline or toolchain policy is introduced.

Tests preserve direct and messaging success types, success clearing, all
non-running/unknown rejection paths, original non-Copy errors and source/
ownership accessors. Stop failure clears exactly two selected deliveries;
peer FIFO [11, 22] and later work remain available. One captured timestamp,
source/severity/identifier and Recorded outcome are exact. Saturation retains
the older event and original error; explicit retry after time advances keeps
the original timestamp. Repeated failed stops create no duplicate. The
receiving-owner equal-slot alias remains a documented identity limitation.

## Independent consumer

Executed the [guide recipe](../STOP_FAILURE_EVENTS.md#verify-identical-source-independently)
at `C:/Users/drrsm/AppData/Local/Temp/rff-stop-failure-events-6b031fb65d1d4104a08b9cbc7b83ff83`.
After cargo new/add, every command below passes with that absolute package
path substituted for `<consumer>`:

```text
cargo check --manifest-path <consumer>/Cargo.toml --locked --offline --all-targets
cargo clippy --manifest-path <consumer>/Cargo.toml --locked --offline --all-targets -- -D warnings
cargo test --manifest-path <consumer>/Cargo.toml --locked --offline --test stop_failure_events
cargo metadata --manifest-path <consumer>/Cargo.toml --locked --offline --format-version 1
```

All eight identical integration tests pass. Metadata confirms one independent
edition 2024 library member, its sole framework path dependency, two graph
packages and no framework dependencies. Test source bytes match exactly:
21,252 bytes / 617 lines, SHA256
`0635a216ee229c38b3fa8e75a9794c96b9c504bd8499f67ef312c0141efb39bc`.
The temporary package supplies controlled downstream evidence, not another
maintained crate or proof of general adoption.

Raw commands/logs are retained under ignored
`target/review-2026-11-26-stop-events/`. The initial test draft referenced
an unavailable report convenience method; exact public outcome inspection
replaced it before focused and final checks. No required baseline check fails.

## Review boundary

Source inspection confirms each method delegates once through ordinary stop.
The messaging owner completes terminal commitment and selected cleanup before
the event constructor reads the clock. Public tests can observe post-return
state/counts; they do not reborrow the mutably borrowed owner inside the clock.
Root and independent Codex accept the complete source/test/design/guide review
without findings. Whole-tree audit passes 61 Rust files / 30,727 physical and
1,599 comment-only lines with zero width findings and three unchanged fulfilled
expectations. All 843 relative paths, 99 fragments and 284 unique exact test
references across 133 Markdown files resolve. All 12 changed documents pass
rendered content/structure inspection and independent manual review. This does
not claim browser pixel QA. Temporary aids and review records live under the
ignored evidence folder's audit directory; they establish no new checker gate.

## Publication status

Local verification and complete source/document/diff review are accepted by
Codex. Commit and authorized fast-forward publication follow; hosted success
is not inferred from local checks. Exact revision and completed hosted evidence
will be recorded after publication.

Callbacks and clocks must return. The event is a framework observation after
cleanup, not the application-internal fault instant. Reporting supplies no
retry, recovery, rollback, origin validation, reserved capacity or guaranteed
delivery. Host-retained copies need their own finite bound. Controlled
consumer evidence does not establish adoption, stability or qualification.
