use std::path::{Path, PathBuf};

pub struct IconResolver;

impl IconResolver {
    /// Return the list of base directories where icon themes are located
    pub fn get_icon_search_dirs() -> Vec<PathBuf> {
        let mut dirs = Vec::new();

        if let Ok(home) = std::env::var("HOME") {
            dirs.push(PathBuf::from(&home).join(".local/share/icons"));
            dirs.push(PathBuf::from(&home).join(".icons"));
        }

        dirs.push(PathBuf::from("/usr/local/share/icons"));
        dirs.push(PathBuf::from("/usr/share/icons"));
        dirs.push(PathBuf::from("/usr/share/pixmaps"));

        dirs
    }

    /// Resolve an icon name into an absolute filesystem path
    pub fn resolve_icon_path(icon_name: &str, preferred_size: u32) -> Option<PathBuf> {
        if icon_name.is_empty() {
            return None;
        }

        // Direct path
        if icon_name.starts_with('/') {
            let p = Path::new(icon_name);
            if p.is_file() {
                return Some(p.to_path_buf());
            }
        }

        let extensions = ["svg", "png", "xpm"];
        let base_dirs = Self::get_icon_search_dirs();
        let themes = ["hicolor", "Adwaita", "breeze", "Papirus", "Yaru"];
        let size_str = format!("{}x{}", preferred_size, preferred_size);

        // 1. Check in themes with preferred size & scalable
        for base in &base_dirs {
            for theme in &themes {
                let theme_path = base.join(theme);
                if !theme_path.exists() {
                    continue;
                }

                // Check size dirs and scalable
                let target_dirs = [
                    theme_path.join(&size_str).join("apps"),
                    theme_path.join("scalable").join("apps"),
                    theme_path.join(&size_str),
                    theme_path.join("scalable"),
                ];

                for dir in &target_dirs {
                    for ext in &extensions {
                        let candidate = dir.join(format!("{}.{}", icon_name, ext));
                        if candidate.is_file() {
                            return Some(candidate);
                        }
                    }
                }
            }
        }

        // 2. Check /usr/share/pixmaps direct lookup
        let pixmaps = PathBuf::from("/usr/share/pixmaps");
        for ext in &extensions {
            let candidate = pixmaps.join(format!("{}.{}", icon_name, ext));
            if candidate.is_file() {
                return Some(candidate);
            }
        }

        None
    }
}
