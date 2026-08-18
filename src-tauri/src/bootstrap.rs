use std::time::Duration;

use tauri::Manager;

use crate::{
    history_admin::HistoryAdminService,
    memory_admin::MemoryAdminService,
    persistence::{
        spawn_autosave, spawn_event_journal, PersistenceBootstrap, PersistenceService,
    },
    privacy::PrivacyPolicyService,
    runtime::RuntimeHandle,
};

const PET_TICK_INTERVAL: Duration = Duration::from_millis(250);
const PERSISTENCE_AUTOSAVE_INTERVAL: Duration = Duration::from_secs(30);
pub(crate) const PERSISTENCE_FINAL_SAVE_TIMEOUT: Duration = Duration::from_secs(2);

/// Build the Pet Runtime and all persistence-backed application state that must exist before
/// platform adapters begin publishing events. Failures deliberately degrade to session-only life.
pub(crate) fn initialize_runtime<R: tauri::Runtime>(
    app: &mut tauri::App<R>,
    persistence_service: PersistenceService,
    memory_admin_service: MemoryAdminService,
    history_admin_service: HistoryAdminService,
    privacy_policy_service: PrivacyPolicyService,
) -> RuntimeHandle {
    let local_data_dir = match app.path().app_local_data_dir() {
        Ok(data_dir) => Some(data_dir),
        Err(error) => {
            eprintln!(
                "Lenvu local-data path unavailable; privacy remains fail-closed and persistence is session-only: {error}"
            );
            None
        }
    };

    if let Some(data_dir) = &local_data_dir {
        if let Err(error) = privacy_policy_service.install(data_dir.join("privacy-rules.json")) {
            eprintln!(
                "Lenvu privacy rules unavailable; active-window identity remains blocked: {error}"
            );
        }
    }

    let persistence_bootstrap = if let Some(data_dir) = local_data_dir {
        let database_path = data_dir.join("lenvu.sqlite3");
        match PersistenceBootstrap::open(&database_path) {
            Ok(bootstrap) => {
                if let Err(error) = memory_admin_service.install(database_path.clone()) {
                    eprintln!("Lenvu Memory Browser unavailable: {error}");
                }
                if let Err(error) = history_admin_service.install(database_path) {
                    eprintln!("Lenvu Activity History unavailable: {error}");
                }
                Some(bootstrap)
            }
            Err(error) => {
                eprintln!("Lenvu persistence unavailable; continuing session-only: {error}");
                None
            }
        }
    } else {
        None
    };

    let initial_state = persistence_bootstrap
        .as_ref()
        .map(PersistenceBootstrap::initial_state)
        .unwrap_or_default();
    let runtime = RuntimeHandle::spawn_with_state(PET_TICK_INTERVAL, initial_state.clone());

    if let Some(bootstrap) = persistence_bootstrap {
        let persistence = bootstrap.into_worker();
        if !persistence.had_saved_state() {
            if let Err(error) = persistence.queue_save(initial_state) {
                eprintln!("Lenvu initial persistence save failed: {error}");
            }
        }
        if let Err(error) = persistence_service.install(persistence.clone()) {
            eprintln!("Lenvu persistence service install failed: {error}");
        }
        spawn_autosave(
            runtime.clone(),
            persistence,
            PERSISTENCE_AUTOSAVE_INTERVAL,
        );
    }

    spawn_event_journal(runtime.clone(), persistence_service);
    runtime
}
