use std::{sync::Arc, time::Duration};

use tauri::{Emitter, Manager};

use crate::{
    history_admin::HistoryAdminService,
    memory_admin::MemoryAdminService,
    persistence::{
        spawn_autosave, spawn_event_journal, PersistenceBootstrap, PersistenceService,
    },
    privacy::PrivacyPolicyService,
    runtime::{RuntimeHandle, SnapshotObserver},
    worker::{WorkerPhase, WorkerSupervisor},
};

const PET_TICK_INTERVAL: Duration = Duration::from_millis(250);
const PERSISTENCE_AUTOSAVE_INTERVAL: Duration = Duration::from_secs(30);
pub(crate) const PERSISTENCE_FINAL_SAVE_TIMEOUT: Duration = Duration::from_secs(3);
const PET_RUNTIME_SNAPSHOT_EVENT: &str = "pet-runtime-snapshot";

/// Build the Pet Runtime and all persistence-backed application state that must exist before
/// platform adapters begin publishing events. Persistence failures deliberately degrade to
/// session-only life; failure to spawn the core Pet Runtime is fatal to application startup.
pub(crate) fn initialize_runtime<R: tauri::Runtime>(
    app: &mut tauri::App<R>,
    persistence_service: PersistenceService,
    memory_admin_service: MemoryAdminService,
    history_admin_service: HistoryAdminService,
    privacy_policy_service: PrivacyPolicyService,
    worker_supervisor: WorkerSupervisor,
) -> Result<RuntimeHandle, String> {
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
    let app_handle = app.handle().clone();
    let snapshot_observer: SnapshotObserver = Arc::new(move |snapshot| {
        if let Err(error) = app_handle.emit(PET_RUNTIME_SNAPSHOT_EVENT, &snapshot) {
            eprintln!("failed to publish Lenvu runtime snapshot: {error}");
        }
    });
    let runtime_supervisor = worker_supervisor.in_phase(WorkerPhase::Runtime);
    let runtime = RuntimeHandle::spawn_managed_with_state_and_observer(
        &runtime_supervisor,
        PET_TICK_INTERVAL,
        initial_state.clone(),
        Some(snapshot_observer),
    )?;

    if let Some(bootstrap) = persistence_bootstrap {
        let persistence_supervisor = worker_supervisor.in_phase(WorkerPhase::Persistence);
        match bootstrap.into_worker(&persistence_supervisor) {
            Ok(persistence) => {
                if !persistence.had_saved_state() {
                    if let Err(error) = persistence.queue_save(initial_state) {
                        eprintln!("Lenvu initial persistence save failed: {error}");
                    }
                }
                if let Err(error) = persistence_service.install(persistence.clone()) {
                    eprintln!("Lenvu persistence service install failed: {error}");
                }
                if let Err(error) = spawn_autosave(
                    runtime.clone(),
                    persistence,
                    PERSISTENCE_AUTOSAVE_INTERVAL,
                    &worker_supervisor,
                ) {
                    eprintln!("Lenvu persistence autosave unavailable: {error}");
                }
            }
            Err(error) => {
                eprintln!("Lenvu persistence worker unavailable; continuing session-only: {error}");
            }
        }
    }

    let journal_supervisor = worker_supervisor.in_phase(WorkerPhase::Journal);
    if let Err(error) = spawn_event_journal(
        runtime.clone(),
        persistence_service,
        &journal_supervisor,
    ) {
        eprintln!("Lenvu activity journal observer unavailable: {error}");
    }

    Ok(runtime)
}
