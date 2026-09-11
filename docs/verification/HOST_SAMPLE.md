# Combined host sample

The `host-echo` executable is an experimental host demonstration. It is not
flight-qualified, safety-certified, NASA-affiliated, TRL-proven, or suitable
for operational spacecraft, safety-critical systems, or human-rated systems.
It claims no cFS, cFE, OSAL, PSP, CCSDS, or RTOS compatibility, real-time
behavior, or fault tolerance.

## Run and inspect

From the repository root:

```text
cargo run --example host-echo
cargo test --test host_sample
cargo test --test host_adapters
```

The binary prints a scope notice, runs the
[shared scenario](../../examples/host-echo/sample.rs), then prints these exact
command/telemetry records followed by its structured debug report:

```text
command [01, 00] -> telemetry [81, 00]
command [01, 2A] -> telemetry [81, 2A]
command [01, 64] -> telemetry [81, 64]
```

Inspect the report's values rather than depending on Rust debug punctuation
as a stable format. The data is copied from completed operations and actual
work callbacks. Stdout happens outside callbacks; a write failure terminates
the executable without undoing the completed scenario.

## Expected trace

Application pairs are ordered echo, then telemetry. Each fresh run starts
with a new runtime, clock origin, table, observation slots, mailbox, and fault
switch. Application identifiers must stay paired with that runtime.

| Report field | Expected observation |
| --- | --- |
| `lifecycle` | Both Registered, both Running, both Stopped, both Running |
| `malformed_command` | Unknown identifier `0xff`, before invalid percentage |
| `telemetry` | `[0x81, 0]`, `[0x81, 42]`, `[0x81, 100]` |
| `schedule` | Waiting at zero; echo then telemetry at 10 ms; Complete |
| `scheduled_observations` | Both revision 1, value 10 |
| `configuration.revisions` | Activation 2, rollback 1, fresh activation 3 |
| `configuration.observations` | `(2,20)`, `(2,20)`, `(1,10)`, `(3,30)` |
| `configuration.rejection` | Value 101 rejected by the retained validator |
| `configuration.consumed_rollback` | No rollback available on second attempt |
| `restarted_observations` | Both freshly observe revision 3, value 30 |
| `fault.failure` | Original injected echo work error; one queued discard; Recorded event |
| `fault.pending_after_failure` | Echo 0, telemetry 1 |
| `fault.observations` | Failed echo and later healthy telemetry work both see revision 3, value 30 |
| `fault.retained_telemetry` | `[0x81, 7]`; command 9 was cleared with echo's inbox |
| `fault.event` | Echo source, Error severity, EchoWorkFailed identifier, elapsed 20 ms |
| `fault.next_event` | None after draining the one recorded event |
| `fault.final_states` | Echo Failed; telemetry Running |

The host first queues command 7 and dispatches echo, leaving telemetry 7 in
the peer inbox. It then queues command 9 for echo. Failure is a separate
ordinary-work call; it clears echo's queued command while retaining the peer
record. Telemetry subsequently completes work and message dispatch.

## Ownership and limits

[ADR-0022](../adr/0022-combined-host-sample.md) records the complete driver
order, alternatives, resource accounting, and revisit conditions. The private
[work fixture](../../examples/host-echo/mission/work.rs) uses two latest-value
slots and one explicit echo fault switch. Host reads consume each observation
so a previous callback cannot supply evidence for a later phase. The one-byte
configuration is an observation value, not a physical setting or protocol.

No recurrence, automatic work loop, recovery, physical output, application
event API, or scheduled event reporting is added. The terminal error path
handles a cooperative returned ordinary-work error only. Existing adapter
tests separately cover malformed input, full/unavailable routing, host output
saturation, and retained output. This sample does not repeat every fault case.

Five [sample tests](../../tests/host_sample.rs) use the same driver and assert
the field values above, including the concrete error source chain and equality
of two complete fresh reports. This is controlled-input repeatability, not a
timing, resource exhaustion, panic/hang, or fault-tolerance guarantee.

The combined demonstration supports the executable host-sample release gate.
CI execution, RFF-REQ-001 entry-point review, dependency review, and the human
v0.1 architecture review remain separate gates.
