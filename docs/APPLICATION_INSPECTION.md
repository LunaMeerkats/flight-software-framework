# Inspect application-owned state

Use `Runtime::application(id)` or `MessagingRuntime::application(id)` to
borrow the application selected by that owner's runtime-local ID. This lets
the host read an ordinary application-owned observation between callbacks
without wiring a borrowed Cell slot into the application.

```rust
runtime.work(id)?;
let work_calls = runtime.application(id)?.observation.works;
```

The application chooses which fields or shared-reference methods are visible.
For mission enum composition, match the returned enum reference to read the
selected variant. The complete [public-only consumer](../tests/application_inspection.rs)
owns its observation fields and exercises both runtime owners.

## Ownership, errors and lifecycle

The result is `Result<&A, LifecycleError>`. Inspection is available while
Registered, Running, Stopped or Failed; it does not invoke an application
callback or change the record's lifecycle state. A stopped or failed value
remains the same retained application, so its diagnostics can be observed
without starting work or attempting recovery. Messaging lookup neither
dequeues nor clears an inbox. Existing stop/failure operations retain their
own clearing behavior.

An ID whose slot is absent returns `LifecycleError::UnknownApplication`.
Keep IDs with the owner that issued them: equal-position IDs from different
owners can compare equal and select a local record. Lookup does not validate
origin or provide authentication or persistent identity.

The shared reference borrows its owner. Copy an observation or finish using
the reference before calling a mutable owner operation. No mutable application
or inner-runtime reference is exposed. An application method requiring
`&mut self` cannot be called through this borrow. The API documentation has
compile-fail examples for both restrictions.

Application-defined shared-reference methods may still have effects, including
interior mutation through Cell or RefCell. Inspection is not a guarantee of
arbitrary-code isolation or application immutability. Lookup itself allocates
no storage; application-provided observation methods retain their own resource
and failure behavior. Shared inspection does not drain non-Copy output,
acknowledge a message or prove cleanup, recovery or external delivery.

Existing host-owned output-slot recipes remain useful when the host must take
ownership, apply backpressure or transfer output. This API does not replace
their explicit handoff policy. The library remains experimental host software
with provisional APIs.

## Verify the consumer outside the workspace

Use a fresh destination outside this checkout. The test source uses only
public library APIs and std, with no private sample imports or external slots.

```powershell
$framework = 'E:/Projects/flight software framework'
$consumer = Join-Path $env:TEMP ('rff-application-inspection-' + [guid]::NewGuid().ToString('N'))
cargo new --lib --edition 2024 --vcs none $consumer
cargo add --manifest-path "$consumer/Cargo.toml" rust-flight-framework --path $framework --offline
New-Item -ItemType Directory -Path "$consumer/tests" | Out-Null
Copy-Item -LiteralPath "$framework/tests/application_inspection.rs" -Destination "$consumer/tests/application_inspection.rs"
cargo check --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets
cargo clippy --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets -- -D warnings
cargo test --manifest-path "$consumer/Cargo.toml" --locked --offline --test application_inspection
cargo metadata --manifest-path "$consumer/Cargo.toml" --locked --offline --format-version 1
Get-FileHash -Algorithm SHA256 "$framework/tests/application_inspection.rs", "$consumer/tests/application_inspection.rs"
```

Inspect metadata for one independent workspace member, two graph packages
and the sole framework path dependency. Compare exact source bytes as well
as SHA256 hashes. The [focused review](verification/APPLICATION_INSPECTION_REVIEW.md)
records actual commands and results, including positive counterparts for the
negative borrowing examples. This verifies the controlled consumer rather
than external adoption or general integration readiness.
