#[cfg(target_os = "linux")]
fn autostart_path() -> std::path::PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("~/.config"))
        .join("autostart")
        .join("openrgb-ruler.desktop")
}

pub fn get_autostart() -> bool {
    #[cfg(target_os = "linux")]
    {
        autostart_path().exists()
    }
    #[cfg(not(target_os = "linux"))]
    false
}

pub fn set_autostart(enabled: bool) -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        let path = autostart_path();
        if enabled {
            let exe = std::env::current_exe()
                .map_err(|e| format!("Cannot determine executable path: {}", e))?;
            let exe_str = exe.to_string_lossy();
            let desktop = format!(
                "[Desktop Entry]\nType=Application\nName=OpenRGB Ruler\nExec={exe_str} --tray\nX-GNOME-Autostart-enabled=true\n",
            );
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create autostart dir: {}", e))?;
            }
            std::fs::write(&path, desktop)
                .map_err(|e| format!("Failed to write .desktop file: {}", e))?;
        } else if path.exists() {
            std::fs::remove_file(&path)
                .map_err(|e| format!("Failed to remove .desktop file: {}", e))?;
        }
        Ok(())
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = enabled;
        Err("Autostart is only supported on Linux".into())
    }
}
