# Standalone configuration sequence review

Date: **2026-10-15**
Status: **Local checkpoint accepted; publication and hosted CI pending**

## Question and decision

Does composing accepted replacement, rejected replacement and rollback preserve
the ADR-0005/0017 snapshot lineage? Twelve public standalone scenarios and three
private boundary tests cover individual traces; none generates combined
histories. RFF-REQ-006 defines existing behavior. The current project state
nominates this assessment after the completed schedule checkpoint.

Codex root and config_gap confirm the gap and freeze [PLANS.md](../../PLANS.md)
before implementation. One separate integration target keeps the oracle
coherent and preserves scenario evidence. No external research, source reuse,
production code, API, dependency, requirement, ADR, lint or workflow change is
needed. Runtime work visibility and exhaustion remain separate evidence.

Input `ecfe954cd7550a933bf08821d9a4154c71023953` is clean on codex/nightly and
matches refreshed origin. Fresh locked local baseline passes 146 workspace
tests. [Input hosted run 38404703855](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/38404703855)
passes that exact logged checkout and all 20 steps. Root and baseline_review
inspect results separately; input CI does not cover the new target.

## Frozen domain and oracle

| Parameter | Explored values |
| --- | --- |
| Table | One standalone table, inline bound two |
| Initial acceptance | [1], revision one, no rollback history |
| Accepted replacements | A=[1], B=[2,3] |
| Semantic rejection | [0], concrete mission validation error |
| Oversize rejection | [0,0,0], TooLarge before semantic rejection |
| Other operation | Consume-once Rollback |
| Generated lengths | Every sequence from zero through six |
| Generated totals | 19,531 fresh traces; 112,305 operations |
| Supplemental probes | Rollback, ReplaceA, Rollback after each trace |

Supplemental probes are separate from generated lengths/counts. They expose a
latent rollback predecessor after a maximal prefix, the next unused revision
after trailing rejection or restoration, and the newly retained predecessor.
The three probes add 58,593 observations within the same operation domain.
Base-five enumeration is bijective per length and both generated totals are
asserted. Every trace starts from a freshly validated table and oracle.

The reference appends accepted content and its predecessor identity to an
acceptance history, and records acceptance/restoration in a movement history.
The latest movement derives current identity and whether its acceptance still
permits restoration. Acceptance position supplies revision identity. It does
not store the production active/rollback/high-water triple or use production
replacement/rollback logic. Rejections add no accepted entry or movement.

Each operation compares its exact typed result and source relationship, then
active used bytes and original revision. Equal-content replacements still
accept new identities. Length changes reveal stale-prefix exposure. The
oversize candidate is semantically invalid too, distinguishing error precedence.
Repeated restoration and replacement after restoration expose slot consumption
and non-reuse. Existing private tests prove exhaustion and validator suppression
at their separately controlled boundaries.

## Review and verification

`tests/configuration_sequences.rs` implements one model test. Its final fixture
accepts nonempty bytes with a nonzero first byte; otherwise it returns the
concrete mission validation error. Root and baseline_review identify an earlier
exact-pattern fixture/document mismatch; config_gap aligns the fixture with the
frozen policy before final verification and isolated fault trials.

Focused locked test and focused warnings-denied Clippy pass. Initial/final full
locked local baselines pass 146/147 workspace tests, formatting, all-target
check, Clippy, warnings-denied rustdoc and whitespace. Root and baseline_review
review the complete test and accept enumeration, independent lineage, exact
source observations and supplemental probes. Final test blob is
`00ca28860a92cffa7af79e6108bf9c40d3d9c231`. No lint exception is added.
Root, config_gap, baseline_review and sensitivity accept the source/oracle
within the frozen scope. Selected final-copy faults compile, then fail model
behavioral assertions; the clean isolated-copy control passes. Root separately
inspects raw logs, exact witnesses, final test/source hashes and unchanged
tracked production. Earlier trials against the superseded fixture remain
ignored and are not the final evidence.

| Isolated fault | First observed disagreement |
| --- | --- |
| Rollback creates redo | [A,Rollback], supplemental step 3 returns revision 2 instead of NoRollbackAvailable |
| Retain earliest predecessor | [A,A], supplemental step 3 restores revision 1 instead of 2 |
| Rejection evicts history | [A,Reject], supplemental step 3 loses revision 1 restoration |
| Allocate from active revision | [A], supplemental replacement at step 3 reuses revision 2 instead of 3 |
| Equal-content short circuit | Empty generated prefix, supplemental step 2 returns revision 1 instead of 2 |
| Short replacement leaks storage | Empty generated prefix, supplemental step 2 exposes [1,0] instead of [1] |

Each trial runs `cargo test --locked --test configuration_sequences -- --nocapture`
in its own ignored source copy. Clean exit is zero; all six compiling faults
exit 101 at behavioral assertions. These are selected sensitivity trials,
not comprehensive mutation coverage. Final copies/results/logs are under
target/review-2026-10-15-configuration/mutations/final. The tracked production
configuration blob remains `68d3eca06760a59d9e8d56f4cb691fb13fa3356b`.
Root and parallel baseline_review accept the complete source/document/diff with
no blocking finding. Final audit covers 39 Rust files/15,333 lines and 1,105
comment-only lines, zero width findings and three unchanged fulfilled lint
expectations. All 352 relative links across 68 Markdown files and 111 exact
traceability function references resolve. Six changed documents pass complete
rendered content and HTML structure inspection; no pixel or human-review claim
is made. Review aids are not adopted tooling or CI gates.

```text
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
cargo test --locked --test configuration_sequences
cargo clippy --locked --test configuration_sequences -- -D warnings
git diff --check
git show --format= --check --diff-merges=first-parent HEAD
```

Rustdoc uses RUSTDOCFLAGS=-D warnings. Local Rust/Cargo 1.98.0, rustfmt
1.9.0-stable and Clippy 0.1.98 match the recorded baseline. Conditional extra
host commands, ADR-0018/0019 probes and actionlint are not triggered because
their source/workflow is unchanged. Hosted CI continues to run both focused
host targets and the sample; its results remain separate from local checks.

## Limits

This proves comparison over the frozen finite domain, including supplemental
probes, rather than arbitrary operation histories. Other bounds, validators,
candidate bytes, initial failures, revision exhaustion, runtime lifecycle/work,
messaging/events/scheduling, allocation failure, panics/hangs and concurrency
are outside it. The pure validator is a fixture; the framework cannot establish
arbitrary mission-validator purity or termination. These tests do not establish
physical memory bounds, performance, qualification or operational suitability.
