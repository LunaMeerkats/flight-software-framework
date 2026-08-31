# Configuration work-context borrowing experiment

Date: **2026-09-01**
Status: **Probes verified; production integration unimplemented**

## Question and limits

Can a host owner mutably invoke one application while lending it immutable
configuration bytes and the original acceptance revision from a separate
field? Can the callback vocabulary omit the table's error type and const bound?

These executable Markdown snippets support
[ADR-0018](../adr/0018-configuration-aware-work-context.md). They use the actual
`ConfigurationTable` but a deliberately minimal one-application probe owner,
not `Runtime` or a new framework API. They do not exercise lifecycle gates,
fallible callbacks, scheduling, events, or messaging. They cannot complete
RFF-REQ-006. No production source or public callback changes in this checkpoint.
This is experimental host-only evidence, with no flight-readiness, safety,
real-time, or compatibility claim.

The Rust Reference describes struct fields as separately borrowable; the
rustdoc book describes executable and compile-fail examples. Those language
facts inform this local experiment, not the runtime architecture itself.
(`SRC-RUST-FIELD-BORROWS`, `SRC-RUSTDOC-TESTS`)

## Reproduce

Run from the repository root in PowerShell, using stable Rust:

```powershell
cargo build --workspace --all-features
if ($LASTEXITCODE -ne 0) { throw 'Library build failed' }
$probeLibrary = 'rust_flight_framework=target/debug/librust_flight_framework.rlib'
rustdoc --test --edition 2024 -D warnings --extern $probeLibrary -L dependency=target/debug/deps docs/verification/CONFIGURATION_CONTEXT_EXPERIMENT.md
if ($LASTEXITCODE -ne 0) { throw 'Configuration context probes failed' }
```

Cargo does not discover standalone Markdown probes. Run the rustdoc command
explicitly when this decision or experiment changes. A passing compile-fail
example proves only rejection; inspect its compiler diagnostic as well to
exclude unrelated errors such as an unresolved import. No stable error-code
matching capability is assumed for rustdoc.

## Executable split-borrow probe

One callback receives `None` on an unconfigured owner and a borrowed view on a
configured owner. Moving a table with existing history preserves its revision
and rollback slot; later replacement is visible on the next callback. The
application retains only copied observations, not borrowed storage. The probe
has no lifecycle or error-handling implementation to mistake for evidence.

```rust
use rust_flight_framework::ConfigurationTable;

struct ConfigurationView<'a> {
    revision: u64,
    bytes: &'a [u8],
}

struct WorkContext<'a> {
    configuration: Option<ConfigurationView<'a>>,
}

trait ProbeApplication {
    fn work(&mut self, context: WorkContext<'_>);
}

struct Observer {
    seen: Option<(u64, u8)>,
}

struct Owner<A, E, const MAX_BYTES: usize> {
    application: A,
    configuration: Option<ConfigurationTable<E, MAX_BYTES>>,
}

impl ProbeApplication for Observer {
    fn work(&mut self, context: WorkContext<'_>) {
        self.seen = context
            .configuration
            .map(|view| (view.revision, view.bytes[0]));
    }
}

impl<A: ProbeApplication, E, const MAX_BYTES: usize> Owner<A, E, MAX_BYTES> {
    fn work(&mut self) {
        let (application, configuration) = (&mut self.application, &self.configuration);
        let configuration = configuration.as_ref().map(|table| {
            let snapshot = table.active();
            ConfigurationView {
                revision: snapshot.revision(),
                bytes: snapshot.bytes(),
            }
        });
        application.work(WorkContext { configuration });
    }
}

fn main() {
    let mut table = ConfigurationTable::<(), 1>::new(&[10], |_| Ok(())).unwrap();
    assert_eq!(table.replace(&[20]), Ok(2));
    let mut configured = Owner {
        application: Observer { seen: None },
        configuration: Some(table),
    };
    configured.work();
    assert_eq!(configured.application.seen, Some((2, 20)));
    assert_eq!(configured.configuration.as_mut().unwrap().rollback(), Ok(1));
    configured.work();
    assert_eq!(configured.application.seen, Some((1, 10)));
    assert_eq!(
        configured.configuration.as_mut().unwrap().replace(&[30]),
        Ok(3)
    );
    configured.work();
    assert_eq!(configured.application.seen, Some((3, 30)));

    let mut unconfigured = Owner::<_, String, 0> {
        application: Observer {
            seen: Some((99, 99)),
        },
        configuration: None,
    };
    unconfigured.work();
    assert_eq!(unconfigured.application.seen, None);
}
```

The private probe exposes its fields to make the field split visible. The
proposed production owner must keep the table private: returning mutable table
access would permit whole-table replacement and reset the revision lineage.
The probe observer's byte indexing is confined to its nonempty fixtures; it is
not a proposed general configuration decoder or validator.

## Read-only bytes reject mutation

Expected rejection: E0594, assignment through an immutable byte slice.

```compile_fail
use rust_flight_framework::ConfigurationTable;

fn main() {
    let table = ConfigurationTable::<(), 1>::new(&[10], |_| Ok(())).unwrap();
    let bytes = table.active().bytes();
    bytes[0] = 20;
}
```

## A live view prevents replacement

Expected rejection: E0502, a mutable table borrow overlaps its still-used
immutable view. Reading the view after replacement is intentional here.

```compile_fail
use rust_flight_framework::ConfigurationTable;

fn main() {
    let mut table = ConfigurationTable::<(), 1>::new(&[10], |_| Ok(())).unwrap();
    let bytes = table.active().bytes();
    table.replace(&[20]).unwrap();
    assert_eq!(bytes, &[10]);
}
```

## Borrowed storage cannot escape its owner lifetime

Expected rejection: the input borrow cannot be returned as a static borrow.
This does not prevent an application from making and retaining its own copy.

```compile_fail
use rust_flight_framework::ConfigurationTable;

fn escape(table: &ConfigurationTable<(), 1>) -> &'static [u8] {
    table.active().bytes()
}

fn main() {
    let table = ConfigurationTable::<(), 1>::new(&[10], |_| Ok(())).unwrap();
    assert_eq!(escape(&table), &[10]);
}
```

## Outcome and implementation gate

On Rust/rustdoc 1.98.0, all four rustdoc probes passed: one executable case and
three expected compile failures. Independently compiling extracted snippets
confirmed E0594 for mutation, E0502 for conflicting replacement, and the
diagnostic `lifetime may not live long enough` for static escape, without
unrelated errors. The positive probe also compiled and ran directly with
warnings denied and unsafe code forbidden, and passed Clippy with the existing
60-line threshold. This adds no production tests to the unchanged suite of 73.

The production integration must separately verify configured and absent
contexts, rejection and rollback visibility, restart retention, original
errors, peer progress, clock/event counts, schedule consumption, and messaging
inbox cleanup. The unchanged
production suite alone cannot establish any new configuration access.
