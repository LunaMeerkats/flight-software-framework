# Retry replies inside a message handler

A message-driven application can own its pending reply and acceptance ledger.
Use ApplicationMessageContext::publish for initial fan-out, then publish_to for
one rejected subscriber during a later callback. The host explicitly queues and
dispatches a retry command; the framework does not schedule retries.

The [public-only consumer](../tests/callback_publication.rs) demonstrates this
through the ordinary MessagingApplication callback, without private sample
modules or host-owned retry state. Copied fixed-size observations expose the
reply and flags to test assertions; only the application changes retry state. Run:

```text
cargo test --locked --test callback_publication
```

## One selected attempt

Within handle_message, call context.publish_to(application_id, &message).
The operation returns `Result<DeliveryStatus, PublishToError>`:

| Result | Meaning and effect |
| --- | --- |
| UnknownApplication | No configured endpoint; no inbox changes |
| NotSubscribed | Known endpoint lacks this topic; no inbox changes |
| Delivered | One copied message appended to the selected inbox |
| InboxFull | Selected older FIFO retained; borrowed message remains available |
| Unavailable | Selected endpoint was not Running before this callback |

Membership and subscription validation precede availability/capacity checks.
Only the selected inbox may change, with no publication report or retry-history
allocation. Subscriptions still apply. The lifecycle snapshot is the same one
used by context.publish: it refreshes before each dispatch and cannot change
during that synchronous callback. The context exposes no lifecycle mutation,
dequeue or nested dispatch. A self-targeted attempt may use the slot freed by
removing the current input, including in a capacity-one inbox.

## Keep reply ownership explicit

The consumer retains one inline reply, two destination IDs and two acceptance
flags inside its application. Initial broadcast seeds those flags. Later retry
commands attempt only rejected destinations; full/unavailable results retain the
reply. A busy new-reply command is consumed without replacing or buffering its
requested value; the host must resubmit it after completion. The copied deferred
value is test instrumentation, not another pending request. After both inboxes
accept, the application clears its slot and permits another reply. Host policy
must choose when to send retries, when to stop, and how to cancel pending work.

These flags record enqueue history. Stopping or failing an accepted destination
can later discard its copy without changing a flag. Repeating publish_to remains
a new enqueue attempt and can duplicate a copy. Equal-position foreign IDs can
alias; pair IDs with the same runtime. Subscriptions do not authenticate callers.

Publication commits immediately. If the callback then returns an error, its
application becomes Failed and its remaining inbox is cleared, including any
self-published copies. Accepted peer copies survive. The reported discard count
excludes the in-flight input. There is no transactional rollback or processing
acknowledgement. The host-side [selective publication guide](SELECTIVE_PUBLICATION.md)
retains its separate caller-owned ledger; ordinary work is unchanged.

The demonstrated bounds are one pending one-byte reply, two acceptance flags,
fixed destination IDs and configured finite inbox capacities. Each retry command
performs at most two selective attempts. Initial broadcast still allocates its
ordinary destination report. Test observation storage is separate from the reply
slot. Logical bounds do not bound process memory, allocator behavior or callback
execution time. No fairness, deadline, automatic progress, physical delivery,
external adoption, API stability or flight suitability is established.

## Verify the independent package

From the framework checkout, stop on every command failure:

```powershell
$framework = (Get-Location).Path
$consumer = Join-Path $env:TEMP ('rff-callback-' + [guid]::NewGuid().ToString('N'))
cargo new --lib --edition 2024 --vcs none $consumer
cargo add --manifest-path "$consumer/Cargo.toml" rust-flight-framework --path $framework --offline
New-Item -ItemType Directory -Path "$consumer/tests" | Out-Null
Copy-Item -LiteralPath "$framework/tests/callback_publication.rs" -Destination "$consumer/tests/callback_publication.rs"
cargo check --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets
cargo clippy --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets -- -D warnings
cargo test --manifest-path "$consumer/Cargo.toml" --locked --offline
cargo metadata --manifest-path "$consumer/Cargo.toml" --locked --offline --format-version 1
Get-FileHash -Algorithm SHA256 "$framework/tests/callback_publication.rs", "$consumer/tests/callback_publication.rs"
```

Inspect metadata: one independent workspace member, the consumer plus its sole
framework path dependency, and no library dependencies. Compare SHA256 and exact
source bytes. The generated cargo-new smoke test is separate from the copied
consumer tests. Record the actual package path and results in the
[focused review](verification/CALLBACK_PUBLICATION_REVIEW.md).
[ADR-0035](adr/0035-callback-owned-selective-publication.md) compares alternatives.
This controlled consumer demonstrates this composition only; publication and
the separate consequential authorization gates remain unchanged.
