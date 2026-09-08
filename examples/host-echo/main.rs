//! Bounded host command/telemetry demonstration, not the full v0.1 sample.
//! Experimental host software; no flight, safety, NASA, TRL, real-time,
//! fault-tolerance, cFS, cFE, OSAL, PSP, CCSDS, or RTOS compatibility claim.

#![forbid(unsafe_code)]

use std::error::Error;
use std::io::{self, Write};

use rust_flight_framework::{MessageDispatchOutcome, PublishClassification};

mod mission;

use mission::applications::OutputMailbox;
use mission::{compose, ingest};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = io::stdout().lock();
    writeln!(
        output,
        "Experimental host-only echo boundary; not flight-qualified or safety-certified."
    )?;
    writeln!(
        output,
        "No NASA affiliation, demonstrated TRL, operational suitability, or compatibility."
    )?;
    let mailbox = OutputMailbox::default();
    let mut mission = compose(&mailbox)?;
    mission.runtime.start(mission.echo_id)?;
    mission.runtime.start(mission.telemetry_id)?;

    for value in [0, 42, 100] {
        let report = ingest(&mut mission.runtime, &[0x01, value])?;
        if report.classification() != PublishClassification::Complete {
            return Err(format!("command was not fully queued: {report:?}").into());
        }
        for application_id in [mission.echo_id, mission.telemetry_id] {
            if mission.runtime.dispatch_one(application_id)? != MessageDispatchOutcome::Dispatched {
                return Err("expected one queued message for explicit dispatch".into());
            }
        }
        let record = mailbox.drain().ok_or("expected one host output record")?;
        // This diagnostic write occurs after drain; failure cannot undo work.
        writeln!(
            output,
            "command [01, {value:02X}] -> telemetry {record:02X?}"
        )?;
    }
    Ok(())
}
