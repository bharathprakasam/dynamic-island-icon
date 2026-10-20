use std::path::PathBuf;
use crate::config::schema::AppConfig;
use crate::errors::{AppError, AppResult};

pub fn get_config_dir() -> AppResult<PathBuf> {
    let home = std::env::var("HOME").map_err(|_| AppError::Config("HOME directory not set".into()))?;
    Ok(PathBuf::from(home).join(".config").join("linux-dynamic-island"))
}

pub fn get_config_path() -> AppResult<PathBuf> {
    Ok(get_config_dir()?.join("config.toml"))
}

pub fn load_config() -> AppResult<AppConfig> {
    let path = get_config_path()?;
    if !path.exists() {
        return Ok(AppConfig::default());
    }

    let contents = std::fs::read_to_string(&path)?;
    toml::from_str(&contents).map_err(|e| AppError::Config(format!("Failed to parse config: {}", e)))
}

pub fn save_config(config: &AppConfig) -> AppResult<()> {
    let dir = get_config_dir()?;
    if !dir.exists() {
        std::fs::create_dir_all(&dir)?;
    }

    let serialized = toml::to_string_pretty(config)
        .map_err(|e| AppError::Config(format!("Failed to serialize config: {}", e)))?;
    std::fs::write(get_config_path()?, serialized)?;
    Ok(())
}
