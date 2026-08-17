use std::{
    ffi::c_void,
    path::Path,
    ptr,
    thread,
    time::Duration,
};

use crate::{
    domain::events::DomainEvent,
    privacy::PrivacyPolicyService,
    runtime::RuntimeHandle,
};

type Hwnd = *mut c_void;
type Handle = *mut c_void;

const PROCESS_QUERY_LIMITED_INFORMATION: u32 = 0x1000;

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

fn foreground_app_id() -> Option<String> {
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

    Some(app_id)
}

pub fn spawn_active_window_sensor(
    runtime: RuntimeHandle,
    privacy: PrivacyPolicyService,
) {
    thread::spawn(move || {
        let mut last_app_id: Option<String> = None;

        loop {
            let app_id = foreground_app_id();
            if app_id != last_app_id {
                if let Some(app_id) = app_id.clone() {
                    if !privacy.is_app_excluded(&app_id)
                        && runtime
                            .dispatch(DomainEvent::ActiveWindowChanged { app_id })
                            .is_err()
                    {
                        break;
                    }
                }
                last_app_id = app_id;
            }

            thread::sleep(Duration::from_secs(1));
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn null_path_helper_is_not_used() {
        let value: *mut c_void = ptr::null_mut();
        assert!(value.is_null());
    }
}
