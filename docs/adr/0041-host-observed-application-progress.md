# ADR-0041: Host observation of application progress

- Status: Accepted for the Stage 5 consumer
- Date: 2026-11-13
- Scope: RFF-LIB-REQ-016, revision 1

## Context

A sampler can return successfully on every scheduled turn while its input
stays empty. Lifecycle state reports whether work is admissible, and the
[work error path](0016-returned-work-failure-events.md) reports errors that
the callback returns. The host also needs to recognize when a mission-defined
output has stayed absent across completed turns.

## Decision

Use a borrowed capacity-one sample slot. After each work call, take the slot
contents once. Use a fresh sample only after successful work. The serial host
owns one manual clock for all observations and starts the window after startup.
On success, a sample resets the elapsed-time baseline and re-arms the warning.
The first success without a sample at or after the threshold attempts one
warning, even if the event queue is full. Return the exact warning and queue
outcome to the host. App state stays Running.

Delegate work to `Runtime::work_with_failure_event`. An error returns through
the existing typed path before the host checks the sample threshold. A
returned application error commits Failed and attempts one Error event. A
lifecycle rejection yields no event attempt. Both diagnostics use the selected
application ID and the same clock origin.

## Alternatives and consequences

- Count successful callbacks or inspect lifecycle state. Both expose framework
  activity. Neither can establish that the sampler produced a fresh reading.
- Convert elapsed silence into a returned application error. This would stop
  the application at the first overdue observation even when the input could
  recover on a later turn. This consumer needs a diagnostic while Running.
- Observe the sample at the host boundary. The host can choose a window and
  distinguish optional diagnostics from application failures with current APIs.
  Its observation runs after the callback returns.
- Monitor work from a separate execution context. That requires independent
  progress, isolation and recovery decisions under [ADR-0001](0001-caller-driven-host-runtime.md).
  The selected serial consumer demonstrates overdue output on completed turns.

The [source register](../research/SOURCES.md#src-rff-host-progress--completed-turn-observation)
records upstream context and Rust primitives. The consumer retains bounded
state and performs no scheduling or recovery. A full queue rejects the new
warning and the turn result exposes that decision. Consider a reusable
monitor API when another consumer demonstrates shared policy. Revisit execution
isolation when observation must continue during a callback that never returns.
