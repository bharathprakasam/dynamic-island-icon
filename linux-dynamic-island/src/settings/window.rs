use crate::errors::AppResult;

pub struct SettingsWindow {
    pub is_visible: bool,
}

impl SettingsWindow {
    pub fn new() -> Self {
        Self { is_visible: false }
    }

    pub fn open(&mut self) -> AppResult<()> {
        self.is_visible = true;
        log::info!("Opening settings dialog");
        Ok(())
    }

    pub fn close(&mut self) -> AppResult<()> {
        self.is_visible = false;
        log::info!("Closing settings dialog");
        Ok(())
    }
}
