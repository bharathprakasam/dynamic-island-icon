#[derive(Debug, Clone)]
pub struct StatusBadge {
    pub label: String,
    pub icon_name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct IconButton {
    pub id: String,
    pub icon_name: String,
    pub tooltip: Option<String>,
}
