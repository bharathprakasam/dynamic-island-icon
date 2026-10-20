#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnchorPosition {
    TopCenter,
    TopLeft,
    TopRight,
    BottomCenter,
}

#[derive(Debug, Clone)]
pub struct IslandPosition {
    pub anchor: AnchorPosition,
    pub offset_x: i32,
    pub offset_y: i32,
}

impl Default for IslandPosition {
    fn default() -> Self {
        Self {
            anchor: AnchorPosition::TopCenter,
            offset_x: 0,
            offset_y: 12,
        }
    }
}
