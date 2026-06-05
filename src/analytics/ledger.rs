use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::privacy::{redact_command_preview, LocalStoragePolicy, PrivacyConfig};

use super::estimate_tokens;
use super::parity::{classify_command_parity, CommandParityStatus};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PassthroughReason {
    UnsafeShell,
    Unsupported,
    LargerOutput,
    StoreDisabled,
    FilterError,
    Other(String),
}

impl PassthroughReason {
    pub fn as_str(&self) -> &str {
        match self {
            Self::UnsafeShell => "unsafe_shell",
            Self::Unsupported => "unsupported",
            Self::LargerOutput => "larger_output",
            Self::StoreDisabled => "store_disabled",
            Self::FilterError => "filter_error",
            Self::Other(reason) => reason.as_str(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SafetyDecision {
    Filtered,
    Passthrough(PassthroughReason),
    Denied(String),
}

impl SafetyDecision {
    pub fn label(&self) -> &str {
        match self {
            Self::Filtered => "filtered",
            Self::Passthrough(_) => "passthrough",
            Self::Denied(_) => "denied",
        }
    }

    pub fn passthrough_reason(&self) -> Option<&str> {
        match self {
            Self::Passthrough(reason) => Some(reason.as_str()),
            _ => None,
        }
    }

    pub fn is_failure_for_gain(&self, raw_bytes: u64, emitted_bytes: u64) -> bool {
        matches!(self, Self::Passthrough(_)) || emitted_bytes > raw_bytes
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalyticsEvent {
    pub command: Vec<String>,
    pub command_category: String,
    pub filter_name: String,
    pub safety_decision: SafetyDecision,
    pub raw_bytes: u64,
    pub emitted_bytes: u64,
    pub omitted_bytes: u64,
    pub raw_tokens_estimate: u64,
    pub emitted_tokens_estimate: u64,
    pub omitted_tokens_estimate: u64,
    pub provider_cache_caveat: bool,
    pub command_parity: CommandParityStatus,
    pub timestamp_ms: u128,
}

impl AnalyticsEvent {
    pub fn new<I, S>(
        command: I,
        command_category: impl Into<String>,
        filter_name: impl Into<String>,
        safety_decision: SafetyDecision,
        raw_bytes: u64,
        emitted_bytes: u64,
    ) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let command = command
            .into_iter()
            .map(|part| part.as_ref().to_string())
            .collect::<Vec<_>>();
        let omitted_bytes = raw_bytes.saturating_sub(emitted_bytes);
        let command_parity = classify_command_parity(command.iter().map(String::as_str));

        Self {
            command,
            command_category: command_category.into(),
            filter_name: filter_name.into(),
            safety_decision,
            raw_bytes,
            emitted_bytes,
            omitted_bytes,
            raw_tokens_estimate: estimate_tokens(raw_bytes),
            emitted_tokens_estimate: estimate_tokens(emitted_bytes),
            omitted_tokens_estimate: estimate_tokens(omitted_bytes),
            provider_cache_caveat: true,
            command_parity,
            timestamp_ms: now_ms(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AnalyticsLedger {
    path: PathBuf,
    config: PrivacyConfig,
}

impl AnalyticsLedger {
    pub fn new(path: impl Into<PathBuf>, config: PrivacyConfig) -> Self {
        Self {
            path: path.into(),
            config,
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn record(&self, event: AnalyticsEvent) -> io::Result<()> {
        let policy = LocalStoragePolicy::from_env(self.config.clone());
        if !policy.analytics_enabled {
            return Ok(());
        }

        ensure_private_parent_dir(&self.path)?;

        let mut options = OpenOptions::new();
        options.create(true).append(true);
        set_private_file_mode(&mut options);

        let mut file = options.open(&self.path)?;
        writeln!(file, "{}", self.to_json_line(&event))
    }

    fn to_json_line(&self, event: &AnalyticsEvent) -> String {
        let command_preview =
            redact_command_preview(event.command.iter().map(String::as_str), &self.config);
        let passthrough_reason = event.safety_decision.passthrough_reason().unwrap_or("");

        format!(
            concat!(
                "{{",
                "\"timestamp_ms\":{},",
                "\"command_preview\":\"{}\",",
                "\"command_category\":\"{}\",",
                "\"filter_name\":\"{}\",",
                "\"command_parity\":\"{}\",",
                "\"safety_decision\":\"{}\",",
                "\"passthrough_reason\":\"{}\",",
                "\"raw_bytes\":{},",
                "\"emitted_bytes\":{},",
                "\"omitted_bytes\":{},",
                "\"raw_tokens_estimate\":{},",
                "\"emitted_tokens_estimate\":{},",
                "\"omitted_tokens_estimate\":{},",
                "\"provider_cache_caveat\":{}",
                "}}"
            ),
            event.timestamp_ms,
            escape_json(&command_preview),
            escape_json(&event.command_category),
            escape_json(&event.filter_name),
            event.command_parity.as_str(),
            event.safety_decision.label(),
            escape_json(passthrough_reason),
            event.raw_bytes,
            event.emitted_bytes,
            event.omitted_bytes,
            event.raw_tokens_estimate,
            event.emitted_tokens_estimate,
            event.omitted_tokens_estimate,
            event.provider_cache_caveat
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GainReport {
    pub raw_bytes: u64,
    pub emitted_bytes: u64,
    pub omitted_bytes: u64,
    pub raw_tokens_estimate: u64,
    pub emitted_tokens_estimate: u64,
    pub omitted_tokens_estimate: u64,
    pub event_count: u64,
    pub failure_count: u64,
    pub optimized_events: u64,
    pub passthrough_compatible_events: u64,
    pub planned_events: u64,
    pub blocked_events: u64,
    pub provider_cache_caveat: bool,
    pub command_rows: Vec<GainCommand>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GainCommand {
    pub command: String,
    pub count: u64,
    pub raw_tokens_estimate: u64,
    pub emitted_tokens_estimate: u64,
    pub omitted_tokens_estimate: u64,
}

impl GainReport {
    pub fn from_ledger(ledger: &AnalyticsLedger) -> io::Result<Self> {
        let contents = match fs::read_to_string(ledger.path()) {
            Ok(contents) => contents,
            Err(error) if error.kind() == io::ErrorKind::NotFound => String::new(),
            Err(error) => return Err(error),
        };

        let mut report = Self {
            raw_bytes: 0,
            emitted_bytes: 0,
            omitted_bytes: 0,
            raw_tokens_estimate: 0,
            emitted_tokens_estimate: 0,
            omitted_tokens_estimate: 0,
            event_count: 0,
            failure_count: 0,
            optimized_events: 0,
            passthrough_compatible_events: 0,
            planned_events: 0,
            blocked_events: 0,
            provider_cache_caveat: true,
            command_rows: Vec::new(),
        };
        let mut command_rows = BTreeMap::<String, GainCommand>::new();

        for line in contents.lines().filter(|line| !line.trim().is_empty()) {
            report.event_count += 1;
            let raw = extract_u64(line, "raw_bytes").unwrap_or(0);
            let emitted = extract_u64(line, "emitted_bytes").unwrap_or(0);
            let omitted = extract_u64(line, "omitted_bytes").unwrap_or(0);

            report.raw_bytes += raw;
            report.emitted_bytes += emitted;
            report.omitted_bytes += omitted;
            let raw_tokens = extract_u64(line, "raw_tokens_estimate").unwrap_or(0);
            let emitted_tokens = extract_u64(line, "emitted_tokens_estimate").unwrap_or(0);
            let omitted_tokens = extract_u64(line, "omitted_tokens_estimate").unwrap_or(0);
            report.raw_tokens_estimate += raw_tokens;
            report.emitted_tokens_estimate += emitted_tokens;
            report.omitted_tokens_estimate += omitted_tokens;

            let command = extract_string(line, "command_preview")
                .filter(|value| !value.is_empty())
                .unwrap_or_else(|| String::from("(unknown command)"));
            let row = command_rows.entry(command.clone()).or_insert(GainCommand {
                command,
                count: 0,
                raw_tokens_estimate: 0,
                emitted_tokens_estimate: 0,
                omitted_tokens_estimate: 0,
            });
            row.count += 1;
            row.raw_tokens_estimate += raw_tokens;
            row.emitted_tokens_estimate += emitted_tokens;
            row.omitted_tokens_estimate += omitted_tokens;

            if line.contains("\"safety_decision\":\"passthrough\"") || emitted > raw {
                report.failure_count += 1;
            }

            match extract_string(line, "command_parity").as_deref() {
                Some("optimized") => report.optimized_events += 1,
                Some("passthrough-compatible") => report.passthrough_compatible_events += 1,
                Some("planned") => report.planned_events += 1,
                Some("blocked") => report.blocked_events += 1,
                _ => {}
            }
        }

        report.command_rows = command_rows.into_values().collect::<Vec<_>>();
        report.command_rows.sort_by(|left, right| {
            right
                .omitted_tokens_estimate
                .cmp(&left.omitted_tokens_estimate)
                .then_with(|| right.count.cmp(&left.count))
                .then_with(|| left.command.cmp(&right.command))
        });

        Ok(report)
    }

    pub fn human_summary(&self) -> String {
        let mut output = String::new();
        let savings_pct = percent(self.omitted_tokens_estimate, self.raw_tokens_estimate);

        output.push_str("TSS Token Savings (Local Scope)\n");
        output.push_str("============================================================\n\n");
        writeln!(output, "{:<24} {:>12}", "Total commands:", self.event_count).unwrap();
        writeln!(
            output,
            "{:<24} {:>12}",
            "Input tokens:",
            compact_number(self.raw_tokens_estimate)
        )
        .unwrap();
        writeln!(
            output,
            "{:<24} {:>12}",
            "Output tokens:",
            compact_number(self.emitted_tokens_estimate)
        )
        .unwrap();
        writeln!(
            output,
            "{:<24} {:>12} ({:>5.1}%)",
            "Tokens saved:",
            compact_number(self.omitted_tokens_estimate),
            savings_pct
        )
        .unwrap();
        writeln!(
            output,
            "{:<24} {:>12}",
            "Safety fallbacks:", self.failure_count
        )
        .unwrap();
        writeln!(
            output,
            "{:<24} {} {:>5.1}%",
            "Efficiency meter:",
            meter(savings_pct),
            savings_pct
        )
        .unwrap();

        output.push_str("\nCommand Coverage\n");
        output.push_str("------------------------------------------------------------\n");
        writeln!(output, "{:<28} {:>12}", "optimized", self.optimized_events).unwrap();
        writeln!(
            output,
            "{:<28} {:>12}",
            "passthrough-compatible", self.passthrough_compatible_events
        )
        .unwrap();
        writeln!(output, "{:<28} {:>12}", "planned", self.planned_events).unwrap();
        writeln!(output, "{:<28} {:>12}", "blocked", self.blocked_events).unwrap();

        output.push_str("\nBy Command\n");
        output.push_str("------------------------------------------------------------\n");
        if self.command_rows.is_empty() {
            output.push_str("No command events recorded yet.\n");
        } else {
            output
                .push_str(" #  Command                         Count     Saved    Avg%  Impact\n");
            output.push_str("------------------------------------------------------------\n");
            for (index, row) in self.command_rows.iter().take(10).enumerate() {
                let avg = percent(row.omitted_tokens_estimate, row.raw_tokens_estimate);
                writeln!(
                    output,
                    "{:>2}. {:<30} {:>6} {:>9} {:>6.1}%  {}",
                    index + 1,
                    truncate(&row.command, 30),
                    row.count,
                    compact_number(row.omitted_tokens_estimate),
                    avg,
                    meter(avg)
                )
                .unwrap();
            }
        }

        output.push_str(
            "\nestimated from bytes; actual billing depends on tokenizer and provider cache behavior.\n",
        );
        output
    }

    pub fn to_json(&self) -> String {
        let command_rows = self
            .command_rows
            .iter()
            .map(|row| {
                format!(
                    concat!(
                        "{{",
                        "\"command\":\"{}\",",
                        "\"count\":{},",
                        "\"raw_tokens_estimate\":{},",
                        "\"emitted_tokens_estimate\":{},",
                        "\"omitted_tokens_estimate\":{}",
                        "}}"
                    ),
                    escape_json(&row.command),
                    row.count,
                    row.raw_tokens_estimate,
                    row.emitted_tokens_estimate,
                    row.omitted_tokens_estimate
                )
            })
            .collect::<Vec<_>>()
            .join(",");

        format!(
            concat!(
                "{{",
                "\"raw_bytes\":{},",
                "\"emitted_bytes\":{},",
                "\"omitted_bytes\":{},",
                "\"raw_tokens_estimate\":{},",
                "\"emitted_tokens_estimate\":{},",
                "\"omitted_tokens_estimate\":{},",
                "\"event_count\":{},",
                "\"failure_count\":{},",
                "\"optimized_events\":{},",
                "\"passthrough_compatible_events\":{},",
                "\"planned_events\":{},",
                "\"blocked_events\":{},",
                "\"provider_cache_caveat\":{},",
                "\"command_rows\":[{}]",
                "}}"
            ),
            self.raw_bytes,
            self.emitted_bytes,
            self.omitted_bytes,
            self.raw_tokens_estimate,
            self.emitted_tokens_estimate,
            self.omitted_tokens_estimate,
            self.event_count,
            self.failure_count,
            self.optimized_events,
            self.passthrough_compatible_events,
            self.planned_events,
            self.blocked_events,
            self.provider_cache_caveat,
            command_rows
        )
    }
}

fn percent(numerator: u64, denominator: u64) -> f64 {
    if denominator == 0 {
        0.0
    } else {
        (numerator as f64 / denominator as f64) * 100.0
    }
}

fn compact_number(value: u64) -> String {
    if value >= 1_000_000 {
        format!("{:.1}M", value as f64 / 1_000_000.0)
    } else if value >= 1_000 {
        format!("{:.1}K", value as f64 / 1_000.0)
    } else {
        value.to_string()
    }
}

fn meter(percent: f64) -> String {
    let filled = ((percent.clamp(0.0, 100.0) / 5.0).round() as usize).min(20);
    let empty = 20 - filled;
    format!("[{}{}]", "#".repeat(filled), "-".repeat(empty))
}

fn truncate(value: &str, max_chars: usize) -> String {
    if value.chars().count() <= max_chars {
        return value.to_string();
    }

    let mut truncated = value
        .chars()
        .take(max_chars.saturating_sub(3))
        .collect::<String>();
    truncated.push_str("...");
    truncated
}

fn now_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or(0)
}

fn extract_u64(line: &str, key: &str) -> Option<u64> {
    let needle = format!("\"{}\":", key);
    let start = line.find(&needle)? + needle.len();
    let rest = &line[start..];
    let digits = rest
        .chars()
        .take_while(|character| character.is_ascii_digit())
        .collect::<String>();
    digits.parse().ok()
}

fn extract_string(line: &str, key: &str) -> Option<String> {
    let needle = format!("\"{}\":\"", key);
    let start = line.find(&needle)? + needle.len();
    let rest = &line[start..];
    let end = rest.find('"')?;
    Some(rest[..end].to_string())
}

fn escape_json(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for character in value.chars() {
        match character {
            '\\' => escaped.push_str("\\\\"),
            '"' => escaped.push_str("\\\""),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            character if character.is_control() => {
                escaped.push_str(&format!("\\u{:04x}", character as u32))
            }
            character => escaped.push(character),
        }
    }
    escaped
}

#[cfg(unix)]
fn set_private_file_mode(options: &mut OpenOptions) {
    use std::os::unix::fs::OpenOptionsExt;
    options.mode(0o600);
}

#[cfg(not(unix))]
fn set_private_file_mode(_options: &mut OpenOptions) {}

fn ensure_private_parent_dir(path: &Path) -> io::Result<()> {
    let Some(parent) = path.parent() else {
        return Ok(());
    };

    let missing = missing_ancestors(parent);
    fs::create_dir_all(parent)?;
    for directory in missing {
        set_private_dir_mode(&directory)?;
    }
    Ok(())
}

fn missing_ancestors(path: &Path) -> Vec<PathBuf> {
    let mut missing = Vec::new();
    let mut current = Some(path);
    while let Some(candidate) = current {
        if candidate.as_os_str().is_empty() {
            break;
        }
        if candidate.exists() {
            break;
        }
        missing.push(candidate.to_path_buf());
        current = candidate.parent();
    }
    missing.reverse();
    missing
}

#[cfg(unix)]
fn set_private_dir_mode(path: &Path) -> io::Result<()> {
    use std::os::unix::fs::PermissionsExt;

    let mut permissions = fs::metadata(path)?.permissions();
    permissions.set_mode(0o700);
    fs::set_permissions(path, permissions)
}

#[cfg(not(unix))]
fn set_private_dir_mode(_path: &Path) -> io::Result<()> {
    Ok(())
}
