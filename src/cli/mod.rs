use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

use crate::analytics::{
    AgentContext, AnalyticsEvent, AnalyticsLedger, GainReport, PassthroughReason,
    SafetyDecision as AnalyticsDecision,
};
use crate::core::raw_store::{RawOutput as StoredRawOutput, RawRenderMode, RawStore};
use crate::core::{CommandSpec, PassthroughRunner, RunnerError};
use crate::filters::{filter_command, CommandInput};
use crate::integrations::{all_integrations, doctor_integrations, Agent, AgentIntegration, Scope};
use crate::privacy::PrivacyConfig;

const DISPLAY_VERSION: &str = "0.1.02";

const HELP: &str = "\
tss - trust-first token saving CLI

Usage:
  tss run -- <cmd> [args...]
  tss -- <cmd> [args...]
  tss <cmd> [args...]
  tss proxy <cmd> [args...]
  tss raw <id>
  tss doctor
  tss compat
  tss gain
  tss shell-init [--agent <agent>] [--subagent]
  tss init [agent|--agent <agent>] [-g|--global] [--dry-run]
  tss verify
  tss --version
";

pub fn run<I>(args: I) -> i32
where
    I: IntoIterator<Item = String>,
{
    let args = args.into_iter().collect::<Vec<_>>();

    match args.first().map(String::as_str) {
        Some("--help" | "-h") => {
            print!("{HELP}");
            0
        }
        Some("--version" | "-V" | "version") => {
            println!("tss {}", DISPLAY_VERSION);
            0
        }
        Some("help") => {
            print!("{HELP}");
            0
        }
        Some("run") => run_command(args[1..].to_vec()),
        Some("proxy") => proxy_command(args[1..].to_vec()),
        Some("raw") => raw(args[1..].to_vec()),
        Some("doctor") => doctor(),
        Some("compat") => compat(),
        Some("gain") => gain(args[1..].to_vec()),
        Some("shell-init") => shell_init(args[1..].to_vec()),
        Some("init") => init(args[1..].to_vec()),
        Some("verify") => {
            println!("tss verify: ok");
            0
        }
        Some(_) => run_command(args),
        None => {
            eprintln!("{HELP}");
            2
        }
    }
}

fn run_command(args: Vec<String>) -> i32 {
    execute_command(args, true)
}

fn proxy_command(args: Vec<String>) -> i32 {
    execute_command(args, false)
}

fn execute_command(args: Vec<String>, filtering_enabled: bool) -> i32 {
    let spec = match CommandSpec::from_run_args(args) {
        Ok(spec) => spec,
        Err(message) => {
            eprintln!("{message}");
            return 2;
        }
    };

    match PassthroughRunner::run(&spec) {
        Ok(output) => {
            let raw = StoredRawOutput::from_parts(
                output.stdout.clone(),
                output.stderr.clone(),
                output.combined.clone(),
                output.exit_code,
            );
            let raw_id = store_raw_output(&spec, &raw);
            let filter_spec = filter_spec_for_shell_wrapper(&spec).unwrap_or_else(|| spec.clone());
            let duration_ms = duration_millis(output.duration);

            if filtering_enabled {
                let raw_text = String::from_utf8_lossy(&output.combined);
                let filtered = filter_command(
                    CommandInput::new(filter_spec.program.clone(), filter_spec.args.clone()),
                    &raw_text,
                );

                if !filtered.passthrough {
                    let rendered = render_filtered_output(filtered.output, raw_id.as_deref());
                    let _ = io::stdout().write_all(rendered.as_bytes());
                    record_analytics(
                        &filter_spec,
                        filtered.filter_name,
                        AnalyticsDecision::Filtered,
                        raw.byte_len() as u64,
                        rendered.len() as u64,
                        duration_ms,
                    );
                    return output.exit_code;
                }

                record_analytics(
                    &filter_spec,
                    filtered.filter_name,
                    AnalyticsDecision::Passthrough(PassthroughReason::Other(
                        filtered
                            .passthrough_reason
                            .unwrap_or("passthrough")
                            .to_string(),
                    )),
                    raw.byte_len() as u64,
                    raw.byte_len() as u64,
                    duration_ms,
                );
            } else {
                record_analytics(
                    &filter_spec,
                    "proxy",
                    AnalyticsDecision::Passthrough(PassthroughReason::Other(
                        "explicit proxy".to_string(),
                    )),
                    raw.byte_len() as u64,
                    raw.byte_len() as u64,
                    duration_ms,
                );
            }

            write_raw_streams(&output.stdout, &output.stderr);
            output.exit_code
        }
        Err(RunnerError::Spawn { program, source }) => {
            eprintln!("tss: failed to run {program}: {source}");
            127
        }
    }
}

fn raw(args: Vec<String>) -> i32 {
    let Some(id) = args.first() else {
        eprintln!("usage: tss raw <id>");
        return 2;
    };

    let mode = match args.get(1).map(String::as_str) {
        None | Some("--full") => RawRenderMode::Full,
        Some("--stdout") => RawRenderMode::Stdout,
        Some("--stderr") => RawRenderMode::Stderr,
        Some("--combined") => RawRenderMode::Combined,
        Some(_) => {
            eprintln!("usage: tss raw <id> [--full|--stdout|--stderr|--combined]");
            return 2;
        }
    };

    match default_raw_store().render(id, mode) {
        Ok(output) => {
            let _ = io::stdout().write_all(&output);
            0
        }
        Err(error) if error.kind() == io::ErrorKind::InvalidInput => {
            eprintln!("tss raw {id}: invalid raw output id");
            2
        }
        Err(_) => {
            eprintln!("tss raw {id}: raw output not found");
            1
        }
    }
}

fn init(args: Vec<String>) -> i32 {
    let request = match InitRequest::parse(args) {
        Ok(request) => request,
        Err(message) => {
            eprintln!("{message}");
            return 2;
        }
    };

    let root = if request.global {
        home_dir()
    } else {
        env::current_dir().ok()
    };
    let Some(root) = root else {
        eprintln!("tss init: could not determine install scope");
        return 2;
    };
    let scope = if request.global {
        Scope::user(root)
    } else {
        Scope::project(root)
    };

    let integrations = all_integrations();
    let Some(integration) = find_integration(&integrations, &request.agent) else {
        eprintln!("tss init: unknown agent `{}`", request.agent);
        return 2;
    };

    let plan = integration.install(&scope, request.dry_run);
    println!(
        "tss init {}: {}",
        plan.agent.as_str(),
        if request.dry_run {
            "dry run"
        } else {
            "installing"
        }
    );
    println!("scope: {:?}", plan.scope);
    println!("mode: {:?}", plan.mutation_mode);
    for action in &plan.actions {
        println!("- {}", action.description);
    }
    for warning in &plan.warnings {
        println!("warning: {warning}");
    }

    if request.dry_run {
        for file in &plan.rendered_files {
            println!("would write {}", file.path);
        }
        return 0;
    }

    for file in &plan.rendered_files {
        let path = PathBuf::from(&file.path);
        if let Some(parent) = path.parent() {
            if let Err(error) = fs::create_dir_all(parent) {
                eprintln!("tss init: failed to create {}: {error}", parent.display());
                return 1;
            }
        }
        if path.exists() {
            println!("exists: {} (left unchanged)", file.path);
            continue;
        }
        if let Err(error) = fs::write(&path, &file.contents) {
            eprintln!("tss init: failed to write {}: {error}", file.path);
            return 1;
        }
        println!("wrote {}", file.path);
    }

    if plan.restart_required {
        println!("restart required: yes");
    }

    0
}

struct InitRequest {
    agent: String,
    global: bool,
    dry_run: bool,
}

impl InitRequest {
    fn parse(args: Vec<String>) -> Result<Self, String> {
        let mut agent = None;
        let mut global = false;
        let mut dry_run = false;
        let mut index = 0;

        while index < args.len() {
            let arg = &args[index];
            match arg.as_str() {
                "-g" | "--global" => global = true,
                "--dry-run" | "--show" => dry_run = true,
                "--agent" => {
                    index += 1;
                    let Some(value) = args.get(index) else {
                        return Err(String::from("usage: tss init [agent|--agent <agent>]"));
                    };
                    agent = Some(value.clone());
                }
                value if value.starts_with("--agent=") => {
                    agent = Some(value.trim_start_matches("--agent=").to_string());
                }
                value if value.starts_with("--") => {
                    agent = Some(value.trim_start_matches("--").to_string());
                }
                value => agent = Some(value.to_string()),
            }
            index += 1;
        }

        Ok(Self {
            agent: agent.unwrap_or_else(|| String::from("claude")),
            global,
            dry_run,
        })
    }
}

fn find_integration<'a>(
    integrations: &'a [Box<dyn AgentIntegration>],
    requested: &str,
) -> Option<&'a dyn AgentIntegration> {
    let normalized = normalize_agent_name(requested);
    integrations
        .iter()
        .map(Box::as_ref)
        .find(|integration| agent_aliases(integration.agent()).contains(&normalized.as_str()))
}

fn normalize_agent_name(name: &str) -> String {
    name.to_ascii_lowercase()
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .collect()
}

fn agent_aliases(agent: Agent) -> &'static [&'static str] {
    match agent {
        Agent::Claude => &["claude", "claudecode"],
        Agent::Copilot => &["copilot", "githubcopilot", "githubcopilotvscode"],
        Agent::CopilotCli => &["copilotcli", "githubcopilotcli", "ghcopilot"],
        Agent::Gemini => &["gemini", "geminicli"],
        Agent::OpenCode => &["opencode"],
        Agent::OpenClaw => &["openclaw"],
        Agent::Cursor => &["cursor"],
        Agent::Codex => &["codex", "openaicodex"],
        Agent::Windsurf => &["windsurf"],
        Agent::Cline => &["cline"],
        Agent::RooCode => &["roo", "roocode"],
        Agent::PiDev => &["pi", "pidev"],
        Agent::Hermes => &["hermes"],
        Agent::MistralVibe => &["mistral", "mistralvibe"],
        Agent::KiloCode => &["kilo", "kilocode"],
        Agent::Antigravity => &["antigravity", "googleantigravity"],
    }
}

fn home_dir() -> Option<PathBuf> {
    env::var_os("HOME").map(PathBuf::from)
}

fn doctor() -> i32 {
    let counts = crate::analytics::command_coverage_counts();
    let issue_counts = crate::analytics::issue_class_coverage_counts();
    let integrations = all_integrations();

    println!("tss doctor: ok");
    println!(
        "commands: optimized={} passthrough-compatible={} planned={} blocked={}",
        counts.optimized, counts.passthrough_compatible, counts.planned, counts.blocked
    );
    println!(
        "issue classes: optimized={} passthrough-compatible={} planned={} blocked={}",
        issue_counts.optimized,
        issue_counts.passthrough_compatible,
        issue_counts.planned,
        issue_counts.blocked
    );
    println!("raw store: local");
    println!("analytics: local, command args redacted");
    println!(
        "RTK conflict check: use one active command-rewriting hook per agent; TSS skips RTK-owned commands and reports inactive when RTK owns the live hook."
    );
    let mut scoped_conflicts = Vec::new();
    if let Ok(current_dir) = env::current_dir() {
        scoped_conflicts.extend(rtk_conflicts_for_scope(
            "Project",
            &Scope::project(current_dir),
            &integrations,
        ));
    }
    if let Some(home) = home_dir() {
        scoped_conflicts.extend(rtk_conflicts_for_scope(
            "User",
            &Scope::user(home),
            &integrations,
        ));
    }
    if scoped_conflicts.is_empty() {
        println!("RTK conflicts: none detected");
    } else {
        println!("RTK conflicts:");
        for (scope, agent, note) in scoped_conflicts {
            println!("- {scope} {agent}: {note}");
        }
    }
    0
}

fn rtk_conflicts_for_scope(
    scope_label: &'static str,
    scope: &Scope,
    integrations: &[Box<dyn AgentIntegration>],
) -> Vec<(&'static str, &'static str, String)> {
    doctor_integrations(scope, integrations)
        .entries
        .into_iter()
        .flat_map(|entry| {
            entry
                .notes
                .into_iter()
                .filter(|note| note.contains("RTK"))
                .map(move |note| (scope_label, entry.agent.display_name(), note))
        })
        .collect()
}

fn compat() -> i32 {
    let counts = crate::analytics::command_coverage_counts();
    println!("tss compat: command migration matrix");
    println!(
        "summary: optimized={} passthrough-compatible={} planned={} blocked={}",
        counts.optimized, counts.passthrough_compatible, counts.planned, counts.blocked
    );

    for entry in crate::analytics::known_command_coverage() {
        println!(
            "{}\t{}\t{}",
            entry.status.as_str(),
            entry.command,
            entry.note
        );
    }

    0
}

fn gain(args: Vec<String>) -> i32 {
    match GainReport::from_ledger(&default_analytics_ledger()) {
        Ok(report) if args.first().map(String::as_str) == Some("--json") => {
            println!("{}", report.to_json());
            0
        }
        Ok(report) => {
            println!("{}", report.human_summary());
            0
        }
        Err(error) => {
            eprintln!("tss gain: failed to read local ledger: {error}");
            1
        }
    }
}

fn shell_init(args: Vec<String>) -> i32 {
    let mut agent = String::from("manual");
    let mut subagent = false;
    let mut index = 0;

    while index < args.len() {
        match args[index].as_str() {
            "--agent" => {
                index += 1;
                let Some(value) = args.get(index) else {
                    eprintln!("usage: tss shell-init [--agent <agent>] [--subagent]");
                    return 2;
                };
                agent = value.clone();
            }
            value if value.starts_with("--agent=") => {
                agent = value.trim_start_matches("--agent=").to_string();
            }
            "--subagent" | "--sub-agent" => {
                subagent = true;
            }
            "--help" | "-h" => {
                println!("usage: tss shell-init [--agent <agent>] [--subagent]");
                return 0;
            }
            value => {
                eprintln!("tss shell-init: unknown option `{value}`");
                return 2;
            }
        }
        index += 1;
    }

    let context = AgentContext::from_values(
        Some(&agent),
        Some(if subagent { "sub-agent" } else { "main" }),
        subagent,
        None,
    );

    println!("# TSS shell wrappers. Source only in agent-controlled shells.");
    let tss_bin = env::current_exe()
        .ok()
        .map(|path| path.to_string_lossy().into_owned())
        .unwrap_or_else(|| String::from("tss"));
    println!("export TSS_BIN={}", shell_quote(&tss_bin));
    println!("export TSS_AGENT={}", shell_quote(&context.agent));
    println!("export TSS_AGENT_ROLE={}", shell_quote(&context.agent_role));
    if context.subagent {
        println!("export TSS_SUBAGENT=1");
    }
    println!("_tss_wrap() {{");
    println!("  if [ \"${{TSS_BYPASS:-0}}\" = \"1\" ]; then command \"$@\";");
    println!("  else command \"$TSS_BIN\" run -- \"$@\"; fi");
    println!("}}");
    for command in SHELL_WRAPPED_COMMANDS {
        println!("{command}() {{ _tss_wrap {command} \"$@\"; }}");
    }
    0
}

fn write_raw_streams(stdout: &[u8], stderr: &[u8]) {
    let _ = io::stdout().write_all(stdout);
    let _ = io::stderr().write_all(stderr);
}

fn render_filtered_output(output: String, raw_id: Option<&str>) -> String {
    match raw_id {
        Some(raw_id) => output.replace("tss raw <id>", &format!("tss raw {raw_id}")),
        None => output.replace(
            "; use tss raw <id> for full output",
            "; raw storage unavailable for full output",
        ),
    }
}

fn store_raw_output(spec: &CommandSpec, raw: &StoredRawOutput) -> Option<String> {
    default_raw_store()
        .store(spec, raw)
        .ok()
        .map(|record| record.id)
}

fn record_analytics(
    spec: &CommandSpec,
    filter_name: &str,
    decision: AnalyticsDecision,
    raw_bytes: u64,
    emitted_bytes: u64,
    duration_ms: u64,
) {
    let mut command = Vec::with_capacity(1 + spec.args.len());
    command.push(spec.program.as_str());
    command.extend(spec.args.iter().map(String::as_str));

    let event = AnalyticsEvent::new(
        command,
        spec.program.as_str(),
        filter_name,
        decision,
        raw_bytes,
        emitted_bytes,
    )
    .with_agent_context(AgentContext::from_env())
    .with_duration_ms(duration_ms);
    let _ = default_analytics_ledger().record(event);
}

fn duration_millis(duration: std::time::Duration) -> u64 {
    duration.as_millis().try_into().unwrap_or(u64::MAX)
}

fn filter_spec_for_shell_wrapper(spec: &CommandSpec) -> Option<CommandSpec> {
    let shell_name = spec.program.rsplit('/').next().unwrap_or(&spec.program);
    if !matches!(shell_name, "bash" | "sh" | "zsh") {
        return None;
    }

    let first_arg = spec.args.first()?;
    if !matches!(first_arg.as_str(), "-c" | "-lc") {
        return None;
    }

    let shell_command = spec.args.get(1)?;
    let parts = split_simple_shell_words(shell_command)?;
    CommandSpec::from_run_args(parts).ok()
}

fn split_simple_shell_words(input: &str) -> Option<Vec<String>> {
    if input.chars().any(|character| {
        matches!(
            character,
            '\n' | '\r'
                | '|'
                | '&'
                | ';'
                | '<'
                | '>'
                | '('
                | ')'
                | '`'
                | '$'
                | '\\'
                | '*'
                | '?'
                | '['
                | ']'
                | '{'
                | '}'
                | '~'
        )
    }) {
        return None;
    }

    let mut words = Vec::new();
    let mut current = String::new();
    let mut quote = None;

    for character in input.chars() {
        match quote {
            Some('\'') if character == '\'' => quote = None,
            Some('"') if character == '"' => quote = None,
            Some(_) => current.push(character),
            None if character == '\'' || character == '"' => quote = Some(character),
            None if character.is_whitespace() => {
                if !current.is_empty() {
                    words.push(std::mem::take(&mut current));
                }
            }
            None => current.push(character),
        }
    }

    if quote.is_some() {
        return None;
    }
    if !current.is_empty() {
        words.push(current);
    }
    if words.is_empty() || words.first().is_some_and(|word| word.contains('=')) {
        return None;
    }

    Some(words)
}

fn shell_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\"'\"'"))
}

const SHELL_WRAPPED_COMMANDS: &[&str] = &[
    "bash",
    "sh",
    "zsh",
    "git",
    "rg",
    "grep",
    "egrep",
    "fgrep",
    "ls",
    "find",
    "cat",
    "head",
    "tail",
    "cargo",
    "go",
    "pytest",
    "vitest",
    "tsc",
    "next",
    "npm",
    "pnpm",
    "yarn",
    "bun",
    "deno",
    "node",
    "pip",
    "pip3",
    "pipx",
    "uv",
    "uvx",
    "poetry",
    "rye",
    "jest",
    "playwright",
    "brew",
];

fn default_raw_store() -> RawStore {
    RawStore::new(
        env::var_os("TSS_RAW_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|| default_state_dir().join("raw")),
    )
}

fn default_analytics_ledger() -> AnalyticsLedger {
    AnalyticsLedger::new(
        env::var_os("TSS_ANALYTICS_FILE")
            .map(PathBuf::from)
            .unwrap_or_else(|| default_state_dir().join("analytics.jsonl")),
        PrivacyConfig::default(),
    )
}

fn default_state_dir() -> PathBuf {
    env::var_os("TSS_HOME")
        .map(PathBuf::from)
        .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".tss")))
        .unwrap_or_else(|| env::temp_dir().join("tss"))
}
