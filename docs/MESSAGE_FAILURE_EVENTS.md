# Report a returned message-callback error

Use MessagingRuntime::dispatch_one_with_failure_event when a host needs
a structured diagnostic for a returned handle_message error. The method
has the same dispatch behavior as dispatch_one and borrows a clock and
event queue for that call only. Ordinary work keeps its separate
work_with_failure_event method and concrete WorkError type.

```rust
match runtime.dispatch_one_with_failure_event(
    application_id,
    MissionEventId::CommandReturnedError,
    &clock,
    &mut events,
) {
    Ok(outcome) => observe_dispatch(outcome),
    Err(error) => {
        let discarded = error.discarded_deliveries();
        let (dispatch_error, attempt) = error.into_operation_error().into_parts();
        observe_failure(dispatch_error, discarded, attempt);
    }
}
```

This fragment uses mission-defined names. The complete compiling consumer
is [tests/message_dispatch_events.rs](../tests/message_dispatch_events.rs).
Run it with `cargo test --locked --test message_dispatch_events`.

## Ordering and outcomes

Dispatch first validates the selected identity and Running state, removes
at most one oldest message and invokes its callback. On a returned error,
the owner commits Failed and clears the selected remaining inbox before
reading the supplied clock once and attempting one event. The event uses
the application identity from the error, Error severity, the supplied
identifier and captured framework timestamp.

An empty inbox, successful callback or lifecycle rejection reads no clock
and emits nothing. Lifecycle rejection happens before dequeue. Failure
preserves the original MessageDispatchError and concrete MessageError;
Error::source traverses each wrapper. The outer MessagingOperationError
retains the exact clear count. It excludes the attempted in-flight item
and includes accepted self-publications still queued. Peer publications
accepted during the callback survive the selected failure.

The event attempt exposes Recorded or QueueFull and its exact record in
either case. QueueFull preserves older FIFO entries. To retry, retain that
record and call EventQueue::emit on a later explicit host turn. This keeps
the original timestamp; calling dispatch again on the Failed application
returns lifecycle rejection and creates no new failure diagnostic. Bound
any host retention separately and clear it only after successful admission.

No implicit retry, rollback, recovery or guaranteed diagnostic delivery is
provided. Callbacks and clocks must return. Application IDs and clock
domains remain caller-scoped; equal-slot foreign application IDs can alias.
Queue limits bound retained records, not arbitrary application allocation
or execution time. This is experimental host evidence, without operational
or flight qualification, API-stability or external-adoption claims.

## Verify identical source independently

From the framework checkout in PowerShell:

```powershell
$framework = (Get-Location).Path
$consumer = Join-Path ([System.IO.Path]::GetTempPath()) (
    'rff-message-failure-events-' + [guid]::NewGuid().ToString('N'))
cargo new --lib --edition 2024 --name message_failure_events_consumer $consumer
if ($LASTEXITCODE -ne 0) { throw 'cargo new failed' }
cargo add --manifest-path "$consumer/Cargo.toml" rust-flight-framework --path $framework
if ($LASTEXITCODE -ne 0) { throw 'cargo add failed' }
New-Item -ItemType Directory -Path "$consumer/tests" | Out-Null
Copy-Item -LiteralPath "$framework/tests/message_dispatch_events.rs" `
    -Destination "$consumer/tests/message_dispatch_events.rs"
Get-FileHash -Algorithm SHA256 -LiteralPath `
    "$framework/tests/message_dispatch_events.rs", "$consumer/tests/message_dispatch_events.rs"
cargo check --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets
if ($LASTEXITCODE -ne 0) { throw 'consumer check failed' }
cargo clippy --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets -- -D warnings
if ($LASTEXITCODE -ne 0) { throw 'consumer lint failed' }
cargo test --manifest-path "$consumer/Cargo.toml" --locked --offline --test message_dispatch_events
if ($LASTEXITCODE -ne 0) { throw 'consumer tests failed' }
cargo metadata --manifest-path "$consumer/Cargo.toml" --locked --offline --format-version 1
if ($LASTEXITCODE -ne 0) { throw 'consumer metadata failed' }
```

Inspect metadata for one independent edition 2024 library member and the
sole framework path dependency. Compare exact source bytes as well as
SHA256. The [focused record](verification/MESSAGE_FAILURE_EVENTS_REVIEW.md)
records the actual package path, commands, test results and limitations.
