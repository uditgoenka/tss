#![allow(dead_code, unused_imports)]

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
    assert_eq!(codex.mutation_mode, MutationMode::InstructionOnly);
    assert!(codex
        .blind_spots
        .iter()
        .any(|spot| spot.contains("no general")));

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
        file.contents.contains("startsWith(\"tss \")")
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
