mod domain;
#[cfg(target_os = "windows")]
mod platform;
mod runtime;

use std::time::Duration;

use domain::pet::PetInteraction;
use runtime::{PetRuntimeSnapshot, RuntimeHandle};
use tauri::Manager;

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
    let runtime = RuntimeHandle::spawn(Duration::from_millis(250));

    #[cfg(target_os = "windows")]
    {
        platform::windows::spawn_idle_sensor(runtime.clone());
        platform::windows::spawn_active_window_sensor(runtime.clone());
    }

    let builder = tauri::Builder::default().manage(runtime);

    #[cfg(target_os = "windows")]
    let builder = {
        let hit_test = platform::windows::CursorHitTestHandle::default();
        let sensor_hit_test = hit_test.clone();

        builder.manage(hit_test).setup(move |app| {
            if let Some(pet_window) = app.get_webview_window("pet") {
                platform::windows::spawn_cursor_passthrough_sensor(
                    pet_window,
                    sensor_hit_test.clone(),
                );
            }
            Ok(())
        })
    };

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
