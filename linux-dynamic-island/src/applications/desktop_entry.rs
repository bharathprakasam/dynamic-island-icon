use std::path::{Path, PathBuf};
use crate::errors::{AppError, AppResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesktopEntry {
    pub id: String,
    pub name: String,
    pub comment: Option<String>,
    pub exec: String,
    pub icon: Option<String>,
    pub terminal: bool,
    pub categories: Vec<String>,
    pub file_path: PathBuf,
    pub no_display: bool,
    pub startup_wm_class: Option<String>,
}

impl DesktopEntry {
    /// Parse a `.desktop` file from disk
    pub fn parse_file(path: &Path) -> AppResult<Option<Self>> {
        if !path.exists() {
            return Err(AppError::DesktopEntry(format!(
                "File not found: {}",
                path.display()
            )));
        }

        let content = std::fs::read_to_string(path)?;
        Self::from_str(&content, path.to_path_buf())
    }

    /// Parse desktop entry from raw INI-style content
    pub fn from_str(content: &str, file_path: PathBuf) -> AppResult<Option<Self>> {
        let mut in_desktop_entry_section = false;
        let mut entry_type = String::new();
        let mut name = None;
        let mut comment = None;
        let mut exec = None;
        let mut icon = None;
        let mut terminal = false;
        let mut no_display = false;
        let mut categories = Vec::new();
        let mut startup_wm_class = None;

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            if trimmed.starts_with('[') && trimmed.ends_with(']') {
                let section = &trimmed[1..trimmed.len() - 1];
                in_desktop_entry_section = section == "Desktop Entry";
                continue;
            }

            if !in_desktop_entry_section {
                continue;
            }

            if let Some((key, val)) = trimmed.split_once('=') {
                let key = key.trim();
                let val = val.trim();

                match key {
                    "Type" => entry_type = val.to_string(),
                    "Name" => {
                        if name.is_none() {
                            name = Some(val.to_string());
                        }
                    }
                    "Comment" => {
                        if comment.is_none() {
                            comment = Some(val.to_string());
                        }
                    }
                    "Exec" => exec = Some(val.to_string()),
                    "Icon" => icon = Some(val.to_string()),
                    "Terminal" => terminal = val.eq_ignore_ascii_case("true"),
                    "NoDisplay" => no_display = val.eq_ignore_ascii_case("true"),
                    "StartupWMClass" => startup_wm_class = Some(val.to_string()),
                    "Categories" => {
                        categories = val
                            .split(';')
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                    }
                    _ => {}
                }
            }
        }

        // Only Application entries that are not hidden are kept for launcher
        if entry_type != "Application" || no_display {
            return Ok(None);
        }

        let name = match name {
            Some(n) if !n.is_empty() => n,
            _ => return Ok(None),
        };

        let exec = match exec {
            Some(e) if !e.is_empty() => e,
            _ => return Ok(None),
        };

        let id = file_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or(&name)
            .to_string();

        Ok(Some(Self {
            id,
            name,
            comment,
            exec,
            icon,
            terminal,
            categories,
            file_path,
            no_display,
            startup_wm_class,
        }))
    }

    /// Returns the command line with field codes (%f, %u, etc.) stripped away
    pub fn clean_command(&self) -> String {
        self.exec
            .split_whitespace()
            .filter(|arg| !arg.starts_with('%'))
            .collect::<Vec<_>>()
            .join(" ")
    }
}
