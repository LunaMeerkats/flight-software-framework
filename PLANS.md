# Stage 4 application-identity scope review

Date: **2026-09-22**
Status: **Complete: checkpoint published and exact-revision hosted CI passed**

## Objective and context

Verify the public consequence of the caller-scoped `ApplicationId` decision:
equal-position identities from separate registries or runtimes compare equal and
address the corresponding local slot. The starting revision is
`a46ad83078a52609b4a75eabcdbec68ff69d0351`, clean and equal to refreshed
`origin/codex/nightly`. Its hosted run 35534913845 passed; the initial locked
local baseline passes 124 tests on Rust/Cargo 1.98.0.

This is a bounded Stage 4 public-API risk checkpoint. ADR-0003 and source
documentation already state that identity origin is not encoded and that
cross-owner mixing is a caller error. Existing tests cover stable local keys,
out-of-range foreign keys, transitions, callback suppression, and capacity, but
do not execute the equal-position alias case.

## Acceptance criteria and files

- Add one public-API registry test proving exact equality and local-slot
  transition when a same-position key comes from another registry.
- Add one public-API runtime test proving that the same-shaped foreign key
  invokes the local owned application, not the foreign owner.
- Preserve production code, API, requirements, dependencies, and lint policy.
- Record the observed caller-contract boundary and alternatives in
  `docs/verification/APPLICATION_ID_SCOPE_REVIEW.md`; update README, project
  state, roadmap, and traceability. Confirm explicitly when no new external
  source was needed and leave the source register unchanged.

## Verification

Format before running the two focused targets, then run the required locked
Cargo baseline with warnings-denied Clippy and rustdoc. Audit Rust physical and
comment widths, relative links, traceability names, changed rendered documents,
and the complete diff. Unchanged host adapters, workflow, and ADR-0018/0019
experiments do not trigger separate local commands.

After local acceptance, commit on `codex/nightly`, refresh origin, publish by
ordinary fast-forward, and inspect exact-revision hosted CI. Record completed
publication evidence separately without projecting a pass onto a later commit.

## Risks and safe stopping point

The tests deliberately demonstrate an existing aliasing hazard; they do not
authorize cross-owner key mixing or establish issuer provenance. Adding an
origin token would change the public identity model and requires a separate
design checkpoint with bounded construction, equality, exhaustion, and
serialization implications. If verification fails, repair or remove only this
run's changes. Stop after this one evidence checkpoint and its publication
record.

## Completed local checks

Both new regressions pass: six lifecycle-registry, twelve application-runtime,
and 126 workspace tests. Formatting, all-target checking, warnings-denied
Clippy/rustdoc, and whitespace checks pass. No production change, dependency,
or lint exception was needed. The whole-tree audit passes 32 Rust files with
zero width findings, no block comments, and three unchanged fulfilled
expectations, plus 45 Markdown files, 194 resolving relative links, and 90 exact
traceability test references. Rendered document structure and exact content
comparisons pass; complete source/diff review found no blocking defect.

## Publication result

Checkpoint `9ea7f48f827031fcbeb63f712c97dfbe14cc629c` was published by
ordinary fast-forward; the remote head matched. Hosted run 35658016416 passed
that exact push and checkout: one Windows job, all configured steps, 126
workspace tests including both new regressions, 14 focused adapter tests, five
focused sample tests, and the sample executable.

This documentation-only follow-up records completed evidence with unchanged
Rust, Cargo, workflow, and lint inputs. Its rendered content, links, and diff
are reviewed separately. A later revision requires its own hosted result.
