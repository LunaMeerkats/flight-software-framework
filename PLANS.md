# Runnable bounded command session

Status: Complete verified implementation checkpoint — 2026-11-05 Australia/Sydney

## Objective and selection

Make command ingress, local feedback and subscriber output runnable through
public APIs. The previous consumer exists only as an asserting test fixture.
Root Codex and independent consumer_assessment select a standalone tested example
rather than extracting that fixture or adding a framework service. Preserve
existing callback and correlated test sources byte-for-byte. Define
RFF-LIB-REQ-014 and ADR-0039 before implementation.

## Acceptance and components

- One serial host issuer, producer, two subscribers, finite inboxes, one pending
  operation with two acceptance flags, one result and one output per subscriber.
- Explicit bounded producer/subscriber turns gate dispatch on occupied host
  slots. Host draining remains separate. Validate ingress; rejected requests
  remain caller-owned. No callback printing or test-only observations drive policy.
- Run a finite scenario exposing partial acceptance, busy feedback, gated turns,
  rejected-only retry and same-ID resubmission. Test malformed input, FIFO,
  output/ingress saturation, correlated controls, restart retention and issuer
  exhaustion for this independently authored policy.
- Add a tested Cargo example and guide/independent binary recipe. Update
  requirements, decision/source register, traceability, current state and entry
  points; record actual results in a focused review.
- Focused test/run, independent package check/Clippy/test/run/metadata/bytes/hash,
  locked baseline, source-form/link/rendered-document and complete diff review.
  Commit on main; refresh origin, push ordinarily, inspect exact hosted revision.

## Baseline, risks and stopping point

Clean main faab18be2fb06a4bec43f0889338409c5b71b26a matches fetched origin.
Independent verification_support accepts exact input hosted run 41642653583,
all 20 steps, 243 workspace/14 adapter/five sample tests and unchanged 222-line sample.
Initial locked format/check/Clippy/243-test baseline, warnings-denied rustdoc
and both whitespace checks pass before implementation.
Local tools remain Rust/Cargo 1.98.0, rustfmt 1.9.0, Clippy 0.1.98.

This is serial trusted host policy, not a framework runner or wire protocol.
Results describe local handling and enqueue snapshots; printing establishes no
physical/durable delivery. No library API, dependency, concurrency, release or
platform commitment. Revert only own unfinished changes if necessary, preserving
useful evidence. Stop after one tested, documented and reviewed increment.

## Actual local results

Ten focused tests and the 24-line executable pass. The final locked baseline
passes 253 workspace tests and warnings-denied rustdoc. Identical independent
binary check/Clippy/ten tests/run/metadata/bytes/SHA256 and exact output equality
pass; actual package/source identity is recorded in the focused review. Root and
independent consumer_assessment accept all source/tests without findings after
correcting driver admission handling. Existing callback/correlated sources are
unchanged. Root and independent verification_support accept the complete
14-file source/document/evidence diff without findings. Whole-tree source/waiver/
link/test-reference and all 12 changed rendered-document content/structure checks
pass.

Implementation 3d171dc22d9c2661baf4d077871ebc60cad83003 is published by ordinary
fast-forward to origin/main; the committed patch equals the accepted staged
patch exactly. Exact [push run 41855764215](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/41855764215),
job 126663830711, passes all 20 steps: 253 workspace tests including ten new session
cases and unchanged earlier consumers, 14 adapter/five sample tests,
warnings-denied rustdoc, both whitespace checks and the unchanged 222-line sample.
Root and independent verification_support accept actual checkout/log/source
metadata and the hosted result. Hosted Rust/Cargo 1.99.1, rustfmt 1.10.0-stable,
Clippy 0.1.99, runner 2.338.2 and windows-2025-vs2026 image 20261030.294.1
match the input run. Local tools remain separate on 1.98.0. This record-only
completion changes no source or configuration and receives separate complete
review, publication and exact hosted inspection in automation memory/report.
No second capability is selected.
