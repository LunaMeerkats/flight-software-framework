# Event-queue resource and failure review

Date: **2026-09-20**
Scope: **Existing ADR-0013 construction, saturation, and slot reuse**

## Decision and inspected boundary

Retain the current standalone queue and strengthen its public observations.
No production defect was found in `src/events.rs` at starting revision
`4bfbda5907b92b3b71cded0422c25dfca8d987a7`. This checkpoint supplements
RFF-REQ-005 and [ADR-0013](../adr/0013-bounded-structured-event-queue.md);
it changes neither their meaning nor runtime integration.

The constructor rejects zero, then calls `VecDeque::try_reserve_exact` before
returning an empty queue. It maps reservation errors to the requested logical
record count. Emission checks that count before copying an event; dequeue
returns the oldest accepted record. Full queues do not overwrite, recurse,
retry, or grow through an alternative path. These are source-inspected facts.

The rechecked Rust 1.98.1 standard-library documentation under
`SRC-RUST-DEQUE-RESERVATION` in the [source register](../research/SOURCES.md)
distinguishes capacity overflow from allocator failure and permits allocator
over-reservation. Keep an explicit logical bound independent of actual
allocation capacity. A custom allocator or unsafe oversized input fixture
would add risk without being necessary to observe this existing error path;
neither is introduced. There is no new architectural decision requiring an ADR.

## Executed public observations

The two regressions in `tests/event_queue.rs` are:

- `oversized_event_capacity_returns_exact_reservation_error`: requesting
  `usize::MAX` records returns exactly
  `CapacityAllocationFailed { requested: usize::MAX }`. Event records have
  nonzero size, so the requested storage is unrepresentable. The test does
  not exhaust available memory or inspect an allocator failure.
- `repeated_saturation_and_reuse_preserve_exact_fifo_and_logical_capacity`:
  eight cycles on each of two queues with logical capacities one and three
  fill exactly, reject two distinct records, remove one oldest record, accept
  an explicit retry, reject another emission while full, and drain the entire
  retained FIFO. Counts are checked after mutations/rejections; capacity and
  empty-state checks cover saturation and reuse. Repeated empty dequeues
  return `None`, and the next cycle refills the same queue.

Whole-event equality checks source, identifier, severity, and timestamp.
Distinct decreasing timestamps identify records across cycles and ensure
FIFO follows acceptance order. The rejected record whose later emission is
also rejected never appears. The four earlier standalone tests remain and
cover metadata, zero capacity, and capacity-two behavior.

## Resource and evidence limits

The queue retains at most the configured number of records; dequeued records
are caller-owned. Actual allocator bytes, copied references in identifiers,
and consumer retention are outside that logical bound. The source reserves
the complete bound before emission, but this run does not instrument
allocation/deallocation or demonstrate an actual heap-allocation failure.
Repeated reuse does not prove a particular internal ring-buffer layout.

No concurrency, event filtering, delivery guarantee, persistence, automatic
retry, panic/hang containment, or timing guarantee is added or established.
The existing opt-in returned-work event tests remain separate integration
evidence. Human v0.1 acceptance and the wider Stage 4 review remain open.

## Verification

The initial locked baseline passes 120 tests on Rust/Cargo 1.98.0,
rustfmt 1.9.0-stable, and Clippy 0.1.98. The focused command
`cargo test --locked --test event_queue` passes all six tests. After applying
rustfmt's requested line wrapping, all-target Clippy with warnings denied
passes without a new lint expectation. The final locked baseline passes
122 tests and every command below.

Completed final commands:

```text
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
git diff --check
```

Rustdoc uses `RUSTDOCFLAGS=-D warnings`. Host adapters, sample, workflow,
and ADR-0018/0019 probes are unchanged, so their separate local commands are
not triggered. The full suite includes both host test targets. Source widths,
relative links, rendered documents, and complete diff received local review.

Author and independent Codex reviews found no blocking defect. The whole-tree
audit passes 43 Markdown files, 187 relative links, 86 exact traceability test
references, and 32 Rust files with zero physical/comment width findings and
three unchanged fulfilled expectations. Seven rendered changed documents match
source content and pass browser DOM/layout review at 1280px with no page
overflow or console warnings/errors. The browser inventory's initial timeout
recovered on retry. Screenshot capture timed out; pixel inspection is
unavailable. Rendered-content/DOM inspection is the recorded adaptation and
does not imply a screenshot pass. Temporary aids/logs remain under
`target/review-2026-09-20`; no checker or gate was adopted.

## Published checkpoint and continuation

Commit `adb77e76ca47b629e2cdc18fd64bd29be58cf472` contains both regressions
and the reviewed checkpoint. Ordinary fast-forward publication succeeded and
the remote head matched. On 2026-09-19 UTC (2026-09-20 Sydney),
[hosted run 35466593661](https://github.com/LunaMeerkats/flight-software-framework/actions/runs/35466593661)
passed that exact push/checkout revision: one Windows job and all 20 steps.
Logs confirm 122 workspace tests including both new regression names,
14 focused adapter tests, five focused sample tests, the sample executable,
and every configured baseline command.

Actual runner: 2.337.0; image: `windows-2025-vs2026` version `20260907.229.1`
(requested label `windows-2025`). Actual Rust/Cargo: 1.98.1; rustfmt:
1.9.0-stable; Clippy: 0.1.98. This matches the previously reviewed hosted
environment. The all-target hosted baseline and companion whole-tree source
review preserve existing policy without a new waiver. Local Rust/Cargo remain
1.98.0. JSON and full logs are under `target/review-2026-09-20`.

This documentation-only follow-up records the completed result with unchanged
Rust, Cargo, workflow, and lint inputs; it does not establish a hosted pass
for its own later revision or human v0.1 acceptance. The likely next bounded
task is lifecycle registration/construction ownership and resource review,
selected after reconciling existing tests. Broader scope remains unapproved.
