# Move application-owned output to a serial host

## Objective and selection

Implement RFF-LIB-REQ-019 revision 1 in one public-only runnable consumer.
Input main `356697a288d11fc168e450cff0d2579f363557d1` is clean and matches
refreshed origin/main. The 296-test locked baseline and exact input hosted
run 44959878838 pass. The preceding shared-inspection increment is complete.

Owned observations are now accessible, but existing output recipes require
external borrowed slots and Copy values. Demonstrate ownership transfer of
a non-Copy record using an application-defined shared drain and existing
lookup before inventing a framework output API.

This closes a concrete integration gap, is independently verifiable and
leaves existing APIs/consumers intact. More retry machinery has lower value
after the completed priority checkpoint. Unrestricted mutable access and a
new output trait add policy without evidence. Cell, RefCell and external
slot alternatives are compared in ADR-0044.

## Acceptance and components

- `examples/owned_output.rs`: fixed application slot, shared take, one host
  slot, production gate and typed sink rejection returning ownership.
- Tests in the test-enabled example exercise both owners, no duplicate take,
  local rejection/acceptance, lifecycle retention, failure and peer isolation.
- `docs/OWNED_OUTPUT.md`: runnable/independent recipe and explicit ownership,
  lifecycle, resource and sink-result boundaries.
- Requirements, ADR, source register, architecture, roadmap, traceability,
  repository guidance and concise state track actual results.
- No library API, dependency, workflow, lint or historical consumer changes.

## Verification and safe stopping point

Run focused example test/run, the locked format/check/Clippy/workspace-test/
warnings-denied-rustdoc/whitespace baseline, independent binary check/lint/
test/run, metadata, bytes/SHA256 and transcript comparison. Audit handwritten
Rust widths, waivers, relative links and exact traceability references;
render and inspect changed documents and review the complete diff with an
independent Codex reviewer. Commit and ordinary fast-forward push main after
acceptance, then inspect exact-head hosted CI.

Callbacks and sink turns must return. The sink contract is definite local
rejection returning the entire record; adapting ambiguous external effects
requires a different policy. Logical slots do not bound arbitrary consumer
allocation or runtime. Stop after this one increment. Preserve input as the
fallback; revert only this run's changes if a healthy result is impossible.

## Local result

The example and eight focused tests pass. The identical independent binary
passes all-target check/Clippy/eight tests/run, with exact source and stdout
equality and verified independent metadata. The final locked baseline passes
304 tests and all static/rustdoc checks. Root and independent Codex accept
the source hash in the [focused review](docs/verification/OWNED_OUTPUT_REVIEW.md).
Whole-tree width/link/test-reference audit and all 12 changed rendered
documents pass review. Reviewed tree `8cff135607143d77091fffe8afacd3cbb6431d68`
is implementation `6fef2a7896aad016e585c6b08b4ce773d7e55343`, published by
ordinary fast-forward. Exact hosted run 45134831252 passes all 20 steps,
304 workspace tests, separate adapter/sample checks and unchanged 222-line
host output. This record-only completion gets its own review and hosted
check. Codex accepts the bounded increment; no implementation remains.
