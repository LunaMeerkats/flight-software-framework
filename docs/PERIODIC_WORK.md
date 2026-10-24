# Repeated work with a host-owned release cursor

Use `PeriodicWork` when a housekeeping or output producer needs repeated work
without rebuilding finite agendas. It stores one application, a positive
interval and one optional next release inline. The host chooses when to poll,
when to invoke work and when to drain output. There is no hidden worker or queue.

The [public-only consumer tests](../tests/periodic_work.rs) demonstrate a bounded
configured producer, direct and messaging-owned work, manual time and explicit
output drain. Run them with:

```text
cargo test --locked --test periodic_work
```

## Poll, then choose whether to execute

Construct a cursor with `PeriodicWork::new(ScheduledWork::new(id, first_at),
interval)`. A zero interval returns `PeriodicWorkCreateError::ZeroInterval`;
the first release instant may be zero. Neither construction nor polling checks
application registration or lifecycle. Keep the ID paired with its runtime and
the elapsed instant paired with its intended clock origin.

The host can use this fragment after creating its runtime, clock and cursor:

```rust
match releases.poll(&clock) {
    PeriodicWorkOutcome::Released { scheduled_work, .. } => {
        // The release has already been consumed, even if work returns an error.
        runtime.work(scheduled_work.application_id())?;
        // Explicitly drain or transfer mission output after successful work.
    }
    PeriodicWorkOutcome::Waiting { .. } | PeriodicWorkOutcome::Exhausted => {}
}
```

The same `work` call is available through `Runtime` and `MessagingRuntime`.
Both retain their existing identity, configuration and lifecycle contracts. The
messaging owner clears only the selected inbox after a returned work error;
polling itself does not touch inboxes or dispatch messages. Use the owning work
method so its cleanup/error behavior remains intact. There is no periodic work
error wrapper: handle the exact ordinary-work result.

`Released` means the host has received one item. It does not prove that a
callback ran or succeeded. The host may discard the item. A rejected or failed
work call cannot put that release back into the cursor. Future independent
releases remain configured; they are not retries of the failed item. The tested
host explicitly drops its selected cursor after terminal work failure and keeps
the healthy peer's cursor. Choose a policy appropriate to your mission.

## Timing and exhaustion

An active poll reads the supplied clock exactly once. Before the next instant,
`Waiting` exposes the retained item and observation without changing the cursor.
At equality or later, it returns that original item plus `observed_at`, and
re-arms before returning:

```text
next_scheduled_at = observed_at + interval
```

For a first release at 1 s and interval 2 s, polling at 10 s releases the 1 s
item once and retains 12 s. Another poll at 10 s waits. It does not generate
3/5/7/9 s backlog. This is delay from observation: lateness shifts the phase,
and callback duration is not measured. If a callback lasts past the next instant,
the next poll can release immediately. A clock reading that regresses simply
waits until the retained instant. Choose host polling order for multiple cursors;
there is no combined ordering, priority or fairness policy.

Checked addition preserves full Duration precision. If the next instant cannot
be represented, the valid due item still returns once with
`next_scheduled_at: None`. The cursor then has no next item; subsequent polls
return `Exhausted` without reading the clock. Inspect this field even when work
fails, if exhaustion matters to host diagnostics. There is no saturation at the
maximum instant and no implicit restart. Reconstruct a cursor only as an explicit
new host decision. `next_scheduled_work()` exposes retained state without a read.

## Resources and evidence limits

The cursor performs no allocation at construction or polling and retains no
release history. The consumer uses bounded application observations and one
pending output slot per producer; messaging scenarios use two capacity-two
inboxes and one-byte payloads. The host drains output explicitly. A full pending
slot is a typed application error with terminal lifecycle failure in this
consumer. Cursor storage does not bound callback allocation, process memory or
execution time. Cell-based observations are a serial mission policy, not
synchronization or physical output.

The existing finite `WorkSchedule` remains useful for ordered one-shot agendas
and keeps its previous semantics. This cursor adds no runtime-owned schedule,
fixed phase, automatic retry, failure reporting, transaction or panic/hang
containment. APIs remain provisional and the library remains experimental host
software, with no real-time, flight-qualification or operational-suitability claim.

## Verify the consumer outside the workspace

Use an unused destination outside this checkout. This copies only the public-API
tests; it does not load sample internals or create another maintained crate.

```powershell
$framework = 'E:/Projects/flight software framework'
$consumer = Join-Path $env:TEMP ('rff-periodic-work-' + [guid]::NewGuid().ToString('N'))
cargo new --lib --edition 2024 --vcs none $consumer
cargo add --manifest-path "$consumer/Cargo.toml" rust-flight-framework --path $framework --offline
New-Item -ItemType Directory -Path "$consumer/tests" | Out-Null
Copy-Item -LiteralPath "$framework/tests/periodic_work.rs" -Destination "$consumer/tests/periodic_work.rs"
cargo check --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets
cargo clippy --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets -- -D warnings
cargo test --manifest-path "$consumer/Cargo.toml" --locked --offline --test periodic_work
cargo metadata --manifest-path "$consumer/Cargo.toml" --locked --offline --format-version 1
Get-FileHash -Algorithm SHA256 "$framework/tests/periodic_work.rs", "$consumer/tests/periodic_work.rs"
```

Verify an independent workspace root with one member, two graph packages, the
sole framework path dependency and identical test-source hashes. This verifies
this controlled consumer, not external adoption or general API stability. Actual
results are in [the focused record](verification/PERIODIC_WORK_REVIEW.md);
[ADR-0030](adr/0030-host-owned-periodic-releases.md) records alternatives and
revisit conditions.
