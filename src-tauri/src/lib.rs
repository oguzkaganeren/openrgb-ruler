mod model;
mod config;
mod commands;
mod openrgb;
mod watcher;

use commands::RulesState;
use std::sync::{Arc, RwLock};
use tauri::{
    Manager,
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let tray_only = std::env::args().any(|a| a == "--tray");
    let rules = Arc::new(RwLock::new(config::load_rules().unwrap_or_default()));

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(RulesState(rules.clone()))
        .setup(move |app| {
            watcher::start_watchers(rules);

            // When launched with --tray (e.g. from autostart), keep window hidden.
            if tray_only {
                if let Some(w) = app.get_webview_window("main") {
                    let _ = w.hide();
                }
            }

            // Build tray menu
            let show = MenuItem::with_id(app, "show", "Show Window", true, None::<&str>)?;
            let hide = MenuItem::with_id(app, "hide", "Hide Window", true, None::<&str>)?;
            let sep1 = PredefinedMenuItem::separator(app)?;
            let enable_all = MenuItem::with_id(app, "enable_all", "Enable All Rules", true, None::<&str>)?;
            let disable_all = MenuItem::with_id(app, "disable_all", "Disable All Rules", true, None::<&str>)?;
            let sep2 = PredefinedMenuItem::separator(app)?;
            let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

            let menu = Menu::with_items(app, &[
                &show, &hide, &sep1, &enable_all, &disable_all, &sep2, &quit,
            ])?;

            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(w) = app.get_webview_window("main") {
                            let _ = w.show();
                            let _ = w.set_focus();
                        }
                    }
                    "hide" => {
                        if let Some(w) = app.get_webview_window("main") {
                            let _ = w.hide();
                        }
                    }
                    "enable_all" => {
                        let state = app.state::<RulesState>();
                        let mut rules = state.0.write().unwrap();
                        for r in rules.iter_mut() {
                            r.enabled = true;
                        }
                        let _ = config::save_rules(&rules);
                    }
                    "disable_all" => {
                        let state = app.state::<RulesState>();
                        let mut rules = state.0.write().unwrap();
                        for r in rules.iter_mut() {
                            r.enabled = false;
                        }
                        let _ = config::save_rules(&rules);
                    }
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
                        let app = tray.app_handle();
                        if let Some(w) = app.get_webview_window("main") {
                            let _ = w.show();
                            let _ = w.set_focus();
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                let _ = window.hide();
                api.prevent_close();
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_rules,
            commands::save_rules,
            commands::add_rule,
            commands::delete_rule,
            commands::toggle_rule,
            commands::generate_id,
            commands::test_action,
            commands::get_openrgb_profiles,
            commands::get_openrgb_devices,
            commands::check_openrgb_available,
            commands::get_autostart,
            commands::set_autostart,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
