pub mod desktop_entry;
pub mod discovery;
pub mod icons;
pub mod launcher;

pub use desktop_entry::DesktopEntry;
pub use discovery::ApplicationDiscovery;
pub use icons::IconResolver;
pub use launcher::ApplicationLauncher;
