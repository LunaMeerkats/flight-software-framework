//! Finite caller-owned polling against elapsed host time.
//!
//! Tests inject manual time into the same turn. Host waiting remains outside
//! the library and callbacks; five turns bound calls, not wall-clock duration.

#![forbid(unsafe_code)]

use std::convert::Infallible;
use std::error::Error;
use std::thread;
use std::time::Duration;

use rust_flight_framework::{
    Application, ApplicationWorkContext, Clock, FrameworkInstant, HostClock, PeriodicWork,
    PeriodicWorkOutcome, Runtime, RuntimeWorkError, ScheduledWork,
};

const POLL_TURNS: usize = 5;
const INTERVAL: Duration = Duration::from_millis(20);
const HOST_WAIT: Duration = Duration::from_millis(10);

#[derive(Default)]
struct Counter {
    work_calls: usize,
}

impl Application for Counter {
    type StartError = Infallible;
    type WorkError = Infallible;
    type StopError = Infallible;
    type RestartError = Infallible;

    fn start(&mut self) -> Result<(), Self::StartError> {
        Ok(())
    }

    fn work(&mut self, _context: ApplicationWorkContext<'_>) -> Result<(), Self::WorkError> {
        // This private consumer invokes at most one callback per finite turn.
        self.work_calls += 1;
        Ok(())
    }

    fn stop(&mut self) -> Result<(), Self::StopError> {
        Ok(())
    }

    fn restart(&mut self) -> Result<(), Self::RestartError> {
        Ok(())
    }
}

fn poll_turn(
    runtime: &mut Runtime<Counter>,
    releases: &mut PeriodicWork,
    clock: &dyn Clock,
) -> Result<PeriodicWorkOutcome, RuntimeWorkError<Infallible>> {
    let outcome = releases.poll(clock);
    if let PeriodicWorkOutcome::Released { scheduled_work, .. } = outcome {
        runtime.work(scheduled_work.application_id())?;
    }
    Ok(outcome)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut runtime = Runtime::new(1)?;
    let id = runtime.register(Counter::default())?;
    runtime.start(id)?;
    let clock = HostClock::new();
    let mut releases = PeriodicWork::new(ScheduledWork::new(id, FrameworkInstant::ZERO), INTERVAL)?;

    for turn in 0..POLL_TURNS {
        // On work rejection, propagate the exact error and end this host loop.
        poll_turn(&mut runtime, &mut releases, &clock)?;
        if turn + 1 < POLL_TURNS {
            thread::sleep(HOST_WAIT);
        }
    }
    let work_calls = runtime.application(id)?.work_calls;
    runtime.stop(id)?;
    println!("host polling: turns={POLL_TURNS} work_calls={work_calls} state=Stopped");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_flight_framework::{ApplicationState, ManualClock};

    fn setup() -> (Runtime<Counter>, PeriodicWork) {
        let mut runtime = Runtime::new(1).unwrap();
        let id = runtime.register(Counter::default()).unwrap();
        runtime.start(id).unwrap();
        let releases = PeriodicWork::new(ScheduledWork::new(id, instant(20)), INTERVAL).unwrap();
        (runtime, releases)
    }

    fn instant(milliseconds: u64) -> FrameworkInstant {
        FrameworkInstant::from_elapsed(Duration::from_millis(milliseconds))
    }

    #[test]
    fn controlled_turn_waits_then_runs_at_equality_without_duplicate_work() {
        let (mut runtime, mut releases) = setup();
        let id = releases.next_scheduled_work().unwrap().application_id();
        let mut clock = ManualClock::new();
        assert!(matches!(
            poll_turn(&mut runtime, &mut releases, &clock).unwrap(),
            PeriodicWorkOutcome::Waiting { observed_at, .. } if observed_at == instant(0)
        ));
        assert_eq!(runtime.application(id).unwrap().work_calls, 0);

        clock.advance(INTERVAL).unwrap();
        assert_eq!(
            poll_turn(&mut runtime, &mut releases, &clock),
            Ok(PeriodicWorkOutcome::Released {
                scheduled_work: ScheduledWork::new(id, instant(20)),
                observed_at: instant(20),
                next_scheduled_at: Some(instant(40)),
            })
        );
        assert!(matches!(
            poll_turn(&mut runtime, &mut releases, &clock).unwrap(),
            PeriodicWorkOutcome::Waiting { .. }
        ));
        assert_eq!(runtime.application(id).unwrap().work_calls, 1);
    }

    #[test]
    fn overdue_turn_coalesces_and_lifecycle_rejection_keeps_consumption() {
        let (mut runtime, mut releases) = setup();
        let id = releases.next_scheduled_work().unwrap().application_id();
        let mut clock = ManualClock::from_elapsed(Duration::from_millis(100));
        assert_eq!(
            poll_turn(&mut runtime, &mut releases, &clock),
            Ok(PeriodicWorkOutcome::Released {
                scheduled_work: ScheduledWork::new(id, instant(20)),
                observed_at: instant(100),
                next_scheduled_at: Some(instant(120)),
            })
        );
        runtime.stop(id).unwrap();
        clock.advance(INTERVAL).unwrap();
        let expected = runtime.work(id).unwrap_err();
        assert_eq!(
            poll_turn(&mut runtime, &mut releases, &clock),
            Err(expected)
        );
        assert_eq!(
            releases.next_scheduled_work(),
            Some(ScheduledWork::new(id, instant(140)))
        );
        assert_eq!(runtime.application(id).unwrap().work_calls, 1);
        assert_eq!(runtime.state(id), Ok(ApplicationState::Stopped));
    }

    #[test]
    fn live_clocks_support_finite_public_polling_without_timing_thresholds() {
        for clock in [HostClock::new(), HostClock::default()] {
            let (mut runtime, initial_releases) = setup();
            let id = initial_releases
                .next_scheduled_work()
                .unwrap()
                .application_id();
            let mut releases =
                PeriodicWork::new(ScheduledWork::new(id, FrameworkInstant::ZERO), INTERVAL)
                    .unwrap();
            for _ in 0..POLL_TURNS {
                poll_turn(&mut runtime, &mut releases, &clock).unwrap();
            }
            let calls = runtime.application(id).unwrap().work_calls;
            assert!((1..=POLL_TURNS).contains(&calls));
            runtime.stop(id).unwrap();
            assert_eq!(runtime.state(id), Ok(ApplicationState::Stopped));
        }
    }
}
