# Stage 4 scheduled-work identity scope review

Date: **2026-09-24**
Status: **Complete: checkpoint published and exact-revision hosted CI passed**

## Objective and context

Verify the public consequence of copying a caller-scoped `ApplicationId` into
`ScheduledWork`: a due same-position key from another owner selects the
receiving owner's local application. Exercise both the direct `Runtime` and
the lifecycle/inbox-owning `MessagingRuntime` scheduling paths.

The starting revision is `4ef3b74296700259ad80ad5a6792de054fddbea2`, clean
and equal to refreshed `origin/codex/nightly`; its prior exact hosted run
35778881015 succeeded. The initial locked local baseline passes 127 tests on
Rust/Cargo 1.98.0, rustfmt 1.9.0-stable, and Clippy 0.1.98.

This is one bounded Stage 4 public-API checkpoint. ADR-0003, ADR-0015, and
ADR-0021 already state that application identities remain caller-scoped and
that callers must pair a schedule with the intended owner. Existing schedule
tests cover out-of-range identities but do not execute the equal-position
foreign-issuer case through either scheduling owner.

## Acceptance criteria and files

- Add one public `tests/scheduled_work.rs` regression proving that a due item
  containing a foreign same-position identity invokes only the receiving
  direct runtime's corresponding local application.
- Add one public `tests/messaging_scheduled_work.rs` regression proving the
  same selector behavior through the messaging owner while preserving the
  local inbox and both issuing owners' independent state.
- Preserve production code, public API, requirements, dependencies, licences,
  scheduling policy, and lint policy.
- Record the observed boundary in a focused verification note; update README,
  project state, roadmap, and traceability. No new external source is expected.

## Verification

Format before running
`cargo test --locked --test scheduled_work --test messaging_scheduled_work`,
then run the required locked Cargo baseline with warnings-denied Clippy and
rustdoc. Audit Rust physical/comment widths, relative Markdown links, exact
traceability test names, changed rendered documents, and the complete diff.
Unchanged host adapters, workflow, and ADR-0018/0019 experiments do not trigger
their separate commands.

After local acceptance, commit on `codex/nightly`, refresh origin, publish by
ordinary fast-forward, and inspect exact-revision hosted CI. Record completed
publication evidence separately without projecting a pass onto a later commit.

## Risks and safe stopping point

The regressions deliberately execute an existing caller-discipline boundary;
they do not authorize cross-owner key mixing. An origin-bearing redesign
remains a separate architecture decision with bounded issuer, equality,
exhaustion, persistence, event-source, and public-API consequences. The tests
will not imply clock-origin validation, global identity, or human API
acceptance. If verification fails, remove only this run's changes. Stop after
this evidence checkpoint and its publication record.

## Completed local checks

Both focused targets pass: ten direct-schedule tests and eight messaging-owned
schedule tests. The final locked workspace baseline passes 129 tests.
Formatting, all-target checking, warnings-denied Clippy/rustdoc, and whitespace
checks pass without a new lint exception. Production code, public API,
dependencies, licences, workflow, source register, and scheduling policy are
unchanged. The whole-tree audit passes 32 Rust files with zero width findings,
no block comments, three unchanged fulfilled expectations, 47 Markdown files,
208 resolving relative links, 35 source definitions, and 93 exact traceability
test references. Generated HTML for all documents passes structural checks;
the six changed documents pass exact content comparison and rendered-layout
inspection without visible overflow or malformed sections.

## Publication result

Checkpoint `9f3aac6947943c3f9affbe38383e5e4ad09f382d` was published by
ordinary fast-forward; the remote head matched. Hosted run 35914683806 passed
that exact push and checkout: one Windows job, all configured steps, both new
regressions, 129 workspace tests in aggregate, 14 focused adapter tests, five
focused sample tests, and the sample executable.

This documentation-only follow-up records completed evidence with unchanged
Rust, Cargo, workflow, and lint inputs. Its rendered content, links, and diff
are reviewed separately. A later revision requires its own hosted result.
