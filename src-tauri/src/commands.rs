use tauri::State;
use uuid::Uuid;
use std::sync::{Arc, RwLock};
use crate::model::{DeviceTarget, Rule, RgbAction, RgbDevice};
use crate::config;

pub struct RulesState(pub Arc<RwLock<Vec<Rule>>>);

#[tauri::command]
pub fn get_rules(state: State<RulesState>) -> Vec<Rule> {
    state.0.read().unwrap().clone()
}

#[tauri::command]
pub fn save_rules(rules: Vec<Rule>, state: State<RulesState>) -> Result<(), String> {
    config::save_rules(&rules)?;
    *state.0.write().unwrap() = rules;
    Ok(())
}

#[tauri::command]
pub fn add_rule(rule: Rule, state: State<RulesState>) -> Result<(), String> {
    let mut rules = state.0.write().unwrap();
    rules.push(rule);
    config::save_rules(&rules)
}

#[tauri::command]
pub fn delete_rule(id: String, state: State<RulesState>) -> Result<(), String> {
    let mut rules = state.0.write().unwrap();
    rules.retain(|r| r.id != id);
    config::save_rules(&rules)
}

#[tauri::command]
pub fn toggle_rule(id: String, state: State<RulesState>) -> Result<(), String> {
    let mut rules = state.0.write().unwrap();
    if let Some(rule) = rules.iter_mut().find(|r| r.id == id) {
        rule.enabled = !rule.enabled;
    }
    config::save_rules(&rules)
}

#[tauri::command]
pub fn generate_id() -> String {
    Uuid::new_v4().to_string()
}

#[tauri::command]
pub fn test_action(action: RgbAction, device_target: Option<DeviceTarget>) -> Result<(), String> {
    let target = device_target.unwrap_or(DeviceTarget::All);
    crate::openrgb::execute_action(&action, &target)
}

#[tauri::command]
pub fn get_openrgb_profiles() -> Result<Vec<String>, String> {
    crate::openrgb::list_profiles()
}

#[tauri::command]
pub fn get_openrgb_devices() -> Result<Vec<RgbDevice>, String> {
    crate::openrgb::list_devices()
}

#[tauri::command]
pub fn check_openrgb_available() -> bool {
    crate::openrgb::is_available()
}

// ── Autostart (Linux XDG) ─────────────────────────────────────────────────

#[cfg(target_os = "linux")]
fn autostart_path() -> std::path::PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("~/.config"))
        .join("autostart")
        .join("openrgb-action-gui.desktop")
}

#[tauri::command]
pub fn get_autostart() -> bool {
    #[cfg(target_os = "linux")]
    {
        autostart_path().exists()
    }
    #[cfg(not(target_os = "linux"))]
    false
}

#[tauri::command]
pub fn set_autostart(enabled: bool) -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        let path = autostart_path();
        if enabled {
            let exe = std::env::current_exe()
                .map_err(|e| format!("Cannot determine executable path: {}", e))?;
            let exe_str = exe.to_string_lossy();
            let desktop = format!(
                "[Desktop Entry]\nType=Application\nName=OpenRGB Action\nExec={exe_str} --tray\nX-GNOME-Autostart-enabled=true\n",
            );
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create autostart dir: {}", e))?;
            }
            std::fs::write(&path, desktop)
                .map_err(|e| format!("Failed to write .desktop file: {}", e))?;
        } else {
            if path.exists() {
                std::fs::remove_file(&path)
                    .map_err(|e| format!("Failed to remove .desktop file: {}", e))?;
            }
        }
        Ok(())
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = enabled;
        Err("Autostart is only supported on Linux".into())
    }
}
