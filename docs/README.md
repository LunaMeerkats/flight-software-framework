# Rust Flight Framework documentation

[Get started](GETTING_STARTED.md) by running the quickstart and using the library from your own Cargo project. Run `cargo doc --locked --no-deps --open` from the framework checkout to browse the API reference.

## Build an application

- [Compose independent applications](COMPOSING_APPLICATIONS.md) with typed errors and bounded messaging.
- [Run a command session](COMMAND_SESSION.md) with framed input, reply routing, and host output.
- [Retry selected subscribers](SELECTIVE_PUBLICATION.md), [send replies from a callback](CALLBACK_PUBLICATION.md), or [correlate retries and cancellations](CORRELATED_COMMANDS.md).

## Schedule and observe work

- [Release recurring work](PERIODIC_WORK.md) using an injected clock.
- [Transfer ordinary-work output](ORDINARY_WORK_OUTPUT.md) through a bounded slot and account for backpressure.
- [Attribute diagnostics to an application](APPLICATION_WORK_IDENTITY.md) and [check for overdue samples](HOST_PROGRESS.md).
- [Filter optional events](EVENT_FILTERING.md) or [write queued events to a host stream](EVENT_OUTPUT.md).
- [Run the combined host sample](verification/HOST_SAMPLE.md) for lifecycle, configuration, scheduling, and a returned work failure.

## Explore the project

- [Project state](PROJECT_STATE.md) and [roadmap](ROADMAP.md) show the current milestone and planned work.
- [Architecture](ARCHITECTURE.md) and [design decisions](adr/) explain how the framework is organised.
- [Requirements](REQUIREMENTS.md) and [verification traceability](verification/TRACEABILITY.md) connect behaviour to its checks.
- [Contributing](../CONTRIBUTING.md) covers local checks, issue reports and pull requests.
