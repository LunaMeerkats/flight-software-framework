# Bounded host command-result feedback

Status: Complete verified implementation checkpoint — 2026-11-04 Australia/Sydney

## Objective and selection

Let a serial host distinguish a busy New command, matching cancellation and
stale controls after dispatch. Existing enqueue outcomes and copied test-only
pending observations cannot provide this consumer contract. Root Codex and
independent consumer_assessment select a capacity-one typed result mailbox over
bus-routed acknowledgements or persistent identities. RFF-LIB-REQ-013 revision 1
and ADR-0038 precede implementation; retain all seven correlated histories and
the original callback consumer unchanged.

## Acceptance and components

- Extend tests/correlated_commands.rs with one shared typed result slot and an
  explicit host turn that skips producer dispatch while the slot is occupied.
- Report exact command, reply acceptance flags, busy pending ID, cancellation
  or unmatched control; test full-slot bypass, FIFO/backpressure, malformed and
  routing errors, explicit draining and restart retention without false results.
- Update its guide, requirements, ADR, source register, focused review,
  traceability, roadmap, project state, README and contributor commands.
- Run focused tests/Clippy, locked baseline and independent-package recipe;
  compare bytes/hashes/metadata and preserve old test bodies. Review complete
  diff, source form, links and rendered documentation with independent Codex.
- Commit on main after applicable checks; refresh origin, push ordinarily and
  inspect hosted CI at the exact published revision.

## Baseline, risks and stopping point

Clean main input 4f64a4e70037609c6f382b20ff6c13967a54e05e matches fetched
origin/main. Local Rust/Cargo 1.98.0, rustfmt 1.9.0-stable and Clippy 0.1.98
are unchanged. Initial locked baseline passes 236 workspace tests. Exact input
hosted run 41451380499 passes all 20 steps with independent raw-log inspection.

The gate is a serial mission policy and precedes lifecycle validation. Bypassing
it returns a callback error and uses ordinary terminal failure/queue cleanup;
it cannot recover the consumed command. Results are local snapshots and cannot
prove destination processing or survival after lifecycle clearing. Retain session
issuer/correlation limits. No library API, dependency, workflow, licence or
consequential gate changes. If checks fail, remove only this run's source changes
where safe and preserve useful decision/failure evidence. Stop after this one
verified increment.

## Actual local results

14 focused tests and Clippy pass. All seven original test bodies are unchanged.
Final locked baseline passes 243 workspace tests and warnings-denied rustdoc.
Identical independent Cargo consumer check/Clippy/14 tests plus generated smoke
test pass; metadata confirms an independent workspace and sole framework path
dependency. Exact source bytes/SHA256 match the focused record. Root and
independent verification_review accept the complete source/document diff without
findings. Whole-tree width/waiver/link/exact-test-reference and 12 changed rendered
document content/structure checks pass. Implementation
4ee3c5091ff49e85b9c9595958e040c051f8bec5 equals the accepted staged patch and
is published by ordinary fast-forward to origin/main. Exact run 41641979417/job
125937510804 passes all 20 steps, 243 workspace/14 adapter/five sample tests and
unchanged 222-line sample. Hosted tools/runner match input; image advances to
20261030.294.1. Root/independent raw-log review accepts this evidence. Record-only
completion receives separate review/publication/exact hosted inspection in
automation memory/report. No further behavior is selected.
