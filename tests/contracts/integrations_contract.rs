#![allow(dead_code, unused_imports)]

use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};

#[path = "../../src/analytics/mod.rs"]
mod analytics;
#[path = "../../src/integrations/mod.rs"]
mod integrations;
#[path = "../../src/privacy/mod.rs"]
mod privacy;

use integrations::{all_integrations, doctor_integrations, Agent, MutationMode, Scope};

#[test]
fn every_agent_install_plan_has_version_check_dry_run_and_rollback() {
    let scope = Scope::project("repo");
    let integrations = all_integrations();

    let agents = integrations
        .iter()
        .map(|integration| integration.agent())
        .collect::<Vec<_>>();
    assert_eq!(
        agents,
        vec![
            Agent::Claude,
            Agent::Copilot,
            Agent::CopilotCli,
            Agent::Gemini,
            Agent::OpenCode,
            Agent::OpenClaw,
            Agent::Cursor,
            Agent::Codex,
            Agent::Windsurf,
            Agent::Cline,
            Agent::RooCode,
            Agent::PiDev,
            Agent::Hermes,
            Agent::MistralVibe,
            Agent::KiloCode,
            Agent::Antigravity,
        ]
    );

    for integration in integrations {
        let plan = integration.install(&scope, true);

        assert_eq!(plan.agent, integration.agent());
        assert!(
            plan.dry_run,
            "{:?} plan must preserve dry-run intent",
            plan.agent
        );
        assert!(
            plan.actions.iter().any(|action| action.is_version_check()),
            "{:?} plan must include a version or capability check",
            plan.agent
        );
        assert!(
            plan.actions.iter().all(|action| action.rollback.is_some()),
            "{:?} install actions must describe rollback",
            plan.agent
        );
        assert!(
            !plan.commands_intercepted.is_empty() || !plan.blind_spots.is_empty(),
            "{:?} plan must state intercepted commands or blind spots",
            plan.agent
        );
    }
}

#[test]
fn unsupported_command_mutation_modes_are_reported_plainly() {
    let scope = Scope::project("repo");
    let plans = all_integrations()
        .into_iter()
        .map(|integration| integration.install(&scope, true))
        .collect::<Vec<_>>();

    let codex = plans
        .iter()
        .find(|plan| plan.agent == Agent::Codex)
        .unwrap();
    assert_eq!(codex.mutation_mode, MutationMode::BashCommandRewrite);
    assert_eq!(
        codex.commands_intercepted,
        vec!["tool_input.cmd", "tool_input.command"]
    );
    assert!(codex
        .blind_spots
        .iter()
        .any(|spot| spot.contains(".codex/hooks.json")));
    assert!(codex.rendered_files.iter().any(|file| {
        file.path.ends_with(".codex/hooks/tss-pre-tool-use.py")
            && file.contents.contains("updatedInput")
            && file.contents.contains("TSS_AGENT=codex")
            && file.contents.contains("bash -lc")
            && file.contents.contains("tool_input")
            && file.contents.contains("\"cmd\"")
            && file.contents.contains("\"command\"")
            && file.contents.contains("TSS_BYPASS=1")
    }));
    assert!(codex.rendered_files.iter().any(|file| {
        file.path.ends_with(".codex/hooks.tss.json")
            && file.contents.contains("PreToolUse")
            && file.contents.contains("tss-pre-tool-use.py")
    }));

    let cursor = plans
        .iter()
        .find(|plan| plan.agent == Agent::Cursor)
        .unwrap();
    assert_eq!(cursor.mutation_mode, MutationMode::InstructionOnly);
    assert!(cursor
        .blind_spots
        .iter()
        .any(|spot| spot.contains("Terminal")));

    let pi_dev = plans
        .iter()
        .find(|plan| plan.agent == Agent::PiDev)
        .unwrap();
    assert_eq!(pi_dev.mutation_mode, MutationMode::ToolArgsRewrite);
    assert!(pi_dev
        .rendered_files
        .iter()
        .any(|file| file.path.ends_with(".pi/extensions/tss.ts")
            && file.contents.contains("ExtensionAPI")
            && file.contents.contains("pi.on(\"tool_call\"")
            && file.contents.contains("event.input.command")));

    let global_scope = Scope::user("home");
    let global_pi_dev = all_integrations()
        .into_iter()
        .find(|integration| integration.agent() == Agent::PiDev)
        .unwrap()
        .install(&global_scope, true);
    assert!(global_pi_dev
        .rendered_files
        .iter()
        .any(|file| file.path.ends_with(".pi/agent/extensions/tss.ts")));

    let openclaw = plans
        .iter()
        .find(|plan| plan.agent == Agent::OpenClaw)
        .unwrap();
    assert!(openclaw.rendered_files.iter().any(|file| {
        file.contents.contains("before_tool_call")
            && file.contents.contains("event.params")
            && file.contents.contains("params:")
    }));

    let opencode = plans
        .iter()
        .find(|plan| plan.agent == Agent::OpenCode)
        .unwrap();
    assert!(opencode.rendered_files.iter().any(|file| {
        file.contents.contains("alreadyOwned")
            && file.contents.contains("rtk")
            && file.contents.contains("tss-wrapper.sh")
            && file.contents.contains("shellQuote")
            && !file
                .contents
                .contains("JSON.stringify(output.args.command)")
    }));

    let mistral_vibe = plans
        .iter()
        .find(|plan| plan.agent == Agent::MistralVibe)
        .unwrap();
    assert_eq!(mistral_vibe.mutation_mode, MutationMode::InstructionOnly);
    assert!(mistral_vibe
        .blind_spots
        .iter()
        .any(|spot| spot.contains("planned integration")));
}

#[test]
fn hook_detection_requires_active_settings_reference() {
    let root = temp_scope_root("active-hook-detection");
    let scope = Scope::user(&root);
    let claude = all_integrations()
        .into_iter()
        .find(|integration| integration.agent() == Agent::Claude)
        .unwrap();
    let codex = all_integrations()
        .into_iter()
        .find(|integration| integration.agent() == Agent::Codex)
        .unwrap();
    let copilot = all_integrations()
        .into_iter()
        .find(|integration| integration.agent() == Agent::Copilot)
        .unwrap();

    fs::create_dir_all(root.join(".claude/hooks")).unwrap();
    fs::write(
        root.join(".claude/hooks/tss-pre-tool-use.py"),
        "#!/usr/bin/env python3\n",
    )
    .unwrap();
    fs::write(
        root.join(".claude/settings.json"),
        r#"{"hooks":{"PreToolUse":[{"matcher":"Bash","hooks":[{"type":"command","command":"rtk hook claude"}]}]}}"#,
    )
    .unwrap();
    let claude_inactive = claude.detect(&scope);
    assert!(claude_inactive.installed);
    assert!(
        !claude_inactive.active,
        "Claude must not report active when settings still point to another hook"
    );
    assert!(claude_inactive
        .notes
        .iter()
        .any(|note| note.contains("RTK")));

    fs::write(
        root.join(".claude/settings.json"),
        r#"{"hooks":{"PreToolUse":[{"matcher":"Bash","hooks":[{"type":"command","command":"python3 \"$HOME/.claude/hooks/tss-pre-tool-use.py\""}]}]}}"#,
    )
    .unwrap();
    assert!(claude.detect(&scope).active);

    fs::create_dir_all(root.join(".codex/hooks")).unwrap();
    fs::write(
        root.join(".codex/hooks/tss-pre-tool-use.py"),
        "#!/usr/bin/env python3\n",
    )
    .unwrap();
    fs::write(
        root.join(".codex/hooks.json"),
        r#"{"hooks":{"PreToolUse":[{"matcher":"Bash","hooks":[{"type":"command","command":"node \"$HOME/.claude/hooks/scout-block.cjs\""}]}]}}"#,
    )
    .unwrap();
    let codex_inactive = codex.detect(&scope);
    assert!(codex_inactive.installed);
    assert!(
        !codex_inactive.active,
        "Codex must not report active until hooks.json references TSS"
    );

    fs::write(
        root.join(".codex/hooks.json"),
        r#"{"hooks":{"PreToolUse":[{"matcher":"Bash","hooks":[{"type":"command","command":"rtk hook codex"}]}]}}"#,
    )
    .unwrap();
    assert!(codex
        .detect(&scope)
        .notes
        .iter()
        .any(|note| note.contains("RTK")));

    fs::write(
        root.join(".codex/hooks.json"),
        r#"{"hooks":{"PreToolUse":[{"matcher":"Bash","hooks":[{"type":"command","command":"python3 \"$HOME/.codex/hooks/tss-pre-tool-use.py\""}]}]}}"#,
    )
    .unwrap();
    assert!(codex.detect(&scope).active);

    fs::create_dir_all(root.join(".github/hooks")).unwrap();
    fs::write(
        root.join(".github/hooks/tss.json"),
        r#"{"command":"rtk hook copilot"}"#,
    )
    .unwrap();
    let copilot_inactive = copilot.detect(&scope);
    assert!(copilot_inactive.installed);
    assert!(!copilot_inactive.active);
    assert!(copilot_inactive
        .notes
        .iter()
        .any(|note| note.contains("RTK")));
    assert!(copilot
        .install(&scope, true)
        .warnings
        .iter()
        .any(|warning| warning.contains("RTK")));
}

#[test]
fn python_hooks_skip_existing_rtk_owned_commands() {
    let scope = Scope::project("repo");
    let plans = all_integrations()
        .into_iter()
        .map(|integration| integration.install(&scope, true))
        .collect::<Vec<_>>();

    let claude_hook = rendered_contents(&plans, Agent::Claude, ".claude/hooks/tss-pre-tool-use.py");
    let codex_hook = rendered_contents(&plans, Agent::Codex, ".codex/hooks/tss-pre-tool-use.py");
    let copilot_hook =
        rendered_contents(&plans, Agent::Copilot, ".github/hooks/tss-pre-tool-use.py");

    let claude_output = run_python_hook(
        "claude-rtk-skip",
        claude_hook.as_str(),
        r#"{"tool_name":"Bash","tool_input":{"command":"env RTK_DEBUG=1 rtk git status"}}"#,
    );
    assert!(claude_output.contains("env RTK_DEBUG=1 rtk git status"));
    assert!(!claude_output.contains("TSS_AGENT=claude-code"));

    let codex_output = run_python_hook(
        "codex-rtk-skip",
        codex_hook.as_str(),
        r#"{"tool_name":"Bash","tool_input":{"cmd":"command rtk git diff"}}"#,
    );
    assert!(codex_output.contains("command rtk git diff"));
    assert!(!codex_output.contains("TSS_AGENT=codex"));

    let copilot_output = run_python_hook(
        "copilot-rtk-skip",
        copilot_hook.as_str(),
        r#"{"toolArgs":{"command":"RTK_SCOPE=global rtk gain"}}"#,
    );
    assert_eq!(copilot_output, "{}");

    let path_rtk_output = run_python_hook(
        "claude-path-rtk-skip",
        claude_hook.as_str(),
        r#"{"tool_name":"Bash","tool_input":{"command":"/opt/homebrew/bin/rtk gain"}}"#,
    );
    assert!(path_rtk_output.contains("/opt/homebrew/bin/rtk gain"));
    assert!(!path_rtk_output.contains("TSS_AGENT=claude-code"));

    let env_flag_rtk_output = run_python_hook(
        "codex-env-flag-rtk-skip",
        codex_hook.as_str(),
        r#"{"tool_name":"Bash","tool_input":{"cmd":"env -i RTK_SCOPE=global rtk gain"}}"#,
    );
    assert!(env_flag_rtk_output.contains("env -i RTK_SCOPE=global rtk gain"));
    assert!(!env_flag_rtk_output.contains("TSS_AGENT=codex"));

    let wrapper_output = run_python_hook(
        "codex-wrapper-skip",
        codex_hook.as_str(),
        r#"{"tool_name":"Bash","tool_input":{"cmd":"./.codex/tss-wrapper.sh git status"}}"#,
    );
    assert!(wrapper_output.contains("./.codex/tss-wrapper.sh git status"));
    assert!(!wrapper_output.contains("TSS_AGENT=codex"));

    let env_split_output = run_python_hook(
        "claude-env-s-rtk-skip",
        claude_hook.as_str(),
        r#"{"tool_name":"Bash","tool_input":{"command":"env -S 'rtk gain'"}}"#,
    );
    assert!(env_split_output.contains("env -S 'rtk gain'"));
    assert!(!env_split_output.contains("TSS_AGENT=claude-code"));

    let quoted_assignment_output = run_python_hook(
        "copilot-quoted-assignment-rtk-skip",
        copilot_hook.as_str(),
        r#"{"toolArgs":{"command":"VAR='x y' rtk gain"}}"#,
    );
    assert_eq!(quoted_assignment_output, "{}");
}

#[test]
fn opencode_hook_skips_quoted_rtk_owned_commands() {
    let scope = Scope::project("repo");
    let plans = all_integrations()
        .into_iter()
        .map(|integration| integration.install(&scope, true))
        .collect::<Vec<_>>();
    let opencode_hook =
        rendered_contents(&plans, Agent::OpenCode, ".opencode/plugins/tss-plugin.js");
    let root = temp_scope_root("opencode-js-hook");
    fs::create_dir_all(&root).unwrap();
    let plugin_file = root.join("tss-plugin.mjs");
    fs::write(&plugin_file, opencode_hook).unwrap();

    let node_script = r#"
const { pathToFileURL } = await import("node:url");
const mod = await import(pathToFileURL(process["env"]["PLUGIN_FILE"]).href);
const plugin = await mod.TssPlugin();
const before = plugin["tool.execute.before"];
for (const command of ["VAR='x y' rtk gain", "env -S 'rtk gain'", "/opt/homebrew/bin/rtk gain"]) {
  const output = { args: { command } };
  await before({ tool: "bash" }, output);
  if (output.args.command !== command) {
    throw new Error(`expected RTK-owned command to stay raw: ${output.args.command}`);
  }
}
const output = { args: { command: "git status --short" } };
await before({ tool: "bash" }, output);
if (!output.args.command.includes("TSS_AGENT=opencode tss run")) {
  throw new Error(`expected normal command to be wrapped: ${output.args.command}`);
}
"#;

    let mut child = Command::new("node")
        .arg("--input-type=module")
        .env("PLUGIN_FILE", &plugin_file)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn node");
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(node_script.as_bytes())
        .unwrap();
    let output = child.wait_with_output().expect("wait node");
    assert!(
        output.status.success(),
        "node hook test failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn claude_hook_plan_mutates_only_bash_and_never_auto_allows() {
    let scope = Scope::project("repo");
    let claude = all_integrations()
        .into_iter()
        .find(|integration| integration.agent() == Agent::Claude)
        .unwrap();
    let plan = claude.install(&scope, true);

    assert_eq!(plan.mutation_mode, MutationMode::BashCommandRewrite);
    assert_eq!(plan.commands_intercepted, vec!["Bash.command"]);
    assert!(plan
        .blind_spots
        .iter()
        .any(|spot| spot.contains("non-Bash")));
    let removed_auto_allow_env = ["TSS", "_CLAUDE", "_AUTO", "_ALLOW"].concat();
    assert!(plan.rendered_files.iter().any(|file| file
        .path
        .ends_with(".claude/hooks/tss-pre-tool-use.py")
        && file.contents.contains("updatedInput")
        && file.contents.contains("TSS_AGENT=claude-code")
        && !file.contents.contains("permissionDecision")
        && !file.contents.contains(&removed_auto_allow_env)));
}

#[test]
fn integration_assets_tag_gain_agent_keys_and_subagent_guidance() {
    let scope = Scope::project("repo");
    let plans = all_integrations()
        .into_iter()
        .map(|integration| integration.install(&scope, true))
        .collect::<Vec<_>>();

    let expected = [
        (Agent::Claude, "TSS_AGENT=claude-code"),
        (Agent::Copilot, "TSS_AGENT=copilot"),
        (Agent::CopilotCli, "TSS_AGENT=copilot-cli"),
        (Agent::Gemini, "TSS_AGENT=gemini"),
        (Agent::OpenCode, "TSS_AGENT=opencode"),
        (Agent::OpenClaw, "TSS_AGENT=openclaw"),
        (Agent::Cursor, "TSS_AGENT=cursor"),
        (Agent::Codex, "TSS_AGENT=codex"),
        (Agent::Windsurf, "TSS_AGENT=windsurf"),
        (Agent::Cline, "TSS_AGENT=cline"),
        (Agent::RooCode, "TSS_AGENT=roo-code"),
        (Agent::PiDev, "TSS_AGENT=pi-dev"),
        (Agent::Hermes, "TSS_AGENT=hermes"),
        (Agent::MistralVibe, "TSS_AGENT=mistral-vibe"),
        (Agent::KiloCode, "TSS_AGENT=kilo-code"),
        (Agent::Antigravity, "TSS_AGENT=antigravity"),
    ];

    for (agent, marker) in expected {
        let plan = plans.iter().find(|plan| plan.agent == agent).unwrap();
        let contents = plan
            .rendered_files
            .iter()
            .map(|file| file.contents.as_str())
            .collect::<Vec<_>>()
            .join("\n");

        assert!(
            contents.contains(marker),
            "{agent:?} integration must include {marker}"
        );
        assert!(
            contents.contains("TSS_SUBAGENT")
                || matches!(
                    agent,
                    Agent::Copilot
                        | Agent::OpenCode
                        | Agent::OpenClaw
                        | Agent::PiDev
                        | Agent::Hermes
                ),
            "{agent:?} integration must include sub-agent guidance or automatic tagging"
        );
    }
}

#[test]
fn doctor_reports_active_status_interception_and_blind_spots() {
    let scope = Scope::project("repo");
    let report = doctor_integrations(&scope, &all_integrations());

    assert_eq!(report.entries.len(), 16);
    assert!(report.parity.optimized > 0);
    assert!(report.parity.passthrough_compatible > 0);
    assert!(report.parity.planned > 0);
    assert!(report.parity.blocked > 0);
    assert!(report.issue_classes.optimized > 0);
    assert!(report.issue_classes.passthrough_compatible > 0);
    assert!(report.issue_classes.planned > 0);
    assert!(report.issue_classes.blocked > 0);
    assert!(report.entries.iter().any(|entry| {
        entry.agent == Agent::Claude
            && entry.commands_intercepted == vec!["Bash.command"]
            && !entry.blind_spots.is_empty()
    }));
    assert!(report.entries.iter().any(|entry| {
        entry.agent == Agent::PiDev && entry.commands_intercepted == vec!["tool_call.command"]
    }));
    assert!(report
        .entries
        .iter()
        .any(|entry| entry.agent == Agent::MistralVibe && !entry.active));
    assert!(report.summary.contains("blind spots"));
    assert!(report.summary.contains("Command coverage"));
    assert!(report.summary.contains("Issue classes"));
}

fn temp_scope_root(test_name: &str) -> std::path::PathBuf {
    let unique = format!(
        "tss-integrations-{test_name}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    std::env::temp_dir().join(unique)
}

fn rendered_contents(plans: &[integrations::InstallPlan], agent: Agent, suffix: &str) -> String {
    plans
        .iter()
        .find(|plan| plan.agent == agent)
        .unwrap()
        .rendered_files
        .iter()
        .find(|file| file.path.ends_with(suffix))
        .unwrap()
        .contents
        .clone()
}

fn run_python_hook(test_name: &str, contents: &str, payload: &str) -> String {
    let root = temp_scope_root(test_name);
    fs::create_dir_all(&root).unwrap();
    let script = root.join("hook.py");
    fs::write(&script, contents).unwrap();

    let mut child = Command::new("python3")
        .arg(&script)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn hook");
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(payload.as_bytes())
        .unwrap();
    let output = child.wait_with_output().expect("wait hook");
    assert!(
        output.status.success(),
        "hook failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        output.stderr.is_empty(),
        "hook must not write stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}
