use crate::config::schema::AppConfig;
use crate::errors::AppResult;
use crate::platform::capabilities::{PlatformCapabilities, WindowingBackend};
use crate::services::event_bus::EventBus;
use crate::state::AppState;
use crate::widget::window::IslandWindow;

pub struct Application {
    pub config: AppConfig,
    pub state: AppState,
    pub window: IslandWindow,
    pub event_bus: EventBus,
    pub backend: WindowingBackend,
}

impl Application {
    pub fn new() -> AppResult<Self> {
        let config = AppConfig::default();
        let state = AppState::new();
        let backend = PlatformCapabilities::detect_backend();

        let window = IslandWindow::new(
            config.appearance.width_compact as f64,
            config.appearance.height_compact as f64,
            config.appearance.width_expanded as f64,
            config.appearance.height_expanded as f64,
        );

        let event_bus = EventBus::new();

        Ok(Self {
            config,
            state,
            window,
            event_bus,
            backend,
        })
    }

    pub fn run(&mut self) -> AppResult<()> {
        log::info!(
            "Linux Dynamic Island initialized on backend: {:?}",
            self.backend
        );
        log::info!(
            "Geometry: compact {}x{}, expanded {}x{}",
            self.config.appearance.width_compact,
            self.config.appearance.height_compact,
            self.config.appearance.width_expanded,
            self.config.appearance.height_expanded
        );
        Ok(())
    }
}
