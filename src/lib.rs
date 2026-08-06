#![forbid(unsafe_code)]
#![doc = r#"
Experimental host-side building blocks for the Rust Flight Framework.

The crate is not flight-qualified, safety-certified, NASA-affiliated, or
suitable for operational or safety-critical use. Its APIs and behavior are
pre-v0.1 experiments and make no real-time, fault-tolerance, cFS, CCSDS, or RTOS
compatibility claim.
"#]

mod lifecycle;
mod runtime;

pub use lifecycle::{
    ApplicationId, ApplicationState, LifecycleError, LifecycleOperation, LifecycleRegistry,
    RegistrationError, RegistryCreateError,
};
pub use runtime::{
    Application, Runtime, RuntimeCreateError, RuntimeRegistrationError, RuntimeStartError,
    RuntimeStopError,
};
