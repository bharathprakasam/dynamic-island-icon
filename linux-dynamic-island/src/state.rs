use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum IslandMode {
    #[default]
    Compact,
    Expanded,
    Notification,
    Media,
}

#[derive(Debug, Clone, Default)]
pub struct AppState {
    pub is_expanded: bool,
    pub current_mode: IslandMode,
    pub active_app_title: Option<String>,
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn toggle_expanded(&mut self) {
        self.is_expanded = !self.is_expanded;
    }

    pub fn set_mode(&mut self, mode: IslandMode) {
        self.current_mode = mode;
    }
}
