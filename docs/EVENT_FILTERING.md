# Filter optional events before enqueue

Use `EventFilter` when routine optional diagnostics should not occupy the same
bounded storage as admitted warnings or errors. This is an explicit policy for
each emission attempt; `EventQueue::emit` continues to attempt every record.
The [public-only consumer tests](../tests/event_filter.rs) exercise the policy
and an ordinary-work producer without loading private sample modules.

## Select and inspect the result

```rust
use std::time::Duration;
use rust_flight_framework::{
    Event, EventEmitOutcome, EventFilter, EventQueue, EventSeverity, EventSource,
    EventTimestamp, FilteredEventOutcome,
};

let mut events = EventQueue::new(1)?;
let filter = EventFilter::new(EventSeverity::Warning);
let optional = Event::new(
    EventSource::Framework,
    EventSeverity::Informational,
    1_u8,
    EventTimestamp::from_elapsed(Duration::ZERO),
);
assert_eq!(events.emit_filtered(&optional, filter), FilteredEventOutcome::FilteredOut);
assert!(events.is_empty());

let error = Event::new(
    EventSource::Framework,
    EventSeverity::Error,
    2_u8,
    EventTimestamp::from_elapsed(Duration::ZERO),
);
match events.emit_filtered(&error, filter) {
    FilteredEventOutcome::FilteredOut => unreachable!("Error meets this threshold"),
    FilteredEventOutcome::Emitted(EventEmitOutcome::Recorded) => {}
    FilteredEventOutcome::Emitted(EventEmitOutcome::QueueFull { capacity }) => {
        // The host retains `error` and chooses whether/when to retry.
        assert_eq!(capacity, 1);
    }
}
# Ok::<(), rust_flight_framework::EventQueueCreateError>(())
```

The guide's snippet passes a standalone Markdown rustdoc test. The integration
tests below provide the consumer evidence. The local threshold order is explicit:

| Minimum severity | Informational | Warning | Error |
| --- | --- | --- | --- |
| Informational | Admitted | Admitted | Admitted |
| Warning | Filtered out | Admitted | Admitted |
| Error | Filtered out | Filtered out | Admitted |

`minimum_severity()` returns the chosen threshold and `allows(severity)` exposes
the same decision without a queue. The policy is a copied immutable value;
passing another value on the next call changes only that attempt. It does not
remove previously accepted records or alter later unfiltered calls. This order
does not add priority, response or compatibility semantics to `EventSeverity`.

Filtering precedes capacity inspection. A suppressed record returns
`FilteredOut`, even when storage is full, without changing retained records.
An admitted record delegates to the existing `emit` behavior: append in FIFO
order or reject the newest with the exact logical capacity. The caller retains
the borrowed event after every outcome. Dequeue frees a slot for an explicit
retry; no retry, recursive diagnostic or background work is created.

## Compose optional diagnostics with work

The consumer gives ordinary work a caller-owned event queue and injected clock.
It uses the selected context identity as source and chooses an Error threshold
for optional Informational diagnostics. Suppression is successful work in this
mission policy, so repeated callbacks continue while queue capacity remains
available. A later Error can occupy that slot with its original identity and
timestamp. Admitted saturation returns a typed application error; the selected
application enters terminal `Failed` while a healthy peer remains operable.

This policy differs from the [identity consumer](APPLICATION_WORK_IDENTITY.md),
which commits its demonstrated effect only after `Recorded`. Decide whether a
diagnostic is optional before treating `FilteredOut` as acceptable. Suppression
does not establish that a record was stored or delivered. The checked serial
`RefCell` borrow can itself fail before filtering; the filter does not grant
access to an already borrowed queue or provide synchronization.

Runtime failure-event wrappers continue to use unfiltered `emit` and the
existing `EventEmitOutcome`. This opt-in API does not intercept those records.
An admitted Warning can still fill the queue and cause a later Error to be
rejected. There is no reserved critical capacity, eviction or delivery guarantee.
Applications still create their events and timestamps before calling the API;
filtering does not promise to skip that work or avoid a caller's clock read.

The filter retains one severity and allocates nothing. The queue keeps its
existing positive record bound, which does not bound process memory, callback
allocation or execution time. APIs remain provisional; this is experimental host
software without flight qualification or operational-suitability evidence.

## Verify the consumer outside the workspace

Use an unused destination outside this checkout:

```powershell
$framework = 'E:/Projects/flight software framework'
$consumer = Join-Path $env:TEMP ('rff-event-filter-' + [guid]::NewGuid().ToString('N'))
cargo new --lib --edition 2024 --vcs none $consumer
cargo add --manifest-path "$consumer/Cargo.toml" rust-flight-framework --path $framework --offline
New-Item -ItemType Directory -Path "$consumer/tests" | Out-Null
Copy-Item -LiteralPath "$framework/tests/event_filter.rs" -Destination "$consumer/tests/event_filter.rs"
cargo check --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets
cargo clippy --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets -- -D warnings
cargo test --manifest-path "$consumer/Cargo.toml" --locked --offline --test event_filter
cargo metadata --manifest-path "$consumer/Cargo.toml" --locked --offline --format-version 1
Get-FileHash -Algorithm SHA256 "$framework/tests/event_filter.rs", "$consumer/tests/event_filter.rs"
```

Inspect the independent workspace root, one member, two graph packages, sole
framework path dependency and identical source bytes/hashes. No maintained crate
is added. Actual commands/results and review are in
[the focused record](verification/EVENT_FILTERING_REVIEW.md);
[ADR-0031](adr/0031-explicit-event-severity-filtering.md) records the alternatives.
This controlled consumer does not establish external adoption or API stability.
