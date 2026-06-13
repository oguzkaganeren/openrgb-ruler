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
pub struct Rule {
    pub id: String,
    pub name: String,
    pub enabled: bool,
    pub trigger: Trigger,
    pub action: RgbAction,
    #[serde(default)]
    pub device_target: DeviceTarget,
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
