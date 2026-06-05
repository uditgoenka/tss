#![allow(dead_code)]

#[path = "../../src/core/command.rs"]
mod command;
#[path = "../../src/core/filter_engine.rs"]
mod filter_engine;
#[path = "../../src/core/policy.rs"]
mod policy;
#[path = "../../src/core/raw_store.rs"]
mod raw_store;
#[path = "../../src/core/shell.rs"]
mod shell;

use command::CommandSpec;
use filter_engine::{FilterOutcome, StructuredFormat, ValidationError, Validator};
use policy::{CommandClass, SafetyDecision, SafetyGate, Support};
use raw_store::RawOutput;

fn command<const N: usize>(program: &str, args: [&str; N]) -> CommandSpec {
    CommandSpec::new(program, args.iter().map(|arg| (*arg).to_string()).collect())
}

#[test]
fn complex_shell_syntax_passes_through_even_when_filter_supports_command() {
    let gate = SafetyGate::default();

    let decision = gate.decide(
        &command("cargo", ["test", "|", "head", "-n", "20"]),
        Support::Exact,
    );

    assert_eq!(
        decision,
        SafetyDecision::PassthroughUnsafe("pipe syntax is unsafe to filter")
    );
}

#[test]
fn exact_supported_simple_command_can_be_filtered() {
    let gate = SafetyGate::default();

    let decision = gate.decide(
        &command("git", ["status", "--short", "--branch"]),
        Support::Exact,
    );

    assert_eq!(decision, SafetyDecision::FilterAllowed);
}

#[test]
fn unsupported_plain_text_command_passes_through_without_filtering() {
    let gate = SafetyGate::default();

    let decision = gate.decide(
        &command("unknown-tool", ["--version"]),
        Support::Unsupported,
    );

    assert_eq!(decision, SafetyDecision::PassthroughUnsupported);
}

#[test]
fn command_vocabulary_classifies_without_claiming_unimplemented_parity() {
    let gate = SafetyGate::default();

    assert_eq!(
        gate.classify(
            &command("git", ["status", "--short", "--branch"]),
            Support::Exact,
        ),
        CommandClass::Optimized
    );
    assert_eq!(
        gate.classify(
            &command("unknown-tool", ["--version"]),
            Support::Unsupported,
        ),
        CommandClass::PassthroughCompatible
    );
    assert_eq!(
        gate.classify(&command("kubectl", ["get", "pods"]), Support::Unsupported,),
        CommandClass::Planned("kubectl")
    );
    assert_eq!(
        gate.classify(&command("cargo", ["test", "|", "head"]), Support::Exact),
        CommandClass::BlockedByTrustContract("pipe syntax is unsafe to filter")
    );
}

#[test]
fn planned_parity_command_passes_through_honestly() {
    let gate = SafetyGate::default();

    let decision = gate.decide(&command("kubectl", ["get", "pods"]), Support::Unsupported);

    assert_eq!(decision, SafetyDecision::PassthroughPlanned("kubectl"));
}

#[test]
fn structured_output_requires_exact_filter_support() {
    let gate = SafetyGate::default();

    let decision = gate.decide(
        &command("cargo", ["metadata", "--format=json"]),
        Support::Unsupported,
    );

    assert_eq!(
        decision,
        SafetyDecision::PassthroughUnsafe("structured output requires exact filter support")
    );
}

#[test]
fn destructive_guard_can_deny_explicitly_destructive_commands() {
    let gate = SafetyGate::with_destructive_guard();

    let decision = gate.decide(&command("rm", ["-rf", "target"]), Support::Exact);

    assert_eq!(
        decision,
        SafetyDecision::Deny("destructive guard blocked command")
    );
}

#[test]
fn validator_rejects_changed_exit_code() {
    let raw = RawOutput::new(b"compiler failed\n".to_vec(), Vec::new(), 101);
    let filtered = FilterOutcome::lossless("compiler failed\n", 0);

    let error = Validator
        .validate(&raw, &filtered)
        .expect_err("changed exit code must be rejected");

    assert_eq!(
        error,
        ValidationError::ExitCodeChanged {
            raw: 101,
            filtered: 0
        }
    );
}

#[test]
fn validator_rejects_fake_success_output_on_non_zero_exit() {
    let raw = RawOutput::new(
        b"test failed: expected true\n".to_vec(),
        b"error: assertion failed\n".to_vec(),
        1,
    );
    let filtered = FilterOutcome::lossless("all tests passed\n", 1);

    let error = Validator
        .validate(&raw, &filtered)
        .expect_err("non-zero output cannot be rewritten as success");

    assert_eq!(error, ValidationError::NonZeroExitLooksSuccessful);
}

#[test]
fn validator_requires_lossy_marker_when_bytes_are_removed() {
    let raw = RawOutput::new(b"line 1\nline 2\nline 3\n".to_vec(), Vec::new(), 0);
    let filtered = FilterOutcome::lossless("line 1\n", 0);

    let error = Validator
        .validate(&raw, &filtered)
        .expect_err("removed bytes must be marked lossy");

    assert_eq!(
        error,
        ValidationError::BytesRemovedWithoutLossyMarker {
            raw_bytes: 21,
            filtered_bytes: 7
        }
    );
}

#[test]
fn validator_rejects_invalid_json_when_structured_output_is_required() {
    let raw = RawOutput::new(br#"{"ok":true}"#.to_vec(), Vec::new(), 0);
    let filtered = FilterOutcome::lossless(r#"{"ok":true"#, 0)
        .requiring_structured_format(StructuredFormat::Json);

    let error = Validator
        .validate(&raw, &filtered)
        .expect_err("invalid JSON must pass through instead of being emitted");

    assert_eq!(
        error,
        ValidationError::InvalidStructuredOutput(StructuredFormat::Json)
    );
}

#[test]
fn validator_rejects_corrupted_diff_when_diff_output_is_required() {
    let raw = RawOutput::new(
        b"diff --git a/src/lib.rs b/src/lib.rs\n--- a/src/lib.rs\n+++ b/src/lib.rs\n@@ -1 +1 @@\n-old\n+new\n"
            .to_vec(),
        Vec::new(),
        0,
    );
    let filtered = FilterOutcome::lossy("old -> new\n", 0, 87, 1)
        .requiring_structured_format(StructuredFormat::Diff);

    let error = Validator
        .validate(&raw, &filtered)
        .expect_err("corrupted diff must pass through instead of being emitted");

    assert_eq!(
        error,
        ValidationError::InvalidStructuredOutput(StructuredFormat::Diff)
    );
}
