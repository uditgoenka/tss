use super::{CommandArgumentStorage, PrivacyConfig};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetentionPolicy {
    pub days: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalStoragePolicy {
    pub raw_store_enabled: bool,
    pub analytics_enabled: bool,
    pub command_args_stored: bool,
    pub cwd_stored: bool,
    pub retention: RetentionPolicy,
}

impl LocalStoragePolicy {
    pub fn from_env(config: PrivacyConfig) -> Self {
        let no_store = env_enabled("TSS_NO_STORE");
        let no_analytics = env_enabled("TSS_NO_ANALYTICS");

        Self {
            raw_store_enabled: config.raw_store_enabled && !no_store,
            analytics_enabled: config.analytics_enabled && !no_analytics,
            command_args_stored: config.command_argument_storage
                == CommandArgumentStorage::FullCommand,
            cwd_stored: config.include_cwd,
            retention: RetentionPolicy {
                days: config.retention_days,
            },
        }
    }
}

fn env_enabled(name: &str) -> bool {
    match std::env::var(name) {
        Ok(value) => matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "1" | "true" | "yes" | "on"
        ),
        Err(_) => false,
    }
}
