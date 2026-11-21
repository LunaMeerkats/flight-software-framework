# Inspect application-owned observations through public runtime APIs

## Objective and context

Implement RFF-LIB-REQ-018 revision 1 and ADR-0043. A host should read ordinary
owned application observations between callbacks without exporting them into
borrowed Cell slots. At input both owners exposed only lifecycle state.
Clean main input is `304570ec5341dfa65b8ab1da7a3e2b786afadb01`, matching
refreshed origin/main. Preserve all earlier consumers and their semantics.

## Acceptance and components

- Add shared `application` lookup to Runtime and MessagingRuntime; all known
  lifecycle states, existing unknown-slot error, no callback/inbox mutation.
- Add a public-only `tests/application_inspection.rs` consumer with ordinary
  owned fields, lifecycle/work/dispatch observations and failure/peer history.
- Record shared-reference/interior-mutability limits in API docs and a guide.
- Verify negative borrowing probes with positive counterparts, and identical
  consumer bytes/hash/metadata in an independent Cargo package.
- Update architecture, requirements, roadmap, traceability and concise state.

## Verification and stopping point

Run the locked baseline before significant edits and after implementation:
format, all-target check/Clippy, workspace tests, warnings-denied rustdoc and
whitespace. Run the focused integration test and independent recipe. Inspect
whole-tree widths, relative Markdown links and changed rendered documents.
Review the full diff and obtain independent Codex review. After acceptance,
commit and fast-forward push main; inspect exact-revision hosted CI.

Shared inspection does not prevent application interior mutation, supply
mutable access or transfer non-Copy output. No new dependency or maintained
crate. Safe fallback is the reviewed input; revert only this run's own changes
if needed. Stop after this one verified increment, recording actual evidence.

## Local result

Input baseline passed 285 tests. Both accessors and seven public-only consumer
histories pass; all four rustdoc examples and explicit negative/positive
compiler probes pass their expected outcomes. The identical independent
package passes check/Clippy/seven tests with matching bytes/hash and metadata.
Final locked baseline passes 296 tests and all static/documentation checks.
An initial OS4551 block before one existing test's execution cleared on an
unchanged retry; both attempts are recorded. Source review by root and
independent Codex accepts the three source blobs in the
[focused record](docs/verification/APPLICATION_INSPECTION_REVIEW.md).
Complete root/independent Codex document/diff review passes, as do whole-tree
width/link/traceability and 12 rendered-document inspections. Publication and
exact hosted verification remain; no source publication is claimed yet.
