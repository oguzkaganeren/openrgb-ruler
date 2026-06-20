use std::process::Command;
use std::sync::{Mutex, OnceLock};
use crate::model::{DeviceTarget, RgbAction, RgbDevice};

static DEVICES_CACHE: OnceLock<Mutex<Vec<RgbDevice>>> = OnceLock::new();
static PROFILES_CACHE: OnceLock<Mutex<Vec<String>>> = OnceLock::new();

/// Pre-warm device and profile caches in background threads so the editor
/// opens instantly instead of blocking the GTK main thread.
pub fn prefetch() {
    std::thread::spawn(|| {
        let devices = list_devices().unwrap_or_default();
        let _ = DEVICES_CACHE.get_or_init(|| Mutex::new(devices));
    });
    std::thread::spawn(|| {
        let profiles = list_profiles().unwrap_or_default();
        let _ = PROFILES_CACHE.get_or_init(|| Mutex::new(profiles));
    });
}

pub fn list_devices_cached() -> Vec<RgbDevice> {
    if let Some(m) = DEVICES_CACHE.get() {
        return m.lock().unwrap().clone();
    }
    list_devices().unwrap_or_default()
}

pub fn list_profiles_cached() -> Vec<String> {
    if let Some(m) = PROFILES_CACHE.get() {
        return m.lock().unwrap().clone();
    }
    list_profiles().unwrap_or_default()
}

pub fn is_available() -> bool {
    Command::new("openrgb")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

pub fn execute_action(action: &RgbAction, target: &DeviceTarget) -> Result<(), String> {
    let mut cmd = Command::new("openrgb");
    cmd.arg("--noautoconnect");
    if let DeviceTarget::Specific { ids } = target {
        for id in ids {
            cmd.args(["--device", &id.to_string()]);
        }
    }
    cmd.args(action_to_args(action));
    let status = cmd
        .status()
        .map_err(|e| format!("Failed to run openrgb: {}", e))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("openrgb exited with status: {}", status))
    }
}

/// Parse `openrgb --noautoconnect --list-devices` output.
/// Lines like `0: Corsair K70 RGB` are extracted.
pub fn list_devices() -> Result<Vec<RgbDevice>, String> {
    let output = Command::new("openrgb")
        .args(["--noautoconnect", "--list-devices"])
        .output()
        .map_err(|e| format!("Failed to run openrgb: {}", e))?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut devices = Vec::new();
    for line in stdout.lines() {
        let line = line.trim();
        if let Some(colon) = line.find(": ") {
            let idx_str = &line[..colon];
            if let Ok(id) = idx_str.trim().parse::<u32>() {
                let name = line[colon + 2..].trim().to_string();
                if !name.is_empty() {
                    devices.push(RgbDevice { id, name });
                }
            }
        }
    }
    Ok(devices)
}

pub fn list_profiles() -> Result<Vec<String>, String> {
    let output = Command::new("openrgb")
        .args(&["--noautoconnect", "--list-profiles"])
        .output()
        .map_err(|e| format!("Failed to run openrgb: {}", e))?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let profiles = stdout
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect();
    Ok(profiles)
}

fn action_to_args(action: &RgbAction) -> Vec<String> {
    match action {
        RgbAction::TurnOff => vec!["-m".to_string(), "Off".to_string()],
        RgbAction::SetColor { hex, percent } => {
            let hex = if *percent == 100 {
                hex.clone()
            } else {
                let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
                let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
                let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
                let scale = |c: u8| -> u8 { (c as u32 * *percent as u32 / 100) as u8 };
                format!("{:02X}{:02X}{:02X}", scale(r), scale(g), scale(b))
            };
            vec!["-m".to_string(), "Static".to_string(), "-c".to_string(), hex]
        }
        RgbAction::LoadProfile { name } => vec!["-p".to_string(), name.clone()],
    }
}
