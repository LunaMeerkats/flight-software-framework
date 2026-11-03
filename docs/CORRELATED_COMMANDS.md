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
No bus acknowledgement queue, persistence, multiple-issuer coordination or atomic
transaction is introduced. Accepted peer effects survive later callback failure.

## Read bounded command results

The RFF-LIB-REQ-013 extension adds one typed host result slot. This is the
operational feedback path; the copied pending observation remains test-only.
The producer and sole serial host share an `Rc<Cell<Option<CommandResult>>>`.
After a command callback succeeds, the record contains its exact Command and:

| Outcome | What the host can conclude |
| --- | --- |
| Reply with two acceptance flags | These destinations have accepted this operation's reply by the end of this command, including earlier attempts. Both true means the producer cleared its pending slot. |
| Busy with pending ID | New was consumed without staging its requested operation. Retain the request and resubmit the same ID after the current operation ends. |
| Cancelled | The matching pending operation was cleared during this callback. Already accepted destination copies remain independent. |
| NoMatchingOperation | This control had no matching pending operation at dispatch. It does not distinguish completed, cancelled, idle or different-ID state. |

Use `producer_turn()` for producer dispatch. It returns ResultPending while the
slot is occupied, without removing another queued command. Otherwise it calls
dispatch_one at most once and preserves its full operation/error/discard-count
wrapper. Empty inboxes produce no result. The host explicitly calls
`result.take()` to free the slot, decides what that result means for its mission,
then chooses the next turn. Reading a result does not automatically resubmit,
retry or cancel anything. One slot bounds feedback; neglecting it stops producer
dispatch and can fill the ordinary command inbox. Rejected ingress remains a
host-owned request. The gate precedes lifecycle validation, so ResultPending
does not assert that the producer is Running; inspect lifecycle separately.

The callback defensively rejects direct dispatch while the slot is occupied
with ResultOccupied before parsing or command/publication effects. The old result
survives. Ordinary returned-error behavior still consumes the in-flight command,
makes that producer Failed and clears its remaining command inbox. This is a
composition error path, not the normal response to a full result slot.
Malformed commands and routing errors return their existing typed runtime errors
without a fresh result. A later routing error can follow an earlier accepted peer
publication; result absence does not roll back that effect.

Stop/restart retains both a recorded result and the pending operation. Stop may
clear queued commands that never ran, and they have no result. A saved Reply
result remains an enqueue snapshot even if a destination later stops and clears
its accepted copy. These results observe local callback handling, not subscriber
processing, persistent acknowledgements or reliable command transport. Repeated
New commands can repeat effects; no request-result history suppresses duplicates.
Host/app access to the cell is trusted; writing it externally can violate policy.
This consumer supplies no synchronization or concurrent host support.

Operational state is one pending two-byte reply, two flags, two destination IDs,
the host issuer, one typed result slot and finite inboxes. One shared result
allocation is constructed once; commands add no growing history. Each New or
matching Retry makes at most two selective attempts. Fixed copied
observations are test instrumentation. Logical bounds do not establish process
memory or callback timing bounds, fairness, eventual progress, external adoption,
API stability, flight suitability or release readiness.

[ADR-0037](adr/0037-correlated-operation-cancellation.md) compares correlation
alternatives; [ADR-0038](adr/0038-bounded-host-command-results.md) compares host
feedback with bus acknowledgements and a framework API change.
RFF-LIB-REQ-012/013 define this consumer. The
[focused review](verification/CORRELATED_COMMANDS_REVIEW.md) records actual
correlation verification; the [result review](verification/COMMAND_RESULTS_REVIEW.md)
records the feedback extension's verification and Codex provenance.

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
