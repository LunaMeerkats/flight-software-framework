//! Combined experimental host sample under controlled manual time.
//! No flight, safety, NASA, TRL, real-time, fault-tolerance, cFS, cFE, OSAL,
//! PSP, CCSDS, or RTOS compatibility claim.

#![forbid(unsafe_code)]

use std::error::Error;
use std::io::{self, Write};

mod mission;
mod sample;

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = io::stdout().lock();
    writeln!(
        output,
        "Experimental host sample; not flight-qualified or safety-certified."
    )?;
    writeln!(
        output,
        "No NASA affiliation, demonstrated TRL, operational suitability, or compatibility."
    )?;
    let report = sample::run()?;
    // Diagnostics follow the complete scenario; write failure cannot undo it.
    for (value, record) in [0, 42, 100].into_iter().zip(report.telemetry) {
        writeln!(
            output,
            "command [01, {value:02X}] -> telemetry {record:02X?}"
        )?;
    }
    writeln!(
        output,
        "Sample trace (elapsed simulated time; no physical delivery):"
    )?;
    writeln!(output, "{report:#?}")?;
    Ok(())
}
