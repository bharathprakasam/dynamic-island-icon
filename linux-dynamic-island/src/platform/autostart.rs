use std::path::PathBuf;
use crate::errors::AppResult;

pub struct AutostartManager;

impl AutostartManager {
    pub fn get_autostart_file_path() -> AppResult<PathBuf> {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
        Ok(PathBuf::from(home).join(".config/autostart/linux-dynamic-island.desktop"))
    }

    pub fn is_autostart_enabled() -> bool {
        Self::get_autostart_file_path().map(|p| p.exists()).unwrap_or(false)
    }

    pub fn set_autostart(enable: bool) -> AppResult<()> {
        let path = Self::get_autostart_file_path()?;
        if enable {
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let desktop_content = "[Desktop Entry]\nType=Application\nName=Linux Dynamic Island\nExec=linux-dynamic-island\nHidden=false\nNoDisplay=false\nX-GNOME-Autostart-enabled=true\n";
            std::fs::write(path, desktop_content)?;
        } else if path.exists() {
            std::fs::remove_file(path)?;
        }
        Ok(())
    }
}
