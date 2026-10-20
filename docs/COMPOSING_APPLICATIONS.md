# Compose independently implemented applications

This second controlled consumer shows how a host can reuse two distinct
application implementations in one runtime. Start with the
[single-application quickstart](GETTING_STARTED.md) for dependency setup and
basic lifecycle/report semantics. The library remains experimental, unpublished
and provisional. It is not flight-qualified, safety-certified, NASA-affiliated
or operationally suitable. It claims no compatibility with cFS, CCSDS or an RTOS;
no TRL is proven.

## Run the composition

From the framework checkout:

```text
cargo test --locked --example composition
cargo run --locked --example composition
```

The [complete source](../examples/composition.rs) imports public library APIs and
`std` only. It does not load private sample modules. The host starts a relay and
an output application, publishes a percentage command, explicitly dispatches the
relay to publish to the output inbox, then dispatches the output and drains its
single host slot. The host also selects ordinary work, stop and restart.

Expected stdout:

```text
Experimental host composition; not flight-qualified or safety-certified.
No NASA affiliation, demonstrated TRL, operational suitability, or compatibility.
relay -> sink -> host output: [42, 7]
```

## Keep reusable applications separate from mission policy

`Runtime<A>` stores one concrete `A`. Each application struct implements
`Application` and `MessagingApplication` independently. `MissionApplication`
holds either struct and delegates `start`, `work`, `stop`, `restart` and
`handle_message` by matching the active variant. The contained values remain
the same through stop and in-place restart. Borrowed host observations belong
to the caller and outlive the runtime.

The wrapper is mission code: it chooses this application's set, shared topic
enum and one-byte payload bound. Its applications communicate through the
message context, without calling each other. Only message callbacks receive
publication access; ordinary work currently receives configuration only.

Distinct associated error types need a common mission vocabulary. The
`MissionError` variants wrap each concrete message error, and `Error::source`
returns the underlying cause. Callers can match the typed variant or inspect
the framework wrapper's source chain. No string conversion is needed to retain
the failure's application identity or cause. Lifecycle/work methods in this
recipe are infallible; combining distinct errors for those operations requires
equivalent operation-specific mapping and is outside this demonstrated pair.

The explicit wrapper keeps all five delegation bodies visible. A public sum
adapter might reduce repeated code after more consumers establish common
semantics. [ADR-0027](adr/0027-independent-application-composition.md) compares
that option and trait-object erasure. One working pair does not establish a
general-purpose composition API or application interoperability.

## Resource and failure policy

The mission fixes two runtime records and one inbox per registered application.
The relay inbox holds two commands; the sink inbox holds one output message.
Both reject the newest delivery when full, retaining previously queued values.
Inbox positions follow registration order and attachment precedes start. Each
message has at most one payload byte. The output slot holds at most one byte;
the caller must drain it before another successful output callback. Logical
queue counts do not bound the entire process, callback allocation or duration.
The mission enum occupies inline storage for its largest variant plus overhead;
applications, errors and publication reports may own additional allocations.

The relay inspects the callback publication report. An incomplete publication
is a typed relay failure under this recipe's deliberately strict policy. A
returned callback error enters terminal `Failed` and clears the selected queued
inputs; a healthy peer retains accepted messages and can still dispatch. A full
output slot produces a different concrete output error while retaining the old
slot value. Neither policy retries automatically or reverses earlier effects.
Restart is available from `Stopped`, not `Failed`. Panics, hangs and allocator
exhaustion remain outside cooperative containment.

Setup failures end the one-shot demo. Registration/attachment wrappers are
consumed locally for diagnostics; integrating hosts can recover their owned
values as explained in the quickstart guide. Every successful publication still
needs outcome inspection; a successful call does not guarantee all destinations
accepted a message. In-process output observation does not prove physical I/O.

## Verify the independent package

Use a new directory outside the checkout. These PowerShell commands create an
independent binary with this library as its only direct dependency:

```powershell
$framework = (Get-Location).Path
$consumer = Join-Path (Split-Path $framework -Parent) 'rff-composition-consumer'
cargo new --bin --edition 2024 --vcs none $consumer
cargo add --manifest-path "$consumer/Cargo.toml" rust-flight-framework --path $framework --offline
Copy-Item -LiteralPath "$framework/examples/composition.rs" -Destination "$consumer/src/main.rs"
cargo check --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets
cargo clippy --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets -- -D warnings
cargo test --manifest-path "$consumer/Cargo.toml" --locked --offline
cargo run --manifest-path "$consumer/Cargo.toml" --locked --offline
cargo metadata --manifest-path "$consumer/Cargo.toml" --locked --offline --format-version 1
Get-FileHash -Algorithm SHA256 "$framework/examples/composition.rs", "$consumer/src/main.rs"
```

Stop after any failed command and inspect the error before continuing. Cargo add
creates the initial independent lockfile. Metadata should show one independent
workspace member, two graph packages and only the library path dependency; the
two source hashes should match. No extra maintained workspace crate is needed.

The [focused review](verification/COMPOSITION_REVIEW.md) records actual commands,
source identity, failure observations and autonomous Codex review for
RFF-LIB-REQ-002. Evidence covers this controlled consumer, not external adoption,
API stability, release readiness or operational use.
