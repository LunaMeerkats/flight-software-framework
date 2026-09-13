# Stage 4 dependency and scope review

Date: **2026-09-13**
Status: **Autonomous evidence checkpoint; human v0.1 acceptance pending**

## Reviewed boundary

Reviewed input: `c472c57c1aa241d821b891674ba25f42b25ae723` on `codex/nightly`.
The working tree was clean and equal to refreshed origin. This checkpoint
changes documentation only, including the crate-level rustdoc notice; runtime
behavior, tests, manifests, lockfile, licence files, workflow, and lint policy
remain unchanged. Review/corrections commit:
`e17c5d3363092dbe55ba1fc6bb94a6e66956c667`.

Codex performed the review, with independent dependency/licence and scope
audits. No human code review, RFF-REQ-001 acceptance, v0.1 architecture approval,
public API freeze, or release authorization is established by it.

## Dependency and feature inventory

The following commands completed successfully on Cargo 1.98.0:

```text
cargo metadata --locked --format-version 1 --all-features
cargo tree --locked --workspace --all-features --target all --edges all
cargo tree --locked --workspace --no-default-features --target all --edges all
git ls-files
git check-attr text eol -- LICENSE-MIT LICENSE-APACHE
```

The manifest, version-4 lockfile, full metadata, and both trees agree:

| Reviewed input | Observed result |
| --- | --- |
| Package/workspace | One package, `rust-flight-framework 0.0.0`, edition 2024 |
| Declared/resolved Cargo dependencies | Zero external normal, development, build, optional, or platform packages; one root resolve node |
| Package/enabled features | Empty feature map and empty resolved feature list, including the all-features query |
| Targets | One library, one example, 15 integration-test targets; no custom-build or procedural-macro target |
| Tracked build inputs | No nested manifest, vendor directory, Cargo configuration, build script, or external source/native-link directive found |
| Publication/licence metadata | `publish = false`, metadata `publish: []`, `MIT OR Apache-2.0`; no MSRV declared |

Metadata was queried without `--no-deps` or a platform filter. The explicit
all-target trees corroborate it; Cargo documents that a tree alone is not an
exact description of every compiler invocation. (`SRC-RUST-CARGO-GRAPH`)
This is an empty external Cargo graph, not a dependency-free execution system.

## Licence and tooling boundary

[ADR-0002](../adr/0002-project-name-and-licensing-intent.md) records the human
licence and holder decision. The two tracked licence files and Cargo expression
remain unchanged. The MIT body matches the current OSI terms on content review,
allowing line wrapping, quote typography, and the approved
`Copyright 2026 Daniel Smith` notice. The Apache terms match the ASF text after
trimming: the current remote text has one extra leading newline (11,358 versus
11,357 local bytes). This is content agreement, not byte-identical upstream
evidence. Git attributes report `text: set` and `eol: lf` for both files.
(`SRC-OSI-MIT`, `SRC-ASF-APACHE-2.0`, `SRC-RUST-CARGO-LICENCE`)

The compiler, standard library, linker, operating system, Git, PowerShell,
hosted runner, and maintenance tools are outside the Cargo dependency graph.
The workflow's sole external action is checkout at
`3d3c42e5aac5ba805825da76410c181273ba90b1`; its top-level licence is MIT.
Its bundled dependencies and the full toolchain/runner distribution were not
audited here. The action is referenced, not vendored into the Rust library.
The existing temporary actionlint validator is neither a Cargo dependency nor
a CI step. (`SRC-GITHUB-HOST-CHECKOUT`, `SRC-ACTIONLINT`)

No external Cargo package needs a feature/licence adoption decision at this
revision. Retain direct review of this small inventory; adding a dependency
policy tool now would create a separate maintenance boundary without a package
finding to resolve. A future checker remains a separately justified increment.
This review does not establish vulnerability freedom, legal clearance, or
redistribution clearance for toolchain/CI bundles.

## Scope entry points and findings

| Entry point | Review and result |
| --- | --- |
| README and charter | Explicit experimental, non-flight, safety, affiliation, TRL, operational, and compatibility limits; clarify that the empty dependency graph is Cargo-specific |
| Requirements, architecture, contributor guidance, ADRs, current state | Controlled-input repeatability is defined; human review gates remain; correct ADR-0022's stale CI status and replace its link to the rolling plan with durable sample evidence |
| Cargo description and generated crate landing page | Experimental host package; extend the crate notice to explicitly name TRL, human-rated use, cFE, OSAL, and PSP consistently with README |
| Public generated API documentation | 63 root HTML pages: landing/all-items indexes, 28 enums, 30 structs, three traits; scan item docblocks and inspect Runtime, MessagingRuntime, WorkSchedule, and Clock in detail |
| Sample source, guide, and executed stdout | Experimental notice, generic no-compatibility statement, simulated-time and no-physical-delivery labels; guide/source enumerate the unsupported systems |

Private-module rustdoc redirect pages are not extra public entry points. The
autonomous scan found no positive unsupported flight, compatibility, real-time,
or fault-tolerance claim in this inventory. The crate notice change makes
existing limitations explicit; it does not repair a demonstrated positive
claim or establish the required human release review.

Retain these limits in human review: logical retained-record capacities do not
bound total process memory, caller-owned reports, callback allocations, stack
temporaries, or execution duration. Returned errors do not prove panic/hang
containment. In-place restart from `Stopped` does not recover terminal `Failed`.
Manual-clock replay is not a deadline guarantee. Returned telemetry arrays are
not physical delivery. The API remains pre-v0.1 and caller-scoped identities
do not encode issuer provenance.

## Verification and acceptance boundary

The initial required baseline passes with 115 tests on rustc/Cargo 1.98.0,
rustfmt 1.9.0-stable, and Clippy 0.1.98. The source-policy configuration is
unchanged. `cargo run --locked --example host-echo` completes with the documented
three command records, equal-time work at 10 ms, configuration revisions 2/1/3,
and the 20 ms cooperative failure, one inbox discard, and retained peer work.

After the crate-notice correction, all of these commands pass again:

```text
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
git diff --check
```

Rustdoc ran with `RUSTDOCFLAGS=-D warnings`. All 115 tests pass, including 14
adapter and five sample tests; there are zero Cargo doctests. No focused host
rerun, actionlint run, toolchain installation, or standalone ADR-0018/0019 probe
was needed: those source/workflow boundaries did not change.

The 32-file Rust audit finds zero physical/comment width issues and the same
three fulfilled function-length expectations; there are no block comments to
exclude from this scan. Relative-link, requirement/source-ID, and all 79 exact
traceability test-reference checks pass. Generated Markdown content was compared
with its source, and changed document layouts plus the generated crate notice
were inspected in the browser at 1280 pixels. No page/table overflow was found.
Screenshots of the review and crate landing pages were inspected.

Independent complete-diff review found one evidence-link regression when the
rolling plan was replaced. The CI baseline now links to the completed plan at
the immutable reviewed commit. The sample ADR instead links to its durable
sample guide and traceability. No runtime or source-policy defect was found.
Temporary commands and logs are under `target/review-2026-09-13`; the local
rendering/checking aids are not adopted repository gates.

This checkpoint supplies the bounded dependency/features review and scope
inventory. [Traceability](TRACEABILITY.md) keeps RFF-REQ-001 pending human
review. A resource/failure-path/public-API audit and human entry-point and
architecture acceptance still precede Stage 4 completion or scope expansion.
Earlier hosted results apply only to their recorded revision.

Revisit when manifests, lockfile, features, vendored/build inputs, licence
content, toolchain/CI choices, public documentation entry points, or externally
observable behavior change. Audit new dependencies before adoption; do not
project this empty-graph result onto a later graph.

## Publication evidence

Published `e17c5d3363092dbe55ba1fc6bb94a6e66956c667` by ordinary fast-forward
from `c472c57`; the remote branch head matched. Refreshed ancestry and both
working/staged whitespace checks passed before publication.

[Hosted run 34727157216](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/34727157216)
is a `push` event for `Host Rust baseline`, with workflow and checkout at the
same exact revision. It completed successfully: one Windows job, all 20 steps,
115 workspace tests, 14 focused adapter tests, five focused sample tests,
format/check/Clippy, warnings-denied rustdoc, executable sample, and both Git
whitespace checks, including checkout/bootstrap and cleanup.

The logged image is `windows-2025-vs2026` version `20260907.229.1`, runner
2.337.0. Hosted rustc/Cargo are 1.98.1, rustfmt 1.9.0-stable, Clippy 0.1.98;
this matches the prior hosted toolchain/image. The 32-file source-policy audit
and new notice inspection accompany the all-target checks. No toolchain pin or
human acceptance is implied. Logs and job/step JSON were inspected and retained
under the temporary review directory.

This documentation-only follow-up records the completed checkpoint result;
later revisions require their own CI inspection. Source/manifest/workflow
content is unchanged from the verified corrections commit.
