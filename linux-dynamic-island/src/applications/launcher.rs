use std::process::Command;
use crate::applications::desktop_entry::DesktopEntry;
use crate::errors::{AppError, AppResult};

pub struct ApplicationLauncher;

impl ApplicationLauncher {
    pub fn launch(entry: &DesktopEntry) -> AppResult<()> {
        log::info!("Launching desktop application: {}", entry.name);

        let sanitized_exec = entry
            .exec
            .split_whitespace()
            .take_while(|arg| !arg.starts_with('%'))
            .collect::<Vec<_>>()
            .join(" ");

        if sanitized_exec.is_empty() {
            return Err(AppError::DesktopEntry("Empty exec command".into()));
        }

        Command::new("sh")
            .arg("-c")
            .arg(format!("{} &", sanitized_exec))
            .spawn()
            .map_err(|e| AppError::General(format!("Failed to spawn process: {}", e)))?;

        Ok(())
    }
}
