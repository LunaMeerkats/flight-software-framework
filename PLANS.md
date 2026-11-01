# Explicit cancellation of callback-owned replies

Status: Local implementation and review complete; hosted checks pending — 2026-11-02

## Objective and selection

An unavailable, permanently Failed subscriber can keep the callback consumer's
single reply pending indefinitely and block every later new reply. Root Codex
and independent consumer_assessment select explicit mission cancellation over
automatic lifecycle cancellation, admission acknowledgements or a retry service.
Existing public APIs suffice. RFF-LIB-REQ-011 precedes implementation; existing
RFF-LIB-REQ-010 revision 1 contracts and seven regressions remain unchanged.

## Acceptance and components

- Add a Cancel topic to tests/callback_publication.rs. On dispatch it clears
  only the current application-owned reply, with no publication or peer cleanup.
- Verify partial acceptance, rejected peer FIFO, idle/repeated cancellation,
  retry after cancellation, new-reply admission, permanently Failed subscribers,
  command inbox saturation/FIFO and deliberate producer stop/restart retention.
- Record the local policy and alternatives in ADR-0036; update the existing
  consumer guide, requirements, traceability, roadmap, architecture and state.
- Run focused tests, locked baseline, identical independent package checks,
  source hashes/metadata, width/waiver/link audits and rendered document review.
  Obtain independent Codex source and full-diff review before committing main.
- Refresh origin; ordinary fast-forward source push and inspect exact hosted CI.
  Stop after this one verified increment, retaining separate release gates.

## Baseline and risks

Clean main input 1972d66f20d08ab3590e84035073bc4d015b35b9 matches fetched
origin/main. Initial locked format/check/Clippy/224 workspace tests and
warnings-denied rustdoc pass, as do working/committed whitespace checks.
Input run 41204977193/job 124548108020 passes all 20 steps at that exact head;
raw logs are retained locally. Source audit: 53 Rust files, 23,070 physical and
1,355 comment-only lines, no width findings, three existing expectations;
102 Markdown files, 583 relative links and 188 exact test references resolve.
Local tools remain Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy 0.1.98.

Cancel targets whatever reply exists when dispatched, without an operation ID.
It cannot retract accepted copies or promise cancellation while its command is
full, unavailable or awaiting dispatch. Lifecycle operations do not implicitly
clear the application's pending reply. No acknowledgement or authentication is
added. Command ordering remains a host responsibility.

## Safe stopping point

One verified consumer policy with no library, dependency or workflow change.
If checks fail, remove only current-run implementation where safe and retain
decision/failure evidence. No unrelated work, release, tag or deployment.

## Actual local results

Focused tests/Clippy pass all 12 tests. Final locked baseline passes 229 workspace
tests. An independent package passes check/Clippy/12 identical tests plus its
generated smoke test; source bytes, SHA256 and metadata match the focused record.
Whole-tree audit finds 53 Rust files, 23,239 physical/1,357 comment-only lines,
zero width findings and three unchanged expectations. All 595 relative links and
193 exact test references resolve across 104 Markdown files. All 12 changed
documents pass rendered content/structure review, without a pixel-QA claim.
Root and independent cancellation_review accept the complete 13-file diff with
no findings. Source is unchanged after verification. Publication and exact hosted
inspection follow, with no further behavior selected.
