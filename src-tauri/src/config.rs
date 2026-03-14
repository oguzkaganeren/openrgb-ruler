use std::path::PathBuf;
use std::fs;
use serde_json;
use crate::model::Rule;

fn config_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("~/.config"))
        .join("openrgb-action-gui")
}

fn rules_path() -> PathBuf {
    config_dir().join("rules.json")
}

pub fn load_rules() -> Result<Vec<Rule>, String> {
    let path = rules_path();
    if !path.exists() {
        return Ok(Vec::new());
    }
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read rules file: {}", e))?;
    serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse rules file: {}", e))
}

pub fn save_rules(rules: &[Rule]) -> Result<(), String> {
    let dir = config_dir();
    fs::create_dir_all(&dir)
        .map_err(|e| format!("Failed to create config dir: {}", e))?;
    let content = serde_json::to_string_pretty(rules)
        .map_err(|e| format!("Failed to serialize rules: {}", e))?;
    fs::write(rules_path(), content)
        .map_err(|e| format!("Failed to write rules file: {}", e))
}
