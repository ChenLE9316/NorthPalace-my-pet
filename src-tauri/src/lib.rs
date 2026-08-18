mod bootstrap;
mod commands;
mod domain;
mod history_admin;
mod memory_admin;
mod persistence;
#[cfg(target_os = "windows")]
mod platform;
mod privacy;
mod runtime;
mod screen_context;
mod shell;
mod worker;

use std::time::Duration;

use persistence::PersistenceService;
use privacy::PrivacyPolicyService;
use runtime::RuntimeHandle;
use screen_context::ScreenContextBroker;
use tauri::Manager;
use worker::{ShutdownReport, WorkerPhase, WorkerSupervisor};

const WORKER_PHASE_SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(2);

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let persistence_service = PersistenceService::default();
    let privacy_policy_service = PrivacyPolicyService::default();
    let screen_context_broker = ScreenContextBroker::default();
    let worker_supervisor = WorkerSupervisor::default();
    let setup_supervisor = worker_supervisor.clone();

    let builder = tauri::Builder::default()
        .manage(persistence_service.clone())
        .manage(privacy_policy_service.clone())
        .manage(screen_context_broker.clone())
        .manage(worker_supervisor);

    #[cfg(target_os = "windows")]
    let builder = builder.plugin(tauri_plugin_autostart::init(
        tauri_plugin_autostart::MacosLauncher::LaunchAgent,
        None,
    ));

    #[cfg(target_os = "windows")]
    let (builder, sensor_hit_test) = {
        let hit_test = platform::windows::CursorHitTestHandle::default();
        let sensor_hit_test = hit_test.clone();
        (builder.manage(hit_test), sensor_hit_test)
    };

    let builder = builder.setup(move |app| {
        shell::install_tray(app)?;

        let runtime = bootstrap::initialize_runtime(
            app,
            persistence_service.clone(),
            privacy_policy_service.clone(),
            setup_supervisor.clone(),
        )
        .map_err(std::io::Error::other)?;
        app.manage(runtime.clone());

        #[cfg(target_os = "windows")]
        {
            platform::windows::spawn_local_time_sensor(
                runtime.clone(),
                screen_context_broker.clone(),
                &setup_supervisor,
            )
            .map_err(std::io::Error::other)?;
            platform::windows::spawn_idle_sensor(
                runtime.clone(),
                screen_context_broker.clone(),
                &setup_supervisor,
            )
            .map_err(std::io::Error::other)?;
            platform::windows::spawn_active_window_sensor(
                runtime.clone(),
                privacy_policy_service.clone(),
                screen_context_broker.clone(),
                &setup_supervisor,
            )
            .map_err(std::io::Error::other)?;

            if let Some(pet_window) = app.get_webview_window("pet") {
                platform::windows::spawn_cursor_passthrough_sensor(
                    pet_window.clone(),
                    sensor_hit_test.clone(),
                    &setup_supervisor,
                )
                .map_err(std::io::Error::other)?;
                platform::windows::spawn_pet_motion_controller(
                    pet_window,
                    runtime.clone(),
                    &setup_supervisor,
                )
                .map_err(std::io::Error::other)?;
            }
        }

        Ok(())
    });

    let builder = builder.on_window_event(|window, event| {
        if window.label() != "companion" {
            return;
        }

        if let tauri::WindowEvent::CloseRequested { api, .. } = event {
            api.prevent_close();
            if let Err(error) = window.hide() {
                eprintln!("failed to hide companion window: {error}");
            }
        }
    });

    #[cfg(target_os = "windows")]
    let builder = builder.invoke_handler(tauri::generate_handler![
        commands::get_pet_snapshot,
        commands::pet_interact,
        commands::worker_status_get,
        commands::memory_list,
        commands::memory_search,
        commands::memory_create,
        commands::memory_update,
        commands::memory_delete,
        commands::activity_list,
        commands::activity_get,
        commands::privacy_get,
        commands::privacy_add_excluded_app,
        commands::privacy_remove_excluded_app,
        commands::privacy_set_accessibility_context_enabled,
        commands::screen_context_get,
        commands::startup_get,
        commands::startup_set,
        commands::toggle_companion_window,
        commands::hide_companion_window,
        commands::get_display_context,
        commands::configure_pet_hit_regions
    ]);

    #[cfg(not(target_os = "windows"))]
    let builder = builder.invoke_handler(tauri::generate_handler![
        commands::get_pet_snapshot,
        commands::pet_interact,
        commands::worker_status_get,
        commands::memory_list,
        commands::memory_search,
        commands::memory_create,
        commands::memory_update,
        commands::memory_delete,
        commands::activity_list,
        commands::activity_get,
        commands::privacy_get,
        commands::privacy_add_excluded_app,
        commands::privacy_remove_excluded_app,
        commands::privacy_set_accessibility_context_enabled,
        commands::screen_context_get,
        commands::startup_get,
        commands::startup_set,
        commands::toggle_companion_window,
        commands::hide_companion_window
    ]);

    let app = builder
        .build(tauri::generate_context!())
        .expect("error while building NorthPalace-my-pet");

    app.run(|app_handle, event| {
        if !matches!(event, tauri::RunEvent::ExitRequested { .. }) {
            return;
        }

        let Some(supervisor) = app_handle.try_state::<WorkerSupervisor>() else {
            return;
        };

        let producers = supervisor.shutdown_phase_and_join(
            WorkerPhase::Producers,
            WORKER_PHASE_SHUTDOWN_TIMEOUT,
        );
        report_detached_workers("producers", &producers);

        if let Some(runtime) = app_handle.try_state::<RuntimeHandle>() {
            if let Err(error) = runtime.close_event_input() {
                eprintln!("Lenvu runtime input gate could not close cleanly: {error}");
            }
        }

        let runtime = supervisor
            .shutdown_phase_and_join(WorkerPhase::Runtime, WORKER_PHASE_SHUTDOWN_TIMEOUT);
        report_detached_workers("runtime", &runtime);

        let frozen_snapshot = app_handle
            .try_state::<RuntimeHandle>()
            .and_then(|runtime| runtime.snapshot().ok());

        let journal = supervisor
            .shutdown_phase_and_join(WorkerPhase::Journal, WORKER_PHASE_SHUTDOWN_TIMEOUT);
        report_detached_workers("journal", &journal);

        if let (Some(snapshot), Some(persistence)) = (
            frozen_snapshot,
            app_handle.try_state::<PersistenceService>(),
        ) {
            if let Err(error) = persistence.save_and_flush(
                snapshot.state,
                bootstrap::PERSISTENCE_FINAL_SAVE_TIMEOUT,
            ) {
                eprintln!("Lenvu final persistence save failed: {error}");
            }
        }

        let persistence = supervisor.shutdown_phase_and_join(
            WorkerPhase::Persistence,
            WORKER_PHASE_SHUTDOWN_TIMEOUT,
        );
        report_detached_workers("persistence", &persistence);
    });
}

fn report_detached_workers(phase: &str, report: &ShutdownReport) {
    if report.detached.is_empty() {
        return;
    }

    eprintln!(
        "Lenvu {phase} shutdown deadline expired; detached workers: {}",
        report.detached.join(", ")
    );
}
