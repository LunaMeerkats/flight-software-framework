# Present events through bounded host turns

Run the public-only consumer:

```text
cargo run --locked --example event_output
```

The [example](../examples/event_output.rs) transfers structured events from an
`EventQueue` to a standard `std::io::Write` sink. It keeps one pending record,
one fixed-capacity rendered line and the accepted-byte offset. Each host turn
removes at most one queued event and makes at most one sink write. This lets the
host interleave output attempts with other work without a drain-until-empty
loop inside the presenter. A writer call can still block; the turn limit is not
a time bound or a scheduling guarantee.

This is a mission-owned presentation recipe. It uses existing public APIs and
does not add a framework sink service, event queue policy or output protocol.
[ADR-0032](adr/0032-bounded-host-event-presentation.md) records the alternatives.

The sample presents two admitted records in this format:

```text
t=0.000000000 source=framework severity=info event=started
t=0.250000000 source=worker severity=warning event=degraded
```

## Retain progress across output attempts

`EventOutput` owns its writer so that successive turns continue with the same
writer value. A positive short write advances only the accepted prefix. The
remaining bytes are offered on a later turn; the prefix is never deliberately
offered again. Completing one line clears the pending slot without removing
another event in that turn. An empty queue with no pending line returns idle
without calling the sink.

Every write error preserves the pending event, complete rendered line and byte
offset, and returns the original `io::Error`. This includes `Interrupted` and
`WouldBlock`: the presenter does not retry either error inside that turn. The
host chooses whether and when to call it again. A zero-byte write with a
nonempty remainder becomes `WriteZero` with the same retained state. Other
errors can require the host to stop output entirely. Retention does not mean
that a failed writer can recover.

Reusing the suffix requires the same continuous stream and exclusive ownership
of this output sequence. Reconnecting or replacing the underlying destination
can invalidate the offset even if the writer value remains present. There is
no cross-stream resume, acknowledgement protocol or crash recovery. A writer
that violates the standard write contract is outside this recipe's assumptions.

The sample uses a finite host loop for two admitted records and explicitly
flushes after their lines have been accepted. `run_sample` propagates any error
and drops its volatile presenter, including any pending event. Resuming progress
requires retaining the same `EventOutput` and calling `turn` again; restarting
`run_sample` or replaying a whole record does not preserve that cursor.
Flush can fail after records have left the presenter; replaying them then can
duplicate bytes already forwarded. Writer acceptance, successful flush and
physical or durable delivery are different observations. This example supplies
no record-atomic or durable delivery guarantee.

## Choose names and account for storage

The presenter maps its one known application ID to `worker` and framework
events to `framework`. `ApplicationId` is opaque and runtime-local; these names
are explicit mission policy rather than a numeric ID encoding. An unknown
application source produces a typed rendering error while retaining the raw
event. Equal-position keys from another runtime cannot be distinguished by this
mapping, so callers must retain the correct runtime association.

Severity and event names are also explicit finite mappings. Elapsed timestamps
use integer seconds and nine fractional nanosecond digits, preserving their
precision through `Duration::MAX`. This line format is example presentation,
not a stable serialization contract or compatibility claim.

The queue's configured capacity bounds its retained records. The presenter can
hold one additional dequeued event, including after a rendering or write error.
That pending record is outside the queue bound: dequeue has already freed its
old slot, so producers may refill it while output is incomplete. The presenter
uses a 96-byte fixed line plus its offset and one temporary fixed render buffer;
rendering rejection does not silently discard the structured record. It does
not consume the next FIFO event while one is pending.

The consumer tests apply [optional-event filtering](EVENT_FILTERING.md) before
output. A filtered-out record has no queued line to present. Admitted events retain
the queue's reject-newest policy and can still saturate storage. Queue capacity
plus one pending record bounds this composition's event retention, not process
memory, writer allocation or execution time. APIs remain provisional, and this
experimental host example does not establish qualification or operational use.

## Verify the independent package

Use an unused destination outside the checkout. Run the following PowerShell
commands from the framework checkout, stopping on any failure:

```powershell
$framework = (Get-Location).Path
$consumer = Join-Path $env:TEMP ('rff-event-output-' + [guid]::NewGuid().ToString('N'))
cargo new --bin --edition 2024 --vcs none $consumer
cargo add --manifest-path "$consumer/Cargo.toml" rust-flight-framework --path $framework --offline
Copy-Item -LiteralPath "$framework/examples/event_output.rs" -Destination "$consumer/src/main.rs"
cargo check --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets
cargo clippy --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets -- -D warnings
cargo test --manifest-path "$consumer/Cargo.toml" --locked --offline
cargo run --manifest-path "$consumer/Cargo.toml" --locked --offline
cargo metadata --manifest-path "$consumer/Cargo.toml" --locked --offline --format-version 1
Get-FileHash -Algorithm SHA256 "$framework/examples/event_output.rs", "$consumer/src/main.rs"
```

Inspect the independent workspace root, one member, two graph packages and sole
framework path dependency. The example and copied main source must have
identical bytes and SHA256 hashes. No additional maintained crate is required.
The [focused review](verification/EVENT_OUTPUT_REVIEW.md) records actual
commands, consumer results and Codex review for RFF-LIB-REQ-007; the recipe
itself does not establish a verification pass, adoption or API stability.
