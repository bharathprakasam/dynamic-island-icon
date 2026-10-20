#[derive(Debug, Clone)]
pub struct ShortcutItem {
    pub label: String,
    pub icon_name: String,
    pub exec_command: String,
}

#[derive(Debug, Clone, Default)]
pub struct ShortcutGrid {
    pub columns: u32,
    pub items: Vec<ShortcutItem>,
}

impl ShortcutGrid {
    pub fn new(columns: u32) -> Self {
        Self {
            columns,
            items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, item: ShortcutItem) {
        self.items.push(item);
    }
}
