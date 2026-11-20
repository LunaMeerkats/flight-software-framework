# Keep periodic peer work running after an application error

```text
cargo run --locked --example periodic_peer
```

The [example](../examples/periodic_peer.rs) starts two samplers and a heartbeat
application in one direct runtime. The host has one manual clock, one
periodic cursor per application and a shared event queue with capacity one.
The sampler returns `SampleUnavailable` on its first work turn, at one
second. The host prints the typed failure's diagnostic outcome and
continues heartbeat work at two and three seconds while the event is queued.
At three seconds the sampler's selection is disabled. The host drains the
failure event with its original one-second timestamp, then runs the
heartbeat again at four seconds. The backup sampler stays idle in this first
trace, preserving the original demonstration.

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
or console write can stall the serial driver. The three cursors and one
queued event are logical storage bounds. The sampler remains terminally
`Failed`. See the [queue retry example](HOST_PROGRESS.md#retry-a-work-failure-rejected-by-the-queue)
for a host that retains rejected failure events.

## Retry two independent failures

The second trace starts a fresh host and clock with a warning already in
the capacity-one FIFO. The first sampler fails at one second and the backup
fails at two seconds. Both exact events are retained in two inline slots,
in the order the host observed the failures. Both original typed work
errors remain available at their call sites. The heartbeat can still run
while the FIFO and both slots are occupied.

Call `retry_failure_event` on a separate turn. It offers only the oldest
retained event once. `QueueFull` preserves both slots; `Recorded` clears
only that oldest event and shifts the other into its place. An empty turn
returns `None`. After draining the older warning, the trace admits and
dequeues each failure over separate turns, preserving its source and
original timestamp.

This host has exactly two applications that can return work errors and
permanently disables each cursor after its error. That bounds retention at
two events. Accepted events and lifecycle rejections add no retry record.
Adding another failing role, re-enabling a failed cursor, or retaining other
diagnostics requires revisiting the bound. These two slots are additional
to the FIFO's one event; this is a logical record bound, with no byte or
execution-time guarantee.

Retained failures retry in observation order. The shared FIFO orders
successful acceptance. A fresh event can be accepted before an older
rejected event if the caller frees space and selects work before retry.
The host chooses retry timing and does not reserve capacity. This example
demonstrates in-process queue acceptance and dequeue; it adds no durable
storage or reliable external delivery.

## Admit retained failures before fresh sampler work

The third trace opts into `turn_after_retained_failures`. While a failure
remains in the retry slots, an enabled sampler returns
`DeferredForDiagnostics` before polling. Its next release and Running
state remain unchanged. The heartbeat still uses its ordinary release
policy, and an already disabled sampler still returns `Disabled`.

Retry and dequeue remain separate turns. Even an empty FIFO does not
remove the gate until the host explicitly admits the retained event.
After admission, the overdue backup may run while that older event is
still queued. Its new failure is then rejected and retained with the
later callback timestamp. Drain the older event before retrying the newer
one. This prioritizes retained admission rather than waiting for output.

Use this guarded turn consistently for the mission's samplers when that
ordering matters. The original `turn` remains available for callers that
choose their own ordering. This gate uses mission knowledge: heartbeat
work cannot fail here, and failed cursors stay disabled. Revisit the policy
before changing those facts. A host must still select retries and drains;
sampler work can remain deferred indefinitely if it does not. Healthy
work remains selectable without a fairness or completion guarantee.

The original 14-line trace remains an exact prefix. The added trace shows
the rejected first failure, deferred backup, healthy peer work, explicit
older-event admission, and the newer retained failure.

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
