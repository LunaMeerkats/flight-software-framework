#![forbid(unsafe_code)]
#![doc = r#"
Experimental host-side building blocks for the Rust Flight Framework.

The crate is not flight-qualified, safety-certified, NASA-affiliated, or
suitable for operational spacecraft, safety-critical, or human-rated use.
No Technology Readiness Level is demonstrated. Its APIs and behavior are
pre-v0.1 experiments and make no real-time or fault-tolerance claim, or claim
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
    Event, EventEmitOutcome, EventQueue, EventQueueCreateError, EventSeverity, EventSource,
    EventTimestamp,
};
pub use lifecycle::{
    ApplicationId, ApplicationState, LifecycleError, LifecycleOperation, LifecycleRegistry,
    RegistrationError, RegistryCreateError,
};
pub use messaging::{
    ApplicationInboxConfig, DeliveryStatus, DestinationOutcome, InboxAccessError, Message,
    MessageBus, MessageBusCreateError, MessageCreateError, PublishClassification, PublishError,
    PublishReport, RuntimeInboxConfig,
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
    ScheduledWork, ScheduledWorkError, ScheduledWorkOutcome, WorkSchedule, WorkScheduleCreateError,
};
