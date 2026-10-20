# Independent application composition review

Date: **2026-10-21**
Status: **Local consumer and complete review passed; publication/hosted CI pending**

## Objective and input

RFF-LIB-REQ-002 revision 1 enables a consumer to compose two independently
implemented applications through public APIs, preserving distinct concrete
message errors. Clean input f539755e50b0f15aaa4361cba2e2eae4e7b3dca9 matches
refreshed origin. Exact [input run 39258514095](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/39258514095)
passes its logged checkout, job 118079807809 and all 20 steps. Fresh required
locked local baseline passes 154 workspace tests and both whitespace checks.

Root Codex and independent consumer_assessment identify an undocumented public
composition pattern: the private sample's inner application structs do not
independently implement the framework traits. [ADR-0027](../adr/0027-independent-application-composition.md)
compares explicit mission enum forwarding, a public sum adapter, trait objects
and broader ordinary-work service access. The selected consumer requires no
new runtime API. Official Rust enum/Error and Cargo example documentation is
registered under SRC-RUST-TYPED-COMPOSITION and SRC-CARGO-CONSUMER-RECIPE.

## Implementation and evidence boundary

[composition.rs](../../examples/composition.rs) independently defines Relay and
Sink, each implementing Application and MessagingApplication. MissionApplication
forwards all five callbacks. Infallible lifecycle/work methods use the same
associated error; distinct RelayError and SinkError map to MissionError variants
and concrete Error::source causes. The host selects all operations.

Two records have relay inbox capacity two, sink capacity one, one-byte payloads
and one borrowed host output slot. A command traverses relay publication, sink
dispatch and host drain. Stop/restart retains this composition and it processes
42 then 7. Small caller-owned status flags make delegated lifecycle/work effects
observable; they are not authoritative runtime lifecycle state.

The failure cases distinguish a relay's rejected callback publication from a
sink's occupied host output. Selected failure/cleanup preserves healthy peer
progress. The source and guide explicitly retain terminal Failed, immediate
nontransactional publication and in-process observation limits. No upstream code,
new dependency, library behavior or previous test is changed.

## Verification and review

Source Git blob: `8fb7dbfc8b793f23dd96d042cd3fb6eaf1049073`.
Source SHA256:
`12eb88546081c9c0a67de4186cec0e73a9f9f6c893693df237db350bc7d8aef1`.

The test-enabled example runs four regressions in the unchanged CI workspace
command:

- `distinct_applications_forward_lifecycle_work_and_messages`: caller-owned
  flags observe each delegated start/work/stop/restart, retained work observation,
  message delivery and the complete 42/7 scenario.
- `relay_failure_preserves_typed_source_and_peer_delivery`: a full sink rejects
  the relay publication. The exact report identifies that peer and InboxFull;
  the relay enters Failed and one additional queued command is discarded,
  excluding the in-flight item. The healthy sink still dispatches retained 9.
- `sink_failure_preserves_older_output_and_leaves_relay_operable`: full host
  output returns the distinct SinkError, retains 9, and leaves relay work and
  stop/restart available. No additional sink input is discarded in this case.
- `both_applications_reject_invalid_input_before_output`: empty and [101] input
  each fail through both application variants before publication/output effects;
  exact errors, terminal selected state and healthy peer state are observed.

Both saturation regressions downcast the actual framework wrapper's source chain
through MessageDispatchError, MissionError and the concrete application error,
while separately checking the selected runtime-local application identity.

Required local commands pass:

```text
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
git diff --check
git show --format= --check --diff-merges=first-parent HEAD
cargo test --locked --example composition
cargo run --locked --example composition
cargo test --locked --example quickstart
cargo run --locked --example quickstart
```

Rustdoc uses RUSTDOCFLAGS=-D warnings. Initial/final workspace totals are 154/158;
focused composition passes four, quickstart three, and both executables pass.
The composition's three stdout lines exactly match the guide. Local tools remain
Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy 0.1.98. Combined host source,
workflow and ADR-0018/0019 probes are unchanged, so their extra conditional local
commands do not apply. The quickstart source and setup commands are unchanged;
its guide receives only a navigation link. Existing hosted checks remain separate.

Actual external package:
`C:/Users/drrsm/AppData/Local/Temp/rff-composition-e744314ae7d44639b0b0c178e04c3700`.
The [guide's independent commands](../COMPOSING_APPLICATIONS.md#verify-the-independent-package)
are executed with $framework set to this checkout and $consumer set to that
unused temporary path:

```powershell
cargo new --bin --edition 2024 --vcs none $consumer
cargo add --manifest-path "$consumer/Cargo.toml" rust-flight-framework --path $framework --offline
Copy-Item -LiteralPath "$framework/examples/composition.rs" -Destination "$consumer/src/main.rs"
cargo check --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets
cargo clippy --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets -- -D warnings
cargo test --manifest-path "$consumer/Cargo.toml" --locked --offline
cargo run --manifest-path "$consumer/Cargo.toml" --locked --offline
cargo metadata --manifest-path "$consumer/Cargo.toml" --locked --offline --format-version 1
Get-FileHash -Algorithm SHA256 "$framework/examples/composition.rs", "$consumer/src/main.rs"
```

All commands pass, including four independent tests. Metadata inspection confirms
one independent workspace member, two graph packages, the consumer's sole path
dependency edge and no library dependencies. Source hashes match byte for byte;
both stdout forms match all three guide lines. This generated package is not
another maintained framework crate.

Root Codex reviews the complete source, metadata/hashes and raw local logs.
Independent Codex composition_review accepts the exact source blob above,
independently runs its four focused tests and accepts external manifest/lockfile,
metadata, matching source hashes and raw check/Clippy/test/run logs. Review requests
for full outer-cause assertions, malformed-input coverage and exact inbox limits
are resolved before acceptance. This is autonomous Codex review, not human review.

Whole-tree audit covers 45 Rust files, 17,991 physical and 1,155 comment-only lines,
zero width findings and three unchanged fulfilled expectations. Rendered Markdown
content/structure inspection checks full text, headings, code blocks, links,
tables and HTML nesting. Generated crate entry-point documentation is inspected.
All 429 relative links and 122 exact traceability test references resolve.
Twelve run-changed rendered documents pass full content/structure review; this
does not claim pixel or human review. Root and independent composition_review
accept the complete source/document/diff and raw local evidence with no blocking
findings. The final module-comment clarification preserves behavior; its source
identity and copied consumer are checked again. Exact published hosted results
remain a separate pending gate for this checkpoint.

Raw logs and temporary review aids are under
`target/review-2026-10-21-composition`. These are generated review output, not a
new adopted checker. An early in-progress inventory correctly flagged two
overlong comments and not-yet-created record links; both are resolved. An early
consumer inspector ran before the workspace run log existed and was rerun after
completion. Final whitespace review caught an extra plan EOF blank line; it was
removed and both whitespace checks pass. These were review-aid/document findings,
not suppressed runtime failures.

## Limits and next candidate

This is a controlled fixed mission, not external adoption or general readiness.
Topic and payload vocabulary is shared; different lifecycle/work error mapping,
dynamic registration and arbitrary third-party interoperability remain untested.
Logical capacities do not bound allocator use or callback duration. Panics/hangs,
physical output and general recovery remain outside the evidence. Current
consequential authorization gates remain unchanged.

Assess a concrete ordinary-work producer next if host-mediated output materially
obstructs its use; compare it with a narrow public service capability. This run
does not implement that separate objective or establish a generic adapter need.
