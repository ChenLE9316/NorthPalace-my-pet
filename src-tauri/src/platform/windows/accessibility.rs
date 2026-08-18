use std::time::{Duration, Instant};

use windows::{
    Win32::{
        Foundation::RECT,
        System::Com::{
            CoCreateInstance, CoInitializeEx, CoUninitialize, CLSCTX_INPROC_SERVER,
            COINIT_MULTITHREADED,
        },
        UI::Accessibility::{CUIAutomation, IUIAutomation},
    },
};

use crate::{
    privacy::PrivacyPolicyService,
    screen_context::{AccessibilityContext, ScreenContextBroker, WindowBounds},
    worker::WorkerSupervisor,
};

use super::active_window::foreground_app;

const ACCESSIBILITY_POLL_INTERVAL: Duration = Duration::from_secs(2);
const ACCESSIBILITY_RETRY_INTERVAL: Duration = Duration::from_secs(10);

struct ComApartment;

impl ComApartment {
    fn initialize() -> Result<Self, String> {
        unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) }
            .ok()
            .map_err(|error| format!("failed to initialize COM for accessibility context: {error}"))?;
        Ok(Self)
    }
}

impl Drop for ComApartment {
    fn drop(&mut self) {
        unsafe { CoUninitialize() };
    }
}

struct AccessibilityReader {
    // Drop the COM interface before the apartment guard.
    automation: IUIAutomation,
    _apartment: ComApartment,
}

impl AccessibilityReader {
    fn new() -> Result<Self, String> {
        let apartment = ComApartment::initialize()?;
        let automation: IUIAutomation = unsafe {
            CoCreateInstance(&CUIAutomation, None, CLSCTX_INPROC_SERVER)
        }
        .map_err(|error| format!("failed to create Windows UI Automation client: {error}"))?;

        Ok(Self {
            automation,
            _apartment: apartment,
        })
    }

    fn read_focused_element(&self, expected_process_id: u32) -> Option<AccessibilityContext> {
        let element = unsafe { self.automation.GetFocusedElement() }.ok()?;
        let process_id = unsafe { element.CurrentProcessId() }.ok()?;
        if process_id < 0 || process_id as u32 != expected_process_id {
            return None;
        }

        let control_type_id = unsafe { element.CurrentControlType() }.ok().map(|value| value.0);
        let is_enabled = unsafe { element.CurrentIsEnabled() }
            .ok()
            .map(|value| value.as_bool());
        let is_keyboard_focusable = unsafe { element.CurrentIsKeyboardFocusable() }
            .ok()
            .map(|value| value.as_bool());
        let has_keyboard_focus = unsafe { element.CurrentHasKeyboardFocus() }
            .ok()
            .map(|value| value.as_bool());
        let is_offscreen = unsafe { element.CurrentIsOffscreen() }
            .ok()
            .map(|value| value.as_bool());
        let is_password = unsafe { element.CurrentIsPassword() }
            .ok()
            .map(|value| value.as_bool());
        let bounds = unsafe { element.CurrentBoundingRectangle() }
            .ok()
            .and_then(bounds_from_rect);

        Some(AccessibilityContext {
            control_type_id,
            is_enabled,
            is_keyboard_focusable,
            has_keyboard_focus,
            is_offscreen,
            is_password,
            bounds,
        })
    }
}

fn bounds_from_rect(rect: RECT) -> Option<WindowBounds> {
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

pub(super) fn spawn_accessibility_sensor(
    privacy: PrivacyPolicyService,
    screen_context: ScreenContextBroker,
    supervisor: &WorkerSupervisor,
) -> Result<(), String> {
    supervisor.spawn("windows-accessibility", move |token| {
        let mut reader: Option<AccessibilityReader> = None;
        let mut retry_after: Option<Instant> = None;

        while !token.is_cancelled() {
            let policy = privacy.snapshot();
            if policy.fail_closed {
                reader = None;
                retry_after = None;
                screen_context.observe_accessibility_blocked();
                if token.wait_timeout(ACCESSIBILITY_POLL_INTERVAL) {
                    break;
                }
                continue;
            }

            if !policy.accessibility_context_enabled {
                // Releasing the reader also releases COM/UI Automation resources while the
                // capability is explicitly disabled.
                reader = None;
                retry_after = None;
                screen_context.observe_accessibility_disabled();
                if token.wait_timeout(ACCESSIBILITY_POLL_INTERVAL) {
                    break;
                }
                continue;
            }

            let Some(foreground) = foreground_app() else {
                if token.wait_timeout(ACCESSIBILITY_POLL_INTERVAL) {
                    break;
                }
                continue;
            };

            if privacy.is_app_excluded(&foreground.app_id) {
                screen_context.observe_accessibility_blocked();
                if token.wait_timeout(ACCESSIBILITY_POLL_INTERVAL) {
                    break;
                }
                continue;
            }

            if reader.is_none() {
                let now = Instant::now();
                if retry_after.is_some_and(|deadline| now < deadline) {
                    screen_context.observe_accessibility_unavailable();
                    if token.wait_timeout(ACCESSIBILITY_POLL_INTERVAL) {
                        break;
                    }
                    continue;
                }

                match AccessibilityReader::new() {
                    Ok(new_reader) => {
                        reader = Some(new_reader);
                        retry_after = None;
                    }
                    Err(error) => {
                        eprintln!("Lenvu accessibility context unavailable: {error}");
                        retry_after = Some(now + ACCESSIBILITY_RETRY_INTERVAL);
                        screen_context.observe_accessibility_unavailable();
                        if token.wait_timeout(ACCESSIBILITY_POLL_INTERVAL) {
                            break;
                        }
                        continue;
                    }
                }
            }

            let context = reader
                .as_ref()
                .and_then(|reader| reader.read_focused_element(foreground.process_id));
            screen_context.observe_accessibility_for_app(&foreground.app_id, context);

            if token.wait_timeout(ACCESSIBILITY_POLL_INTERVAL) {
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
    fn accessibility_bounds_preserve_negative_monitor_coordinates() {
        let bounds = bounds_from_rect(RECT {
            left: -800,
            top: 25,
            right: -200,
            bottom: 425,
        })
        .expect("valid bounds");
        assert_eq!(bounds.x, -800);
        assert_eq!(bounds.y, 25);
        assert_eq!(bounds.width, 600);
        assert_eq!(bounds.height, 400);
    }

    #[test]
    fn invalid_accessibility_bounds_are_not_exposed() {
        assert!(bounds_from_rect(RECT {
            left: 10,
            top: 20,
            right: 10,
            bottom: 40,
        })
        .is_none());
    }
}
