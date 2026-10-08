# Autonomous host v0.1 review

Date: **2026-10-09**
Disposition: **Accepted for the experimental serial host milestone**

## Revision, authority, and reviewers

Reviewed input: `ba42f800d0b74637b28c833a55192fc541912fe5` on
`codex/nightly`, initially clean and equal to refreshed origin.
[ADR-0024](../adr/0024-autonomous-host-review-authority.md) delegates routine
host acceptance. This record reports an actual review, not just that authority.

Codex performed the review: the primary nightly maintainer integrated fresh
verification and the dispositions; parallel Codex reviewers `entrypoint_review`,
`architecture_review`, and `scope_audit` separately inspected the host entry
point, architecture/resource/failure contracts, and scope/provenance inventory.
They are autonomous reviewers, not independent human reviewers. The final
documentation reconciliation also passes complete-diff and rendered-content
review before publication.

The input's [hosted run 37675969512](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/37675969512)
is confirmed successful for its exact logged checkout. The review record and
its documentation corrections require their own publication/CI evidence;
the input run does not cover later revisions.

## Entry-point and provenance inventory

Reviewed README, charter, contributor guidance, requirements, roadmap, state,
architecture, all 24 ADRs, research register, traceability, and the relevant
verification checkpoints. Dated earlier pending-human statements describe
historical evidence; ADR-0024 supersedes that routine acceptance policy.

| Current entry point | Evidence and finding |
| --- | --- |
| README and charter | Experimental host purpose and explicit qualification, safety, affiliation, TRL, operational, and compatibility limits; sample command and limitations are visible |
| Manifest and crate documentation | Experimental description; crate landing notice names the unsupported systems and maturity claims; package stays unpublished at version 0.0.0 |
| Generated public rustdoc | Content inspected across 63 root pages: landing/all-items, 28 enums, 30 structs, three traits; detailed owner/clock/schedule/application contracts retain their limits |
| Generated source views and module redirects | Eleven source-view pages and 61 private-module item redirects refer to the same first-party source; redirects introduce no separate public API |
| Sample source, guide, and executed output | All six example files and both shared-source test targets reviewed; stdout labels simulated time and absence of physical delivery and carries the scope notice |
| Current architecture and evidence documents | Repeatability means controlled initial state, time, and ordered inputs; logical storage and cooperative failure claims have explicit boundaries |

No positive unsupported qualification, compatibility, timing, fault-tolerance,
or agency-compliance claim was found. Source provenance distinguishes observed
upstream responsibilities from independently designed local Rust contracts.
No external research, reused source, or new upstream claim was needed here.

Fresh complete Cargo metadata, all-target/all-edge trees with and without
default features, and manifest/lockfile inspection agree: one package, no
external normal/development/build/optional/platform packages, no package or
resolved features, one library, one example, and 16 integration-test targets.
No custom build, procedural macro, generated Rust, nested manifest, or vendor
input is present. This empty external Cargo graph excludes the standard library,
compiler/linker, host, and CI tool dependencies.

Licence files, approved holder notice, manifest expression, and CI action remain
unchanged from the [dependency review](DEPENDENCY_SCOPE_REVIEW.md). Git reports
`text: set` and `eol: lf` for both licences. No new licence decision, security
clearance, legal opinion, or full CI/toolchain distribution audit is inferred.

## Requirement and architecture dispositions

The [traceability register](TRACEABILITY.md) retains the original implementation
revisions and exact tests. All 141 current workspace tests pass afresh; focused
reruns are additional execution evidence, not additional unique tests.

| Gate | Reviewed evidence | Disposition |
| --- | --- | --- |
| RFF-REQ-001 scope truth | Entry-point inventory above, generated documentation content, executed notice, current documentation reconciliation | Accepted for current entry points; future or changed claims require renewed review |
| RFF-REQ-002 lifecycle | Exhaustive LC1 matrix and `two_applications_complete_lifecycle_with_running_work`; combined sample's ordered two-application states | Verified at the explicit owned host boundary |
| RFF-REQ-003 bounded messaging | Exact payload/inbox limits, fan-out/FIFO/saturation, unavailable subscribers, selected cleanup, refreshed dispatch and in-flight self-publication tests | Verified for serial caller-selected publication/dispatch |
| RFF-REQ-004 time and scheduling | Injected manual time, zero/one clock-read tests, equal-time order, inclusive due decision, final one-item consumption, replay | Verified for the fixed finite one-shot agenda |
| RFF-REQ-005 events | Structured fields, exact retained-record limit, reject-newest saturation, exact rejected event and clock-captured ordinary-work failure tests | Verified for standalone events and opt-in ordinary-work failure reporting |
| RFF-REQ-006 configuration | Validation/activation/rejection, used-prefix views, ownership recovery, revision exhaustion/non-reuse, consume-once rollback and runtime visibility tests | Verified at the bounded in-memory ordinary-work boundary |
| RFF-REQ-007 host adapters | Shared mission source; all accepted percentages, malformed lengths/identifiers/values, validation precedence, staging, saturation and drain tests | Verified for the local caller-framed slice/returned-array boundary |
| RFF-REQ-008 error containment | Concrete returned error, selected terminal failure/inbox clearing/event, retained peer queues and later peer work | Verified for cooperative returned ordinary-work errors |
| Executable integration | Five `host_sample` tests exercise the same driver as the binary and compare complete fresh-run reports | Accepted as the documented host demonstrator |
| Dependency/features and provenance | Fresh graph queries, unchanged approved licence/action inputs, register/ADR/source review | Accepted within the Cargo and recorded provenance boundary |
| Source quality and hosted baseline | Tracked format/lint/safe-Rust policy, whole-tree review, fresh local checks, exact input hosted run | Accepted with local/manual and hosted evidence kept separate |
| Architecture | One unpublished package; explicit owners, safe synchronous callbacks, narrow contexts, caller-selected service composition | Accepted for this host milestone; no API freeze or broader target commitment |

The complete sample tests are:

- `combined_sample_runs_both_lifecycles_and_preserves_exact_adapter_records`
- `combined_sample_waits_then_runs_equal_time_work_with_observed_configuration`
- `combined_sample_observes_rejection_rollback_revision_non_reuse_and_restart_retention`
- `combined_sample_records_exact_failure_and_preserves_peer_work_and_queued_telemetry`
- `combined_sample_replays_the_complete_fixed_observable_report`

The executed report matches 0/42/100 command/telemetry, equal-time work at
10 ms, configuration revisions 2/1/3 with retained revision 3 after restart,
and the 20 ms injected echo failure: one discarded command, one recorded event,
peer work still Running, and retained telemetry `[0x81, 7]`.

## Resource, failure, and API findings

No blocking defect was found against the current requirements. Review accepts
the following disclosed constraints for the experimental host milestone:

| Boundary | Reviewed contract and accepted limitation |
| --- | --- |
| Lifecycle and identity | Finite records and typed pre-callback rejection; keys encode a local position, not issuer provenance; LC1 Failed is terminal and restart retains the same stopped application |
| Messaging ownership | Complete fresh topology owned with runtime; only Running accepts delivery; stop/returned errors clear exactly the selected inbox; valid positional configurations can still be accidentally swapped |
| Routing and dispatch | Inline payloads, copied topology, pre-reserved inboxes and dispatch-state storage; ordered reports allocate before publication; immediate partial fan-out is not transactional, and peer delivery survives later callback error |
| Process resources | Logical limits do not bound whole-process bytes, inline stack use, callback allocation, caller-retained reports/configuration/event copies, or execution duration; real allocator exhaustion and allocation counts are untested |
| Configuration | One active snapshot and one rollback slot plus high-water revision; work borrows only used bytes; no automatic rollback after errors and no message/lifecycle configuration access |
| Time and events | Clock origin/monotonicity are caller responsibilities; FIFO is emission order, not timestamp sorting; observation occurs after failure commitment and messaging cleanup; saturation can reject a high-severity event |
| Scheduling | Fixed copied agenda, stable equal-time order, one final consumed attempt; no recurrence, retries, general work loop, or scheduled/message failure-event integration |
| Callback/effect boundary | Cooperative returned errors are preserved; application internal/external effects are not undone; panics, hangs, process/allocator/hardware faults and host stdout failures are outside containment |
| Generic copied identifiers | Copying a topic/event identifier need not isolate arbitrary shared referents or make user equality bounded in execution time |
| Public API and platform | Contexts are intentionally separate and still experimental; future consolidation may change APIs; no concurrency, security, hardware, RTOS, or no_std commitment is made |

Existing identity, construction, ownership, diagnostic, stop/message/work-error,
and schedule review checkpoints corroborate these contracts. Source inspection
is distinguished from injected fault tests; capacity-overflow tests do not
simulate physical allocator exhaustion. No rewrite or extra abstraction is
justified by this review.

One nonblocking current documentation finding is corrected: the research
register described a default-branch push filter, while the tracked workflow
filters `codex/nightly`. The correction names the actual branch and changes no
workflow policy or historical test result. README's long historical status is
a navigability observation; its runnable instructions are correct and a broad
editorial restructure is outside this checkpoint.

## Fresh verification

All commands below completed successfully on the reviewed input:

```text
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
cargo test --locked --test host_adapters
cargo test --locked --test host_sample
cargo run --locked --example host-echo
cargo metadata --locked --format-version 1 --all-features
cargo tree --locked --workspace --all-features --target all --edges all
cargo tree --locked --workspace --no-default-features --target all --edges all
git check-attr text eol -- LICENSE-MIT LICENSE-APACHE
git diff --check
git show --format= --check --diff-merges=first-parent HEAD
```

Rustdoc uses `RUSTDOCFLAGS=-D warnings`. Workspace tests: 141 passed, none
failed; zero Cargo doctests. Focused adapters: 14 passed; sample: five passed.
Local Rust/Cargo 1.98.0, rustfmt 1.9.0-stable, Clippy 0.1.98 remain unchanged.
The input hosted run passes one Windows job and all 20 reported steps (17 named
workflow steps plus lifecycle), with the same test/sample evidence. Its runner
2.337.0, `windows-2025-vs2026` image `20260925.250.1`, Rust/Cargo 1.99.0,
rustfmt 1.10.0-stable, and Clippy 0.1.99 match the previous policy re-audit.
No local 1.99.0 execution is claimed.

Whole-tree source review covers 33 handwritten Rust files and 12,666 physical
lines: zero physical lines over 100 columns, zero comment-only lines over 80,
no block comments, no broad allows, and the same three fulfilled item-level
function-size expectations. Review examines names, progressive ordering,
ownership, cohesion, and error paths separately from lexical width checks.

ADR-0018/0019 probe sources and decisions are unchanged. Their historical
language-shape results are retained; no fresh standalone probe or actionlint
execution is claimed. Cargo does not discover those probes. Temporary logs,
audits, and rendering aids remain ignored under `target/review-2026-10-09-host`.

The final locked baseline also passes after documentation reconciliation, with
the same 141 tests. All 320 relative links across 62 Markdown files and 105
exact traceability function references resolve. Eleven changed documents pass
rendered HTML heading/code/link/table/text and nesting inspection. Review of
rendered content and the complete diff checks the final wording as well as the
input. No pixel-level or independent-human acceptance is claimed.

## Milestone disposition and next work

RFF-REQ-001 and the routine host entry-point/architecture gates are accepted.
Together with the existing requirement, sample, dependency and exact-hosted
evidence, this completes Stage 4 for the experimental serial host v0.1 target.
The requirements and APIs remain provisional, and Cargo version stays 0.0.0.
This is not human review, flight qualification, operational suitability,
compatibility, a public API freeze, or authorization to tag, release, publish
a crate, deploy, or widen into consequential scope.

The next candidate is a small dependency-free model-based test of bounded
message publication/dequeue sequences against an independent reference queue.
Existing regressions test selected traces; sequence composition is a distinct
hardening question. Freeze the finite operation alphabet and explored bounds
before implementation, report their limits, and revisit the candidate if fresh
repository evidence points to higher-value work.
