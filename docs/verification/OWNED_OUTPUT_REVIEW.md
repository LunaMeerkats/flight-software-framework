# Application-owned output handoff review

- Date: 2026-11-23 Australia/Sydney
- Reviewer: Codex; independent Codex source review accepts the exact source below
- Scope: RFF-LIB-REQ-019 revision 1 and
  [ADR-0044](../adr/0044-application-owned-output-handoff.md)

## Input and selected need

Clean main `356697a288d11fc168e450cff0d2579f363557d1` matches refreshed
origin/main. Exact input hosted
[run 44959878838](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/44959878838),
job 137016113797, completed all 20 steps successfully. No unfinished work
was absorbed. The preceding shared-inspection checkpoint is complete.

The pre-edit locked baseline passed 296 tests and all six commands below.
Local tools remain rustc/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy
0.1.98. Tracked max_width 100, comment review width 80, individual
too_many_lines threshold 60 and unsafe-code prohibition remain unchanged.
No new library API, dependency, workflow or lint is selected.

The integration gap is ownership transfer of an application-owned non-Copy
record. Existing borrowed output recipes and shared observations stay intact.
Cell/RefCell, borrowed slots, mutable access and a framework output service
were compared before implementation in ADR-0044. Official std Cell/Option
facts are recorded in `SRC-RUST-OWNED-OUTPUT` in the source register.

## Commands and evidence boundary

Required baseline, before and after implementation:

```text
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
git diff --check
```

PowerShell sets `$env:RUSTDOCFLAGS = '-D warnings'` before rustdoc.

All six final commands pass, with 304 passing workspace tests (296 input
plus eight new example tests). Focused commands also pass:

```text
cargo check --locked --example owned_output
cargo clippy --locked --example owned_output --tests -- -D warnings
cargo test --locked --example owned_output
cargo run --locked --example owned_output
```

An initial draft failed compilation because its host closure expected unit
instead of Runtime::work's returned ApplicationState. The signature was
corrected before the passing focused and full checks; no policy was weakened.
There were no final check failures or environment exceptions.

## Consumer and test result

`examples/owned_output.rs` contains the complete public-only consumer. Its
Output implements neither Copy, Clone nor Default. Cell::take moves the
optional record; callback staging uses an exclusive get_mut borrow and
rejects a full slot. The Output includes the existing work-call ordinal,
so replacement during the second rejected callback is observably wrong.
One host slot defers work and taking, retains exact rejected ownership and
reopens work through the same host gate after acceptance.

The eight [traceability histories](TRACEABILITY.md#stage-5-application-owned-output-handoff--2026-11-23)
pass. They cover both owners, empty/repeated take, Stopped take, stop/restart
retention, full-slot typed failure, Failed drain, selected inbox clearing,
preserved peer queues/work/dispatch and an exact seven-line runnable trace.
No historical consumer or library source changes.

Source is 16,869 bytes / 510 lines, SHA256
`e1c9057374e1fec611140e13a9ef120c90fc6122b4f0b02be3160ecdd0fc50ca`.
Normalized Git blob is `2acd8a7f56992cdcbebe1b208251f564ce51b753`.

## Independent binary

Executed the [guide recipe](../OWNED_OUTPUT.md#verify-identical-source-independently)
at `C:/Users/drrsm/AppData/Local/Temp/rff-owned-output-70a15e6081e6456b8dbd16dc3185e2c2`.
After cargo new/add, these commands all pass, with that package's absolute
Cargo.toml supplied to every manifest-path argument:

```text
cargo check --manifest-path <consumer>/Cargo.toml --locked --offline --all-targets
cargo clippy --manifest-path <consumer>/Cargo.toml --locked --offline --all-targets -- -D warnings
cargo test --manifest-path <consumer>/Cargo.toml --locked --offline
cargo run --manifest-path <consumer>/Cargo.toml --locked --offline
cargo metadata --manifest-path <consumer>/Cargo.toml --locked --offline --format-version 1
```

All eight identical tests pass. Metadata identifies one independent edition
2024 binary member, two graph packages and exactly one framework path
dependency. Source bytes and SHA256 match exactly. Native stdout matches
the workspace example: seven lines / 234 bytes, SHA256
`1ca8f41b7bdd4bc84d173c08c587a54e9e7f2d8814681cdfcfff3ba4b803cb08`.
No maintained downstream crate was added.

## Review and disposition

Root and independent Codex source review accept the exact source hash and
Cargo target relative to input `356697a288d11fc168e450cff0d2579f363557d1`.
Both checked ownership, gate ordering, full-slot preservation, selected and
peer lifecycle/inbox behavior, typed rejection and test strength. No source
findings remain. Root accepts the complete source/document diff. Independent
Codex final document/diff acceptance is recorded before publication.

Whole-tree audit passes: 59 handwritten Rust files, 28,676 physical lines
and 1,519 comment-only lines, with zero width findings and three unchanged
fulfilled lint expectations. All 782 relative paths and 87 fragments across
127 Markdown files resolve. All 264 exact traceability test references
resolve uniquely to tests. All 12 changed Markdown documents pass rendered
HTML content/structure inspection: prose, headings, code, links and tables
match, with no malformed nesting or replacement characters. This is not
browser pixel QA. Temporary aids remain review evidence, not adopted gates.

This verifies this serial consumer. The sink contract requires definite
rejection before acceptance/external effects and return of the whole record.
Callbacks/sinks must return. Printing errors exit the fixed script; no
process-exit retention, reliable transport, durability, concurrency, recovery,
external adoption, API stability or flight suitability is established.

## Publication

Not yet committed or pushed. Local checks do not establish hosted success.
Exact tested revision and hosted result will be recorded after publication.
Raw local logs and temporary review aids live under ignored
`target/review-2026-11-23-output-access/`.
