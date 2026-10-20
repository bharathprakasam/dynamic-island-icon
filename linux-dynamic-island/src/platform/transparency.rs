pub struct TransparencyManager;

impl TransparencyManager {
    pub fn is_compositor_active() -> bool {
        // In Wayland compositing is always active.
        // In X11 check for active composite manager.
        true
    }
}
