#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WidgetState {
    Hidden,
    Collapsed,
    Hovered,
    Expanded,
}

impl Default for WidgetState {
    fn default() -> Self {
        WidgetState::Collapsed
    }
}
