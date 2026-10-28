# Produce output during ordinary work

This controlled consumer produces a configured value during ordinary work,
then lets the host explicitly publish it to one bounded subscriber. It uses
public APIs and std only. Start with [dependency setup](GETTING_STARTED.md) and
[application composition](COMPOSING_APPLICATIONS.md) for the earlier recipes.
The library remains experimental, unpublished and provisional. It is not
flight-qualified, safety-certified, NASA-affiliated, TRL-proven or operationally
suitable; it claims no compatibility with cFS, CCSDS or an RTOS.

## Run the consumer

```text
cargo test --locked --example work_output
cargo run --locked --example work_output
```

The [complete source](../examples/work_output.rs) reads validated runtime
configuration in ordinary work, stages one value, and transfers it through the
host. A finite one-shot schedule supplies an ordinary-work invocation using
manual time. Nothing publishes until the host explicitly requests transfer.
The host selects dispatch and output draining as separate operations.

Expected stdout:

```text
Experimental host work output; not flight-qualified or safety-certified.
No NASA affiliation, demonstrated TRL, operational suitability, or compatibility.
ordinary/scheduled work -> host -> sink: [42, 7]
recurring work with deferred host drain -> sink: [42, 7, 9, 11]
```

## Ownership and transfer

Ordinary work receives immutable configuration. It borrows a caller-owned
capacity-one pending slot, checks occupancy, and deposits its observation. The
slot outlives the runtime. An accepted configuration replacement affects later
work; it does not change an already staged value. The library neither owns nor
automatically clears this mission output.

The host reads a copy of the pending value and calls MessagingRuntime::publish.
It checks that the report contains exactly the expected single destination
and that its status is Delivered before clearing the pending slot. Full or
unavailable destinations retain the value for explicit retry. Publication errors
also leave it pending. The host controls when another attempt is useful.

A mismatched destination report is a host topology defect, returned as a
separate error. It is detected after publication and may already contain a
delivery. Do not retry that error blindly; the example stops for diagnosis.

The constructor freezes exactly one subscriber. No delivery occurs on that
subscriber's full/unavailable outcomes, so a retained retry cannot duplicate an
earlier accepted delivery. Do not extend this recipe to general fan-out: a
partial report can contain both acceptance and rejection, and replaying the
whole message can duplicate the accepted deliveries. A delivery report records
inbox acceptance, not application processing or physical I/O.

## Work, lifecycle and failures

The host should transfer pending output before requesting further producer
work. Calling work with an occupied slot returns a concrete application error,
retains the older value and commits terminal Failed under the existing runtime
contract. It is a deliberately strict mission policy. Ordinary work failure
clears only the selected application's inbox; a healthy peer can still work
and dispatch. The error source chain retains the concrete work failure.

Host publication rejection is a separate outcome and does not itself fail the
producer. Stop/restart retains the caller-owned pending slot. Already staged
output may be transferred after producer stop or failure under this recipe's
explicit policy. New work is still gated by the producer's lifecycle; rejected
work does not enter its callback. Restart is available from Stopped, not Failed.
There is no automatic retry, recovery or rollback of earlier effects.

Finite scheduled work uses the same ordinary-work path. An attempt before its
scheduled instant does nothing. A due item is consumed before work invocation,
including a lifecycle rejection or returned application error. Retrying host output does
not restore a consumed scheduled item or create a periodic schedule.

## Coordinate recurring work

The same source also composes [PeriodicWork](PERIODIC_WORK.md) with a slow host
consumer. Calling work whenever a release is due can hit PendingFull; dispatching
while the host output slot is occupied can hit OutputFull. Those strict direct
call contracts remain useful and unchanged. This host policy checks capacity
before invoking either callback.

Each `host_turn` performs these stages in order:

| Stage | Capacity/lifecycle guard | Maximum operation count |
| --- | --- | --- |
| Sink dispatch | Output slot empty and sink Running | One oldest message |
| Pending transfer | Existing sole-subscriber report validation | One publication |
| Recurring production | Pending slot empty after transfer | One poll and one work callback |

The report separates skipped/attempted dispatch, transfer and production.
In this host report, Polled(Released) is returned only after work succeeds;
the underlying PeriodicWork release alone still does not establish execution.
Newly produced output waits for a later turn. The caller explicitly takes an
output value between turns when ready to observe it; the turn never drains it.
An occupied output slot can therefore fill the inbox and then the pending slot.
Further turns preserve all three older values without invoking producer work
or reading the clock while its pending slot remains occupied. There is no loop
that drains until empty and no automatic retry worker.

When capacity returns, a turn can dispatch an older message, transfer the older
pending value and produce one new value. Deferred polling leaves the original
next release intact. A resumed overdue poll releases one item and re-arms from
the newly observed time, coalescing missed intervals. It does not execute a
burst or restore phase. Configuration replacement while blocked preserves the
pending value's original revision and affects only the resumed work observation.

The selected timer is an Option. A work error disables it before returning the
existing concrete error chain, including lifecycle rejection; successful earlier
dispatch/publication remains committed. Disabled or exhausted production does
not prevent later dispatch/transfer turns. A stopped sink is explicitly skipped
for dispatch; publication reports it unavailable and retains pending output.
The host must choose whether to restart it, drain accepted output or stop.
Dispatch or transfer errors stop the turn before polling; they do not silently
disable the timer. Unexpected topology may already have delivered: diagnose it
before considering another transfer. No stage rollback or automatic recovery
is implied.

[ADR-0033](adr/0033-backpressure-aware-host-coordination.md) compares paused
polling with explicitly dropping blocked releases and adding a library runner.
This selected serial pipeline needs no new framework API. Use the exact matching
runtime, IDs and slots; the function cannot authenticate their origin. It is one
mission policy, not fairness, a callback-time bound or a general fan-out retry
solution. The finite runnable scenario deliberately fills the pipeline, replaces
configuration, drains in FIFO order and resumes one overdue observation.

## Bounds and the next API question

This mission fixes two runtime records, one inbox per application, one pending
output value and one host-observation slot. Queue capacities and inline payload
bounds are visible in construction: each inbox holds one message, payloads and
configuration each use one byte. The pending record also retains its acceptance
revision; only its value byte is transmitted. Two host-owned ordinary-work
observation cells retain the latest successful producer/peer work record for
verification. These logical limits do not bound process memory, application
allocation or callback duration. Cell provides shared
interior mutation for this serial host; it supplies no synchronization contract.
Panics, hangs, allocator exhaustion and process failure remain uncontained.

[ADR-0028](adr/0028-host-mediated-work-output.md) compares this bridge with a
narrow work publication context and borrowed event emitter. Existing public
APIs express this selected use, at the cost of an explicit host transfer step.
A consumer needing publication feedback before its work callback returns or
inseparable effect ordering would justify reassessing a service capability.
This evidence does not freeze the current API or establish broad adoption.

## Verify the independent package

Use a new directory outside the checkout. Run these PowerShell commands from
the framework checkout, stopping on any failure:

```powershell
$framework = (Get-Location).Path
$consumer = Join-Path $env:TEMP ('rff-work-output-' + [guid]::NewGuid().ToString('N'))
cargo new --bin --edition 2024 --vcs none $consumer
cargo add --manifest-path "$consumer/Cargo.toml" rust-flight-framework --path $framework --offline
Copy-Item -LiteralPath "$framework/examples/work_output.rs" -Destination "$consumer/src/main.rs"
cargo check --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets
cargo clippy --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets -- -D warnings
cargo test --manifest-path "$consumer/Cargo.toml" --locked --offline
cargo run --manifest-path "$consumer/Cargo.toml" --locked --offline
cargo metadata --manifest-path "$consumer/Cargo.toml" --locked --offline --format-version 1
Get-FileHash -Algorithm SHA256 "$framework/examples/work_output.rs", "$consumer/src/main.rs"
```

Metadata should show one independent workspace member, two graph packages and
only the library path dependency. Source hashes should match. No additional
maintained crate is required. The [focused review](verification/WORK_OUTPUT_REVIEW.md)
records the actual package, commands, observations and autonomous Codex review
for RFF-LIB-REQ-003. Controlled host evidence does not establish API stability,
release readiness, qualification or operational use.
The same recipe now checks the recurring composition under RFF-LIB-REQ-008;
[its focused review](verification/HOST_COORDINATION_REVIEW.md) retains current
source hashes, package path, commands and actual review/publication evidence.
