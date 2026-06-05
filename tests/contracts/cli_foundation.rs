use std::process::Command;

fn manifest_path(file_name: &str) -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(file_name)
}

fn temp_state_dir(test_name: &str) -> std::path::PathBuf {
    let unique = format!(
        "tss-cli-{test_name}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    std::env::temp_dir().join(unique)
}

fn tss_command(state_dir: &std::path::Path) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_tss"));
    command
        .env("TSS_HOME", state_dir)
        .env("TSS_NO_ANALYTICS", "1");
    command
}

fn tss_command_with_analytics(
    state_dir: &std::path::Path,
    analytics_file: &std::path::Path,
) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_tss"));
    command
        .env("TSS_HOME", state_dir)
        .env("TSS_ANALYTICS_FILE", analytics_file);
    command
}

#[test]
fn run_passthrough_preserves_stdout_and_success_exit() {
    let state_dir = temp_state_dir("run-success");
    let output = tss_command(&state_dir)
        .args(["run", "--", "printf", "hello"])
        .output()
        .expect("run tss");

    assert!(
        output.status.success(),
        "expected success, got status {:?}, stderr: {}",
        output.status.code(),
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&output.stdout), "hello");
}

#[test]
fn run_passthrough_preserves_stderr_and_failure_exit() {
    let state_dir = temp_state_dir("run-failure");
    let output = tss_command(&state_dir)
        .args([
            "run",
            "--",
            "/bin/sh",
            "-c",
            "printf out; printf err >&2; exit 7",
        ])
        .output()
        .expect("run tss");

    assert_eq!(output.status.code(), Some(7));
    assert_eq!(String::from_utf8_lossy(&output.stdout), "out");
    assert_eq!(String::from_utf8_lossy(&output.stderr), "err");
}

#[test]
fn package_declares_apache_2_license_and_ships_license_text() {
    let manifest = std::fs::read_to_string(manifest_path("Cargo.toml")).expect("read manifest");
    let license = std::fs::read_to_string(manifest_path("LICENSE")).expect("read license");

    assert!(manifest.contains("license = \"Apache-2.0\""));
    assert!(license.contains("Apache License"));
    assert!(license.contains("Version 2.0, January 2004"));
}

#[test]
fn help_advertises_phase_2_command_surface() {
    let state_dir = temp_state_dir("help");
    let output = tss_command(&state_dir)
        .arg("--help")
        .output()
        .expect("run tss help");

    assert!(
        output.status.success(),
        "expected help success, got stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    for expected in [
        "tss run -- <cmd>",
        "tss -- <cmd>",
        "tss <cmd>",
        "tss proxy <cmd>",
        "tss raw <id>",
        "tss doctor",
        "tss compat",
        "tss gain",
        "tss shell-init",
        "tss init [agent|--agent <agent>]",
        "tss verify",
        "tss --version",
    ] {
        assert!(stdout.contains(expected), "missing help entry: {expected}");
    }
}

#[test]
fn foundation_skeleton_commands_are_recognized() {
    let state_dir = temp_state_dir("skeleton");
    let success_cases: &[(&[&str], &str)] = &[
        (&["gain"], "TSS Token Savings (Local Scope)"),
        (
            &["init", "codex", "--dry-run"],
            "tss init codex: dry run\nscope: Project\nmode: InstructionOnly\n- Check `codex --version`; install only instructions unless the user opts into a shell wrapper.\n- Write reviewable TSS guidance for Codex; merge into AGENTS.md manually after review.\n- Optional shell wrapper for users who explicitly opt in; not enabled automatically.\nwould write ",
        ),
        (&["verify"], "tss verify: ok\n"),
    ];

    for (args, expected_stdout) in success_cases {
        let output = tss_command(&state_dir)
            .args(*args)
            .output()
            .expect("run tss skeleton command");

        assert!(
            output.status.success(),
            "expected success for {:?}, got stderr: {}",
            args,
            String::from_utf8_lossy(&output.stderr)
        );
        let stdout = String::from_utf8_lossy(&output.stdout);
        if args.first() == Some(&"init") {
            assert!(
                stdout.starts_with(expected_stdout),
                "unexpected init output: {stdout}"
            );
            assert!(stdout.contains("AGENTS.tss.md"));
            assert!(stdout.contains(".codex/tss-wrapper.sh"));
        } else if args.first() == Some(&"gain") {
            assert!(stdout.contains(expected_stdout));
            assert!(stdout.contains("No command events recorded yet."));
        } else {
            assert_eq!(stdout, *expected_stdout);
        }
    }

    let doctor = tss_command(&state_dir)
        .arg("doctor")
        .output()
        .expect("run tss doctor");
    assert!(doctor.status.success());
    let doctor_stdout = String::from_utf8_lossy(&doctor.stdout);
    assert!(doctor_stdout.contains("tss doctor: ok"));
    assert!(doctor_stdout.contains("commands: optimized="));
    assert!(doctor_stdout.contains("issue classes:"));

    let output = tss_command(&state_dir)
        .args(["raw", "tssr_missing"])
        .output()
        .expect("run tss raw");

    assert_eq!(output.status.code(), Some(1));
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "tss raw tssr_missing: raw output not found\n"
    );
}

#[test]
fn version_reports_release_version() {
    let state_dir = temp_state_dir("version");
    let output = tss_command(&state_dir)
        .arg("--version")
        .output()
        .expect("run version");

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "tss 0.1.01\n");
}

#[test]
fn familiar_direct_aliases_execute_native_commands() {
    let state_dir = temp_state_dir("direct-aliases");

    let direct = tss_command(&state_dir)
        .args(["printf", "hello"])
        .output()
        .expect("run direct alias");
    assert!(direct.status.success());
    assert_eq!(String::from_utf8_lossy(&direct.stdout), "hello");

    let dashdash = tss_command(&state_dir)
        .args(["--", "printf", "world"])
        .output()
        .expect("run dashdash alias");
    assert!(dashdash.status.success());
    assert_eq!(String::from_utf8_lossy(&dashdash.stdout), "world");
}

#[test]
fn proxy_alias_preserves_exact_stdout_and_stderr() {
    let state_dir = temp_state_dir("proxy");
    let output = tss_command(&state_dir)
        .args([
            "proxy",
            "/bin/sh",
            "-c",
            "printf out; printf err >&2; exit 9",
        ])
        .output()
        .expect("run proxy alias");

    assert_eq!(output.status.code(), Some(9));
    assert_eq!(String::from_utf8_lossy(&output.stdout), "out");
    assert_eq!(String::from_utf8_lossy(&output.stderr), "err");
}

#[test]
fn filtered_output_includes_recoverable_raw_handle() {
    let state_dir = temp_state_dir("raw-recovery");
    let output = tss_command(&state_dir)
        .args([
            "run",
            "--",
            "cat",
            "tests/fixtures/files/cat_long_single_file.txt",
        ])
        .output()
        .expect("run filtered cat");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("use tss raw tssr_"));
    assert!(!stdout.contains("line 10: release verification"));

    let raw_id = stdout
        .split_whitespace()
        .find(|part| part.starts_with("tssr_"))
        .expect("raw id in filtered output")
        .trim_end_matches(|ch: char| !ch.is_ascii_alphanumeric() && ch != '_')
        .to_string();

    let raw = tss_command(&state_dir)
        .args(["raw", &raw_id, "--combined"])
        .output()
        .expect("recover raw output");
    assert!(raw.status.success());
    assert!(String::from_utf8_lossy(&raw.stdout).contains("line 10: release verification"));
}

#[test]
fn gain_renders_readable_terminal_dashboard() {
    let state_dir = temp_state_dir("gain-dashboard");
    let analytics_file = state_dir.join("analytics.jsonl");

    let filtered = tss_command_with_analytics(&state_dir, &analytics_file)
        .args([
            "run",
            "--",
            "cat",
            "tests/fixtures/files/cat_long_single_file.txt",
        ])
        .output()
        .expect("run filtered command for analytics");
    assert!(filtered.status.success());

    let passthrough = tss_command_with_analytics(&state_dir, &analytics_file)
        .args(["proxy", "printf", "hello"])
        .output()
        .expect("run passthrough command for analytics");
    assert!(passthrough.status.success());

    let gain = tss_command_with_analytics(&state_dir, &analytics_file)
        .arg("gain")
        .output()
        .expect("run gain dashboard");
    assert!(gain.status.success());

    let stdout = String::from_utf8_lossy(&gain.stdout);
    assert!(stdout.contains("TSS Token Savings (Local Scope)"));
    assert!(stdout.contains("Total commands:"));
    assert!(stdout.contains("Tokens saved:"));
    assert!(stdout.contains("Efficiency meter:"));
    assert!(stdout.contains("By Agent"));
    assert!(stdout.contains("Manual / Unknown"));
    assert!(stdout.contains("Sub-Agent Usage"));
    assert!(stdout.contains("By Command"));
    assert!(stdout.contains("cat [args redacted: 1]"));
    assert!(stdout.contains("printf [args redacted: 1]"));
    assert!(stdout.contains("Command Coverage"));
    assert!(!stdout.contains("RTK"));
    let blocked_tool_name = ["r", "t", "k"].concat();
    assert!(!stdout.contains(&blocked_tool_name));
    assert!(
        stdout.lines().all(|line| line.chars().count() <= 100),
        "gain output should avoid long wrapped lines:\n{stdout}"
    );
}

#[test]
fn shell_wrapped_simple_command_filters_inner_command_and_tracks_subagent() {
    let state_dir = temp_state_dir("shell-wrapper-gain");
    let analytics_file = state_dir.join("analytics.jsonl");

    let filtered = tss_command_with_analytics(&state_dir, &analytics_file)
        .env("TSS_AGENT", "codex")
        .env("TSS_AGENT_ROLE", "sub-agent")
        .env("TSS_SUBAGENT", "1")
        .env("TSS_SUBAGENT_NAME", "scenario-scanner")
        .args([
            "run",
            "--",
            "bash",
            "-lc",
            "cat tests/fixtures/files/cat_long_single_file.txt",
        ])
        .output()
        .expect("run shell-wrapped filtered command");
    assert!(filtered.status.success());
    let stdout = String::from_utf8_lossy(&filtered.stdout);
    assert!(stdout.contains("file output"));
    assert!(stdout.contains("use tss raw tssr_"));
    assert!(!stdout.contains("line 10: release verification"));

    let gain = tss_command_with_analytics(&state_dir, &analytics_file)
        .arg("gain")
        .output()
        .expect("run gain after shell-wrapped command");
    assert!(gain.status.success());

    let gain_stdout = String::from_utf8_lossy(&gain.stdout);
    assert!(gain_stdout.contains("By Agent"));
    assert!(gain_stdout.contains("Codex"));
    assert!(gain_stdout.contains("Sub-Agent Usage"));
    assert!(gain_stdout.contains("scenario-scanner"));
    assert!(gain_stdout.contains("cat [args redacted: 1]"));
    assert!(!gain_stdout.contains("bash [args redacted"));

    let json = tss_command_with_analytics(&state_dir, &analytics_file)
        .args(["gain", "--json"])
        .output()
        .expect("run gain json after shell-wrapped command");
    assert!(json.status.success());
    let json_stdout = String::from_utf8_lossy(&json.stdout);
    assert!(json_stdout.contains("\"agent\":\"codex\""));
    assert!(json_stdout.contains("\"subagent_event_count\":1"));
    assert!(json_stdout.contains("\"subagent_name\":\"scenario-scanner\""));
}

#[test]
fn shell_init_emits_agent_scoped_wrappers_for_subagent_shells() {
    let state_dir = temp_state_dir("shell-init");
    let output = tss_command(&state_dir)
        .args(["shell-init", "--agent", "kilo-code", "--subagent"])
        .output()
        .expect("run shell-init");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("export TSS_BIN='"));
    assert!(stdout.contains("export TSS_AGENT='kilo-code'"));
    assert!(stdout.contains("export TSS_AGENT_ROLE='sub-agent'"));
    assert!(stdout.contains("export TSS_SUBAGENT=1"));
    assert!(stdout.contains("bash() { _tss_wrap bash \"$@\"; }"));
    assert!(stdout.contains("git() { _tss_wrap git \"$@\"; }"));
    assert!(stdout.contains("rg() { _tss_wrap rg \"$@\"; }"));
    for package_manager in ["npm", "pnpm", "yarn", "bun", "deno", "pip", "pip3", "brew"] {
        assert!(
            stdout.contains(&format!(
                "{package_manager}() {{ _tss_wrap {package_manager} \"$@\"; }}"
            )),
            "shell-init should wrap {package_manager}"
        );
    }
}

#[test]
fn compat_reports_command_migration_matrix() {
    let state_dir = temp_state_dir("compat");
    let output = tss_command(&state_dir)
        .arg("compat")
        .output()
        .expect("run compat");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("tss compat: command migration matrix"));
    assert!(stdout.contains("npm/pnpm/yarn/npx"));
    assert!(stdout.contains("brew"));
    assert!(stdout.contains("env/printenv/set"));
}
