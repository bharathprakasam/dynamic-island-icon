use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Platform error: {0}")]
    Platform(String),

    #[error("Desktop entry parsing error: {0}")]
    DesktopEntry(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Internal service error: {0}")]
    Service(String),

    #[error("General application error: {0}")]
    General(String),
}
