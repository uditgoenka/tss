#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandArgumentStorage {
    Redacted,
    FullCommand,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrivacyConfig {
    pub analytics_enabled: bool,
    pub raw_store_enabled: bool,
    pub command_argument_storage: CommandArgumentStorage,
    pub include_cwd: bool,
    pub retention_days: u16,
}

impl Default for PrivacyConfig {
    fn default() -> Self {
        Self {
            analytics_enabled: true,
            raw_store_enabled: true,
            command_argument_storage: CommandArgumentStorage::Redacted,
            include_cwd: false,
            retention_days: 30,
        }
    }
}

pub fn redact_command_preview<I, S>(command: I, config: &PrivacyConfig) -> String
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let parts = command
        .into_iter()
        .map(|part| part.as_ref().to_string())
        .collect::<Vec<_>>();

    if parts.is_empty() {
        return String::from("<empty>");
    }

    match config.command_argument_storage {
        CommandArgumentStorage::FullCommand => parts.join(" "),
        CommandArgumentStorage::Redacted => {
            if parts.len() == 1 {
                parts[0].clone()
            } else {
                format!("{} [args redacted: {}]", parts[0], parts.len() - 1)
            }
        }
    }
}
