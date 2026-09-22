# Stage 4 detached-message identity scope review

Date: **2026-09-23**
Status: **Complete: checkpoint published and exact-revision hosted CI passed**

## Objective and context

Verify the public consequence of the detached `MessageBus` identity contract:
topology validation checks registration position but cannot distinguish a
same-position `ApplicationId` issued by another registry. The starting revision
is `3041407580c09236adbc46ad05819c66984b2c9e`, clean and equal to refreshed
`origin/codex/nightly`; the initial locked local baseline passes 126 tests on
Rust/Cargo 1.98.0.

This is one bounded Stage 4 service/public-API checkpoint. ADR-0010 and source
documentation already state the caller-scoped limitation, while the prior
application-identity review executes registry/runtime lookup only. Existing
message-bus tests reject an out-of-range identity but do not execute the
equal-position foreign-issuer case.

## Acceptance criteria and files

- Add one public `tests/message_bus.rs` regression proving that a foreign
  same-position identity passes detached topology validation and addresses the
  configured local inbox after publication.
- Preserve both issuing registries so the test distinguishes value equality
  from shared ownership or mutation.
- Preserve production code, API, requirements, dependencies, and lint policy.
- Record the observed boundary and the `MessagingRuntime` mitigation in a new
  verification note; update README, project state, roadmap, and traceability.
  Confirm that no new external source is needed.

## Verification

Format before running `cargo test --locked --test message_bus`, then run the
required locked Cargo baseline with warnings-denied Clippy and rustdoc. Audit
Rust physical/comment widths, relative Markdown links, exact traceability test
names, changed rendered documents, and the complete diff. Unchanged host
adapters, workflow, and ADR-0018/0019 experiments do not trigger their separate
commands.

After local acceptance, commit on `codex/nightly`, refresh origin, publish by
ordinary fast-forward, and inspect exact-revision hosted CI. Record completed
publication evidence separately without projecting a pass onto a later commit.

## Risks and safe stopping point

The regression deliberately demonstrates an existing aliasing boundary; it
does not authorize cross-owner key mixing. `MessagingRuntime` assigns topology
identities internally but its operation selectors retain ADR-0003's caller
discipline. An origin-bearing redesign remains a separate architecture decision
with bounded issuer, equality, exhaustion, and persistence consequences. If
verification fails, remove only this run's changes. Stop after this evidence
checkpoint and its publication record.

## Completed local checks

The new regression passes with all nine message-bus tests; the final locked
workspace baseline passes 127 tests. Formatting, all-target checking, warnings-
denied Clippy/rustdoc, and whitespace checks pass. No production change,
dependency, or lint exception was needed. The whole-tree audit passes 32 Rust
files with zero width findings, no block comments, and three unchanged
fulfilled expectations, plus 46 Markdown files, 200 resolving relative links,
and 91 exact traceability test references. Rendered document structure and
exact content comparisons pass; pixel-level visual acceptance is not claimed.
Complete source/diff review found no blocking defect.

## Publication result

Checkpoint `f6786d0ada7812bc10b2d7aec03a0f4b8f48b4b3` was published by
ordinary fast-forward; the remote head matched. Hosted run 35778516818 passed
that exact push and checkout: one Windows job, all configured steps, 127
workspace tests including the new regression, 14 focused adapter tests, five
focused sample tests, and the sample executable.

This documentation-only follow-up records completed evidence with unchanged
Rust, Cargo, workflow, and lint inputs. Its rendered content, links, and diff
are reviewed separately. A later revision requires its own hosted result.
