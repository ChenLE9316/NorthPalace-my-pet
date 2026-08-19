use std::time::Duration;

use crate::{
    domain::events::DomainEvent, runtime::RuntimeHandle, screen_context::ScreenContextBroker,
    worker::WorkerSupervisor,
};

#[repr(C)]
#[derive(Default)]
struct SystemTime {
    year: u16,
    month: u16,
    day_of_week: u16,
    day: u16,
    hour: u16,
    minute: u16,
    second: u16,
    milliseconds: u16,
}

#[link(name = "kernel32")]
unsafe extern "system" {
    fn GetLocalTime(system_time: *mut SystemTime);
}

fn normalize_hour(hour: u16) -> u8 {
    hour.min(23) as u8
}

fn local_hour() -> u8 {
    let mut time = SystemTime::default();
    unsafe { GetLocalTime(&mut time) };
    normalize_hour(time.hour)
}

pub fn spawn_local_time_sensor(
    runtime: RuntimeHandle,
    screen_context: ScreenContextBroker,
    supervisor: &WorkerSupervisor,
) -> Result<(), String> {
    supervisor.spawn("windows-local-time", move |token| {
        let mut previous_hour: Option<u8> = None;

        while !token.is_cancelled() {
            let hour = local_hour();
            // Refresh observation freshness every sensor pass while keeping Domain Events
            // hour-change-only and Screen Context sequence semantic-change-only.
            screen_context.observe_local_hour(hour);

            if previous_hour != Some(hour) {
                runtime.dispatch(DomainEvent::TimeOfDayChanged { hour })?;
                previous_hour = Some(hour);
            }

            if token.wait_timeout(Duration::from_secs(30)) {
                break;
            }
        }

        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hour_normalization_never_exceeds_domain_range() {
        assert_eq!(normalize_hour(0), 0);
        assert_eq!(normalize_hour(23), 23);
        assert_eq!(normalize_hour(999), 23);
    }
}
