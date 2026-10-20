#[derive(Debug, Clone)]
pub struct GeneralSettingsView {
    pub autostart_enabled: bool,
    pub auto_hide_timeout_sec: u32,
}

impl Default for GeneralSettingsView {
    fn default() -> Self {
        Self {
            autostart_enabled: true,
            auto_hide_timeout_sec: 3,
        }
    }
}
