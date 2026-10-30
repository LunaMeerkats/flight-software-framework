# Selective publication review

Status: Implementation and exact hosted checkpoint verified — 2026-10-31

## Selected consumer and decision

RFF-LIB-REQ-009/ADR-0034 precede source changes. Root Codex and independent
consumer_assessment selected one host-targeted subscribed destination because
ordinary broadcast retry can duplicate an earlier fan-out acceptance. Separate
destination topics change routing vocabulary; subset/report-driven APIs add
unneeded validation/provenance questions. Official Rust 1.99.1 slice/VecDeque
documentation informs checked lookup and FIFO reuse only. The source register
records locally chosen semantics and unavailable version-prefixed page fetches.

MessageBus and MessagingRuntime expose publish_to(application_id, &message),
with typed UnknownApplication/NotSubscribed before lifecycle/capacity inspection.
A valid attempt reuses existing enqueue, affects only the selected subscriber
and creates no report or retry history. Existing all-destination publication and
callback context are unchanged. The test-only fixed mission retains one message,
two IDs and two acceptance flags; its partial scenario uses two slots per inbox
and one host observation cell per sink. It holds the next message until both
accept, then explicitly retries only unaccepted destinations.

## Input evidence

Started clean on codex/nightly at
986ffd82a7a909d10bc2e6cbd50875c98410e2e4, matching refreshed origin. Live exact
[run 40694990149](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/40694990149),
job 122839265392 passes all 20 steps, 208 workspace tests, 14 separate adapter
and five sample tests. Root inspected actual raw checkout/test/tool/image/sample
logs; connector step summaries agree. The combined sample has 222 unchanged
lines. Local Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy 0.1.98 are
unchanged. Hosted input Rust/Cargo 1.99.1, rustfmt 1.10.0-stable, Clippy 0.1.99,
runner 2.338.0, windows-2025-vs2026 image 20261021.278.1 remain distinct evidence.

## Local commands and actual results

Initial/final locked baseline passes with 208/217 workspace tests. All commands
below pass. Source publication and exact hosted evidence are recorded below:

```text
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
git diff --check
git show --format= --check --diff-merges=first-parent HEAD
cargo test --locked --test selective_publication
```

Both documentation invocations use RUSTDOCFLAGS=-D warnings. The nine focused
public-API tests pass. They cover selected routing/peer exclusion, exact full
retention/FIFO, membership/subscription error precedence, current lifecycle,
selected stop/failure clearing, healthy peers, repeated full/unavailable retry,
next-record gating and the difference between acceptance and processing. A
separate regression confirms repeated selected calls can enqueue duplicates.
Original regressions remain unchanged. No benchmark, fuzzing, timing guarantee,
allocator-exhaustion probe or broad model is claimed.

## Actual independent consumer

Actual package outside the checkout:

```text
C:/Users/drrsm/AppData/Local/Temp/rff-selective-cf471a9c14294281ae8128a670452f5b
```

The guide's new/add/copy commands succeeded. With consumer set to that actual
path, these exact command forms pass:

```text
cargo check --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets
cargo clippy --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets -- -D warnings
cargo test --manifest-path "$consumer/Cargo.toml" --locked --offline
cargo metadata --manifest-path "$consumer/Cargo.toml" --locked --offline --format-version 1
Get-FileHash -Algorithm SHA256 tests/selective_publication.rs,"$consumer/tests/selective_publication.rs"
```

All nine copied integration tests pass; cargo new's separate generated smoke
test also passes and is not new framework evidence. Normalized metadata confirms
one independent workspace member, two graph packages, sole framework path
dependency and no framework dependencies. Byte equality and SHA256 match:

```text
tests/selective_publication.rs
11e68ba00b7fdb69afe0eb225b12cb8e621cab862b6a0f96d6c91646d22f8d0a
src/messaging.rs
2451b8d7f2c9808392ce0dd3599bb6555e8ae1a0175424ff0d10382bb7954914
src/messaging_runtime.rs
dbc7c9f82d4725ec69e7e981df426593ed70d53902ec22cca1a952ae3556cc93
src/lib.rs
6d88c161888735f016676d40efd3b8979419b2a5dc928c3983933527cf58a912
```

These are working-file hashes, distinct from the normalized Git blobs below.
No second maintained crate or new example target.

## Source, document and review boundary

Whole-tree audit currently inspects 52 Rust files, 22,398 physical lines and
1,333 comment-only lines: zero width findings and three unchanged fulfilled
item expectations. Function-size acceptance is supplied by warnings-denied
Clippy, not the lexical inventory. Relative links and exact traceability test
references resolve: 99 Markdown files, 563 relative links and 181 exact test
references. All 13 run-changed documents are rendered and checked for matching
text, headings, code, tables, links and malformed markup; this is content and
structure inspection, not pixel or human acceptance. Generated MessageBus and
MessagingRuntime method/error documentation and PublishToError variant content
are inspected. Root and independent consumer_assessment accept the complete
16-file implementation, test, document and evidence diff with no remaining
findings. Review is by Codex under ADR-0024/0025, not human acceptance.

Root and independent consumer_assessment reviewed the implementation and all
nine tests. One guide capacity description was corrected to match the actual
one/two-slot scenarios before acceptance. Independent consumer logs, metadata,
bytes and hashes agree. Final local/evidence/diff acceptance passes. The committed
implementation patch exactly equals the accepted staged diff.
Raw commands/logs and temporary review aids remain in ignored
target/review-2026-10-30-selective-retry (UTC run date); they are not adopted
repository gates. No workflow, dependency, licence or source-quality policy
change is made.

## Exact source publication and hosted acceptance

Ordinary fast-forward publication produced implementation revision
f57819985c3f0320f992c9b237324edc83874733. Refreshed origin and live remote match;
the tree was clean after publication. Root and independent consumer_assessment
accept that exact implementation and evidence under ADR-0024/0025 with Codex
provenance and no remaining findings. Normalized source Git blobs:

```text
src/messaging.rs 785b4e1a1ec4b7c76d0e8f0ca2dd14df77b78761
src/messaging_runtime.rs 70a36a16c65d16ec25da0e5fcac432bd5c0d3753
src/lib.rs f546f6a3703b387b100f71de6964c96d5e15831c
tests/selective_publication.rs af815bb08cc5528057ceb467e00b2ed579121f00
```

Exact [hosted run 41061628585](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/41061628585),
job 124082536243 passes all 20 steps. Root inspected the logged checkout, all
217 workspace tests including the nine new tests, separate 14 adapter/five sample
tests, warnings-denied rustdoc, both whitespace checks and unchanged 222-line
sample output. Connector step summaries agree. Hosted tools, runner and image
match input exactly; no toolchain policy change or local 1.99.1 execution is
claimed. No workflow change requires actionlint in this checkpoint.

This documentation-only completion records actual evidence and receives its
own complete document/diff review, ordinary publication and exact hosted
inspection. Source and consumer bytes remain unchanged. Final completion-head
inspection is recorded in automation memory and the end-of-run report, avoiding
an endless chain of commits recording their own hashes. No further capability
is included in this run.

## Limits and next assessment

A fixed acceptance ledger is mission policy, not a library retry service.
Acceptance is enqueue only: stop/failure can discard accepted copies without
processing. Repeated calls can duplicate; IDs can alias foreign same-position
owners; subscription is routing rather than authentication. No transaction,
eventual processing, fairness, callback-time bound, physical/durable output,
crash recovery, external adoption, API stability or qualification is established.
Separate consequential gates remain. Stop after this one checkpoint. A later
assessment may compare concrete multi-producer coordination or callback feedback
needs with existing public APIs before adding another abstraction.
