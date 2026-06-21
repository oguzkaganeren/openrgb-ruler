use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum DeviceTarget {
    /// Apply action to all connected devices.
    All,
    /// Apply action only to the listed device indices.
    Specific { ids: Vec<u32> },
}

impl Default for DeviceTarget {
    fn default() -> Self {
        DeviceTarget::All
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RgbDevice {
    pub id: u32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(from = "RuleRaw")]
pub struct Rule {
    pub id: String,
    pub name: String,
    pub enabled: bool,
    pub triggers: Vec<Trigger>,
    pub action: RgbAction,
    #[serde(default)]
    pub device_target: DeviceTarget,
}

/// Raw deserialization helper — supports old single-`trigger` format and new `triggers` array.
#[derive(Deserialize)]
struct RuleRaw {
    id: String,
    name: String,
    enabled: bool,
    #[serde(default)]
    triggers: Vec<Trigger>,
    trigger: Option<Trigger>,
    action: RgbAction,
    #[serde(default)]
    device_target: DeviceTarget,
}

impl From<RuleRaw> for Rule {
    fn from(raw: RuleRaw) -> Self {
        let mut triggers = raw.triggers;
        if triggers.is_empty() {
            if let Some(t) = raw.trigger {
                triggers.push(t);
            }
        }
        Rule { id: raw.id, name: raw.name, enabled: raw.enabled, triggers, action: raw.action, device_target: raw.device_target }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum Trigger {
    SystemLock,
    SystemUnlock,
    ProcessStart { process_name: String },
    ProcessStop { process_name: String },
    SessionIdle { seconds: u64 },
    /// Fired when the screensaver/idle state ends (user becomes active again).
    SessionActive,
    /// Fired just before the system suspends to sleep.
    Suspend,
    /// Fired just after the system resumes from sleep.
    Resume,
    /// Fire at a specific time. `time` is "HH:MM" (24-hour). `days` is 0=Sun…6=Sat; empty = every day.
    TimeOfDay { time: String, days: Vec<u8> },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum RgbAction {
    TurnOff,
    SetColor { hex: String, percent: u8 },
    LoadProfile { name: String },
}
