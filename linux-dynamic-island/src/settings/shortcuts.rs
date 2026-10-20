#[derive(Debug, Clone)]
pub struct ShortcutSettingEntry {
    pub name: String,
    pub binding: String,
}

#[derive(Debug, Clone, Default)]
pub struct ShortcutsSettingsView {
    pub shortcuts: Vec<ShortcutSettingEntry>,
}
