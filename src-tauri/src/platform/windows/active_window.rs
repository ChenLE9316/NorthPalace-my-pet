use std::{
    ffi::c_void,
    path::Path,
    thread,
    time::Duration,
};

use crate::{
    domain::events::DomainEvent,
    privacy::PrivacyPolicyService,
    runtime::RuntimeHandle,
    screen_context::{ScreenContextBroker, WindowBounds},
};

type Hwnd = *mut c_void;
type Handle = *mut c_void;

const PROCESS_QUERY_LIMITED_INFORMATION: u32 = 0x1000;
const DWMWA_EXTENDED_FRAME_BOUNDS: u32 = 9;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct Rect {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}

pub(super) struct ForegroundApp {
    window: Hwnd,
    pub(super) process_id: u32,
    pub(super) app_id: String,
}

#[link(name = "user32")]
unsafe extern "system" {
    fn GetForegroundWindow() -> Hwnd;
    fn GetWindowThreadProcessId(window: Hwnd, process_id: *mut u32) -> u32;
}

#[link(name = "kernel32")]
unsafe extern "system" {
    fn OpenProcess(desired_access: u32, inherit_handle: i32, process_id: u32) -> Handle;
    fn QueryFullProcessImageNameW(
        process: Handle,
        flags: u32,
        exe_name: *mut u16,
        size: *mut u32,
    ) -> i32;
    fn CloseHandle(object: Handle) -> i32;
}

#[link(name = "dwmapi")]
unsafe extern "system" {
    fn DwmGetWindowAttribute(
        window: Hwnd,
        attribute: u32,
        value: *mut c_void,
        value_size: u32,
    ) -> i32;
}

struct OwnedHandle(Handle);

impl Drop for OwnedHandle {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe {
                CloseHandle(self.0);
            }
        }
    }
}

pub(super) fn foreground_app() -> Option<ForegroundApp> {
    let window = unsafe { GetForegroundWindow() };
    if window.is_null() {
        return None;
    }

    let mut process_id = 0_u32;
    unsafe {
        GetWindowThreadProcessId(window, &mut process_id);
    }
    if process_id == 0 {
        return None;
    }

    let process = unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, process_id) };
    if process.is_null() {
        return None;
    }
    let process = OwnedHandle(process);

    let mut buffer = vec![0_u16; 32_768];
    let mut size = buffer.len() as u32;
    let ok = unsafe {
        QueryFullProcessImageNameW(process.0, 0, buffer.as_mut_ptr(), &mut size)
    };
    if ok == 0 || size == 0 {
        return None;
    }

    let path = String::from_utf16_lossy(&buffer[..size as usize]);
    let app_id = Path::new(&path)
        .file_stem()
        .and_then(|name| name.to_str())
        .map(str::to_owned)
        .filter(|name| !name.is_empty())?;

    Some(ForegroundApp {
        window,
        process_id,
        app_id,
    })
}

fn visible_window_bounds(window: Hwnd) -> Option<WindowBounds> {
    let mut rect = Rect::default();
    let result = unsafe {
        DwmGetWindowAttribute(
            window,
            DWMWA_EXTENDED_FRAME_BOUNDS,
            (&mut rect as *mut Rect).cast::<c_void>(),
            std::mem::size_of::<Rect>() as u32,
        )
    };
    if result < 0 {
        return None;
    }
    bounds_from_rect(rect)
}

fn bounds_from_rect(rect: Rect) -> Option<WindowBounds> {
    let width = rect.right.checked_sub(rect.left)?;
    let height = rect.bottom.checked_sub(rect.top)?;
    if width <= 0 || height <= 0 {
        return None;
    }

    Some(WindowBounds {
        x: rect.left,
        y: rect.top,
        width: width as u32,
        height: height as u32,
    })
}

pub fn spawn_active_window_sensor(
    runtime: RuntimeHandle,
    privacy: PrivacyPolicyService,
    screen_context: ScreenContextBroker,
) {
    super::accessibility::spawn_accessibility_sensor(privacy.clone(), screen_context.clone());

    thread::spawn(move || {
        let mut last_app_id: Option<String> = None;
        let mut last_blocked = false;
        let mut last_bounds: Option<WindowBounds> = None;

        loop {
            let foreground = foreground_app();
            let app_id = foreground.as_ref().map(|app| app.app_id.clone());
            let blocked = app_id
                .as_deref()
                .map(|app_id| privacy.is_app_excluded(app_id))
                .unwrap_or(false);

            // Window geometry is a privacy-controlled capability. Never ask DWM for
            // bounds until the process app id has passed the exclusion gate.
            let bounds = if blocked {
                None
            } else {
                foreground
                    .as_ref()
                    .and_then(|app| visible_window_bounds(app.window))
            };

            let identity_changed = app_id != last_app_id || blocked != last_blocked;
            let context_changed = identity_changed || bounds != last_bounds;

            if context_changed {
                match app_id.clone() {
                    Some(_) if blocked => {
                        screen_context.observe_active_app_blocked();
                    }
                    Some(app_id) => {
                        screen_context.observe_active_app(app_id.clone(), bounds);
                        if identity_changed
                            && runtime
                                .dispatch(DomainEvent::ActiveWindowChanged { app_id })
                                .is_err()
                        {
                            break;
                        }
                    }
                    None => {
                        screen_context.clear_active_app();
                    }
                }

                last_app_id = app_id;
                last_blocked = blocked;
                last_bounds = bounds;
            }

            thread::sleep(Duration::from_secs(1));
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rect_to_bounds_preserves_negative_monitor_coordinates() {
        let bounds = bounds_from_rect(Rect {
            left: -1920,
            top: 100,
            right: 0,
            bottom: 1180,
        })
        .expect("valid bounds");
        assert_eq!(bounds.x, -1920);
        assert_eq!(bounds.y, 100);
        assert_eq!(bounds.width, 1920);
        assert_eq!(bounds.height, 1080);
    }

    #[test]
    fn invalid_rect_is_not_exposed_as_context() {
        assert!(bounds_from_rect(Rect {
            left: 10,
            top: 10,
            right: 10,
            bottom: 20,
        })
        .is_none());
    }
}
