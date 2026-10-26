# Bounded plan: explicit event severity filtering

Date: **2026-10-27**
Status: **Complete: verified, reviewed, published and implementation CI passed**

## Objective and context

Enable a recurring diagnostic producer to omit optional low-severity events
before they occupy bounded event storage. Select one immutable severity filter
and explicit suppression result; retain unfiltered queue and runtime failure
contracts. Root Codex and independent consumer_assessment assess current source
friction and alternatives. This is controlled consumer evidence, not adoption.

Input is clean codex/nightly at 1759cbe2139220126fe4030c9b1d2b39f7745883.
The initial locked local format/check/Clippy/test and warnings-denied rustdoc
baseline passes all 186 workspace tests. Local Rust/Cargo 1.98.0, rustfmt
1.9.0-stable and Clippy 0.1.98 are unchanged. RFF-LIB-REQ-006 revision 1 and
ADR-0031 establish scope and acceptance before implementation.

## Acceptance and components

- src/events.rs defines EventFilter, its minimum accessor and allows decision,
  FilteredEventOutcome, and explicit EventQueue::emit_filtered. src/lib.rs
  exposes the small surface. No general EventSeverity ordering is added.
- Excluded events return FilteredOut before capacity inspection or mutation;
  allowed events delegate to existing emit and retain exact outcomes/FIFO.
  Each call supplies its filter; no persistent queue policy or storage is added.
- tests/event_filter.rs covers all nine severity decisions; empty, partial
  and full queue suppression; allowed saturation; record/FIFO retention and
  explicit retry. Existing event and runtime failure tests remain regressions.
- A public-only ordinary-work producer attributes events using its selected
  context ID and treats optional suppression explicitly. Tests distinguish
  omission, recording and rejection, including later allowed error acceptance
  and error rejection when allowed records already fill the queue.
- Identical consumer tests pass in an independent Cargo package. Inspect its
  metadata for one independent member and sole framework path dependency, and
  compare SHA256 source hashes.
- docs/EVENT_FILTERING.md explains setup, policy, errors, resources and host
  drain. The focused EVENT_FILTERING_REVIEW.md records actual results and
  limits; requirements, ADR, sources, traceability, roadmap and state agree.

## Verification and review

Run the full AGENTS.md locked baseline and
`cargo test --locked --test event_filter`, followed by independent-package
check/Clippy/test, metadata and
source-hash inspection. Inspect whole-tree handwritten source widths and touched
waivers, resolving relative Markdown links, rendered Markdown/generated API
documents and the complete diff. Complete root and independent Codex source,
document, consumer and evidence review before commit/publication.

Official Rust Reference match expressions and VecDeque documentation inform
language branching and existing FIFO operations only. Severity selection is
local policy. No workflow, host sample, existing consumer source, dependency or
borrowing-experiment change is planned; conditional checks follow actual scope.

## Risks and safe stopping point

Suppression is deliberate omission, not acceptance or delivery. Allowed records
can still fill storage and reject errors. Unfiltered callers and runtime failure
reporting bypass the per-attempt policy. No priority, reservation, all-off state,
counter, source rule, retry or new event service is added. Existing diagnostic
identity tests keep their acceptance-before-effect contract.

Stop after this coherent increment passes applicable verification/review,
ordinary fast-forward source publication and exact hosted inspection. If checks
cannot pass, remove only this run's implementation where safe and retain useful
decisions and the exact blocker. Do not start unrelated dispatch, recovery or
scheduling work. Record actual results before marking this plan complete.

## Actual results

Final locked baseline passes 193 workspace tests, seven focused and seven
identical independent tests; check/Clippy, metadata, source hashes and the guide's
one standalone Markdown doctest pass. Source review finds zero width findings
in 50 Rust files and no new waivers. All 510 relative links/157 exact test
references resolve. Twelve changed documents pass rendered content/structure
review, and generated APIs are inspected. Root and independent Codex filter_review
accept the complete frozen 15-file source/document/diff and actual evidence with
no remaining findings. Ordinary fast-forward publication of
db994e776c68fb5432e646afb2ae5344cb2444e5 passes exact hosted run 40329281385,
job 121594066549 and all 20 steps, 193 workspace/14 adapter/five sample tests,
rustdoc, both whitespace checks and the unchanged 222-line sample. Hosted tools
and runner image match input. This documentation-only completion receives its
own full review and exact hosted inspection; no later revision inherits an
unobserved pass. Stop after this checkpoint without unrelated work.
