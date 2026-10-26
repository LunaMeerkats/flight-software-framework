# Explicit event filtering review

Date: **2026-10-27**
Status: **Verified, reviewed and published; exact implementation hosted CI passes**

## Objective, contract and source identity

RFF-LIB-REQ-006 revision 1 and
[ADR-0031](../adr/0031-explicit-event-severity-filtering.md) precede production
changes. Root Codex and consumer_assessment select severity admission over more
dispatch/recovery guidance because that guidance already covers the observed
failure paths. This is a source-based need assessment, not external adoption.

The clean input is 1759cbe2139220126fe4030c9b1d2b39f7745883, matching refreshed
origin/codex/nightly. Live input run 40195931334/job 121176634812 passes all
20 steps, 186 workspace tests, separate 14 adapter/five sample tests and the
222-line sample. Raw logs and connector step summaries agree.

EventFilter retains one copied minimum severity; emit_filtered rejects excluded
severities before capacity inspection and otherwise delegates to existing emit.
FilteredEventOutcome separates deliberate omission from the exact unchanged
EventEmitOutcome. No dependency, queue configuration, counter, history, priority,
reservation or runtime failure-event change is introduced. The
[guide](../EVENT_FILTERING.md) explains caller-owned policy and event drain.

Reviewed source SHA256 values:

| File | SHA256 |
| --- | --- |
| src/events.rs | 78e41f3ea59bcb2b34b3d4a7a71573058e40a95d40067b126b894161f38fc2cb |
| src/lib.rs | ddfe8a5e5b1fbb76e23e20d82bd32753ef5afe0c3ed988ff2a21c9f34534ea52 |
| tests/event_filter.rs | 7339c6316ea74ec9136c74c23a55bcdf44da8da8d5e6d9f32a9a828a50c7a7e4 |

## Executed local verification

Initial and final locked baseline commands all pass:

```text
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
git diff --check
git show --format= --check --diff-merges=first-parent HEAD
```

RUSTDOCFLAGS is set to -D warnings before Cargo documentation. The initial
baseline has 186 passing tests and the final baseline 193. Focused
`cargo test --locked --test event_filter` passes all seven new regressions.
The committed-tip command initially checks the input; the new tip receives its
own check after commit. Local tools remain Rust/Cargo 1.98.0, rustfmt
1.9.0-stable and Clippy 0.1.98. No local hosted-toolchain execution is claimed.

The guide snippet also passes one explicit Markdown doctest:

```text
cargo build --locked
rustdoc --test --edition 2024 docs/EVENT_FILTERING.md --extern rust_flight_framework=target/debug/librust_flight_framework.rlib -L dependency=target/debug/deps
```

An intermediate all-target check passes before consumer tests are added. No
workflow, host adapter/sample, existing consumer or borrowing experiment changes
trigger additional local conditional commands. The unchanged hosted workflow
continues to execute the focused host tests and sample separately.

## Public-only and independent consumer evidence

Seven tests verify the explicit nine-case severity table; suppression without
mutation in empty/partial/full storage; complete FIFO records with out-of-order
timestamps; admitted saturation and retry of the same caller-owned record;
mixed unfiltered calls; and per-attempt policy changes without pruning history.
The ordinary-work consumer uses a capacity-one diagnostic queue, fixed Cell
observations and an injected manual clock. Three optional diagnostic omissions
permit successful work, followed by an admitted Error with selected context
identity and exact time. A full queue of admitted Error records still causes a
typed terminal work failure while a healthy peer completes with suppression.
This differs deliberately from the existing identity consumer's stronger
record-before-effect policy, whose source and tests are retained unchanged.

The guide's independent-package recipe is executed at:

```text
C:/Users/drrsm/AppData/Local/Temp/rff-event-filter-c3e98cd6124d44ea8167c372680f707c
```

Actual commands are:

```powershell
$framework = 'E:/Projects/flight software framework'
$consumer = 'C:/Users/drrsm/AppData/Local/Temp/rff-event-filter-c3e98cd6124d44ea8167c372680f707c'
cargo new --lib --edition 2024 --vcs none $consumer
cargo add --manifest-path "$consumer/Cargo.toml" rust-flight-framework --path $framework --offline
New-Item -ItemType Directory -Path "$consumer/tests" | Out-Null
Copy-Item -LiteralPath "$framework/tests/event_filter.rs" -Destination "$consumer/tests/event_filter.rs"
cargo check --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets
cargo clippy --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets -- -D warnings
cargo test --manifest-path "$consumer/Cargo.toml" --locked --offline --test event_filter
cargo metadata --manifest-path "$consumer/Cargo.toml" --locked --offline --format-version 1
Get-FileHash -Algorithm SHA256 "$framework/tests/event_filter.rs", "$consumer/tests/event_filter.rs"
```

All commands pass, with seven independent tests. Actual metadata confirms one
independent workspace member, two graph packages, a sole framework path dependency
and no library dependencies. Original/copied bytes and the table's SHA256 match.
A temporary inspection initially compared Windows path text with forward-slash
text and failed; normalized Path comparison then confirms the actual dependency.
No source or dependency change was needed. Independent Codex filter_review
separately inspects raw logs, metadata and matching hashes. This is controlled
consumer evidence, not proof of general adoption or API stability.

## Review and publication boundary

Root Codex reviews the entire source/document/diff. Independent Codex
filter_review reviews the implementation and consumer it did not author, checks
unchanged failure-event routing and inspects actual verification evidence.
Initial review corrects the plan's test-target spelling and the guide's consumer
threshold description. Whole-tree source review covers 50 Rust files, 20,707
physical lines and 1,271 comment-only lines with zero width findings. All three
existing item-level expectations are unchanged and fulfilled under Clippy; no
new waiver or checker policy is introduced. All 510 relative links across 91
Markdown files and 157 exact test references resolve. Twelve changed documents
pass rendered full-text, heading, code, link, table and HTML-structure inspection;
the generated EventFilter, FilteredEventOutcome and EventQueue API pages are
inspected. This is content/structure review, not pixel or human acceptance.
A temporary renderer inspection initially misread a doctest's hidden return
line as a heading and stripped literal backticks inside code. Correcting the
inspection to distinguish fenced code from prose confirms the unchanged guide.
Root Codex and independent filter_review accept the complete 15-file staged
source/document/diff and actual local/consumer evidence, with no remaining
findings. Independent review confirms the exact three source hashes, all 193
passing workspace tests, focused/independent logs, metadata and rendered checks.
Pending-hosted wording is clarified before publication. This is actual Codex
review of the changed implementation, not independent human review.
Publication results are recorded as they complete. Temporary raw logs and review
aids are under target/review-2026-10-27-event-filter.

The filter allocates nothing and has no retained policy history. Logical queue
bounds still do not bound process memory, callback allocation or time. Filtering
does not skip caller event construction or clock reads, grant RefCell access,
authenticate source identities, reserve error capacity or guarantee delivery.
No human review, qualification, compatibility, operational suitability, API
stability, release, crate publication or deployment is established.

## Published implementation and completion

Ordinary fast-forward publication sends
db994e776c68fb5432e646afb2ae5344cb2444e5 to origin/codex/nightly; the live
remote matches. The committed diff equals the accepted staged snapshot and its
committed-tip whitespace check passes. Exact
[push run 40329281385](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/40329281385)
at the logged checkout passes Windows job 121594066549 and all 20 steps.
Raw logs confirm 193 workspace tests including each of the seven new tests,
separate 14 adapter/five sample tests, warnings-denied rustdoc, both whitespace
checks and the 222-line sample exactly matching input run 40195931334. Connector
step summaries independently confirm all successful steps. Root Codex inspects
the actual source and hosted evidence; filter_review separately inspects it.

Hosted Rust/Cargo 1.99.1, rustfmt 1.10.0-stable, Clippy 0.1.99, runner 2.338.0
and windows-2025-vs2026 image 20261021.278.1 match input. The whole-tree audit
retains the same source-quality policy with no new waivers. The independent
consumer and guide doctest remain separate local evidence. No local 1.99.1
execution is claimed.

This documentation-only completion records the observed implementation result
and updates current state/traceability. Source, test, dependency, licence and
workflow bytes remain unchanged. Root and independent Codex review its complete
diff and rendered documents before its own ordinary source push. Its exact
hosted result and final revision are retained in automation memory after
inspection, avoiding a self-referential commit hash. No further capability is
started; stop after this filtering checkpoint.

Final documentation audit covers 91 Markdown files, 511 resolving relative
links and 157 exact test references. All thirteen run-changed documents pass
fresh rendered content/structure inspection. This follows the implementation's
earlier twelve-document/510-link audit. The complete follow-up diff is confined
to seven evidence documents; the three source SHA256 values remain unchanged.
Root Codex and independent filter_review accept this complete follow-up diff,
actual checkpoint logs and final rendered audit with no remaining findings.
