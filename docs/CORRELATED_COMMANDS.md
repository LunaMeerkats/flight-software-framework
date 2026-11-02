# Correlate cancellation with one pending operation

A delayed control command should identify the operation it intends to affect.
The [public-only consumer](../tests/correlated_commands.rs) composes this policy
through existing MessagingApplication and ApplicationMessageContext APIs. It is
a separate consumer: the [original callback consumer](CALLBACK_PUBLICATION.md)
still cancels whichever reply exists when Cancel is dispatched.

```text
cargo test --locked --test correlated_commands
```

## Session identity and command ownership

One host-owned issuer allocates IDs 1..=255, then returns an explicit exhaustion
error without wrapping. Keep that same issuer across producer stop/restart.
Allocate once per new logical operation and retain the ID when resubmitting a
rejected or busy command. Never reset or replace the issuer while any old command,
reply or saved host request can survive. The small finite domain makes exhaustion
fully testable; longer sessions need a separately designed wider or persistent
identity policy. This is not a framework ApplicationId or a standard protocol.

The controlled payload grammar is New=[nonzero ID, value], Retry=[nonzero ID]
and Cancel=[nonzero ID]. Parse exact shape and nonzero identity before application
effects. A reply carries [ID, value], preserving correlation for destination
observations. The host queues commands and explicitly dispatches the producer;
test-only copied observations do not control application behavior.

Malformed commands return the typed InvalidCommand error before reply or control
effects. Existing callback failure policy then makes the producer terminal Failed
and clears its remaining command inbox; accepted peer copies survive. Logical
full/unavailable destination outcomes remain retained backpressure rather than
application errors. Routing errors indicate a mission composition problem and
retain their concrete source through the consumer error.

## One pending reply

The producer owns one inline reply, its ID and two destination acceptance flags.
New while idle stages the operation and attempts both fixed subscribers. A busy
New command is consumed without replacement or another buffered operation; the
host must resubmit it when appropriate. Retry(id) attempts only rejected
destinations of the matching pending operation. InboxFull and Unavailable retain
that operation. Once both inboxes accept it, the producer clears its slot.

Cancel(id) clears only matching pending state. Stale, idle and repeated Cancel
commands do nothing. A delayed Cancel(A) behind New(B) cannot clear B when A and
B have distinct session IDs. Neither cancellation nor a stale retry changes
accepted destination copies or older rejected FIFO values. These are enqueue
acceptance flags: they do not prove processing, retention after later lifecycle
clearing or durable delivery.

The producer inbox has capacity two, and each destination inbox has capacity two.
A third queued command is rejected without changing older FIFO. Control commands
have no priority: enqueue does not acknowledge dispatch or cancellation. The
host must retain/resubmit rejected commands and choose dispatch order. Successful
producer stop discards queued commands while its no-op stop/restart callbacks
retain the pending operation and flags. Resubmit the same control ID after
restart when still wanted. Terminal Failed producers cannot dispatch commands.

## Identity and resource limits

Equality supplies correlation within this host session only. Bypassing the issuer
or resetting it can reuse an ID. A repeated New after completion can create
another reply; this consumer does not suppress duplicates or authenticate callers.
No acknowledgement queue, persistence, multiple-issuer coordination or atomic
transaction is introduced. Accepted peer effects survive later callback failure.

Operational state is one pending two-byte reply, two flags, two destination IDs,
the host issuer and finite inboxes. Each New or matching Retry makes at most two
selective attempts; control commands add no pending-command history. Fixed copied
observations are test instrumentation. Logical bounds do not establish process
memory or callback timing bounds, fairness, eventual progress, external adoption,
API stability, flight suitability or release readiness.

[ADR-0037](adr/0037-correlated-operation-cancellation.md) compares alternatives;
RFF-LIB-REQ-012 defines this consumer. The
[focused review](verification/CORRELATED_COMMANDS_REVIEW.md) records actual
verification and Codex provenance.

## Verify the independent package

From the framework checkout, stop on every command failure:

```powershell
$framework = (Get-Location).Path
$consumer = Join-Path $env:TEMP ('rff-correlated-' + [guid]::NewGuid().ToString('N'))
cargo new --lib --edition 2024 --vcs none $consumer
cargo add --manifest-path "$consumer/Cargo.toml" rust-flight-framework --path $framework --offline
New-Item -ItemType Directory -Path "$consumer/tests" | Out-Null
Copy-Item -LiteralPath "$framework/tests/correlated_commands.rs" -Destination "$consumer/tests/correlated_commands.rs"
cargo check --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets
cargo clippy --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets -- -D warnings
cargo test --manifest-path "$consumer/Cargo.toml" --locked --offline
cargo metadata --manifest-path "$consumer/Cargo.toml" --locked --offline --format-version 1
Get-FileHash -Algorithm SHA256 "$framework/tests/correlated_commands.rs", "$consumer/tests/correlated_commands.rs"
```

Inspect metadata for one independent workspace member, the consumer plus its sole
framework path dependency and no library dependencies. Compare exact source bytes
and SHA256, and record the actual path/results in the focused review. The generated
smoke test is separate from the copied consumer tests. This verifies this
controlled composition; separate consequential authorization gates remain.
