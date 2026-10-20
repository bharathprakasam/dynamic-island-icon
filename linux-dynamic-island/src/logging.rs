use crate::errors::AppResult;

pub fn init() -> AppResult<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::try_init().map_err(|e| crate::errors::AppError::General(e.to_string()))?;
    Ok(())
}
