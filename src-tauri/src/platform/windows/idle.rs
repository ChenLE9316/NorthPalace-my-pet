use std::{thread, time::Duration};

use crate::{
    domain::events::DomainEvent,
    runtime::RuntimeHandle,
    screen_context::ScreenContextBroker,
};

#[repr(C)]
struct LastInputInfo {
    cb_size: u32,
    dw_time: u32,
}

#[link(name = "user32")]
unsafe extern "system" {
    fn GetLastInputInfo(info: *mut LastInputInfo) -> i32;
}

#[link(name = "kernel32")]
unsafe extern "system" {
    fn GetTickCount() -> u32;
}

fn system_idle_ms() -> Option<u64> {
    let mut info = LastInputInfo {
        cb_size: std::mem::size_of::<LastInputInfo>() as u32,
        dw_time: 0,
    };

    let ok = unsafe { GetLastInputInfo(&mut info) };
    if ok == 0 {
        return None;
    }

    let now = unsafe { GetTickCount() };
    Some(now.wrapping_sub(info.dw_time) as u64)
}

pub fn spawn_idle_sensor(
    runtime: RuntimeHandle,
    screen_context: ScreenContextBroker,
) {
    thread::spawn(move || {
        let mut previous_idle_ms = 0_u64;

        loop {
            let Some(idle_ms) = system_idle_ms() else {
                thread::sleep(Duration::from_secs(2));
                continue;
            };

            screen_context.observe_user_idle(idle_ms);

            if previous_idle_ms >= 10_000 && idle_ms <= 1_500 {
                if runtime.dispatch(DomainEvent::UserReturned).is_err() {
                    break;
                }
            }

            if runtime
                .dispatch(DomainEvent::UserIdleChanged { idle_ms })
                .is_err()
            {
                break;
            }

            previous_idle_ms = idle_ms;
            thread::sleep(Duration::from_secs(1));
        }
    });
}

#[cfg(test)]
mod tests {
    #[test]
    fn wrapping_tick_math_matches_windows_tick_semantics() {
        let before = u32::MAX - 500;
        let after = 499_u32;
        assert_eq!(after.wrapping_sub(before), 1_000);
    }
}
