use crate::config::schema::AppConfig;
use crate::errors::AppResult;

pub const CURRENT_CONFIG_VERSION: u32 = 1;

pub fn migrate_if_needed(config: AppConfig) -> AppResult<AppConfig> {
    let mut updated = config;
    if updated.version < CURRENT_CONFIG_VERSION {
        log::info!("Migrating configuration from v{} to v{}", updated.version, CURRENT_CONFIG_VERSION);
        updated.version = CURRENT_CONFIG_VERSION;
    }
    Ok(updated)
}
