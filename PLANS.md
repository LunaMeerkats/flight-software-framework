# Stage 4 failure-event identity scope review

Date: **2026-09-27**
Status: **Complete: checkpoint published and exact-revision hosted CI passed**

## Objective and context

Execute the documented caller-scoped `ApplicationId` limitation through both
opt-in ordinary-work failure-event owners. Prove which receiving application is
invoked, which owner changes state and inbox contents, and which positional key
is copied into the resulting event.

The starting revision is `3d38bf6bcb5c56ad061561a43ca2d05be639175c`,
clean and equal to refreshed `origin/codex/nightly`. Hosted run 36184729804
passed that exact revision. The initial locked local baseline passes 132 tests
on Rust/Cargo 1.98.0, rustfmt 1.9.0-stable, and Clippy 0.1.98.

This is one bounded Stage 4 public-API and failure-attribution checkpoint. The
prior identity reviews execute positional aliasing through standalone/runtime
lookup, detached messaging, and both scheduling owners. ADR-0016 and ADR-0020
document the same caller-scoped origin limitation for failure events, but no
public regression currently observes it through either event-producing owner.

## Acceptance criteria and files

- Add one public `tests/runtime_events.rs` regression using a same-position key
  from a separate live runtime to invoke the receiving runtime's local faulting
  application and record one application-sourced event.
- Add the corresponding `tests/messaging_work_events.rs` regression, proving
  that only the receiving selected inbox is cleared while both foreign inboxes
  and the receiving peer inbox remain unchanged.
- Observe that the foreign owner remains running and uninvoked, and distinguish
  positional aliasing from authorization to mix keys or issuer validation.
- Preserve production code, public API, requirements, dependencies, licences,
  event/messaging policy, and lint policy.
- Record the boundary in a focused verification note; update README, project
  state, roadmap, and traceability. No new external source is expected.

## Verification

Format before running
`cargo test --locked --test runtime_events --test messaging_work_events`, then
run the required locked Cargo baseline with warnings-denied Clippy and rustdoc.
Audit Rust physical/comment widths, relative Markdown links, exact traceability
test names, changed rendered documents, and the complete diff. Unchanged host
adapters, workflow, and ADR-0018/0019 experiments do not trigger their separate
commands.

After local acceptance, commit on `codex/nightly`, refresh origin, publish by
ordinary fast-forward, and inspect exact-revision hosted CI. Record completed
publication evidence separately without projecting a pass onto a later commit.

## Completed local checks

The focused direct and messaging-owned event targets pass five and seven tests,
and the final locked workspace baseline passes 134 tests. Formatting, all-target
checking, warnings-denied Clippy/rustdoc, and whitespace checks pass without a
new lint exception. Production code, public API, dependencies, licences,
workflow, source register, and service policies are unchanged.

The whole-tree audit passes 32 Rust files with zero width findings, no block
comments, and three unchanged fulfilled expectations; 50 Markdown files, 230
resolving relative links, 35 source definitions, and 98 exact traceability test
references. Generated HTML for all documents passes structural and exact-
content checks. Browser visual layout inspection was not run because browser
URL policy blocked local rendered files.

## Risks and safe stopping point

The tests can establish only the current positional-key behavior: a foreign
same-position value compares equal, selects the receiving owner's local record,
and is indistinguishable in the event source. They cannot establish issuer
validation, safe cross-owner mixing, global identity, panic or hang containment,
or human API acceptance. If verification fails, remove only this run's changes.
Stop after this evidence checkpoint and publication record.

## Publication result

Checkpoint `ce232c6a9d1a49dcf6764e0cd6ac2a1d64331241` was published by
ordinary fast-forward; the remote head matched. Hosted run 36276474765 passed
that exact push and checkout: one Windows job, all configured steps, both new
regressions, 134 workspace tests in aggregate, 14 focused adapter tests, five
focused sample tests, and the sample executable.

This documentation-only follow-up records completed evidence with unchanged
Rust, Cargo, workflow, and lint inputs. Its rendered content, links, and diff
are reviewed separately. A later revision requires its own hosted result.
