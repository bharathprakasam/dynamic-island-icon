#[derive(Debug, Clone)]
pub struct EmptyState {
    pub message: String,
    pub icon_name: String,
}

impl Default for EmptyState {
    fn default() -> Self {
        Self {
            message: "No active notifications or shortcuts".into(),
            icon_name: "dialog-information-symbolic".into(),
        }
    }
}
