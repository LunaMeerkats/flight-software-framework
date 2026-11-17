# Periodic peer work review

Date **2026-11-18**. Scope [RFF-LIB-REQ-017 revision 1](../REQUIREMENTS.md#rff-lib-req-017--continue-periodic-peer-work-after-a-returned-failure)
and [ADR-0042](../adr/0042-periodic-peer-work-after-failure.md).

## Input and local verification

Input `ca32a428314055f1b2f4eae5214462c6ab129fdb` matched refreshed
`origin/main`. Its [exact hosted run 43873841092](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/43873841092),
job 133321631099, passed all 20 steps. The local input passed 276
workspace tests.

The following checks passed on Rust/Cargo 1.98.0, rustfmt 1.9.0-stable
and Clippy 0.1.98. `RUSTDOCFLAGS=-D warnings` applied to the doc build.

```text
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo test --locked --example periodic_peer
cargo clippy --locked --example periodic_peer -- -D warnings
cargo run --locked --example periodic_peer
cargo test --locked --test host_adapters
cargo test --locked --test host_sample
cargo doc --locked --workspace --all-features --no-deps
cargo run --locked --example host-echo
git diff --check
```

There were 278 workspace tests, two focused example tests, 14 focused
adapter tests and five focused sample tests. The combined host sample
reproduced the input's 222 lines. The focused tests verify successful
failure-event admission and queue-full rejection in separate histories.
Across both, the selected application is Failed and its cursor is disabled.
The admitted history measures the original peer release and three successful
heartbeat callbacks across a separate diagnostic drain. The rejected history
verifies the original cause, rejected event attribution and retained warning.

## Independent binary

The [consumer recipe](../PERIODIC_PEER.md#verify-the-independent-binary)
passed at
`C:/Users/drrsm/AppData/Local/Temp/rff-periodic-peer-c5f188ddb13b4ffa8c5ae450326913ba`.
Locked offline check, Clippy, both tests, run and metadata succeeded.
Metadata confirmed one edition 2024 binary workspace member and only
the framework path dependency, with two packages in the resolved graph.
The 9,878 source bytes matched with SHA256
`c26ddd18dce5c8d4e631971a1b89a91a801b0034bd9b19609c3e52993bf4da2e`
and normalized Git blob `022c48c3ce3441018cd16e2d30294b55b5155fed`.
Both binaries emitted identical six-line, 233-byte stdout with SHA256
`b418960ccf5780482ee6b48a1362e57fa6e1302613a7120058fafa024211b402`.

## Review scope

Codex and an independent Codex reviewer accepted the complete implementation
diff. Review covered 57 Rust files, 27,205 physical lines and 1,411
comment-only lines with zero width findings and three unchanged fulfilled
Clippy expectations. All 726 relative paths, 74 fragments and 242 exact
test references resolved. The 11 changed Markdown files passed rendered
content and structure checks.

The comparison is limited to this manually advanced serial host with two
application records and one terminal failure. The host must keep polling the
healthy cursor, and callbacks and writes must return. The acceptance evidence
covers event storage and local dequeue.

Raw local results are under the ignored `target/review-2026-11-18-peer-work`
and `target/review-2026-11-18-document-audit` directories.
