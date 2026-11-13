# Contributing

Start with the [quickstart](README.md#run-the-quickstart).

- For a bug, include the commit, operating system, `rustc --version`, the command you ran, and what you expected and observed. [GitHub Issues](https://github.com/LunaMeerkats/flight-software-framework/issues)
- For a feature request, describe the application you are building and where the current API gets in the way.
- For a pull request, explain the problem, keep the change focused, and report the checks you ran. If public behaviour changes, update the relevant guide and show how a caller can verify it.

Keep the root README focused on the quickstart and integration entry points. Link detailed usage from the [guide index](docs/README.md) and record verification in [the project records](docs/verification/).

## Run the checks

Use stable Rust with Cargo, rustfmt and Clippy. If you use rustup, install the components with `rustup component add rustfmt clippy`. Run these checks from the repository root.

```text
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
git diff --check
```

Build the API docs with warnings denied. In PowerShell, run
`$env:RUSTDOCFLAGS = '-D warnings'`, then
`cargo doc --locked --workspace --all-features --no-deps`. In a POSIX shell,
run `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --all-features --no-deps`.

The [repository guidance](AGENTS.md) lists checks for specific examples and subsystems. GitHub Actions runs the [Windows host workflow](.github/workflows/ci.yml), including the shared adapter tests and the combined host example.
