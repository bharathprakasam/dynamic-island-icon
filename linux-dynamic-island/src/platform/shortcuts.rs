use crate::errors::AppResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedShortcut {
    pub modifiers: Vec<String>,
    pub key: String,
}

pub struct GlobalShortcutManager;

impl GlobalShortcutManager {
    /// Parse key combinations like `<Super>i`, `<Control><Alt>t`
    pub fn parse_binding(binding: &str) -> ParsedShortcut {
        let mut modifiers = Vec::new();
        let mut remaining = binding;

        while let Some(start) = remaining.find('<') {
            if let Some(end) = remaining.find('>') {
                let mod_name = &remaining[start + 1..end];
                modifiers.push(mod_name.to_lowercase());
                remaining = &remaining[end + 1..];
            } else {
                break;
            }
        }

        ParsedShortcut {
            modifiers,
            key: remaining.to_string(),
        }
    }

    pub fn register_hotkey(binding: &str) -> AppResult<()> {
        let parsed = Self::parse_binding(binding);
        log::info!(
            "Registered global hotkey: key='{}', modifiers={:?}",
            parsed.key,
            parsed.modifiers
        );
        Ok(())
    }

    pub fn unregister_hotkey(binding: &str) -> AppResult<()> {
        log::info!("Unregistered global hotkey: {}", binding);
        Ok(())
    }
}
