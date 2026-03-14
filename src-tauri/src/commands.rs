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
