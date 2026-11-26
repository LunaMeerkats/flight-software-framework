# Report a returned stop error

Use Runtime::stop_with_failure_event or MessagingRuntime::stop_with_failure_event
when a host needs a structured diagnostic for a returned stop callback error.
Each call borrows the clock and event queue. Ordinary stop remains available
without reporting services.

```rust
match runtime.stop_with_failure_event(
    application_id,
    MissionEventId::ShutdownReturnedError,
    &clock,
    &mut events,
) {
    Ok(outcome) => observe_stopped(outcome.state(), outcome.discarded_deliveries()),
    Err(error) => {
        let discarded = error.discarded_deliveries();
        let (stop_error, attempt) = error.into_operation_error().into_parts();
        observe_failure(stop_error, discarded, attempt);
    }
}
```

This messaging fragment uses mission-defined names. Direct Runtime returns
ApplicationState on success and RuntimeStopEventError on error, without the
outer messaging wrapper. The complete compiling public-only consumer is
[tests/stop_failure_events.rs](../tests/stop_failure_events.rs).
Run `cargo test --locked --test stop_failure_events`.

## Ordering and outcomes

Each method delegates once through its owner's ordinary stop. Success retains
its original state and messaging clear count and emits nothing. A rejected
lifecycle transition invokes no callback, reads no clock and emits nothing.
It preserves any queued messages. On a returned application error, the owner
commits terminal Failed and clears the selected inbox before one clock read
and one event attempt. The event contains the error's application identity,
Error severity, the supplied identifier and the captured framework timestamp.

RuntimeStopEventError retains the exact RuntimeStopError and concrete StopError.
Its stop_error and failure_event_attempt accessors borrow those values;
into_stop_error and into_parts transfer ownership. Error::source preserves
the chain. The messaging wrapper retains the exact selected clear count.
Stop does not dequeue an in-flight command: every selected queued delivery
is included in that count. Peer queues remain intact and selectable.

FailureEventAttempt exposes the exact record and Recorded or QueueFull.
Saturation preserves older FIFO entries and the original stop error. Retain
the rejected record within an explicit host bound and offer it to EventQueue::emit
on a later turn; its original timestamp remains unchanged. A later stop on
the Failed application is lifecycle rejection and cannot duplicate reporting.
Start/restart and work/message reporting retain their separate contracts.

Callbacks and clocks must return. There is no implicit retry, recovery,
rollback, reserved capacity or guaranteed diagnostic delivery. IDs and clock
domains remain caller-scoped; equal-slot foreign IDs can alias a receiving
owner's record. Logical queue bounds do not bound arbitrary application
allocation or callback duration. This is experimental host integration
with provisional APIs and no operational or flight qualification claim.

## Verify identical source independently

From the framework checkout in PowerShell:

```powershell
$framework = (Get-Location).Path
$consumer = Join-Path ([System.IO.Path]::GetTempPath()) (
    'rff-stop-failure-events-' + [guid]::NewGuid().ToString('N'))
cargo new --lib --edition 2024 --name stop_failure_events_consumer $consumer
if ($LASTEXITCODE -ne 0) { throw 'cargo new failed' }
cargo add --manifest-path "$consumer/Cargo.toml" rust-flight-framework --path $framework
if ($LASTEXITCODE -ne 0) { throw 'cargo add failed' }
New-Item -ItemType Directory -Path "$consumer/tests" | Out-Null
Copy-Item -LiteralPath "$framework/tests/stop_failure_events.rs" `
    -Destination "$consumer/tests/stop_failure_events.rs"
Get-FileHash -Algorithm SHA256 -LiteralPath `
    "$framework/tests/stop_failure_events.rs", "$consumer/tests/stop_failure_events.rs"
cargo check --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets
if ($LASTEXITCODE -ne 0) { throw 'consumer check failed' }
cargo clippy --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets -- -D warnings
if ($LASTEXITCODE -ne 0) { throw 'consumer lint failed' }
cargo test --manifest-path "$consumer/Cargo.toml" --locked --offline --test stop_failure_events
if ($LASTEXITCODE -ne 0) { throw 'consumer tests failed' }
cargo metadata --manifest-path "$consumer/Cargo.toml" --locked --offline --format-version 1
if ($LASTEXITCODE -ne 0) { throw 'consumer metadata failed' }
```

Inspect metadata for one independent edition 2024 library member and the
sole framework path dependency. Compare exact source bytes as well as
SHA256. The [focused record](verification/STOP_FAILURE_EVENTS_REVIEW.md)
records the actual package path, commands, test results and limitations.
