use std::sync::{Arc, RwLock};

use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ActiveAppContextState {
    Unknown,
    Available,
    PrivacyBlocked,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ScreenContextSnapshot {
    pub active_app_id: Option<String>,
    pub active_app_state: ActiveAppContextState,
    pub user_idle_ms: u64,
    pub local_hour: u8,
    pub sequence: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ScreenContextState {
    active_app_id: Option<String>,
    active_app_state: ActiveAppContextState,
    user_idle_ms: u64,
    local_hour: u8,
    sequence: u64,
}

impl Default for ScreenContextState {
    fn default() -> Self {
        Self {
            active_app_id: None,
            active_app_state: ActiveAppContextState::Unknown,
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
            user_idle_ms: state.user_idle_ms,
            local_hour: state.local_hour,
            sequence: state.sequence,
        }
    }

    pub fn observe_active_app(&self, app_id: String) {
        self.update(|state| {
            state.active_app_id = Some(app_id);
            state.active_app_state = ActiveAppContextState::Available;
        });
    }

    pub fn observe_active_app_blocked(&self) {
        self.update(|state| {
            state.active_app_id = None;
            state.active_app_state = ActiveAppContextState::PrivacyBlocked;
        });
    }

    pub fn clear_active_app(&self) {
        self.update(|state| {
            state.active_app_id = None;
            state.active_app_state = ActiveAppContextState::Unknown;
        });
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

    #[test]
    fn privacy_block_clears_previous_app_identity() {
        let broker = ScreenContextBroker::default();
        broker.observe_active_app("code".to_owned());
        assert_eq!(broker.snapshot().active_app_id.as_deref(), Some("code"));

        broker.observe_active_app_blocked();
        let snapshot = broker.snapshot();
        assert_eq!(snapshot.active_app_id, None);
        assert_eq!(
            snapshot.active_app_state,
            ActiveAppContextState::PrivacyBlocked
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
