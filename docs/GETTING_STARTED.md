# Getting started with the public API

Use this recipe to run one host command handler, then build the same source in
your own Cargo binary. It needs stable Rust with Cargo and a local checkout.
The recorded local verification uses Rust/Cargo 1.98.0; this is evidence, not an
MSRV promise or toolchain pin. The package is unpublished at 0.0.0, and its APIs
are provisional.

This is experimental host software, not flight-qualified, safety-certified,
NASA-affiliated, TRL-proven or suitable for operational spacecraft, safety-critical
or human-rated systems. It claims no cFS, cFE, OSAL, PSP, CCSDS or RTOS compatibility,
real-time behavior or fault tolerance.

## Run the example

From the repository root:

```text
cargo run --locked --example quickstart
cargo test --locked --example quickstart
```

Expected stdout:

```text
Experimental host quickstart; not flight-qualified or safety-certified.
No NASA affiliation, demonstrated TRL, operational suitability, or compatibility.
publish 42: Complete / Delivered
publish 43: WhollyUndelivered / InboxFull
publish 7: Complete / Delivered
explicit dispatch output: [42, 7]
lifecycle: [Registered, Running, Stopped, Running, Stopped]
```

The [complete source](../examples/quickstart.rs) defines its own application,
using only public `rust_flight_framework` APIs and `std`. The full combined
[host mission](verification/HOST_SAMPLE.md) remains a later reference for
configuration, events and scheduled work.

## Use the library in your own package

These PowerShell commands start in the framework checkout and create a separate
binary beside it. Choose an unused consumer directory. Keep your framework
checkout at the revision you want to evaluate; a path dependency follows local
source changes.

```powershell
$framework = (Get-Location).Path
$consumer = Join-Path (Split-Path $framework -Parent) 'rff-consumer'
cargo new --bin --edition 2024 --vcs none $consumer
cargo add --manifest-path "$consumer/Cargo.toml" rust-flight-framework --path $framework --offline
Copy-Item -LiteralPath "$framework/examples/quickstart.rs" -Destination "$consumer/src/main.rs"
cargo run --locked --offline --manifest-path "$consumer/Cargo.toml"
cargo test --locked --offline --manifest-path "$consumer/Cargo.toml"
```

Stop if any command fails; use the reported Cargo/application error before
continuing. `cargo add` records a local `path` dependency and generates the new
consumer's lockfile. Later `--locked` commands require that resolution to remain
unchanged. No registry package or published release is needed. The consumer
owns its own manifest, lockfile and application source; copying this standalone
recipe does not load private library or combined-sample internals.

## Follow the composition

1. Define an application implementing `Application` and `MessagingApplication`.
   The small handler has no ordinary periodic work; its lifecycle callbacks
   return `Infallible`. Message handling validates the command before changing
   the host output slot.
2. Create a capacity-one `Runtime`, register the application, and retain its
   returned identifier. Attach exactly one `RuntimeInboxConfig` while the
   application is still `Registered`, then explicitly start it. Inbox order
   must match registration order. Identifiers belong to that owner; do not mix
   them across runtimes.
3. Construct a bounded `Message` and publish it. Publication queues input; it
   does not call application code. Inspect the `PublishReport`: an `Ok` result
   can still report full or unavailable subscribers, or no subscribers.
4. Call `dispatch_one` explicitly. It consumes at most one oldest queued input
   for the selected running application. The callback writes one validated
   percentage to the borrowed host slot. The driver drains that slot explicitly.
5. Stop and restart through the messaging owner. Restart retains the application
   value and reconnects an empty inbox; it does not reconstruct application state.
   The caller selects every publication, dispatch and lifecycle operation.

The scenario queues 42, rejects a newer command when its inbox is full, and
observes the original 42 after dispatch. It then stops/restarts and handles 7.
Tests also exercise malformed command handling and occupied host output. These
are controlled in-process observations; they do not establish physical delivery.

## Choose errors and resource policy deliberately

The recipe fixes one application, one inbox of one message, a one-byte payload
limit, and one `Cell<Option<u8>>` output slot. Input saturation rejects the new
message and preserves the older one. The host must drain output before another
successful command can fill it. The inline payload and queue counts are logical
bounds, not a whole-process memory or callback-duration bound. Setup and
publication reports may allocate; application code remains responsible for its
own resource use.

Message construction rejects oversized payloads before publication. Callback
validation rejects unsupported command bytes before changing output. A returned
callback error makes that application terminal `Failed`, removes its in-flight
input and clears remaining selected inbox deliveries. Draining the output does
not recover a failed application; restart is supported from `Stopped`, not
`Failed`. Panics, hangs and allocator exhaustion are outside this boundary.

The one-shot driver reports an error and tears down on failure. It deliberately
reduces ownership-preserving registration/attachment errors to their typed cause
when propagating diagnostics; it does not attempt recovery. A longer-lived host
can inspect `RuntimeRegistrationError::into_application` or
`MessagingRuntimeCreateError::into_runtime` to recover those owned values.
Boxing a wrapper containing an application that borrows host state can impose an
unwanted `'static` lifetime; preserve or consume the wrapper locally instead.

Use the [development commands](../README.md#development),
[requirements](REQUIREMENTS.md#stage-5-integration-requirements--revision-1-2026-10-20)
and [quickstart verification](verification/QUICKSTART_REVIEW.md) for exact
contracts and evidence. This demonstrates one independently packaged composition,
not external adoption, general consumer readiness, API stability or release
readiness. Publication and deployment retain their separate authorization gates.
