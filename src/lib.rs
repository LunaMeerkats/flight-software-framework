#![forbid(unsafe_code)]
#![doc = r#"
Experimental host-side building blocks for the Rust Flight Framework.

The development goal is a practical, reusable Rust library. The reviewed host
v0.1 baseline is a starting point for continued evidence-led development.

Start with `cargo run --locked --example quickstart` in the repository. The
`docs/GETTING_STARTED.md` guide explains using this unpublished library as a
path dependency in an independent Cargo package. The quickstart uses public
APIs only and shows explicit lifecycle, bounded ingress and one-message dispatch.

`cargo run --locked --example composition` and `docs/COMPOSING_APPLICATIONS.md`
extend that recipe to two independently implemented application types with
mission-owned enum delegation, typed errors and bounded peer publication.

`cargo run --locked --example work_output` and `docs/ORDINARY_WORK_OUTPUT.md`
show configured ordinary work staging a bounded output for explicit host
publication and retry to one subscriber, including finite scheduled work.

`docs/SELECTIVE_PUBLICATION.md` shows host-selected publication to one subscribed
destination and a bounded acceptance ledger for retries after partial fan-out.
The caller retries only rejected destinations; enqueue acceptance does not
establish processing or retention after a later stop or failure.

`docs/PERIODIC_WORK.md` shows a host-owned recurring release cursor composed
with the existing direct and messaging work boundaries. Polling releases at
most one item; the host explicitly invokes work and chooses error policy.

`docs/EVENT_FILTERING.md` shows explicit severity admission for optional event
records, with distinct suppression, recording and queue-saturation outcomes.

The crate is not flight-qualified, safety-certified, NASA-affiliated, or
suitable for operational spacecraft, safety-critical, or human-rated use.
No Technology Readiness Level is demonstrated. Its APIs and behavior are
provisional and make no real-time or fault-tolerance claim, or claim
of compatibility with cFS, cFE, OSAL, PSP, CCSDS, or an RTOS.
"#]

mod application_messaging;
mod clock;
mod configuration;
mod events;
mod lifecycle;
mod messaging;
mod messaging_runtime;
mod runtime;
mod runtime_events;
mod scheduling;

pub use application_messaging::{
    ApplicationMessageContext, MessageDispatchError, MessageDispatchOutcome, MessagingApplication,
};
pub use clock::{Clock, FrameworkInstant, ManualClock, ManualClockAdvanceError};
pub use configuration::{ConfigurationError, ConfigurationSnapshot, ConfigurationTable};
pub use events::{
    Event, EventEmitOutcome, EventFilter, EventQueue, EventQueueCreateError, EventSeverity,
    EventSource, EventTimestamp, FilteredEventOutcome,
};
pub use lifecycle::{
    ApplicationId, ApplicationState, LifecycleError, LifecycleOperation, LifecycleRegistry,
    RegistrationError, RegistryCreateError,
};
pub use messaging::{
    ApplicationInboxConfig, DeliveryStatus, DestinationOutcome, InboxAccessError, Message,
    MessageBus, MessageBusCreateError, MessageCreateError, PublishClassification, PublishError,
    PublishReport, PublishToError, RuntimeInboxConfig,
};
pub use messaging_runtime::{
    MessagingOperationError, MessagingRuntime, MessagingRuntimeCreateError,
    MessagingRuntimeCreateErrorKind, MessagingStopOutcome,
};
pub use runtime::{
    Application, ApplicationConfigurationView, ApplicationWorkContext, Runtime,
    RuntimeConfigurationCreateError, RuntimeConfigurationError, RuntimeCreateError,
    RuntimeRegistrationError, RuntimeRestartError, RuntimeStartError, RuntimeStopError,
    RuntimeWorkError,
};
pub use runtime_events::{FailureEventAttempt, RuntimeWorkEventError};
pub use scheduling::{
    PeriodicWork, PeriodicWorkCreateError, PeriodicWorkOutcome, ScheduledWork, ScheduledWorkError,
    ScheduledWorkOutcome, WorkSchedule, WorkScheduleCreateError,
};
