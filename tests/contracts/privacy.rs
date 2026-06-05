#![allow(dead_code, unused_imports)]

#[path = "../../src/analytics/mod.rs"]
mod analytics;
#[path = "../../src/privacy/mod.rs"]
mod privacy;

use analytics::{
    classify_command_parity, estimate_tokens, issue_class_coverage, known_command_coverage,
    AnalyticsEvent, AnalyticsLedger, CommandParityStatus, GainReport, PassthroughReason,
    SafetyDecision,
};
use privacy::{redact_command_preview, LocalStoragePolicy, PrivacyConfig};
use std::sync::{Mutex, OnceLock};

fn env_lock() -> std::sync::MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
}

fn temp_path(name: &str) -> std::path::PathBuf {
    let unique = format!("tss-privacy-{}-{}", name, std::process::id());
    std::env::temp_dir().join(unique)
}

#[test]
fn default_analytics_record_redacts_full_command_args() {
    let _guard = env_lock();
    let path = temp_path("redacts.jsonl");
    let _ = std::fs::remove_file(&path);
    let ledger = AnalyticsLedger::new(path.clone(), PrivacyConfig::default());

    ledger
        .record(AnalyticsEvent::new(
            ["deploy", "--token", "super-secret", "--prod"],
            "shell",
            "passthrough",
            SafetyDecision::Passthrough(PassthroughReason::UnsafeShell),
            1200,
            1200,
        ))
        .unwrap();

    let contents = std::fs::read_to_string(path).unwrap();
    assert!(contents.contains("\"command_preview\":\"deploy [args redacted: 3]\""));
    assert!(!contents.contains("super-secret"));
    assert!(!contents.contains("--token"));
}

#[test]
fn analytics_record_keeps_counts_filter_decision_and_token_estimates_separate() {
    let _guard = env_lock();
    let path = temp_path("counts.jsonl");
    let _ = std::fs::remove_file(&path);
    let ledger = AnalyticsLedger::new(path.clone(), PrivacyConfig::default());

    ledger
        .record(AnalyticsEvent::new(
            ["git", "status", "--short"],
            "git",
            "git-status",
            SafetyDecision::Filtered,
            1000,
            240,
        ))
        .unwrap();

    let contents = std::fs::read_to_string(path).unwrap();
    assert!(contents.contains("\"raw_bytes\":1000"));
    assert!(contents.contains("\"emitted_bytes\":240"));
    assert!(contents.contains("\"omitted_bytes\":760"));
    assert!(contents.contains("\"raw_tokens_estimate\":250"));
    assert!(contents.contains("\"provider_cache_caveat\":true"));
    assert!(contents.contains("\"safety_decision\":\"filtered\""));
    assert!(contents.contains("\"command_parity\":\"optimized\""));
}

#[test]
fn privacy_policy_env_toggles_disable_raw_store_and_analytics() {
    let _guard = env_lock();
    std::env::set_var("TSS_NO_STORE", "1");
    std::env::set_var("TSS_NO_ANALYTICS", "1");

    let policy = LocalStoragePolicy::from_env(PrivacyConfig::default());

    std::env::remove_var("TSS_NO_STORE");
    std::env::remove_var("TSS_NO_ANALYTICS");

    assert!(!policy.raw_store_enabled);
    assert!(!policy.analytics_enabled);
}

#[test]
fn disabled_analytics_does_not_create_a_ledger_file() {
    let _guard = env_lock();
    let path = temp_path("disabled.jsonl");
    let _ = std::fs::remove_file(&path);
    let config = PrivacyConfig {
        analytics_enabled: false,
        ..PrivacyConfig::default()
    };
    let ledger = AnalyticsLedger::new(path.clone(), config);

    ledger
        .record(AnalyticsEvent::new(
            ["git", "status"],
            "git",
            "git-status",
            SafetyDecision::Filtered,
            400,
            120,
        ))
        .unwrap();

    assert!(!path.exists());
}

#[cfg(unix)]
#[test]
fn analytics_creates_private_directory_and_file() {
    use std::os::unix::fs::PermissionsExt;

    let _guard = env_lock();
    let dir = std::env::temp_dir().join(format!(
        "tss-analytics-private-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let path = dir.join("analytics.jsonl");
    let ledger = AnalyticsLedger::new(path.clone(), PrivacyConfig::default());

    ledger
        .record(AnalyticsEvent::new(
            ["cat", "private.log"],
            "cat",
            "cat-lines",
            SafetyDecision::Filtered,
            100,
            20,
        ))
        .unwrap();

    assert_eq!(
        std::fs::metadata(&dir).unwrap().permissions().mode() & 0o777,
        0o700
    );
    assert_eq!(
        std::fs::metadata(&path).unwrap().permissions().mode() & 0o777,
        0o600
    );

    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn gain_report_summarizes_savings_and_failures_without_billing_claims() {
    let _guard = env_lock();
    let path = temp_path("gain.jsonl");
    let _ = std::fs::remove_file(&path);
    let ledger = AnalyticsLedger::new(path.clone(), PrivacyConfig::default());

    ledger
        .record(AnalyticsEvent::new(
            ["git", "status"],
            "git",
            "git-status",
            SafetyDecision::Filtered,
            1000,
            250,
        ))
        .unwrap();
    ledger
        .record(AnalyticsEvent::new(
            ["docker", "ps"],
            "containers",
            "passthrough",
            SafetyDecision::Passthrough(PassthroughReason::Unsupported),
            800,
            800,
        ))
        .unwrap();
    ledger
        .record(AnalyticsEvent::new(
            ["env"],
            "environment",
            "passthrough",
            SafetyDecision::Passthrough(PassthroughReason::Unsupported),
            50,
            50,
        ))
        .unwrap();

    let report = GainReport::from_ledger(&ledger).unwrap();
    assert_eq!(report.raw_bytes, 1850);
    assert_eq!(report.emitted_bytes, 1100);
    assert_eq!(report.omitted_bytes, 750);
    assert_eq!(report.failure_count, 2);
    assert_eq!(report.optimized_events, 1);
    assert_eq!(report.planned_events, 1);
    assert_eq!(report.blocked_events, 1);
    assert!(report.human_summary().contains("estimated"));
    assert!(report.human_summary().contains("Command Coverage"));
    assert!(report.human_summary().contains("Safety fallbacks:"));
    assert!(report
        .command_rows
        .iter()
        .any(|row| row.command == "git [args redacted: 1]" && row.count == 1));
    assert!(report.to_json().contains("\"provider_cache_caveat\":true"));
    assert!(report.to_json().contains("\"planned_events\":1"));
    assert!(report.to_json().contains("\"command_rows\""));
    assert!(report
        .to_json()
        .contains("\"command\":\"git [args redacted: 1]\""));
}

#[test]
fn token_estimate_is_labeled_and_monotonic() {
    assert_eq!(estimate_tokens(0), 0);
    assert_eq!(estimate_tokens(1), 1);
    assert!(estimate_tokens(4096) >= estimate_tokens(1024));
    assert_eq!(
        redact_command_preview(
            ["npm", "test", "--", "--runInBand"],
            &PrivacyConfig::default()
        ),
        "npm [args redacted: 3]"
    );
}

#[test]
fn command_parity_classifies_migration_vocabulary_without_overclaiming() {
    assert_eq!(
        classify_command_parity(["git", "status", "--short"]),
        CommandParityStatus::Optimized
    );
    assert_eq!(
        classify_command_parity(["git", "diff", "--name-only"]),
        CommandParityStatus::PassthroughCompatible
    );
    assert_eq!(
        classify_command_parity(["cargo", "test"]),
        CommandParityStatus::Optimized
    );
    assert_eq!(
        classify_command_parity(["brew", "install", "node"]),
        CommandParityStatus::PassthroughCompatible
    );
    assert_eq!(
        classify_command_parity(["docker", "ps"]),
        CommandParityStatus::Planned
    );
    assert_eq!(
        classify_command_parity(["env"]),
        CommandParityStatus::Blocked
    );
    assert!(
        known_command_coverage()
            .iter()
            .any(|entry| entry.command.contains("env")
                && entry.status == CommandParityStatus::Blocked)
    );
    assert!(issue_class_coverage()
        .iter()
        .any(|entry| entry.class == "project-local-filter-trust"
            && entry.status == CommandParityStatus::Blocked));
}
