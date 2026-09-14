# Stage 4 returned-message failure review

Date: **2026-09-15**
Status: **Local checkpoint complete; publication pending**

## Objective and context

Verify one existing dispatch contract: a callback may publish successfully and
then return an error. Only its own queued deliveries are cleared; accepted
peer deliveries retain their exact FIFO contents, and later callback
publication observes the failed endpoint as unavailable. This reduces an
evidence gap in Stage 4 without changing runtime behavior or public APIs.

Started clean on `codex/nightly` at
`ee63b4a7a7ec6c556bc76536298211fd3ff0d253`, equal to refreshed origin.
The initial locked baseline passes 117 tests on Rust/Cargo 1.98.0, rustfmt
1.9.0-stable, and Clippy 0.1.98. The initial audit finds 32 Rust files with
zero width findings, 40 Markdown files with 170 resolved relative links, and
81 exact traceability test references. Existing source-quality gates apply.

## Acceptance criteria

- Preserve exact selected failure, discard count excluding the in-flight item,
  and terminal lifecycle rejection with no repeated callback effects.
- Observe the actual returned error's source chain through both wrappers.
- Dispatch all older peer messages followed by the accepted callback reply,
  asserting identity, topic, payload, pending counts, and no extra callback.
- Verify a later peer callback refreshes availability after message failure;
  the failed inbox remains empty while healthy delivery continues.
- Record the reviewed source order, ownership/resource limits, evidence gaps,
  and unchanged ADR-0012 policy. Do not claim rollback or arbitrary containment.
- Pass the required baseline, source/link/traceability audit, rendered-document
  inspection, and independent complete-diff review before committing/pushing.
- Publish by ordinary fast-forward, verify the remote head, and inspect hosted
  CI at the exact published revision under the standing source permission.

## Components and verification

Change `tests/message_dispatch.rs` and supporting review, plan, project-state,
roadmap, and traceability records. Clarify ADR-0012's existing source order
without changing its public post-return contract. Prefer extending the existing
chronological regression for full retained FIFO evidence and one focused
availability test over production fault hooks or a new test framework. No new
ADR is necessary unless source review reveals a contract change.

Run `cargo test --locked --test message_dispatch` and all six AGENTS.md
baseline commands with supported locked Cargo forms and
`RUSTDOCFLAGS=-D warnings`. Host adapters, workflow, ADR-0018/0019, dependencies,
and lint policy are unchanged, so their additional local probes are not
triggered. Reuse reviewed temporary rendering and audit aids under
`target/review-2026-09-15`; these are not adopted repository policy tools.

## Risks and safe stopping point

The evidence covers cooperative returned message errors in serial dispatch.
Panics, hangs, allocator exhaustion, callback-retained data, and external
effects remain outside containment or queue-storage bounds. Peer publication
is immediate and is not a transaction. Existing human v0.1 acceptance and
scope gates remain open. Stop after the tested, reviewed contract checkpoint;
preserve unexpected changes and revert only this run's own work if necessary.

## Completed local checks

The focused command passes six message-dispatch tests. The final required
baseline passes 118 workspace tests, formatting, all-target check and Clippy,
warnings-denied rustdoc, and `git diff --check`. No waiver or runtime change
was needed. The existing error assertion now follows the actual returned
source chain before consuming the wrapper to recover its operation error.
One cohesive peer-drain helper preserves the complete chronological assertion
without exceeding the 60-line function policy.

The whole-tree audit finds 32 Rust files with zero physical/comment width
findings and three unchanged expectations, 41 Markdown files with 177 resolved
relative links, 34 source definitions, and 82 exact traceability references.
All eight changed rendered documents match source text, code, tables, and
structure. Browser DOM/layout and screenshots were inspected at 1280px with
no page overflow or console warnings/errors. Independent complete-diff review
found no blocking defect and identified a prose-order precision issue:
ADR-0012 now says the closure drops
the in-flight record before state commitment, matching existing source. This
clarification also passed rendered/content review. Publication is pending.
Logs and temporary aids remain ignored under `target/review-2026-09-15`.
