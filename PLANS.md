# Stage 4 configured messaging-attachment recovery review

Date: **2026-09-26**
Status: **Complete: checkpoint published and exact-revision hosted CI passed**

## Objective and context

Verify that a failed `MessagingRuntime::new` attachment preserves the complete
configuration lineage inside the returned owned runtime. Exercise a failure
after a valid configured runtime and application already exist, then attach a
corrected inbox and observe the active revision, consume-once rollback, and
never-reused revision high-water mark through ordinary work.

The starting revision is `6a60f381736d2b7c7d069088d494000017ceaa75`, clean
and equal to refreshed `origin/codex/nightly`. The initial locked local baseline
passes 131 tests on Rust/Cargo 1.98.0, rustfmt 1.9.0-stable, and Clippy 0.1.98.

This is one bounded Stage 4 ownership and failure-path checkpoint. The existing
messaging construction review proves returned-runtime reuse without a
configuration table and explicitly leaves configuration lineage unobserved.

## Acceptance criteria and files

- Add one public `tests/configuration_runtime.rs` regression that forces an
  unrepresentable inbox reservation during messaging attachment.
- Observe the exact typed attachment error and unchanged registered lifecycle
  state through the returned runtime.
- Correct the inbox configuration, then prove revision 2 remains active,
  rollback restores revision 1 once, and the next replacement receives
  revision 3 through application work observations.
- Preserve production code, public API, requirements, dependencies, licences,
  configuration policy, messaging policy, and lint policy.
- Record the boundary in a focused verification note; update README, project
  state, roadmap, and traceability. No new external source is expected.

## Verification

Format before running `cargo test --locked --test configuration_runtime`, then
run the required locked Cargo baseline with warnings-denied Clippy and rustdoc.
Audit Rust physical/comment widths, relative Markdown links, exact traceability
test names, changed rendered documents, and the complete diff. Unchanged host
adapters, workflow, and ADR-0018/0019 experiments do not trigger their separate
commands.

After local acceptance, commit on `codex/nightly`, refresh origin, publish by
ordinary fast-forward, and inspect exact-revision hosted CI. Record completed
publication evidence separately without projecting a pass onto a later commit.

## Completed local checks

The focused configuration-runtime target passes eleven tests, and the final
locked workspace baseline passes 132 tests. Formatting, all-target checking,
warnings-denied Clippy/rustdoc, and whitespace checks pass without a new lint
exception. Production code, public API, dependencies, licences, workflow,
source register, and service policies are unchanged.

The whole-tree audit passes 32 Rust files with zero width findings, no block
comments, and three unchanged fulfilled expectations; 49 Markdown files, 222
resolving relative links, 35 source definitions, and 96 exact traceability test
references. Generated HTML for all documents passes structural and content
checks. The six changed documents preserve exact content, and their 1280-pixel
browser renders have no visible layout defect in the inspected views.

## Risks and safe stopping point

The test establishes ownership preservation at one deterministic capacity-
overflow attachment error. It does not inject allocator exhaustion, inspect
temporary endpoint destruction, or generalize to panic, concurrency, secure
erasure, or every constructor error. If verification fails, remove only this
run's changes. Stop after this evidence checkpoint and publication record.

## Publication result

Checkpoint `b21b59bff68fe6c16b0d153661865a67a28f8df2` was published by
ordinary fast-forward; the remote head matched. Hosted run 36184310315 passed
that exact push and checkout: one Windows job, all configured steps, the new
regression, 132 workspace tests in aggregate, 14 focused adapter tests, five
focused sample tests, and the sample executable.

This documentation-only follow-up records completed evidence with unchanged
Rust, Cargo, workflow, and lint inputs. Its rendered content, links, and diff
are reviewed separately. A later revision requires its own hosted result.
