# Host elapsed-clock integration

## Objective and selection

Enable host-time polling through a reusable HostClock and one finite runnable
public-API consumer under RFF-LIB-REQ-022 revision 1. Clean input main
`158ce34cd10517e52e0aa3548629e5e1ee43853a` matches refreshed origin/main.
All existing scheduling recipes use ManualClock. Configuration already has
public consumer coverage; further lifecycle wrappers have no demonstrated
consumer need. ADR-0047 compares the narrow adapter with consumer-local code,
SystemTime and a runtime-owned loop.

## Acceptance and components

- One private std::time::Instant origin; one sample and saturating subtraction
  per Clock read. Default creates a fresh clock. No allocation, cached time,
  hidden work, thread, dependency or native-instant public API.
- Preserve manual time and existing consumers. Clarify passive host progression
  and same-origin/platform limitations without promising clock fault recovery.
- Add examples/host_clock.rs: five explicit polling turns, caller-owned waits,
  one periodic cursor, one application and explicit stop. No endless loop.
- Verify exact conversion privately and the public host turn under controlled
  time; smoke-test live use without timing thresholds. Verify the identical
  example independently, including metadata, source hashes and bounded output.
- Update guide, source register, requirements, ADR, traceability and state.

## Verification and stopping point

The locked input format/check/Clippy/324-test/rustdoc/whitespace baseline passes.
Run focused tests/example, the final locked baseline and independent consumer.
Review physical/comment widths, scoped expectations, relative Markdown links,
rendered changed documents and the complete diff with independent Codex review.
Commit on main after acceptance, fast-forward push and inspect exact hosted CI.
Record evidence and stop at this one increment. If it cannot pass, preserve
research and revert only current-run implementation. No release is authorized.

## Risks

Host elapsed time is observational and platform-dependent. Polling/callbacks
and waits must return. Five turns bound invocation count, not execution time.
Platform clock bugs may regress readings; same-origin pairing is caller-owned.
The example is an integration recipe, not a deadline or portability proof.

## Local result

All locked final commands pass, including 332 workspace tests, five focused
clock tests and three example tests. The identical independent binary passes
check/Clippy/three tests/run, source bytes/SHA256 and metadata inspection.
Both observed live outputs report five turns, three work calls and Stopped;
the supported count remains one through five. The
[focused review](docs/verification/HOST_CLOCK_REVIEW.md) records exact commands,
package path and limits. Root and independent Codex accept the complete
16-file source/document/diff review without findings. Whole-tree widths,
links, exact test references and all 12 rendered-document content/structure
reviews pass. Publication and exact hosted inspection remain.
