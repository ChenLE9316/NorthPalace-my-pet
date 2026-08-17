use std::{thread, time::Duration};

use crate::{
    domain::events::DomainEvent,
    runtime::RuntimeHandle,
    screen_context::ScreenContextBroker,
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
) {
    thread::spawn(move || {
        let mut previous_hour: Option<u8> = None;

        loop {
            let hour = local_hour();
            if previous_hour != Some(hour) {
                screen_context.observe_local_hour(hour);
                if runtime
                    .dispatch(DomainEvent::TimeOfDayChanged { hour })
                    .is_err()
                {
                    break;
                }
                previous_hour = Some(hour);
            }

            thread::sleep(Duration::from_secs(30));
        }
    });
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
