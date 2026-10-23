# Attribute diagnostics during ordinary work

`ApplicationWorkContext::application_id()` returns the runtime-local identity
selected for this callback. Use it to attribute application-owned observations
without wiring a second identity into the application after registration:

```rust
let event = Event::new(
    EventSource::Application(context.application_id()),
    EventSeverity::Informational,
    MissionEvent::WorkAccepted,
    EventTimestamp::from_clock(self.clock),
);
```

This fragment belongs inside `Application::work`; `Event`, `EventSource`,
`EventSeverity` and `EventTimestamp` are public library types. `MissionEvent`
and the injected clock are selected by the consumer. The full compiled
[public-API tests](../tests/work_identity.rs) demonstrate the surrounding types,
lifecycle and error handling. Run them with:

```text
cargo test --locked --test work_identity
```

The same context still exposes optional immutable configuration. Direct work,
messaging-owned work, failure-event wrappers and finite scheduled work all use
the same identity contract. In-place stop/restart preserves the record's ID.
Rejected lifecycle operations invoke no callback. The ID is meaningful within
its owning runtime; equal-position keys from different runtimes can compare
equal. Keep runtime ownership explicit when retaining or forwarding an ID.

## Event ownership and callback-time feedback

The accessor grants no event, clock or messaging service. A mission can inject
those dependencies into its application. The tested diagnostic producer borrows
a caller-owned `RefCell<EventQueue<_>>` and a `Clock`; the host can drain between
callbacks while the runtime retains the application. It calls `try_borrow_mut`
and maps a borrow conflict to its concrete work error. It checks `emit` before
committing the demonstrated effect. A full queue retains older records and
returns a typed error; this consumer chooses terminal application failure on
either busy or full. Choose a different policy explicitly if that is unsuitable.

Do not retain a queue borrow while calling a work operation that must borrow
the same queue. `work_with_failure_event` requires its mutable event queue
before invoking the callback, so the same RefCell queue cannot serve both roles
in one wrapper call. The consumer tests use a separate failure-report queue:
the callback's full diagnostic queue rejects its event, while the wrapper
independently reports that typed error. Alternatively, call ordinary work and
explicitly report its returned error after releasing the callback's queue borrow.
Ordinary application event emission is distinct from the framework's opt-in
returned-work failure report.

The frozen fixture uses two application records, a capacity-one event queue,
one clock value and caller-owned attempted/committed work, call-count and message
observations per application. Messaging tests add two capacity-one inboxes and
one-byte payloads. Queue capacity counts records, not total process bytes,
callback allocation or execution time. No hidden worker, automatic drain,
event retry or source authentication is added.

Recording an event before an effect establishes ordering only. A later failure
does not retract the event, and an accepted event does not prove physical output
or atomic delivery. Cell/RefCell are a serial mission policy, not concurrent
queues. Panics, hangs and allocator exhaustion remain outside cooperative errors.
The framework remains experimental host software with provisional APIs; these
tests do not establish adoption, flight qualification or operational suitability.

## Verify the same consumer outside the workspace

Use an unused destination outside this checkout. Substitute the actual framework
path if necessary. Cargo new creates a one-member independent workspace; the
library remains an unpublished path dependency. The copied test source imports
only the public library and `std`, with no private sample modules.

```powershell
$framework = 'E:/Projects/flight software framework'
$consumer = Join-Path $env:TEMP ('rff-work-identity-' + [guid]::NewGuid().ToString('N'))
cargo new --lib --edition 2024 --vcs none $consumer
cargo add --manifest-path "$consumer/Cargo.toml" rust-flight-framework --path $framework --offline
New-Item -ItemType Directory -Path "$consumer/tests" | Out-Null
Copy-Item -LiteralPath "$framework/tests/work_identity.rs" -Destination "$consumer/tests/work_identity.rs"
cargo check --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets
cargo clippy --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets -- -D warnings
cargo test --manifest-path "$consumer/Cargo.toml" --locked --offline --test work_identity
cargo metadata --manifest-path "$consumer/Cargo.toml" --locked --offline --format-version 1
Get-FileHash -Algorithm SHA256 "$framework/tests/work_identity.rs", "$consumer/tests/work_identity.rs"
```

Inspect metadata for the independent workspace root, one workspace member, two
graph packages and the sole framework path dependency. The source hashes must
match. This verifies these controlled consumer tests, not general integration
or external adoption. Actual commands, paths and review results are in the
[focused record](verification/WORK_IDENTITY_REVIEW.md); the design and alternatives
are in [ADR-0029](adr/0029-ordinary-work-identity.md).
