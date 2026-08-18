use std::sync::{Arc, RwLock};

use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ActiveAppContextState {
    Unknown,
    Available,
    PrivacyBlocked,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AccessibilityContextState {
    Disabled,
    Available,
    Unavailable,
    PrivacyBlocked,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct WindowBounds {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilityContext {
    pub control_type_id: Option<i32>,
    pub is_enabled: Option<bool>,
    pub is_keyboard_focusable: Option<bool>,
    pub has_keyboard_focus: Option<bool>,
    pub is_offscreen: Option<bool>,
    pub is_password: Option<bool>,
    pub bounds: Option<WindowBounds>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ScreenContextSnapshot {
    pub active_app_id: Option<String>,
    pub active_app_state: ActiveAppContextState,
    pub active_window_bounds: Option<WindowBounds>,
    pub accessibility_state: AccessibilityContextState,
    pub accessibility: Option<AccessibilityContext>,
    pub user_idle_ms: u64,
    pub local_hour: u8,
    pub sequence: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ScreenContextState {
    active_app_id: Option<String>,
    active_app_state: ActiveAppContextState,
    active_window_bounds: Option<WindowBounds>,
    accessibility_state: AccessibilityContextState,
    accessibility: Option<AccessibilityContext>,
    user_idle_ms: u64,
    local_hour: u8,
    sequence: u64,
}

impl Default for ScreenContextState {
    fn default() -> Self {
        Self {
            active_app_id: None,
            active_app_state: ActiveAppContextState::Unknown,
            active_window_bounds: None,
            accessibility_state: AccessibilityContextState::Disabled,
            accessibility: None,
            user_idle_ms: 0,
            local_hour: 12,
            sequence: 0,
        }
    }
}

#[derive(Clone, Default)]
pub struct ScreenContextBroker {
    inner: Arc<RwLock<ScreenContextState>>,
}

impl ScreenContextBroker {
    pub fn snapshot(&self) -> ScreenContextSnapshot {
        let state = self
            .inner
            .read()
            .map(|state| state.clone())
            .unwrap_or_default();
        ScreenContextSnapshot {
            active_app_id: state.active_app_id,
            active_app_state: state.active_app_state,
            active_window_bounds: state.active_window_bounds,
            accessibility_state: state.accessibility_state,
            accessibility: state.accessibility,
            user_idle_ms: state.user_idle_ms,
            local_hour: state.local_hour,
            sequence: state.sequence,
        }
    }

    pub fn observe_active_app(
        &self,
        app_id: String,
        bounds: Option<WindowBounds>,
    ) {
        if let Ok(mut state) = self.inner.write() {
            let app_changed = state.active_app_state != ActiveAppContextState::Available
                || state.active_app_id.as_deref() != Some(app_id.as_str());
            if app_changed {
                state.accessibility_state = AccessibilityContextState::Unavailable;
                state.accessibility = None;
            }
            state.active_app_id = Some(app_id);
            state.active_app_state = ActiveAppContextState::Available;
            state.active_window_bounds = bounds;
            state.sequence = state.sequence.wrapping_add(1);
        }
    }

    pub fn observe_active_app_blocked(&self) {
        self.update(|state| {
            state.active_app_id = None;
            state.active_app_state = ActiveAppContextState::PrivacyBlocked;
            state.active_window_bounds = None;
            state.accessibility_state = AccessibilityContextState::PrivacyBlocked;
            state.accessibility = None;
        });
    }

    pub fn clear_active_app(&self) {
        self.update(|state| {
            state.active_app_id = None;
            state.active_app_state = ActiveAppContextState::Unknown;
            state.active_window_bounds = None;
            state.accessibility_state = AccessibilityContextState::Unavailable;
            state.accessibility = None;
        });
    }

    pub fn observe_accessibility_disabled(&self) {
        self.update(|state| {
            if state.active_app_state == ActiveAppContextState::PrivacyBlocked {
                state.accessibility_state = AccessibilityContextState::PrivacyBlocked;
            } else {
                state.accessibility_state = AccessibilityContextState::Disabled;
            }
            state.accessibility = None;
        });
    }

    pub fn observe_accessibility_unavailable(&self) {
        self.update(|state| {
            if state.active_app_state == ActiveAppContextState::PrivacyBlocked {
                state.accessibility_state = AccessibilityContextState::PrivacyBlocked;
            } else {
                state.accessibility_state = AccessibilityContextState::Unavailable;
            }
            state.accessibility = None;
        });
    }

    pub fn observe_accessibility_blocked(&self) {
        self.update(|state| {
            state.accessibility_state = AccessibilityContextState::PrivacyBlocked;
            state.accessibility = None;
        });
    }

    pub fn observe_accessibility_for_app(
        &self,
        app_id: &str,
        context: Option<AccessibilityContext>,
    ) {
        let Ok(mut state) = self.inner.write() else {
            return;
        };
        if state.active_app_state != ActiveAppContextState::Available
            || state.active_app_id.as_deref() != Some(app_id)
        {
            return;
        }

        state.accessibility_state = if context.is_some() {
            AccessibilityContextState::Available
        } else {
            AccessibilityContextState::Unavailable
        };
        state.accessibility = context;
        state.sequence = state.sequence.wrapping_add(1);
    }

    pub fn observe_user_idle(&self, idle_ms: u64) {
        self.update(|state| {
            state.user_idle_ms = idle_ms;
        });
    }

    pub fn observe_local_hour(&self, hour: u8) {
        self.update(|state| {
            state.local_hour = hour.min(23);
        });
    }

    fn update(&self, update: impl FnOnce(&mut ScreenContextState)) {
        if let Ok(mut state) = self.inner.write() {
            update(&mut state);
            state.sequence = state.sequence.wrapping_add(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_bounds() -> WindowBounds {
        WindowBounds {
            x: 120,
            y: 80,
            width: 1280,
            height: 720,
        }
    }

    fn example_accessibility() -> AccessibilityContext {
        AccessibilityContext {
            control_type_id: Some(50004),
            is_enabled: Some(true),
            is_keyboard_focusable: Some(true),
            has_keyboard_focus: Some(true),
            is_offscreen: Some(false),
            is_password: Some(false),
            bounds: Some(WindowBounds {
                x: 180,
                y: 140,
                width: 420,
                height: 36,
            }),
        }
    }

    #[test]
    fn privacy_block_clears_previous_app_identity_bounds_and_accessibility() {
        let broker = ScreenContextBroker::default();
        broker.observe_active_app("code".to_owned(), Some(example_bounds()));
        broker.observe_accessibility_for_app("code", Some(example_accessibility()));
        assert!(broker.snapshot().accessibility.is_some());

        broker.observe_active_app_blocked();
        let snapshot = broker.snapshot();
        assert_eq!(snapshot.active_app_id, None);
        assert_eq!(snapshot.active_window_bounds, None);
        assert_eq!(snapshot.accessibility, None);
        assert_eq!(
            snapshot.active_app_state,
            ActiveAppContextState::PrivacyBlocked
        );
        assert_eq!(
            snapshot.accessibility_state,
            AccessibilityContextState::PrivacyBlocked
        );
    }

    #[test]
    fn switching_apps_invalidates_previous_accessibility_context() {
        let broker = ScreenContextBroker::default();
        broker.observe_active_app("code".to_owned(), Some(example_bounds()));
        broker.observe_accessibility_for_app("code", Some(example_accessibility()));
        broker.observe_active_app("notepad".to_owned(), Some(example_bounds()));

        let snapshot = broker.snapshot();
        assert_eq!(snapshot.active_app_id.as_deref(), Some("notepad"));
        assert_eq!(snapshot.accessibility, None);
        assert_eq!(
            snapshot.accessibility_state,
            AccessibilityContextState::Unavailable
        );
    }

    #[test]
    fn stale_accessibility_result_for_previous_app_is_ignored() {
        let broker = ScreenContextBroker::default();
        broker.observe_active_app("notepad".to_owned(), Some(example_bounds()));
        broker.observe_accessibility_for_app("code", Some(example_accessibility()));
        let snapshot = broker.snapshot();
        assert_eq!(snapshot.accessibility, None);
    }

    #[test]
    fn accessibility_disabled_does_not_override_privacy_block() {
        let broker = ScreenContextBroker::default();
        broker.observe_active_app_blocked();
        broker.observe_accessibility_disabled();
        assert_eq!(
            broker.snapshot().accessibility_state,
            AccessibilityContextState::PrivacyBlocked
        );
    }

    #[test]
    fn accessibility_unavailable_does_not_override_privacy_block() {
        let broker = ScreenContextBroker::default();
        broker.observe_active_app_blocked();
        broker.observe_accessibility_unavailable();
        assert_eq!(
            broker.snapshot().accessibility_state,
            AccessibilityContextState::PrivacyBlocked
        );
    }

    #[test]
    fn structured_signals_update_without_screen_pixels() {
        let broker = ScreenContextBroker::default();
        broker.observe_user_idle(42_000);
        broker.observe_local_hour(25);
        let snapshot = broker.snapshot();
        assert_eq!(snapshot.user_idle_ms, 42_000);
        assert_eq!(snapshot.local_hour, 23);
        assert_eq!(snapshot.sequence, 2);
    }
}