# Rust Flight Framework

A Rust library for building and testing flight-software-style applications on a host. Use it to manage application lifecycles, pass messages through bounded queues, and exercise scheduled work with a clock you control. Your program decides when each callback runs.

The library is experimental, with a provisional API. Use it for host development and testing. Flight qualification and operational use are outside its current scope. [Scope and limitations](docs/CHARTER.md#safety-maturity-and-affiliation)

## Run the quickstart

Install Git and [stable Rust](https://doc.rust-lang.org/book/ch01-01-installation.html). Clone the project and run the example.

```text
git clone https://github.com/LunaMeerkats/flight-software-framework.git
cd flight-software-framework
cargo run --locked --example quickstart
```

The example fills a one-message inbox, shows how a second command is rejected, and dispatches the original command.

## Use it in your project

The crate is unpublished. If your Cargo package sits beside the framework checkout, add a path dependency to its `Cargo.toml`.

```toml
[dependencies]
rust-flight-framework = { path = "../flight-software-framework" }
```

- [Getting started](docs/GETTING_STARTED.md) walks through the application code, message handling and setup errors.
- [Examples and guides](docs/README.md) cover scheduling, configuration, events and a complete command and telemetry example.
- Run `cargo doc --locked --no-deps --open` from the framework checkout to browse the API.

## Development

See [Contributing](CONTRIBUTING.md) for local checks and how to report an issue or send a patch. [Windows CI](https://github.com/LunaMeerkats/flight-software-framework/actions/workflows/ci.yml)

## Licence

[MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE), at your option. Copyright 2026 Daniel Smith.
