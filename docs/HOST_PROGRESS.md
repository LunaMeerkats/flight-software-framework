# Observe sample progress on completed host turns

A sampler can complete ordinary work successfully while its input is empty.
When a mission expects fresh readings, the serial host can check for a sample
after each successful work call and report when the absence reaches a threshold.
The public-API consumer uses a one-byte sample and a two-second window.
It covers directly selected work and periodic work with a downstream gate.
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

## Defer periodic work when the downstream slot is full

The periodic consumer adds one `PeriodicWork` cursor for the same application
and one downstream slot. A successful work turn stages a fresh sample in the
slot. The caller takes it separately. While the slot is occupied, the host
returns `Deferred` before polling the cursor. Both the sample and the pending
release stay in place. The sampler's input remains untouched.

The first deferred turn starts the pause even if the retained release is
still in the future. The first later turn that sees room ends the pause, even
if the caller drained the slot earlier. On that turn the host carries forward
the elapsed time from before the pause and keeps the warning-attempt state.
Both boundaries are observations on the same `ManualClock`. With room
available, the host polls once. A late poll releases one pending item and
re-arms from its observation time. A waiting or exhausted cursor yields
`Idle`. Only a release leads to work.

| Clock | Host turn | Sample window |
| --- | --- | --- |
| 0 s | Work produces a sample and fills the slot. | Fresh sample starts the window. |
| 1 s | The host defers with the sample still pending. | One second has elapsed. |
| 10 s | After the caller drains the slot, work runs for the retained 1 s release. | One second still counts. An empty callback returns `NoSample`. |
| 11 s | The next release runs and returns empty. | Two seconds count. The host attempts a warning timestamped at 11 s. |

A fresh sample still resets the window. A work call that returns a callback
error or lifecycle rejection passes that error back to the caller and
disables the cursor. Callback errors include the failure-event attempt.
Later turns return `Disabled`. The cursor and two optional values for the
staged sample and pause timestamp add fixed storage. The window describes
the host's observed opportunity for work. Time spent congested can be
measured separately if the caller needs that diagnostic.
[Periodic policy decision](adr/0041-host-observed-application-progress.md#periodic-work-under-downstream-backpressure)

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
