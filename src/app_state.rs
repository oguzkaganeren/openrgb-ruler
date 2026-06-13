use std::sync::{Arc, RwLock};
use uuid::Uuid;
use crate::model::Rule;
use crate::config;

pub type SharedRules = Arc<RwLock<Vec<Rule>>>;

pub fn new_shared_rules() -> SharedRules {
    let rules = config::load_rules().unwrap_or_default();
    Arc::new(RwLock::new(rules))
}

pub fn get_rules(state: &SharedRules) -> Vec<Rule> {
    state.read().unwrap().clone()
}

pub fn add_rule(state: &SharedRules, rule: Rule) -> Result<(), String> {
    let mut rules = state.write().unwrap();
    rules.push(rule);
    config::save_rules(&rules)
}

pub fn delete_rule(state: &SharedRules, id: &str) -> Result<(), String> {
    let mut rules = state.write().unwrap();
    rules.retain(|r| r.id != id);
    config::save_rules(&rules)
}

pub fn toggle_rule(state: &SharedRules, id: &str) -> Result<(), String> {
    let mut rules = state.write().unwrap();
    if let Some(rule) = rules.iter_mut().find(|r| r.id == id) {
        rule.enabled = !rule.enabled;
    }
    config::save_rules(&rules)
}

pub fn update_rule(state: &SharedRules, rule: Rule) -> Result<(), String> {
    let mut rules = state.write().unwrap();
    if let Some(existing) = rules.iter_mut().find(|r| r.id == rule.id) {
        *existing = rule;
    } else {
        rules.push(rule);
    }
    config::save_rules(&rules)
}

pub fn save_rules(state: &SharedRules, new_rules: Vec<Rule>) -> Result<(), String> {
    config::save_rules(&new_rules)?;
    *state.write().unwrap() = new_rules;
    Ok(())
}

pub fn generate_id() -> String {
    Uuid::new_v4().to_string()
}
