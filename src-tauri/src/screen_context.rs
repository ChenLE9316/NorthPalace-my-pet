use std::{
    sync::{Arc, RwLock},
    time::{SystemTime, UNIX_EPOCH},
};

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
    pub active_app_observed_at_ms: Option<i64>,
    pub accessibility_state: AccessibilityContextState,
    pub accessibility: Option<AccessibilityContext>,
    pub accessibility_observed_at_ms: Option<i64>,
    pub user_idle_ms: u64,
    pub user_idle_observed_at_ms: Option<i64>,
    pub local_hour: u8,
    pub local_hour_observed_at_ms: Option<i64>,
    pub sequence: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ScreenContextState {
    active_app_id: Option<String>,
    active_app_state: ActiveAppContextState,
    active_window_bounds: Option<WindowBounds>,
    active_app_observed_at_ms: Option<i64>,
    accessibility_state: AccessibilityContextState,
    accessibility: Option<AccessibilityContext>,
    accessibility_observed_at_ms: Option<i64>,
    user_idle_ms: u64,
    user_idle_observed_at_ms: Option<i64>,
    local_hour: u8,
    local_hour_observed_at_ms: Option<i64>,
    sequence: u64,
}

impl Default for ScreenContextState {
    fn default() -> Self {
        Self {
            active_app_id: None,
            active_app_state: ActiveAppContextState::Unknown,
            active_window_bounds: None,
            active_app_observed_at_ms: None,
            accessibility_state: AccessibilityContextState::Disabled,
            accessibility: None,
            accessibility_observed_at_ms: None,
            user_idle_ms: 0,
            user_idle_observed_at_ms: None,
            local_hour: 12,
            local_hour_observed_at_ms: None,
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
            active_app_observed_at_ms: state.active_app_observed_at_ms,
            accessibility_state: state.accessibility_state,
            accessibility: state.accessibility,
            accessibility_observed_at_ms: state.accessibility_observed_at_ms,
            user_idle_ms: state.user_idle_ms,
            user_idle_observed_at_ms: state.user_idle_observed_at_ms,
            local_hour: state.local_hour,
            local_hour_observed_at_ms: state.local_hour_observed_at_ms,
            sequence: state.sequence,
        }
    }

    pub fn observe_active_app(&self, app_id: String, bounds: Option<WindowBounds>) {
        let observed_at_ms = now_ms();
        let Ok(mut state) = self.inner.write() else {
            return;
        };

        let app_changed = state.active_app_state != ActiveAppContextState::Available
            || state.active_app_id.as_deref() != Some(app_id.as_str());
        let changed = app_changed || state.active_window_bounds != bounds;

        if app_changed {
            if state.accessibility_state != AccessibilityContextState::Disabled {
                state.accessibility_state = AccessibilityContextState::Unavailable;
            }
            state.accessibility = None;
            state.accessibility_observed_at_ms = None;
        }

        state.active_app_id = Some(app_id);
        state.active_app_state = ActiveAppContextState::Available;
        state.active_window_bounds = bounds;
        state.active_app_observed_at_ms = Some(observed_at_ms);
        if changed {
            state.sequence = state.sequence.wrapping_add(1);
        }
    }

    pub fn observe_active_app_blocked(&self) {
        let observed_at_ms = now_ms();
        let Ok(mut state) = self.inner.write() else {
            return;
        };
        let changed = state.active_app_id.is_some()
            || state.active_app_state != ActiveAppContextState::PrivacyBlocked
            || state.active_window_bounds.is_some()
            || state.accessibility_state != AccessibilityContextState::PrivacyBlocked
            || state.accessibility.is_some();

        state.active_app_id = None;
        state.active_app_state = ActiveAppContextState::PrivacyBlocked;
        state.active_window_bounds = None;
        state.active_app_observed_at_ms = Some(observed_at_ms);
        state.accessibility_state = AccessibilityContextState::PrivacyBlocked;
        state.accessibility = None;
        state.accessibility_observed_at_ms = Some(observed_at_ms);
        if changed {
            state.sequence = state.sequence.wrapping_add(1);
        }
    }

    pub fn clear_active_app(&self) {
        let observed_at_ms = now_ms();
        let Ok(mut state) = self.inner.write() else {
            return;
        };
        let next_accessibility_state =
            if state.accessibility_state == AccessibilityContextState::Disabled {
                AccessibilityContextState::Disabled
            } else {
                AccessibilityContextState::Unavailable
            };
        let changed = state.active_app_id.is_some()
            || state.active_app_state != ActiveAppContextState::Unknown
            || state.active_window_bounds.is_some()
            || state.accessibility_state != next_accessibility_state
            || state.accessibility.is_some();

        state.active_app_id = None;
        state.active_app_state = ActiveAppContextState::Unknown;
        state.active_window_bounds = None;
        state.active_app_observed_at_ms = Some(observed_at_ms);
        state.accessibility_state = next_accessibility_state;
        state.accessibility = None;
        state.accessibility_observed_at_ms = None;
        if changed {
            state.sequence = state.sequence.wrapping_add(1);
        }
    }

    pub fn observe_accessibility_disabled(&self) {
        self.observe_accessibility_state(AccessibilityContextState::Disabled);
    }

    pub fn observe_accessibility_unavailable(&self) {
        self.observe_accessibility_state(AccessibilityContextState::Unavailable);
    }

    pub fn observe_accessibility_blocked(&self) {
        self.observe_accessibility_state(AccessibilityContextState::PrivacyBlocked);
    }

    pub fn observe_accessibility_for_app(
        &self,
        app_id: &str,
        context: Option<AccessibilityContext>,
    ) {
        let observed_at_ms = now_ms();
        let Ok(mut state) = self.inner.write() else {
            return;
        };
        if state.active_app_state != ActiveAppContextState::Available
            || state.active_app_id.as_deref() != Some(app_id)
        {
            return;
        }

        let next_state = if context.is_some() {
            AccessibilityContextState::Available
        } else {
            AccessibilityContextState::Unavailable
        };
        let changed = state.accessibility_state != next_state || state.accessibility != context;
        state.accessibility_state = next_state;
        state.accessibility = context;
        state.accessibility_observed_at_ms = Some(observed_at_ms);
        if changed {
            state.sequence = state.sequence.wrapping_add(1);
        }
    }

    pub fn observe_user_idle(&self, idle_ms: u64) {
        let observed_at_ms = now_ms();
        if let Ok(mut state) = self.inner.write() {
            let changed = state.user_idle_ms != idle_ms;
            state.user_idle_ms = idle_ms;
            state.user_idle_observed_at_ms = Some(observed_at_ms);
            if changed {
                state.sequence = state.sequence.wrapping_add(1);
            }
        }
    }

    pub fn observe_local_hour(&self, hour: u8) {
        let observed_at_ms = now_ms();
        if let Ok(mut state) = self.inner.write() {
            let hour = hour.min(23);
            let changed = state.local_hour != hour;
            state.local_hour = hour;
            state.local_hour_observed_at_ms = Some(observed_at_ms);
            if changed {
                state.sequence = state.sequence.wrapping_add(1);
            }
        }
    }

    fn observe_accessibility_state(&self, requested_state: AccessibilityContextState) {
        let observed_at_ms = now_ms();
        if let Ok(mut state) = self.inner.write() {
            let next_state = if state.active_app_state == ActiveAppContextState::PrivacyBlocked {
                AccessibilityContextState::PrivacyBlocked
            } else {
                requested_state
            };
            let changed = state.accessibility_state != next_state || state.accessibility.is_some();
            state.accessibility_state = next_state;
            state.accessibility = None;
            state.accessibility_observed_at_ms = Some(observed_at_ms);
            if changed {
                state.sequence = state.sequence.wrapping_add(1);
            }
        }
    }
}

fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        .min(i64::MAX as u128) as i64
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
        assert!(snapshot.active_app_observed_at_ms.is_some());
        assert!(snapshot.accessibility_observed_at_ms.is_some());
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
    fn switching_apps_invalidates_previous_accessibility_context_and_freshness() {
        let broker = ScreenContextBroker::default();
        broker.observe_active_app("code".to_owned(), Some(example_bounds()));
        broker.observe_accessibility_for_app("code", Some(example_accessibility()));
        broker.observe_active_app("notepad".to_owned(), Some(example_bounds()));

        let snapshot = broker.snapshot();
        assert_eq!(snapshot.active_app_id.as_deref(), Some("notepad"));
        assert_eq!(snapshot.accessibility, None);
        assert_eq!(snapshot.accessibility_observed_at_ms, None);
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
        assert_eq!(snapshot.accessibility_observed_at_ms, None);
    }

    #[test]
    fn disabled_accessibility_survives_active_app_heartbeats() {
        let broker = ScreenContextBroker::default();
        broker.observe_accessibility_disabled();
        broker.observe_active_app("code".to_owned(), Some(example_bounds()));
        broker.clear_active_app();
        assert_eq!(
            broker.snapshot().accessibility_state,
            AccessibilityContextState::Disabled
        );
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
    fn repeated_heartbeat_refreshes_timestamp_without_advancing_sequence() {
        let broker = ScreenContextBroker::default();
        broker.observe_active_app("code".to_owned(), Some(example_bounds()));
        let first = broker.snapshot();
        broker.observe_active_app("code".to_owned(), Some(example_bounds()));
        let second = broker.snapshot();

        assert!(second.active_app_observed_at_ms.is_some());
        assert_eq!(second.sequence, first.sequence);
    }

    #[test]
    fn structured_signals_include_freshness_without_screen_pixels() {
        let broker = ScreenContextBroker::default();
        broker.observe_user_idle(42_000);
        broker.observe_local_hour(25);
        let snapshot = broker.snapshot();
        assert_eq!(snapshot.user_idle_ms, 42_000);
        assert_eq!(snapshot.local_hour, 23);
        assert!(snapshot.user_idle_observed_at_ms.is_some());
        assert!(snapshot.local_hour_observed_at_ms.is_some());
        assert_eq!(snapshot.sequence, 2);
    }
}
