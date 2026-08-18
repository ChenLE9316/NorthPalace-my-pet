use serde::Serialize;
use tauri::Manager;

use crate::{
    domain::{memory::MemoryKind, pet::PetInteraction},
    history_admin::{ActivityHistoryRecord, HistoryAdminService},
    memory_admin::{MemoryAdminService, MemoryInput, MemoryRecord},
    privacy::{PrivacyPolicyService, PrivacyRulesSnapshot},
    runtime::{PetRuntimeSnapshot, RuntimeHandle},
    screen_context::{ScreenContextBroker, ScreenContextSnapshot},
    worker::{WorkerStatus, WorkerSupervisor},
};

const MEMORY_LIST_LIMIT: u32 = 50;
const ACTIVITY_LIST_LIMIT: u32 = 40;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct StartupStatus {
    supported: bool,
    enabled: bool,
}

#[tauri::command]
pub(crate) fn get_pet_snapshot(
    runtime: tauri::State<'_, RuntimeHandle>,
) -> Result<PetRuntimeSnapshot, String> {
    runtime.snapshot()
}

#[tauri::command]
pub(crate) fn pet_interact(
    kind: PetInteraction,
    runtime: tauri::State<'_, RuntimeHandle>,
) -> Result<(), String> {
    runtime.dispatch(kind.into_event())
}

#[tauri::command]
pub(crate) fn worker_status_get(
    supervisor: tauri::State<'_, WorkerSupervisor>,
) -> Vec<WorkerStatus> {
    supervisor.snapshot()
}

#[tauri::command]
pub(crate) fn memory_list(
    kind: Option<MemoryKind>,
    limit: Option<u32>,
    memory: tauri::State<'_, MemoryAdminService>,
) -> Result<Vec<MemoryRecord>, String> {
    memory.list(kind, limit.unwrap_or(MEMORY_LIST_LIMIT))
}

#[tauri::command]
pub(crate) fn memory_search(
    query: String,
    limit: Option<u32>,
    memory: tauri::State<'_, MemoryAdminService>,
) -> Result<Vec<MemoryRecord>, String> {
    memory.search(&query, limit.unwrap_or(MEMORY_LIST_LIMIT))
}

#[tauri::command]
pub(crate) fn memory_create(
    input: MemoryInput,
    memory: tauri::State<'_, MemoryAdminService>,
) -> Result<i64, String> {
    memory.create(input)
}

#[tauri::command]
pub(crate) fn memory_update(
    id: i64,
    input: MemoryInput,
    memory: tauri::State<'_, MemoryAdminService>,
) -> Result<(), String> {
    memory.update(id, input)
}

#[tauri::command]
pub(crate) fn memory_delete(
    id: i64,
    memory: tauri::State<'_, MemoryAdminService>,
) -> Result<(), String> {
    memory.delete(id)
}

#[tauri::command]
pub(crate) fn activity_list(
    limit: Option<u32>,
    history: tauri::State<'_, HistoryAdminService>,
) -> Result<Vec<ActivityHistoryRecord>, String> {
    history.list(limit.unwrap_or(ACTIVITY_LIST_LIMIT))
}

#[tauri::command]
pub(crate) fn activity_get(
    id: i64,
    history: tauri::State<'_, HistoryAdminService>,
) -> Result<Option<ActivityHistoryRecord>, String> {
    history.get(id)
}

#[tauri::command]
pub(crate) fn privacy_get(
    privacy: tauri::State<'_, PrivacyPolicyService>,
) -> PrivacyRulesSnapshot {
    privacy.snapshot()
}

#[tauri::command]
pub(crate) fn privacy_add_excluded_app(
    app_id: String,
    privacy: tauri::State<'_, PrivacyPolicyService>,
) -> Result<PrivacyRulesSnapshot, String> {
    privacy.add_excluded_app(&app_id)
}

#[tauri::command]
pub(crate) fn privacy_remove_excluded_app(
    app_id: String,
    privacy: tauri::State<'_, PrivacyPolicyService>,
) -> Result<PrivacyRulesSnapshot, String> {
    privacy.remove_excluded_app(&app_id)
}

#[tauri::command]
pub(crate) fn privacy_set_accessibility_context_enabled(
    enabled: bool,
    privacy: tauri::State<'_, PrivacyPolicyService>,
) -> Result<PrivacyRulesSnapshot, String> {
    privacy.set_accessibility_context_enabled(enabled)
}

#[tauri::command]
pub(crate) fn screen_context_get(
    screen_context: tauri::State<'_, ScreenContextBroker>,
) -> ScreenContextSnapshot {
    screen_context.snapshot()
}

#[tauri::command]
pub(crate) fn startup_get(app: tauri::AppHandle) -> Result<StartupStatus, String> {
    #[cfg(target_os = "windows")]
    {
        use tauri_plugin_autostart::ManagerExt;

        let enabled = app
            .autolaunch()
            .is_enabled()
            .map_err(|error| format!("failed to read Windows startup registration: {error}"))?;
        return Ok(StartupStatus {
            supported: true,
            enabled,
        });
    }

    #[cfg(not(target_os = "windows"))]
    {
        let _ = app;
        Ok(StartupStatus {
            supported: false,
            enabled: false,
        })
    }
}

#[tauri::command]
pub(crate) fn startup_set(
    enabled: bool,
    app: tauri::AppHandle,
) -> Result<StartupStatus, String> {
    #[cfg(target_os = "windows")]
    {
        use tauri_plugin_autostart::ManagerExt;

        let manager = app.autolaunch();
        if enabled {
            manager
                .enable()
                .map_err(|error| format!("failed to enable Windows startup: {error}"))?;
        } else {
            manager
                .disable()
                .map_err(|error| format!("failed to disable Windows startup: {error}"))?;
        }

        let actual = manager
            .is_enabled()
            .map_err(|error| format!("failed to verify Windows startup registration: {error}"))?;
        return Ok(StartupStatus {
            supported: true,
            enabled: actual,
        });
    }

    #[cfg(not(target_os = "windows"))]
    {
        let _ = app;
        if enabled {
            Err("Windows startup registration is unavailable on this platform".to_owned())
        } else {
            Ok(StartupStatus {
                supported: false,
                enabled: false,
            })
        }
    }
}

#[tauri::command]
pub(crate) fn toggle_companion_window(app: tauri::AppHandle) -> Result<bool, String> {
    let window = app
        .get_webview_window("companion")
        .ok_or_else(|| "companion window is unavailable".to_owned())?;

    let visible = window.is_visible().map_err(|error| error.to_string())?;
    if visible {
        window.hide().map_err(|error| error.to_string())?;
        Ok(false)
    } else {
        window.show().map_err(|error| error.to_string())?;
        window.set_focus().map_err(|error| error.to_string())?;
        Ok(true)
    }
}

#[tauri::command]
pub(crate) fn hide_companion_window(app: tauri::AppHandle) -> Result<(), String> {
    let window = app
        .get_webview_window("companion")
        .ok_or_else(|| "companion window is unavailable".to_owned())?;
    window.hide().map_err(|error| error.to_string())
}

#[cfg(target_os = "windows")]
#[tauri::command]
pub(crate) fn get_display_context(
    webview_window: tauri::WebviewWindow,
) -> Result<crate::platform::windows::DisplayContext, String> {
    crate::platform::windows::read_display_context(&webview_window)
}

#[cfg(target_os = "windows")]
#[tauri::command]
pub(crate) fn configure_pet_hit_regions(
    regions: Vec<crate::platform::windows::CursorHitRegion>,
    hit_test: tauri::State<'_, crate::platform::windows::CursorHitTestHandle>,
) -> Result<(), String> {
    hit_test.set_regions(regions)
}
