# Poll work against elapsed host time

Use HostClock when a host program needs elapsed time without manual advances.
It implements the same Clock interface as ManualClock. Run the finite
[public-only example](../examples/host_clock.rs):

```text
cargo test --locked --example host_clock
cargo run --locked --example host_clock
```

The program starts one application, polls five times with a 20 ms recurring
interval, requests a 10 ms sleep between polls, and stops the application.
Sleep may last longer because the host schedules execution. It prints:

```text
host polling: turns=5 work_calls=<count> state=Stopped
```

The count is between one and five. Host load and timing affect it; no particular
count or completion deadline is required. The first release is at zero, so the
first poll can run immediately. Each later poll releases at most one callback.
See [periodic work](PERIODIC_WORK.md) for observation-based rearming and lateness.

## Keep one origin with its deadlines

HostClock::new and Default each capture a private Instant origin. A read takes
one host sample and returns its saturating duration from that origin. The clock
stores only the origin. It performs no waiting, work dispatch or background work.
The first reading need not be zero: time can pass after construction.

Construct relative deadlines from this clock's readings or use zero for an
immediate first release. Keep that same instance for later polling and related
event timestamps. FrameworkInstant carries no origin identity, so mixing clocks
is not detected. Replacing a clock does not rebase an existing schedule.

The example's poll_turn reads the periodic cursor, then explicitly calls ordinary
work for a released item. A returned error ends its main loop through `?`.
The release remains consumed even when lifecycle rejects work. Sleeping is a
separate host action between turns. The library creates no thread or executor.

## Choose controlled time for tests

The example tests the same poll_turn with ManualClock: before-deadline waiting,
equality, no duplicate work, overdue coalescing and consumed release on rejection.
The live smoke test uses both HostClock constructors with no sleep or minimum
elapsed threshold. Private conversion tests use synthetic native observations.
This separates repeatable policy checks from host-clock smoke coverage.

The adapter inherits [Rust Instant's platform limitations](https://doc.rust-lang.org/std/time/struct.Instant.html).
A pre-origin sample maps to zero. Later samples are not clamped to prior readings;
platform anomalies can regress them. Do not infer UTC, accuracy, steady tick rate,
suspend accounting or real-time behavior. Waiting, callbacks and output must
return; five turns bound invocations rather than wall-clock runtime.

## Verify identical source independently

Use an unused directory outside this workspace:

```powershell
$framework = 'E:/Projects/flight software framework'
$consumer = Join-Path $env:TEMP ('rff-host-clock-' + [guid]::NewGuid().ToString('N'))
cargo new --bin --edition 2024 --vcs none $consumer
cargo add --manifest-path "$consumer/Cargo.toml" rust-flight-framework --path $framework --offline
Copy-Item -LiteralPath "$framework/examples/host_clock.rs" -Destination "$consumer/src/main.rs"
cargo check --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets
cargo clippy --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets -- -D warnings
cargo test --manifest-path "$consumer/Cargo.toml" --locked --offline
cargo run --manifest-path "$consumer/Cargo.toml" --locked --offline
cargo metadata --manifest-path "$consumer/Cargo.toml" --locked --offline --format-version 1
Get-FileHash -Algorithm SHA256 "$framework/examples/host_clock.rs", "$consumer/src/main.rs"
```

Inspect one independent edition 2024 binary member, its sole framework path
dependency, two graph packages and identical source bytes/hashes. Check output
shape and count bounds; live counts need not match between runs. This proves
the controlled consumer, not general adoption or host timing guarantees.
The [focused review](verification/HOST_CLOCK_REVIEW.md) records actual results.
