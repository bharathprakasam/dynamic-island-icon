pub mod app;
pub mod applications;
pub mod config;
pub mod errors;
pub mod logging;
pub mod platform;
pub mod services;
pub mod settings;
pub mod state;
pub mod ui;
pub mod widget;

use errors::AppResult;

fn main() -> AppResult<()> {
    logging::init()?;
    log::info!("Starting linux-dynamic-island daemon...");

    let mut app = app::Application::new()?;
    app.run()?;

    Ok(())
}
