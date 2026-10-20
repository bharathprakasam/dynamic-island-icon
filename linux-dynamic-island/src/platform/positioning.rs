use crate::errors::AppResult;
use crate::platform::capabilities::{PlatformCapabilities, WindowingBackend};

pub struct PlatformPositioning;

impl PlatformPositioning {
    pub fn apply_layer_shell_anchors() -> AppResult<()> {
        match PlatformCapabilities::detect_backend() {
            WindowingBackend::Wayland => {
                log::info!("Configuring gtk4-layer-shell surface on Wayland");
            }
            WindowingBackend::X11 => {
                log::info!("Configuring X11 _NET_WM_WINDOW_TYPE_DOCK / strut partials");
            }
            WindowingBackend::Unknown => {
                log::warn!("Unknown windowing environment, using standard window position");
            }
        }
        Ok(())
    }
}
