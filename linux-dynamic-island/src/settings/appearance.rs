#[derive(Debug, Clone)]
pub struct AppearanceSettingsView {
    pub selected_theme: String,
    pub opacity_percent: u8,
    pub rounded_corners: bool,
}

impl Default for AppearanceSettingsView {
    fn default() -> Self {
        Self {
            selected_theme: "default-dark".to_string(),
            opacity_percent: 95,
            rounded_corners: true,
        }
    }
}
