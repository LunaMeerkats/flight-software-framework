# Retry selected subscribers

An initial publish can enqueue a message for one subscriber while another inbox
rejects it. Publishing the whole topic again can duplicate the accepted copy.
Use MessageBus::publish_to or the host-facing MessagingRuntime::publish_to to
attempt only one rejected subscriber. The same topic subscription is required:
this operation does not bypass the immutable routing configuration.

The [public-only consumer](../tests/selective_publication.rs) demonstrates two
subscribers, one retained message and a fixed acceptance ledger. Run it with:

```text
cargo test --locked --test selective_publication
```

## One explicit attempt

Call publish_to(application_id, &message). Its result has no publication report
allocation or framework retry history. The message stays owned by the caller;
Delivered copies it into the selected inbox. Other inboxes are untouched.

| Outcome | Meaning | Caller action in this consumer |
| --- | --- | --- |
| Delivered | Selected inbox accepted one copy | Mark that destination accepted |
| InboxFull | Older queued messages retained | Keep pending message; drain before retry |
| Unavailable | Runtime destination is not Running | Keep pending; choose restart, cancellation or stop |
| UnknownApplication error | ID has no configured endpoint | Correct composition; do not blindly retry |
| NotSubscribed error | Known endpoint does not subscribe to this topic | Correct message/topology; do not bypass routing |

Membership is checked before subscription, and subscription before availability
or capacity. Invalid input changes no inbox. The standalone bus treats configured
endpoints as available. The runtime reads current lifecycle state for each
attempt: Registered, Stopped and Failed subscribers return Unavailable. Successful
stop or returned callback failure can clear earlier accepted deliveries under
the existing owner contract. In-place restart reconnects an empty inbox.

This adds no callback capability: ApplicationMessageContext still publishes
through its existing all-subscriber method. Ordinary publish retains its ordered
per-destination report, partial fan-out behavior and report-allocation error.

## Retain acceptance across turns

The consumer seeds two acceptance flags from the original report, then calls
publish_to only for rejected destinations. A drained accepted subscriber gets no
duplicate. Repeated InboxFull or Unavailable outcomes retain the same pending
message. The mission holds the next record until both subscribers accept the
current record, preserving its chosen per-subscriber enqueue order. It keeps
the original message unchanged and pairs reports, flags and IDs with one runtime.

The flags record acceptance, not callback execution or durable acknowledgement.
Stopping an accepted subscriber can discard its copy; this does not clear the
flag or authorize an automatic retry. Repeating publish_to can enqueue another
copy. The library does not deduplicate, authenticate IDs, manage retry ledgers,
guarantee eventual processing or provide exactly-once delivery. Equal-position
foreign IDs can alias; use IDs from the same owner. Subscriptions are routing,
not access control. Choose cancellation and retry limits for the actual mission.

The demonstrated logical storage is two inboxes of one or two messages each,
one inline one-byte pending message, two acceptance flags, fixed destination
IDs and one host-observation cell per sink. The partial-fan-out scenario uses
two slots per inbox so older rejected-side FIFO order can be observed.
These limits do not bound process memory, allocator behavior, callback time or
report storage retained by the caller. Each selective attempt inspects the
selected endpoint and its topic set, then performs at most one enqueue. It
creates no hidden worker, wait loop or timer. Panics, hangs, process failure and
allocator exhaustion remain outside the controlled evidence.

[ADR-0034](adr/0034-subscription-respecting-selective-publication.md) compares
whole-topic retry, per-destination topics and subset APIs. The older
[work-output example](ORDINARY_WORK_OUTPUT.md) intentionally retains its strict
single-subscriber policy; this separate consumer demonstrates the new capability.

## Verify the independent package

Run from the framework checkout, stopping on any failure:

```powershell
$framework = (Get-Location).Path
$consumer = Join-Path $env:TEMP ('rff-selective-' + [guid]::NewGuid().ToString('N'))
cargo new --lib --edition 2024 --vcs none $consumer
cargo add --manifest-path "$consumer/Cargo.toml" rust-flight-framework --path $framework --offline
New-Item -ItemType Directory -Path "$consumer/tests" | Out-Null
Copy-Item -LiteralPath "$framework/tests/selective_publication.rs" -Destination "$consumer/tests/selective_publication.rs"
cargo check --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets
cargo clippy --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets -- -D warnings
cargo test --manifest-path "$consumer/Cargo.toml" --locked --offline
cargo metadata --manifest-path "$consumer/Cargo.toml" --locked --offline --format-version 1
Get-FileHash -Algorithm SHA256 "$framework/tests/selective_publication.rs", "$consumer/tests/selective_publication.rs"
```

Metadata should show one independent workspace member, two graph packages and
the sole library path dependency. Test bytes and hashes should match. The
[focused review](verification/SELECTIVE_PUBLICATION_REVIEW.md) records the actual
package and results. This controlled consumer does not establish external
adoption, API stability, release readiness, qualification or operational use.
