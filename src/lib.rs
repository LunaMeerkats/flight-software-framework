#![forbid(unsafe_code)]
#![doc = r#"
Experimental host-side building blocks for the Rust Flight Framework.

The crate is not flight-qualified, safety-certified, NASA-affiliated, or
suitable for operational or safety-critical use. Its APIs and behavior are
pre-v0.1 experiments and make no real-time, fault-tolerance, cFS, CCSDS, or RTOS
compatibility claim.
"#]

mod application_messaging;
mod clock;
mod events;
mod lifecycle;
mod messaging;
mod messaging_runtime;
mod runtime;
mod scheduling;

pub use application_messaging::{
    ApplicationMessageContext, MessageDispatchError, MessageDispatchOutcome, MessagingApplication,
};
pub use clock::{Clock, FrameworkInstant, ManualClock, ManualClockAdvanceError};
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
    Application, Runtime, RuntimeCreateError, RuntimeRegistrationError, RuntimeRestartError,
    RuntimeStartError, RuntimeStopError, RuntimeWorkError,
};
pub use scheduling::{
    ScheduledWork, ScheduledWorkError, ScheduledWorkOutcome, WorkSchedule, WorkScheduleCreateError,
};
