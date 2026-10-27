# Bounded host event presentation

Status: In progress — 2026-10-28

## Objective and context

Complete RFF-LIB-REQ-007/ADR-0032: a public-only event-output consumer with one
fixed pending line and explicit partial-write progress. The clean input is
6ab84023892b19642b76705bd184a95bb43754df. Initial locked checks pass 193 tests;
the input's exact hosted run 40329895459 passes all 20 steps. Existing filtering
and FIFO APIs suffice; no library API or runtime change is planned.

## Acceptance criteria and components

- examples/event_output.rs owns a writer, one pending event, fixed line and byte
  offset. One host turn attempts at most one write. Tests cover FIFO bytes,
  short writes, errors and no progress, idle behavior, rendering rejection,
  maximum timestamps and optional filtering.
- The example and identical independent Cargo source pass check/Clippy/test/run;
  metadata and SHA256 hashes confirm a sole library path dependency and matching
  source. No maintained consumer crate or dependency is added.
- docs/EVENT_OUTPUT.md documents setup, errors, resources and delivery limits.
  Focused verification, requirements, ADR, sources, traceability, roadmap, state
  and entry points agree. Full baseline, source form, rendered docs, links and
  complete diff receive root and independent Codex review before publication.

## Verification approach

Run the AGENTS.md locked baseline, cargo test/run --locked --example event_output,
and the independent-package guide recipe. Preserve raw logs under ignored
target/review-2026-10-28-event-presentation. Conditional host/probe/workflow checks
follow actual touched scope. Refresh origin, ordinary fast-forward push, verify
remote head and inspect exact hosted checkout/results. Local success does not
establish hosted success.

## Risks and safe stopping point

Short writes are valid; do not retry accepted bytes. Err consumes no bytes under
the Write contract. Writer calls may block; no timing guarantee is introduced.
Flush is separate and its failure cannot safely reconstruct cleared records.
Keep the original event on render failure and never skip past a pending record.
Stop after this coherent reviewed checkpoint, without another capability. If
checks fail, preserve decisions/evidence and remove only current-run implementation
where safe; do not discard other work or weaken checks.

## Actual local acceptance

Final locked baseline passes 201 workspace/eight focused tests and the runnable
output. The identical independent package passes check/Clippy/eight tests/run,
normalized metadata and byte/hash inspection. Whole-tree review finds zero width
issues and no new waiver. All 531 relative links/165 exact test references
resolve; all 12 changed rendered documents pass content/structure inspection.
Root and independent Codex output_review accept the complete 14-file source,
document and diff with actual evidence and no remaining findings. Source
publication and exact hosted inspection remain before checkpoint completion.
