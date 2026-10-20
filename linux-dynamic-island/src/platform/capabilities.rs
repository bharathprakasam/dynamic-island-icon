#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowingBackend {
    Wayland,
    X11,
    Unknown,
}

pub struct PlatformCapabilities;

impl PlatformCapabilities {
    pub fn detect_backend() -> WindowingBackend {
        if let Ok(wayland_display) = std::env::var("WAYLAND_DISPLAY") {
            if !wayland_display.is_empty() {
                return WindowingBackend::Wayland;
            }
        }
        if let Ok(display) = std::env::var("DISPLAY") {
            if !display.is_empty() {
                return WindowingBackend::X11;
            }
        }
        WindowingBackend::Unknown
    }

    pub fn supports_layer_shell() -> bool {
        Self::detect_backend() == WindowingBackend::Wayland
    }
}
