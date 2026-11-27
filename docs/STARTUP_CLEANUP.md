# Handle partial startup and retain cleanup results

Run the public-only consumer:

```text
cargo run --locked --example startup_cleanup
cargo test --locked --example startup_cleanup
```

[The source](../examples/startup_cleanup.rs) registers Power, Navigation and
Payload in one direct Runtime. It demonstrates full startup with a separate
explicit shutdown, then a Payload startup failure with two retained cleanup
failures. The framework package remains experimental and unpublished.

## Choose startup and cleanup policy explicitly

The host starts its fixed three distinct applications in order. After a
successful start, that identity enters the started prefix. On the first
error, startup stops and the host cleans up only that prefix in reverse
order. It saves the exact RuntimeStartError and each RuntimeStopError
without converting either to text. A stop failure does not prevent the next
earlier successful start from receiving its own stop attempt.

Complete startup returns a private StartedMission token. A separate shutdown
call consumes that token and applies the same reverse policy. StartupOutcome
holds either the token or StartupFailure; using an enum keeps the bounded
multi-error report inline without boxing. CleanupReport has three optional
StopAttempt slots in attempt order. Only used slots are occupied.

The helper functions are private mission policy. Their setup contract is three
distinct identities issued by this runtime, initially Registered, in the
intended startup order. The token carries no runtime-origin proof, and no
intervening external lifecycle changes are expected. It is consumed to avoid
an ordinary accidental second cleanup; it does not add origin validation or
recovery to the framework. A lifecycle-rejected start is preserved just like
another start error and does not enter the successful prefix.

## Keep primary and secondary failures

Match StartupOutcome before using the mission. On Failed, inspect the primary
start_error and the cleanup report separately. A successful stop and each
failed stop remain attributable to their attempted application. The concrete
CallbackFailure and its source carry fixed-size cause information. Neither
Clone nor Copy is needed to retain these errors.

The callback trace holds at most six observations: three starts and three
stops. Application-owned start_effect records that a start callback may
change local state before failing. Tests inspect that field through public
application lookup. A failed start remains Failed, receives no cleanup stop
callback and can retain those effects. Later applications remain Registered.
A cleanup failure also becomes terminal Failed. Stop success records callback
success only; it does not independently prove external resource cleanup.

This is a serial, caller-owned recipe. Callbacks must return. It supplies no
transaction, compensation, stop-after-failed-start operation, retry, dependency
graph, event-delivery or resource-isolation guarantee. Existing lifecycle and
messaging consumers retain their contracts. [ADR-0048](adr/0048-bounded-startup-cleanup-consumer.md)
records the alternatives and conditions for considering a reusable group API.

## Verify identical source independently

From the framework checkout in PowerShell:

```powershell
$framework = (Get-Location).Path
$consumer = Join-Path $env:TEMP ("rff-startup-cleanup-" + [guid]::NewGuid().ToString("N"))
cargo new --bin --edition 2024 --vcs none $consumer
cargo add --manifest-path "$consumer/Cargo.toml" rust-flight-framework --path $framework --offline
Copy-Item -LiteralPath "$framework/examples/startup_cleanup.rs" -Destination "$consumer/src/main.rs"
cargo check --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets
cargo clippy --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets -- -D warnings
cargo test --manifest-path "$consumer/Cargo.toml" --locked --offline
cargo run --manifest-path "$consumer/Cargo.toml" --locked --offline
cargo metadata --manifest-path "$consumer/Cargo.toml" --locked --offline --format-version 1
Get-FileHash -Algorithm SHA256 -LiteralPath "$framework/examples/startup_cleanup.rs", "$consumer/src/main.rs"
```

Check exact source bytes as well as hashes, one independent edition-2024
binary workspace member, the framework as its sole path dependency, two graph
packages and no framework dependencies. Compare stdout exactly with the root
example. The [focused review](verification/STARTUP_CLEANUP_REVIEW.md) records
the executed package, commands, tests and limits. Identical-source verification
demonstrates this consumer; it does not establish outside adoption or API stability.
