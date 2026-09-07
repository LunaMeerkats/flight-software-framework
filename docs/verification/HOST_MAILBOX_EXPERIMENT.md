# Borrowed host telemetry mailbox experiment

Date: **2026-09-08**
Status: **Ownership and backpressure probe verified; adapters not implemented**

## Question and limits

Can `MessagingRuntime` own a telemetry application that borrows a host-owned
capacity-one mailbox, while the host drains that mailbox between callbacks
without dropping the runtime? What happens when a callback finds the mailbox
full?

This executable probe supports the ownership and backpressure decision in
[ADR-0019](../adr/0019-host-command-telemetry-boundary.md). It uses the
actual runtime and its existing dispatch/error behavior. The application
borrows `Cell<Option<u8>>`; no `Rc`, interior borrow guard, extra queue,
allocation for mailbox sharing, or new framework API is needed. Rust permits
mutation through a shared `Cell` reference; copying one optional byte does not
introduce a runtime borrow-check failure. (`SRC-RUST-CELL`)

This is a single-threaded mission-composition experiment, not a reusable
mailbox abstraction. Host code owns the cell and can also overwrite it; its
responsibility to drain only between callbacks is a composition rule. `Cell`
does not provide a concurrent channel, automatic delivery, or wall-clock
latency guarantee. The host's copied or drained values remain outside runtime
inbox bounds.

The probe publishes one-byte internal telemetry payloads directly. It neither
implements the external command/telemetry codec nor proves that malformed host
commands are rejected before business logic. It does not verify RFF-REQ-007 or
the complete sample mission. There is no host I/O inside the callback or the
probe. This remains experimental host-only evidence without flight-readiness,
safety, real-time, or compatibility claims.

The experiment gives its single telemetry application two inbox slots so the
full-mailbox scenario can observe one queued delivery being cleared after the
attempted delivery is removed. ADR-0019 instead selects one slot per inbox for
the eventual two-application mission; that topology would report zero remaining
queued discards for the same serial mailbox-full failure. The larger fixture
does not change the selected mission bounds.

## Reproduce

Run from the repository root in PowerShell using stable Rust:

```powershell
cargo build --workspace --all-features
if ($LASTEXITCODE -ne 0) { throw 'Library build failed' }
$mailboxLibrary = 'rust_flight_framework=target/debug/librust_flight_framework.rlib'
rustdoc --test --edition 2024 --deny warnings --forbid unsafe_code --extern $mailboxLibrary -L dependency=target/debug/deps docs/verification/HOST_MAILBOX_EXPERIMENT.md
if ($LASTEXITCODE -ne 0) { throw 'Host mailbox probe failed' }
```

Cargo does not discover standalone Markdown probes. The single executable
Rust block below contains three independently named scenarios. Assertions are
probe expectations, not error handling proposed for production adapters.

To audit the exact snippet with the repository's stable formatting and
60-line Clippy configuration, extract it into the ignored target directory:

```powershell
$mailboxDocument = Get-Content docs/verification/HOST_MAILBOX_EXPERIMENT.md -Raw
$mailboxSnippet = [regex]::Match($mailboxDocument, '(?ms)^```rust\r?\n(.*?)^```')
if (-not $mailboxSnippet.Success) { throw 'Rust probe was not found' }
$mailboxProbeDirectory = 'target/nightly-2026-09-08'
New-Item -ItemType Directory -Force -Path $mailboxProbeDirectory | Out-Null
$mailboxProbePath = "$mailboxProbeDirectory/host_mailbox_probe.rs"
Set-Content -Path $mailboxProbePath -Value $mailboxSnippet.Groups[1].Value -NoNewline -Encoding utf8
rustfmt --check --edition 2024 --config-path rustfmt.toml $mailboxProbePath
if ($LASTEXITCODE -ne 0) { throw 'Host mailbox source formatting failed' }
$previousClippyDirectory = $env:CLIPPY_CONF_DIR
try {
    $env:CLIPPY_CONF_DIR = (Get-Location).Path
    clippy-driver --edition 2024 --deny warnings --deny clippy::too_many_lines --forbid unsafe_code --extern $mailboxLibrary -L dependency=target/debug/deps --emit metadata -o "$mailboxProbeDirectory/host_mailbox_probe.rmeta" $mailboxProbePath
    if ($LASTEXITCODE -ne 0) { throw 'Host mailbox source lint failed' }
} finally {
    $env:CLIPPY_CONF_DIR = $previousClippyDirectory
}
```

Review physical and comment-only Rust widths separately; successful rustfmt
does not prove those limits. The extraction is a convenience for this probe,
not an adopted repository-wide checker.

## Executable ownership and backpressure probe

The mailbox admits exactly one validated percent value. Internal payload
validation precedes the capacity check. On full storage, the application
preserves the older record and returns a concrete error containing the rejected
value. Existing runtime policy then commits terminal `Failed` and clears the
selected application's remaining inbox. Draining the retained output does not
recover that application.

```rust
#![forbid(unsafe_code)]

use std::cell::Cell;
use std::convert::Infallible;
use std::error::Error;
use std::fmt;

use rust_flight_framework::{
    Application, ApplicationId, ApplicationMessageContext, ApplicationState,
    ApplicationWorkContext, LifecycleError, Message, MessageDispatchError, MessageDispatchOutcome,
    MessagingApplication, MessagingRuntime, PublishClassification, Runtime, RuntimeInboxConfig,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MissionTopic {
    Telemetry,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TelemetryError {
    InvalidPayload,
    MailboxFull { rejected: u8 },
}

struct TelemetryApplication<'a> {
    mailbox: &'a Cell<Option<u8>>,
}

type TelemetryRuntime<'a> = MessagingRuntime<TelemetryApplication<'a>, MissionTopic, 1>;

impl fmt::Display for TelemetryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPayload => formatter.write_str("invalid telemetry payload"),
            Self::MailboxFull { rejected } => {
                write!(formatter, "full telemetry mailbox rejected {rejected}")
            }
        }
    }
}

impl Error for TelemetryError {}

impl Application for TelemetryApplication<'_> {
    type StartError = Infallible;
    type WorkError = Infallible;
    type StopError = Infallible;
    type RestartError = Infallible;

    fn start(&mut self) -> Result<(), Self::StartError> {
        Ok(())
    }

    fn work(&mut self, _context: ApplicationWorkContext<'_>) -> Result<(), Self::WorkError> {
        Ok(())
    }

    fn stop(&mut self) -> Result<(), Self::StopError> {
        Ok(())
    }

    fn restart(&mut self) -> Result<(), Self::RestartError> {
        Ok(())
    }
}

impl MessagingApplication<MissionTopic, 1> for TelemetryApplication<'_> {
    type MessageError = TelemetryError;

    fn handle_message(
        &mut self,
        message: &Message<MissionTopic, 1>,
        _context: &mut ApplicationMessageContext<'_, MissionTopic, 1>,
    ) -> Result<(), Self::MessageError> {
        let value = match message.payload() {
            [value] if *value <= 100 => *value,
            _ => return Err(TelemetryError::InvalidPayload),
        };
        if self.mailbox.get().is_some() {
            return Err(TelemetryError::MailboxFull { rejected: value });
        }
        self.mailbox.set(Some(value));
        Ok(())
    }
}

fn main() {
    host_drains_and_reuses_mailbox_while_runtime_lives();
    full_mailbox_preserves_output_and_fails_subscriber();
    invalid_payload_precedes_capacity_check();
}

fn host_drains_and_reuses_mailbox_while_runtime_lives() {
    let mailbox = Cell::new(None);
    let (mut runtime, application_id) = running_telemetry(&mailbox);
    assert_eq!(mailbox.take(), None);

    for value in [0, 100] {
        publish_payload(&mut runtime, &[value]);
        assert_eq!(
            runtime.dispatch_one(application_id),
            Ok(MessageDispatchOutcome::Dispatched)
        );
        assert_eq!(mailbox.take(), Some(value));
        assert_eq!(mailbox.get(), None);
        assert_eq!(runtime.pending(application_id), Ok(0));
        assert_eq!(runtime.state(application_id), Ok(ApplicationState::Running));
    }
    assert_eq!(mailbox.take(), None);
    assert_eq!(
        runtime.dispatch_one(application_id),
        Ok(MessageDispatchOutcome::InboxEmpty)
    );
}

fn full_mailbox_preserves_output_and_fails_subscriber() {
    let mailbox = Cell::new(None);
    let (mut runtime, application_id) = running_telemetry(&mailbox);
    publish_payload(&mut runtime, &[11]);
    runtime.dispatch_one(application_id).unwrap();
    publish_payload(&mut runtime, &[22]);
    publish_payload(&mut runtime, &[33]);
    assert_eq!(runtime.pending(application_id), Ok(2));

    let error = runtime.dispatch_one(application_id).unwrap_err();
    assert_eq!(
        error.operation_error(),
        &MessageDispatchError::Application {
            application_id,
            source: TelemetryError::MailboxFull { rejected: 22 },
        }
    );
    assert_eq!(error.discarded_deliveries(), 1);
    assert_eq!(mailbox.get(), Some(11));
    assert_eq!(runtime.pending(application_id), Ok(0));
    assert_eq!(runtime.state(application_id), Ok(ApplicationState::Failed));

    assert_eq!(mailbox.take(), Some(11));
    let rejected = runtime.dispatch_one(application_id).unwrap_err();
    assert_eq!(
        rejected.operation_error(),
        &MessageDispatchError::Lifecycle(LifecycleError::NotRunning {
            application_id,
            state: ApplicationState::Failed,
        })
    );
    assert_eq!(rejected.discarded_deliveries(), 0);
    assert_eq!(mailbox.get(), None);
}

fn invalid_payload_precedes_capacity_check() {
    for payload in [&[][..], &[101][..]] {
        let mailbox = Cell::new(Some(7));
        let (mut runtime, application_id) = running_telemetry(&mailbox);
        publish_payload(&mut runtime, payload);

        let error = runtime.dispatch_one(application_id).unwrap_err();
        assert_eq!(
            error.operation_error(),
            &MessageDispatchError::Application {
                application_id,
                source: TelemetryError::InvalidPayload,
            }
        );
        assert_eq!(error.discarded_deliveries(), 0);
        assert_eq!(mailbox.get(), Some(7));
        assert_eq!(runtime.pending(application_id), Ok(0));
        assert_eq!(runtime.state(application_id), Ok(ApplicationState::Failed));
    }
}

fn running_telemetry(mailbox: &Cell<Option<u8>>) -> (TelemetryRuntime<'_>, ApplicationId) {
    let mut runtime = Runtime::new(1).unwrap();
    let application_id = runtime.register(TelemetryApplication { mailbox }).unwrap();
    let configuration = RuntimeInboxConfig::new(2, &[MissionTopic::Telemetry]);
    let mut runtime = MessagingRuntime::new(runtime, &[configuration]).unwrap();
    runtime.start(application_id).unwrap();
    (runtime, application_id)
}

fn publish_payload(runtime: &mut TelemetryRuntime<'_>, payload: &[u8]) {
    let message = Message::try_new(MissionTopic::Telemetry, payload).unwrap();
    let report = runtime.publish(&message).unwrap();
    assert_eq!(report.classification(), PublishClassification::Complete);
}
```

## Outcome and implementation gate

On Rust/rustdoc 1.98.0, the standalone rustdoc command passed one executable
probe containing all three scenarios:

- the host drains the mailbox and the same live runtime dispatches again;
- exact `0` and `100` percent values pass internal payload validation;
- full storage retains the older record, returns the rejected value, commits
  `Failed`, and clears exactly one remaining queued delivery;
- draining the retained record leaves the failed application ineligible; and
- empty and out-of-range internal payloads return their concrete validation
  error before capacity handling and preserve the preexisting mailbox record.

The extracted 185-line Rust snippet passed rustfmt 1.9.0-stable and Clippy
0.1.98 with warnings denied, unsafe code forbidden, and the repository's
60-line threshold. Its physical-line audit found zero lines above 100 columns
and zero comment-only lines above 80 columns. No lint exception was added.

Production acceptance still requires the selected external grammar, typed
validation before publication, the echo business operation, telemetry encoding,
complete publication-result handling, and the two-application command/telemetry
integration cases in ADR-0019. This probe is ownership and backpressure evidence
only.
