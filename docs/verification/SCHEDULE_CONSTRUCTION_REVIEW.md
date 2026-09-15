# Schedule construction review

Date: **2026-09-16**
Status: **Local verification complete; publication pending**

## Reviewed boundary and decision

This Stage 4 checkpoint reviews `WorkSchedule::new` at
`75f7cbdc659d73d2430b9987bfe64b1afa62ff35` and two new public regressions
against [ADR-0015](../adr/0015-caller-driven-scheduled-work.md).
No production defect was found. The previous constructor test rejected only
one two-item descent; no test replaced the caller's input after construction.
The added observations cover later/first-error diagnostics and copied agenda
ownership through ordinary scheduled dispatch.

Retain the existing validated, copied finite agenda. Sorting input would hide
configuration errors and change the recorded policy; borrowing input would
couple its lifetime to dispatch. Neither change is needed. No new architecture
decision, requirement meaning, API, dependency, source reuse, or external
research is introduced. Repository contracts and inspected source are the
authoritative evidence for this checkpoint.

## Source-inspected resource and failure contract

1. `validate_order` walks adjacent pairs in caller order, permits equality,
   and returns the first descent's zero-based index and exact two instants.
2. Validation completes before `try_reserve_exact` requests storage for the
   input item count. A returned reservation error maps to
   `StorageAllocationFailed { requested }`; no partial schedule is returned.
3. Construction copies the complete slice and initializes the cursor to zero.
   The empty slice is valid. Application lifecycle and runtime/clock origins
   are not validated by construction.
4. Dispatch changes only the private cursor in the schedule. No public operation
   appends items. Consumed items remain retained until the schedule is dropped;
   `remaining()` counts unattempted items, not released allocation slots.
5. Direct and messaging owners use the same private timing/consumption decision
   and delegate due work through their respective ordinary-work boundaries.

The logical retained bound is the configured item count. This is not an exact
allocator-byte, stack, process-memory, or callback-resource bound. Source
inspection establishes validation/reservation/copy order and the absence of
agenda growth; the public regressions do not instrument allocation or release.
Unlike an integer-capacity inbox constructor, this API takes a valid borrowed
slice of nonzero-sized items. No invalid slice, unsafe allocator hook, or
memory-exhaustion experiment is introduced to force its allocation-error path.
That path remains source-reviewed rather than experimentally verified.

## Public regression evidence

- `construction_reports_the_first_descending_pair_at_nanosecond_precision`
  covers valid and equal-time prefixes, a descent at the final pair, and
  multiple descents. Exact first-error indices and adjacent instants are
  asserted for values differing within one second.
- `copied_agenda_preserves_items_after_caller_storage_is_changed_and_dropped`
  constructs from a caller-owned vector, replaces all its items, then lets
  that vector leave scope. Three dispatches observe the original identities,
  instants, equal-time order, exact remaining counts, and callback trace.
  A final call reports completion without another callback.

The existing direct and messaging schedule regressions retain their waiting,
clock-read, final-consumption, lifecycle/error, configuration, and peer-inbox
evidence. The new tests supplement RFF-REQ-004; they do not prove allocator
exhaustion handling, arbitrary panic/hang containment, timing guarantees,
recurrence, or runtime/clock-origin validation.

## Verification and continuation

The initial required locked baseline passes 118 tests. The focused command
`cargo test --locked --test scheduled_work` passes all nine tests.
The final workspace suite passes 120 tests. These commands pass locally:

```text
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo doc --locked --workspace --all-features --no-deps
git diff --check
```

Rustdoc uses `RUSTDOCFLAGS=-D warnings`. Local Rust/Cargo are 1.98.0,
rustfmt is 1.9.0-stable, and Clippy is 0.1.98. The adapter, sample, workflow,
and ADR-0018/0019 probes are unchanged, so their separate local commands are
not triggered. The full suite still includes both host test targets.
Independent review finds the two tests cohesive and below the function-size
threshold, with no new exception. Author and independent Codex complete-diff
reviews found no blocking defect. The whole-tree audit passes 42 Markdown
files, 181 relative links, 84 exact traceability test references, and 32 Rust
files with zero physical/comment width findings and three unchanged fulfilled
expectations. All five changed rendered documents match their source content
and pass browser DOM/layout inspection at 1280px with no page overflow or
console warnings/errors. Two screenshot attempts timed out; pixel inspection
is unavailable and is not claimed as passing. Temporary aids/logs remain under
`target/review-2026-09-16`; no checker or policy gate is changed.

The likely next bounded checkpoint is event-queue construction and repeated
saturation/reuse, selected only after reconciling existing evidence. The
broader Stage 4 audit and human v0.1 entry-point/architecture acceptance remain
open. Revisit this record if construction accepts new input forms, schedule
storage becomes mutable, or dispatch ownership changes.
