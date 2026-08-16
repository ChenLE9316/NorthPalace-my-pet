mod domain;
mod runtime;

use std::sync::Mutex;
use domain::pet::{PetBrain, PetInteraction, PetState};

#[tauri::command]
fn get_pet_state(brain: tauri::State<'_, Mutex<PetBrain>>) -> PetState {
    brain.lock().expect("pet brain mutex poisoned").state()
}

#[tauri::command]
fn tick_pet(seconds: u64, brain: tauri::State<'_, Mutex<PetBrain>>) -> PetState {
    // V0 compatibility command. The V0.2 migration moves simulation time into
    // `runtime::RuntimeClock` and will remove this UI-owned ticking path.
    let mut brain = brain.lock().expect("pet brain mutex poisoned");
    brain.tick(seconds);
    brain.state()
}

#[tauri::command]
fn pet_interact(kind: PetInteraction, brain: tauri::State<'_, Mutex<PetBrain>>) -> PetState {
    let mut brain = brain.lock().expect("pet brain mutex poisoned");
    brain.interact(kind);
    brain.state()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Mutex::new(PetBrain::default()))
        .invoke_handler(tauri::generate_handler![get_pet_state, tick_pet, pet_interact])
        .run(tauri::generate_context!())
        .expect("error while running NorthPalace-my-pet");
}
