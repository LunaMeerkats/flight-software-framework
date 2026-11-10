# Run a bounded command session

The [standalone example](../examples/command_session.rs) connects validated
command ingress, a correlated producer, local command feedback and two reply
subscribers through public framework APIs and std. It imports no private sample
modules or integration-test fixtures. The framework remains experimental host
software, with provisional APIs and no flight qualification.

```text
cargo test --locked --example command_session
cargo run --locked --example command_session
```

## Follow the ownership

One serial host owns the session issuer and selects every callback. Allocate one
ID for a logical New operation and keep that ID when resubmitting a busy or
rejected request. IDs are finite and never wrap within this session; retain the
issuer across producer stop/restart and never reset it while old work survives.
Frames use New=[1, nonzero ID, value], Retry=[2, nonzero ID] and
Cancel=[3, nonzero ID]; reply payloads are [ID, value]. Unknown opcodes, zero IDs
and all other lengths are rejected before enqueue. The three-byte command and
two-byte reply grammar is local to this recipe; it is not an external protocol
or a compatibility claim.

The producer owns one pending reply and two acceptance flags. New while busy
returns Busy without replacing the older operation. Matching Retry attempts only
rejected subscribers. Matching Cancel clears only future attempts; stale controls
leave newer work intact. Accepted subscriber copies remain independent and can
later be cleared by lifecycle operations.

The producer records the exact handled command in one capacity-one result slot.
A producer turn checks that slot before dispatching at most one command. The host
explicitly drains each result and chooses another turn. An occupied result
preserves both the older record and queued command FIFO, including while the
host writes the result line. Ingress full/unavailable outcomes leave resubmission
with the host. Enqueue acceptance alone does not establish execution.

Each subscriber has its own capacity-one output slot. A selected subscriber turn
checks that slot before dispatching at most one reply. The callback records the
reply in that slot, and the host writes the line outside callbacks. A command
result can record inbox acceptance. A populated reply slot records the selected
subscriber's callback, and writing its line is a separate fallible host effect.

## Follow the fixed scenario

The driver first sends operation 1/value 3 to both subscribers, then drains only
the first subscriber. Operation 2/value 40 reaches the first inbox while the
second still holds the older reply, yielding ReplyAccepted([true, false]).
New operation 3/value 7 is queued while the result slot is occupied; another
producer turn reports SlotOccupied. After writing the older result, the next
turn reports Busy(operation 2) for that saved request.

The host drains operation 2 from the first subscriber and the older operation 1
from the second. Retry(2) then completes acceptance without duplicating the first
subscriber's reply. After draining the second, the host resubmits its same saved
New operation 3 and observes both outputs. Every callback selection and drain
appears in the finite host script. Tests separately cover
correlated cancellation, saturation and restart behavior.

## Retain a command when the inbox is full

The script then stages a stale control's result and fills the command inbox
with two more stale controls. With dispatch gated, New operation 4/value 9
gets `InboxFull` and stays in a caller-owned `Option<Command>`. The host
finishes writing the staged result and dispatches one older command. It
then admits the same operation 4 behind the other older command, while the
first queued control's result still gates dispatch. The two controls
produce `NoMatch` before operation 4 produces its result and replies.

Call `admit_pending` once when the host chooses to try the saved command.
`Delivered` clears the slot before the host prints the outcome. `InboxFull`,
`Unavailable` and typed errors leave the command available for a later
decision. An empty slot makes no enqueue attempt. Keep later offers upstream
while the slot is occupied if they must follow this command. The host must
decide how to handle persistent unavailability or a routing error.
The turn uses the existing ID and does not invoke a callback. A `Busy`
result belongs to a command that already dispatched and has its own
resubmission policy.

## Read a New request from finite input

The saturation script uses a `Cursor` over `[1, 9]` to stage operation 4.
The input record `[1, value]` requests New. The host issues the ID after
validating the complete record, then stages the typed command in the same
saved slot. The framed input grammar belongs to the example. The script
supplies its own bytes, and Retry and Cancel remain typed host decisions.

Keep the same `NewInput` and session across turns. `read_turn` calls its
reader at most once, requesting only the suffix it still needs. A short
read reports `Partial`. A reader error returns the original `io::Error`
and preserves the prefix. The host chooses when to retry, including after
`Interrupted` or `WouldBlock`. The reader controls whether a call blocks.

`Staged` means the command is ready for an explicit admission turn. A
read while the slot is occupied reports `SlotOccupied` without touching
the reader. `Delivered` frees the slot for the next record. The host can
also clear the slot to abandon the saved command. A bad tag consumes that
two-byte record and returns `Rejected(InvalidFrame)` without issuing an
ID. A valid record after issuer exhaustion returns `Rejected(IdsExhausted)`
and leaves the slot empty.

Zero at a record boundary reports `End`. Zero after a partial record
returns `UnexpectedEof`, and a count beyond the supplied suffix returns
`InvalidData`. Both errors end intake. Later turns report `End` without
another read. The host treats this input as finite.

## Write a result or reply

The host opens a `SlotOutput` over the selected result or reply slot and its
writer. It renders one complete line into a fixed 128-byte buffer before the
first write. If rendering fails, the record stays in the slot and the writer
receives no bytes. Each `write_turn` attempts one write of the remaining
suffix. A positive short write advances the byte offset and reports `Pending`.
A writer error returns its original `io::Error`. Zero progress returns
`WriteZero`, and an impossible count beyond the supplied suffix returns
`InvalidData`. Each write error leaves the record and byte offset available
for another turn.

Full acceptance clears the selected slot and reports `Accepted`. Another turn
on that completed output leaves any later record in the slot alone. While a
line is pending, the occupied slot still gates its application. The peer
subscriber can run and stage a reply if its own slot is free. Its line waits
while the active presenter holds the shared writer's mutable borrow. The host
finishes the current line before selecting another one for that writer.

A recovering host keeps the same presenter and writer and drives `write_turn`
directly. The script uses `finish`, which consumes the presenter and exits on
a writer error. Restarting the script's observation would lose the byte offset.
Incidental diagnostics use one-shot writes.
Acceptance describes the bytes reported by the writer. Flush, physical delivery,
durability and process recovery have separate boundaries.

## Resource and failure policy

The session has three applications, one command inbox of capacity two, two
reply inboxes of capacity one, a three-byte maximum message payload, one pending
reply with two acceptance flags, one local result, two local outputs, one
issuer for IDs 1..=255, one saved ingress command, one two-byte input buffer
with its read position and one fixed rendered line with an accepted-byte
offset. Setup uses the framework's
finite-capacity constructors.
The host owns `HostSlots` and applications borrow its `Cell` slots directly.
The host selects every callback and keeps each occupied record until its line
is accepted.
Logical storage limits do not establish process-memory limits, callback timing,
fairness or eventual progress.

Malformed ingress is rejected before queue mutation. A malformed message inserted
through a lower-level bypass can still fail its callback: the selected producer
becomes terminal Failed, remaining queued commands are cleared, no fresh success
result is invented and peer effects remain independent. Bypassing an occupied
slot gate is a composition error and returns an explicit callback failure.
Ordinary dispatch error wrappers retain selected-inbox discard counts. Stop
and restart retain the same producer value, pending operation, local result
and host issuer, while successful stop clears undispatched selected commands.

Setup propagates failures rather than asserting success. Its one-shot driver
reports an error and tears down; reducing ownership-preserving construction or
registration wrappers to their typed cause does not demonstrate recovery.
Panics, hangs and process/allocator failure remain outside this recipe.

After attaching the inboxes, setup uses `MessagingRuntime::subscribes_to` to
check the producer's Command and both subscribers' Reply routes before starting
any application. A valid topology can still have its inbox entries in the
wrong order. The example reports the first missing required route with the
application ID and topic. Use IDs returned by the same runtime. The query
inspects configured membership independently of availability and capacity,
without publishing a probe message. The host supplies the expected routes.

No result history suppresses duplicate New commands. Session correlation is not
authentication, persistent identity, reliable acknowledgement or a transaction.
This controlled composition establishes neither external adoption nor API
stability, operational suitability or release readiness.

## Verify the identical independent binary

From the framework checkout in PowerShell, stop after every command failure:

```powershell
$framework = (Get-Location).Path
$consumer = Join-Path $env:TEMP ('rff-command-session-' + [guid]::NewGuid().ToString('N'))
cargo new --bin --edition 2024 --vcs none $consumer
cargo add --manifest-path "$consumer/Cargo.toml" rust-flight-framework --path $framework --offline
Copy-Item -LiteralPath "$framework/examples/command_session.rs" -Destination "$consumer/src/main.rs"
cargo check --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets
cargo clippy --manifest-path "$consumer/Cargo.toml" --locked --offline --all-targets -- -D warnings
cargo test --manifest-path "$consumer/Cargo.toml" --locked --offline
cargo run --manifest-path "$consumer/Cargo.toml" --locked --offline
cargo metadata --manifest-path "$consumer/Cargo.toml" --locked --offline --format-version 1
Get-FileHash -Algorithm SHA256 "$framework/examples/command_session.rs", "$consumer/src/main.rs"
```

Inspect one independent workspace member, the binary and its sole framework path
dependency, with no library dependencies. Compare exact source bytes and SHA256
and compare the runnable output. Record the actual path/results in the
[focused review](verification/COMMAND_SESSION_REVIEW.md).
[RFF-LIB-REQ-014 and RFF-LIB-REQ-015](REQUIREMENTS.md),
[ADR-0039](adr/0039-runnable-command-session.md) and
[ADR-0040](adr/0040-read-only-subscription-inspection.md) define the scope. The
original [correlated-command consumer](CORRELATED_COMMANDS.md) retains its
separate exact histories and evidence.
