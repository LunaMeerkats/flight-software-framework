# Stage 4 clock-origin scope review

Date: **2026-09-25**
Status: **Complete locally; publication pending**

## Objective and context

Verify the public consequence of `FrameworkInstant` carrying elapsed time but
no clock provenance: a schedule configured from one `ManualClock` origin can be
released by an unrelated clock with the same elapsed value. Exercise both the
direct `Runtime` and lifecycle/inbox-owning `MessagingRuntime` scheduling APIs.

The starting revision is `f5b6ab82f888b45ea122ab1fb84aed2e202187c2`, clean
and equal to refreshed `origin/codex/nightly`; its exact hosted run 35915158895
succeeded. The initial locked local baseline passes 129 tests on Rust/Cargo
1.98.0, rustfmt 1.9.0-stable, and Clippy 0.1.98.

This is one bounded Stage 4 public-API checkpoint. ADR-0014, ADR-0015, and
ADR-0021 already require callers to pair schedule instants with the intended
clock domain. Existing tests cover controlled elapsed-time decisions but do not
execute the cross-origin case through either scheduling owner.

## Acceptance criteria and files

- Add one public `tests/scheduled_work.rs` regression proving that a deadline
  read from one manual clock waits for and is released by an unrelated direct-
  runtime clock solely according to elapsed value.
- Add one public `tests/messaging_scheduled_work.rs` regression proving the
  same behavior through the messaging owner while preserving both inboxes.
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
they do not authorize cross-clock comparison. An origin-bearing redesign
remains a separate architecture decision with bounded origin, equality,
construction, persistence, event-timestamp, and public-API consequences. The
tests will not imply wall-clock meaning, cross-origin ordering validity, real-
time behavior, or human API acceptance. If verification fails, remove only
this run's changes. Stop after this evidence checkpoint and publication record.

## Completed local checks

Both focused targets pass: eleven direct-schedule tests and nine messaging-
owned schedule tests. The final locked workspace baseline passes 131 tests.
Formatting, all-target checking, warnings-denied Clippy/rustdoc, and whitespace
checks pass without a new lint exception. Production code, public API,
dependencies, licences, workflow, source register, and scheduling policy are
unchanged.

The whole-tree audit passes 32 Rust files with zero width findings, no block
comments, and three unchanged fulfilled expectations; 48 Markdown files, 216
resolving relative links, 35 source definitions, and 95 exact traceability test
references. Generated HTML for all documents passes structural checks. The six
changed documents preserve exact content and have no horizontal overflow; the
new review's rendered layout was inspected without a visible defect.
