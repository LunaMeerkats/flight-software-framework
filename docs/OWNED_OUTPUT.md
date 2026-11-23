# Take ownership of application output

Run the [public-only example](../examples/owned_output.rs):

```text
cargo run --locked --example owned_output
cargo test --locked --example owned_output
```

Use this recipe when the application owns a fixed-size output record and
the host needs to move it to a consumer. The example's Output type implements
neither Copy nor Clone. Its application owns one private
`Cell<Option<Output>>`, and its shared `take_output` method leaves None
when it returns the owned record. A repeated take returns None.

```rust
let output = runtime.application(id)?.take_output();
```

The framework lookup supplies a shared borrow. The application-defined
method performs this intentional interior mutation. This uses the existing
lookup contract; it adds no mutable owner access or framework output service.
The standard library's [Cell::take](https://doc.rust-lang.org/std/cell/struct.Cell.html#method.take)
requires Default for the cell contents, which Option supplies even when
Output has no Default implementation. No RefCell borrow guard is needed.

## Follow the ownership

Production, take and sink offer are separate host turns. Work stages output
only if the application slot is empty. Work attempted against a full slot
returns a typed error and enters the existing terminal failure path, keeping
the older record. Normal host coordination avoids that failure by taking
output and deferring production while the host slot is occupied.

The host owns at most one taken record. It checks that slot before another
take, preserving both an already pending host record and any application
record. A sink offer moves the whole record. Definite rejection returns
ownership, which the host restores to its slot before returning. A later
explicit sink turn reoffers it. Acceptance clears that local obligation;
an empty offer makes no sink call. This typed sink has no partial-acceptance
state or ambiguous external side effect.

Cell is a single-threaded choice. Callbacks and sink turns must return.
There is one fixed-size application slot per registered producer and one
host slot; the tests register two producers for peer-isolation evidence.
Runtime and inbox capacities remain separately configured. These are
logical storage bounds, not bounds on arbitrary application allocation or
execution time.

The runnable scenario prints diagnostics with ordinary fallible writes and
exits on a printing error. It does not retain records across process exit.
The typed whole-record sink and diagnostic printing are separate boundaries.

## Lifecycle and evidence limits

This application's stop/restart callbacks retain staged output. Taking from
a Stopped or Failed record remains possible through shared lookup and does
not change lifecycle state. Messaging lookup and taking neither dispatch
nor clear queued messages. A returned work error still clears the selected
inbox under the existing owner contract and preserves the healthy peer.
Draining after failure is observation/output handling, not recovery.

Keep IDs with their issuing runtime; equal-slot foreign IDs can alias local
records. Application-defined shared methods remain cooperative effects.
Local sink acceptance establishes ownership handoff only. It does not prove
processing, physical or durable delivery, duplicate suppression or reliable
transport. For partial byte writes use the separate [event-output recipe](EVENT_OUTPUT.md).
The existing [borrowed ordinary-work recipe](ORDINARY_WORK_OUTPUT.md) remains
useful when the host should own storage from the start. This library remains
experimental host software with provisional APIs.

## Verify identical source independently

Use a fresh binary outside the framework workspace:

```powershell
$framework = 'E:/Projects/flight software framework'
$consumer = Join-Path $env:TEMP ('rff-owned-output-' + [guid]::NewGuid().ToString('N'))
cargo new --bin --edition 2024 --vcs none $consumer
cargo add --manifest-path "$consumer/Cargo.toml" rust-flight-framework --path $framework --offline
Copy-Item -LiteralPath "$framework/examples/owned_output.rs" -Destination "$consumer/src/main.rs"
cargo check --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets
cargo clippy --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets -- -D warnings
cargo test --manifest-path "$consumer/Cargo.toml" --locked --offline
cargo run --manifest-path "$consumer/Cargo.toml" --locked --offline
cargo metadata --manifest-path "$consumer/Cargo.toml" --locked --offline --format-version 1
Get-FileHash -Algorithm SHA256 "$framework/examples/owned_output.rs", "$consumer/src/main.rs"
```

Compare exact bytes, SHA256 and stdout, and inspect metadata for one
independent binary member with the framework as its sole path dependency.
The [focused review](verification/OWNED_OUTPUT_REVIEW.md) records actual
results and limits. Identical controlled downstream use is evidence for
this recipe, not external adoption, general integration readiness or stability.
