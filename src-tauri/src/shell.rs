use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};

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

/// Install native shell controls without coupling tray behavior to Pet Brain or persistence setup.
pub(crate) fn install_tray<R: tauri::Runtime>(app: &mut tauri::App<R>) -> tauri::Result<()> {
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
    Ok(())
}
