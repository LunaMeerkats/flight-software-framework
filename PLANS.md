# Stage 4 schedule construction review

Date: **2026-09-16**
Status: **Local checkpoint complete; publication pending**

## Objective and context

Verify the existing finite schedule's construction contract: reject the first
descending adjacent pair with exact diagnostics, and retain an independent
copy of the caller's ordered agenda. The starting revision is
`75f7cbdc659d73d2430b9987bfe64b1afa62ff35`, clean and equal to refreshed
`origin/codex/nightly`. Its hosted CI passed; the initial local locked baseline
passes 118 tests on Rust/Cargo 1.98.0.

This closes two public-observation gaps in ADR-0015 without changing the API
or execution policy. An independent event-queue audit also identified useful
capacity-overflow coverage, deferred to keep this checkpoint coherent.

## Acceptance criteria and files

- In `tests/scheduled_work.rs`, assert exact first-error index and instants
  after valid prefixes, including equal instants and nanosecond differences.
- Observe the original agenda's application identities, instants, order,
  remaining count, and callback trace after the caller replaces its input.
- Keep production behavior, dependencies, lint policy, and existing requirements
  unchanged. No unsafe allocation experiment or production test hook.
- Record the source-inspected resource/failure contract and its proof limits in
  `docs/verification/SCHEDULE_CONSTRUCTION_REVIEW.md`; update project state,
  roadmap, and traceability with completed evidence.

## Verification

Run `cargo test --locked --test scheduled_work` and the complete required
locked Cargo baseline with warnings-denied Clippy and rustdoc. Audit physical
and comment widths, exact traceability names, relative links, rendered changed
documents, and the complete diff. Independent Codex review supplements the
author's review. Unchanged host adapters, workflow, and ADR-0018/0019 experiments
do not trigger their separate local commands.

After local acceptance, commit on `codex/nightly`, refresh origin, push by
ordinary fast-forward, and inspect hosted CI at the exact published revision.
Record that result without claiming CI for a later revision.

## Risks and safe stopping point

Capacity-allocation failure remains source-inspected, not fault-injected:
a valid borrowed slice cannot safely impersonate an oversized allocation.
Caller-scoped runtime/clock identity, cooperative callbacks, fixed retained
storage, and human v0.1 acceptance remain unchanged. If verification fails,
repair or remove only this run's changes. Stop after this one verified
construction checkpoint and its publication evidence.

## Completed local evidence

The focused target passes nine tests and the full locked baseline passes
120 tests, formatting, all-target check/Clippy, and warnings-denied rustdoc.
No production change, dependency, or lint exception was needed. Both author
and independent Codex complete-diff/source reviews found no blocking defect.

The temporary review aids under `target/review-2026-09-16` find 42 Markdown
files, 181 resolving relative links, 84 exact traceability test references,
and 32 Rust files with zero physical/comment width findings and three unchanged
fulfilled lint expectations. All five changed rendered documents match source
content and pass browser DOM/layout inspection at 1280px with no page overflow
or console warnings/errors. Two screenshot attempts timed out, so pixel-level
inspection is unavailable; this limitation does not imply a visual screenshot
pass. Git whitespace checks pass. No new checker is adopted.
