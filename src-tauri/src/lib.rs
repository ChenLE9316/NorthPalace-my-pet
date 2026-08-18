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

use std::time::Duration;

use history_admin::HistoryAdminService;
use memory_admin::MemoryAdminService;
use persistence::{
    spawn_autosave, spawn_event_journal, PersistenceBootstrap, PersistenceService,
};
use privacy::PrivacyPolicyService;
use runtime::RuntimeHandle;
use screen_context::ScreenContextBroker;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};

const PET_TICK_INTERVAL: Duration = Duration::from_millis(250);
const PERSISTENCE_AUTOSAVE_INTERVAL: Duration = Duration::from_secs(30);
const PERSISTENCE_FINAL_SAVE_TIMEOUT: Duration = Duration::from_secs(2);

fn show_companion<R: tauri::Runtime>(app: &tauri::AppHandle<R>) {
    if let Some(window) = app.get_webview_window("companion") {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
    }
}

fn toggle_pet_visibility<R: tauri::Runtime>(app: &tauri::AppHandle<R>) {
    let Some(window) = app.get_webview_window("pet") else {
        return;
    };

    match window.is_visible() {
        Ok(true) => {
            let _ = window.hide();
        }
        Ok(false) => {
            let _ = window.show();
        }
        Err(error) => eprintln!("failed to read Lenvu pet-window visibility: {error}"),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let persistence_service = PersistenceService::default();
    let memory_admin_service = MemoryAdminService::default();
    let history_admin_service = HistoryAdminService::default();
    let privacy_policy_service = PrivacyPolicyService::default();
    let screen_context_broker = ScreenContextBroker::default();
    let builder = tauri::Builder::default()
        .manage(persistence_service.clone())
        .manage(memory_admin_service.clone())
        .manage(history_admin_service.clone())
        .manage(privacy_policy_service.clone())
        .manage(screen_context_broker.clone());

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
        let open_companion = MenuItem::with_id(
            app,
            "open_companion",
            "Open Lenvu Companion",
            true,
            None::<&str>,
        )?;
        let toggle_pet = MenuItem::with_id(
            app,
            "toggle_pet",
            "Show / Hide Lenvu",
            true,
            None::<&str>,
        )?;
        let quit = MenuItem::with_id(
            app,
            "quit",
            "Quit NorthPalace-my-pet",
            true,
            None::<&str>,
        )?;
        let tray_menu = Menu::with_items(app, &[&open_companion, &toggle_pet, &quit])?;

        let mut tray = TrayIconBuilder::with_id("lenvu")
            .tooltip("Lenvu · NorthPalace-my-pet")
            .menu(&tray_menu)
            .show_menu_on_left_click(false)
            .on_menu_event(|app, event| match event.id.as_ref() {
                "open_companion" => show_companion(app),
                "toggle_pet" => toggle_pet_visibility(app),
                "quit" => app.exit(0),
                _ => {}
            })
            .on_tray_icon_event(|tray, event| {
                if let TrayIconEvent::Click {
                    button: MouseButton::Left,
                    button_state: MouseButtonState::Up,
                    ..
                } = event
                {
                    show_companion(tray.app_handle());
                }
            });

        if let Some(icon) = app.default_window_icon() {
            tray = tray.icon(icon.clone());
        }
        tray.build(app)?;

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
                    eprintln!(
                        "Lenvu persistence unavailable; continuing session-only: {error}"
                    );
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
        app.manage(runtime.clone());

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

        spawn_event_journal(runtime.clone(), persistence_service.clone());

        #[cfg(target_os = "windows")]
        {
            platform::windows::spawn_local_time_sensor(
                runtime.clone(),
                screen_context_broker.clone(),
            );
            platform::windows::spawn_idle_sensor(
                runtime.clone(),
                screen_context_broker.clone(),
            );
            platform::windows::spawn_active_window_sensor(
                runtime.clone(),
                privacy_policy_service.clone(),
                screen_context_broker.clone(),
            );

            if let Some(pet_window) = app.get_webview_window("pet") {
                platform::windows::spawn_cursor_passthrough_sensor(
                    pet_window.clone(),
                    sensor_hit_test.clone(),
                );
                platform::windows::spawn_pet_motion_controller(pet_window, runtime.clone());
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

        let Some(runtime) = app_handle.try_state::<RuntimeHandle>() else {
            return;
        };
        let Some(persistence) = app_handle.try_state::<PersistenceService>() else {
            return;
        };
        let Ok(snapshot) = runtime.snapshot() else {
            return;
        };

        if let Err(error) =
            persistence.save_and_flush(snapshot.state, PERSISTENCE_FINAL_SAVE_TIMEOUT)
        {
            eprintln!("Lenvu final persistence save failed: {error}");
        }
    });
}
