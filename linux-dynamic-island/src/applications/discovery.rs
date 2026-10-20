use std::collections::HashMap;
use std::path::{Path, PathBuf};
use crate::applications::desktop_entry::DesktopEntry;
use crate::errors::AppResult;

pub struct ApplicationDiscovery;

impl ApplicationDiscovery {
    /// Return all standard XDG application directories
    pub fn get_search_paths() -> Vec<PathBuf> {
        let mut paths = vec![
            PathBuf::from("/usr/share/applications"),
            PathBuf::from("/usr/local/share/applications"),
            PathBuf::from("/var/lib/flatpak/exports/share/applications"),
        ];

        if let Ok(data_dirs) = std::env::var("XDG_DATA_DIRS") {
            for dir in data_dirs.split(':') {
                let p = PathBuf::from(dir).join("applications");
                if !paths.contains(&p) {
                    paths.push(p);
                }
            }
        }

        if let Ok(home) = std::env::var("HOME") {
            let user_apps = PathBuf::from(&home).join(".local/share/applications");
            if !paths.contains(&user_apps) {
                paths.push(user_apps);
            }
            let flatpak_user_apps = PathBuf::from(home).join(".local/share/flatpak/exports/share/applications");
            if !paths.contains(&flatpak_user_apps) {
                paths.push(flatpak_user_apps);
            }
        }

        paths
    }

    /// Discover all valid desktop applications, deduplicated by application ID
    pub fn discover_all() -> AppResult<Vec<DesktopEntry>> {
        let mut apps_by_id: HashMap<String, DesktopEntry> = HashMap::new();

        for search_dir in Self::get_search_paths() {
            if !search_dir.exists() || !search_dir.is_dir() {
                continue;
            }

            Self::scan_directory(&search_dir, &mut apps_by_id)?;
        }

        let mut apps: Vec<DesktopEntry> = apps_by_id.into_values().collect();
        apps.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        Ok(apps)
    }

    /// Recursively scan a directory for `.desktop` files
    fn scan_directory(dir: &Path, apps: &mut HashMap<String, DesktopEntry>) -> AppResult<()> {
        let entries = match std::fs::read_dir(dir) {
            Ok(e) => e,
            Err(_) => return Ok(()),
        };

        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "desktop" {
                        if let Ok(Some(app)) = DesktopEntry::parse_file(&path) {
                            // User-local entries override system-wide ones with the same ID
                            apps.insert(app.id.clone(), app);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Filter a list of applications by search query (matching name, comment, or categories)
    pub fn filter_applications<'a>(apps: &'a [DesktopEntry], query: &str) -> Vec<&'a DesktopEntry> {
        if query.trim().is_empty() {
            return apps.iter().collect();
        }

        let q = query.to_lowercase();
        apps.iter()
            .filter(|app| {
                app.name.to_lowercase().contains(&q)
                    || app
                        .comment
                        .as_ref()
                        .map(|c| c.to_lowercase().contains(&q))
                        .unwrap_or(false)
                    || app
                        .categories
                        .iter()
                        .any(|cat| cat.to_lowercase().contains(&q))
            })
            .collect()
    }
}
