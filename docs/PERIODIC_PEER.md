# Keep periodic peer work running after an application error

```text
cargo run --locked --example periodic_peer
```

The [example](../examples/periodic_peer.rs) starts a sampler and a heartbeat
application in one direct runtime. The host has one manual clock, one
periodic cursor per application and a shared event queue with capacity one.
The sampler returns `SampleUnavailable` on its first work turn, at one
second. The host prints the typed failure's diagnostic outcome and
continues heartbeat work at two and three seconds while the event is queued.
At three seconds the sampler's selection is disabled. The host drains the
failure event with its original one-second timestamp, then runs the
heartbeat again at four seconds.

## Select work and diagnostics independently

1. Create one `PeriodicWork` for each application ID from the same runtime.
   Use the same clock origin for all releases and event timestamps.
2. Poll the selected cursor and invoke `Runtime::work_with_failure_event`
   only when it returns `Released`. Use that item's application ID. A
   waiting or exhausted cursor invokes no callback.
3. On a work error, discard only the selected cursor. The example also
   discards it if the runtime rejects the call before the callback. An
   application error commits `Failed` and carries the original cause and
   a diagnostic attempt. A lifecycle rejection carries no event attempt.
4. Inspect the typed error, then choose the peer's next turn. Dequeue
   diagnostics separately. If the queue was already full, the error holds
   the rejected event and the older record stays queued.

The example handles the expected sampler error before selecting further
turns. Setup and console I/O errors end the script. A host chooses polling
order and must continue taking turns to see progress. A stalled callback
or console write can stall the serial driver. The two cursors and one
queued event are logical storage bounds. The sampler remains terminally
`Failed`. See the [queue retry example](HOST_PROGRESS.md#retry-a-work-failure-rejected-by-the-queue)
for a host that retains rejected failure events.

## Verify the independent binary

Run these PowerShell commands from the framework checkout. Use a fresh
directory outside it.

```powershell
$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true
$framework = (Get-Location).Path
$consumer = Join-Path ([System.IO.Path]::GetTempPath()) ("rff-periodic-peer-" + [guid]::NewGuid().ToString('N'))
cargo new --bin --edition 2024 --vcs none $consumer
cargo add --manifest-path "$consumer/Cargo.toml" rust-flight-framework --path $framework --offline
Copy-Item -LiteralPath "$framework/examples/periodic_peer.rs" -Destination "$consumer/src/main.rs"
cargo check --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets
cargo clippy --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets -- -D warnings
cargo test --manifest-path "$consumer/Cargo.toml" --locked --offline
cargo run --manifest-path "$consumer/Cargo.toml" --locked --offline
cargo metadata --manifest-path "$consumer/Cargo.toml" --locked --offline --format-version 1
Get-FileHash -Algorithm SHA256 "$framework/examples/periodic_peer.rs", "$consumer/src/main.rs"
```

Confirm one independent workspace member and the framework as the sole
path dependency. Compare source hashes and both binaries' output.
[Focused review](verification/PERIODIC_PEER_REVIEW.md).
