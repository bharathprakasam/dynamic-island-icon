pub mod autostart;
pub mod capabilities;
pub mod positioning;
pub mod shortcuts;
pub mod transparency;

pub use autostart::AutostartManager;
pub use capabilities::{PlatformCapabilities, WindowingBackend};
pub use positioning::PlatformPositioning;
pub use shortcuts::{GlobalShortcutManager, ParsedShortcut};
pub use transparency::TransparencyManager;
