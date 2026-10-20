use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub version: u32,
    pub appearance: AppearanceConfig,
    pub behavior: BehaviorConfig,
    pub shortcuts: Vec<ShortcutBinding>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppearanceConfig {
    pub width_compact: u32,
    pub height_compact: u32,
    pub width_expanded: u32,
    pub height_expanded: u32,
    pub corner_radius: f64,
    pub opacity: f64,
    pub theme: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorConfig {
    pub autostart: bool,
    pub auto_hide_ms: u64,
    pub enable_animations: bool,
    pub animation_duration_ms: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortcutBinding {
    pub key: String,
    pub action: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            version: 1,
            appearance: AppearanceConfig {
                width_compact: 200,
                height_compact: 36,
                width_expanded: 480,
                height_expanded: 220,
                corner_radius: 18.0,
                opacity: 0.95,
                theme: "default-dark".to_string(),
            },
            behavior: BehaviorConfig {
                autostart: true,
                auto_hide_ms: 3000,
                enable_animations: true,
                animation_duration_ms: 250,
            },
            shortcuts: vec![
                ShortcutBinding {
                    key: "<Super>i".to_string(),
                    action: "toggle_island".to_string(),
                },
            ],
        }
    }
}
