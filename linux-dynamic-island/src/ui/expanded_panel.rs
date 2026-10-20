#[derive(Debug, Clone)]
pub struct ExpandedPanel {
    pub title: String,
    pub active_tab: PanelTab,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PanelTab {
    Shortcuts,
    Media,
    Notifications,
}

impl Default for ExpandedPanel {
    fn default() -> Self {
        Self {
            title: "Dynamic Island".into(),
            active_tab: PanelTab::Shortcuts,
        }
    }
}
