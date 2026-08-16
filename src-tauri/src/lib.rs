mod domain;
mod persistence;
#[cfg(target_os = "windows")]
mod platform;
mod runtime;

use std::time::Duration;

use domain::pet::PetInteraction;
use persistence::{spawn_autosave, PersistenceBootstrap};
use runtime::{PetRuntimeSnapshot, RuntimeHandle};
use tauri::Manager;

const PET_TICK_INTERVAL: Duration = Duration::from_millis(250);
const PERSISTENCE_AUTOSAVE_INTERVAL: Duration = Duration::from_secs(30);

#[tauri::command]
fn get_pet_snapshot(runtime: tauri::State<'_, RuntimeHandle>) -> Result<PetRuntimeSnapshot, String> {
    runtime.snapshot()
}

#[tauri::command]
fn pet_interact(
    kind: PetInteraction,
    runtime: tauri::State<'_, RuntimeHandle>,
) -> Result<(), String> {
    runtime.dispatch(kind.into_event())
}

#[tauri::command]
fn toggle_companion_window(app: tauri::AppHandle) -> Result<bool, String> {
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
fn hide_companion_window(app: tauri::AppHandle) -> Result<(), String> {
    let window = app
        .get_webview_window("companion")
        .ok_or_else(|| "companion window is unavailable".to_owned())?;
    window.hide().map_err(|error| error.to_string())
}

#[cfg(target_os = "windows")]
#[tauri::command]
fn get_display_context(
    webview_window: tauri::WebviewWindow,
) -> Result<platform::windows::DisplayContext, String> {
    platform::windows::read_display_context(&webview_window)
}

#[cfg(target_os = "windows")]
#[tauri::command]
fn configure_pet_hit_regions(
    regions: Vec<platform::windows::CursorHitRegion>,
    hit_test: tauri::State<'_, platform::windows::CursorHitTestHandle>,
) -> Result<(), String> {
    hit_test.set_regions(regions)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default();

    #[cfg(target_os = "windows")]
    let (builder, sensor_hit_test) = {
        let hit_test = platform::windows::CursorHitTestHandle::default();
        let sensor_hit_test = hit_test.clone();
        (builder.manage(hit_test), sensor_hit_test)
    };

    let builder = builder.setup(move |app| {
        let persistence_bootstrap = match app.path().app_local_data_dir() {
            Ok(data_dir) => {
                let database_path = data_dir.join("lenvu.sqlite3");
                match PersistenceBootstrap::open(&database_path) {
                    Ok(bootstrap) => Some(bootstrap),
                    Err(error) => {
                        eprintln!(
                            "Lenvu persistence unavailable; continuing session-only: {error}"
                        );
                        None
                    }
                }
            }
            Err(error) => {
                eprintln!(
                    "Lenvu local-data path unavailable; continuing session-only: {error}"
                );
                None
            }
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
            spawn_autosave(
                runtime.clone(),
                persistence,
                PERSISTENCE_AUTOSAVE_INTERVAL,
            );
        }

        #[cfg(target_os = "windows")]
        {
            platform::windows::spawn_idle_sensor(runtime.clone());
            platform::windows::spawn_active_window_sensor(runtime.clone());

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
        get_pet_snapshot,
        pet_interact,
        toggle_companion_window,
        hide_companion_window,
        get_display_context,
        configure_pet_hit_regions
    ]);

    #[cfg(not(target_os = "windows"))]
    let builder = builder.invoke_handler(tauri::generate_handler![
        get_pet_snapshot,
        pet_interact,
        toggle_companion_window,
        hide_companion_window
    ]);

    builder
        .run(tauri::generate_context!())
        .expect("error while running NorthPalace-my-pet");
}
