mod domain;
#[cfg(target_os = "windows")]
mod platform;
mod runtime;

use std::time::Duration;

use domain::pet::PetInteraction;
use runtime::{PetRuntimeSnapshot, RuntimeHandle};

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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let runtime = RuntimeHandle::spawn(Duration::from_millis(250));

    #[cfg(target_os = "windows")]
    platform::windows::spawn_idle_sensor(runtime.clone());

    tauri::Builder::default()
        .manage(runtime)
        .invoke_handler(tauri::generate_handler![get_pet_snapshot, pet_interact])
        .run(tauri::generate_context!())
        .expect("error while running NorthPalace-my-pet");
}
