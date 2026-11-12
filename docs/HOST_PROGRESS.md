# Observe sample progress on completed host turns

A sampler can complete ordinary work successfully while its input is empty.
When a mission expects fresh readings, the serial host can check for a sample
after each successful work call and report when the absence reaches a threshold.
The public-API consumer uses a one-byte sample and a two-second window.
[Consumer source](../tests/host_progress.rs)

```text
cargo test --locked --test host_progress
```

## Observe one work turn

The host starts the application and records the initial window start from its
`ManualClock`. Each turn invokes `Runtime::work_with_failure_event` for the
same application and takes the contents of the borrowed `Cell<Option<u8>>`
once. After successful work, a fresh sample resets the window and permits a
future warning. A new sample can repeat the last value. A sample observed
exactly at two seconds starts a new window immediately.

An empty turn before two seconds returns `NoSample`. On the first empty turn
at or after two seconds, the host attempts one application-attributed
`SampleOverdue` event with `Warning` severity and the observation timestamp.
The application stays `Running`. `TurnProgress::Overdue` carries the exact
event and its queue outcome. `Recorded` means the queue accepted the event.
`QueueFull { capacity: 1 }` preserves the older queued record and exposes the
rejected warning in the turn result. Both outcomes use the one warning attempt
for this window. Draining the queue alone leaves later empty turns at
`NoSample` until a fresh sample resets the window.

If the callback returns `ReadFailed`, the runtime commits `Failed` and
returns the typed error and one `WorkReturnedError` event attempt. The host
returns that error before checking sample progress, even at the threshold.
A later work call for the failed application returns a lifecycle error with
no event attempt. The warning and the failure event use the same clock and
application ID, with distinct identifiers and severities.

## Ownership and timing

The consumer reserves one application record and one event queue entry. It
borrows one input cell and one output cell, and holds one clock, one window
start and one warning flag. The host takes each sample during the same turn
and explicitly dequeues accepted events. This is serial ownership. The bounds
apply to the demonstrated slots and records. Event acceptance describes local
storage.

The host observes completed callbacks at the times it chooses to call work.
The threshold is elapsed time since startup or the last observed sample on one
clock, whose readings are nondecreasing. A late turn reports at the observed
instant. A callback that never returns prevents the host from reaching the
check. Scheduling and any action after a warning remain host decisions.
[Design decision](adr/0041-host-observed-application-progress.md)

## Verify the consumer outside the workspace

Use an unused destination outside this checkout. Copy the public-API test
source into an independent Cargo package with one framework path dependency.

```powershell
$framework = 'E:/Projects/flight software framework'
$consumer = Join-Path $env:TEMP ('rff-host-progress-' + [guid]::NewGuid().ToString('N'))
cargo new --lib --edition 2024 --vcs none $consumer
cargo add --manifest-path "$consumer/Cargo.toml" rust-flight-framework --path $framework --offline
New-Item -ItemType Directory -Path "$consumer/tests" | Out-Null
$original = Join-Path $framework 'tests/host_progress.rs'
$copy = Join-Path $consumer 'tests/host_progress.rs'
Copy-Item -LiteralPath $original -Destination $copy
cargo check --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets
cargo clippy --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets -- -D warnings
cargo test --manifest-path "$consumer/Cargo.toml" --locked --offline --test host_progress
cargo metadata --manifest-path "$consumer/Cargo.toml" --locked --offline --format-version 1
Get-FileHash -Algorithm SHA256 $original, $copy
[System.Linq.Enumerable]::SequenceEqual[byte](
    [System.IO.File]::ReadAllBytes($original),
    [System.IO.File]::ReadAllBytes($copy)
)
```

Confirm an independent workspace root, one member, two graph packages and
the sole framework path dependency. Check that the SHA256 hashes match and
the byte comparison returns `True`. The focused review records the exact
commands, source and results.
[Verification record](verification/HOST_PROGRESS_REVIEW.md)
