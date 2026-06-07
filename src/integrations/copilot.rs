use super::installer::{
    file_contains_any, file_exists, remove_file, rendered_file, version_check, write_file, Agent,
    AgentIntegration, Detection, InstallPlan, MutationMode, Scope, UninstallPlan, Verification,
};

const HOOK: &str = include_str!("../../assets/hooks/copilot/tss-pre-tool-use.py");
const CONFIG: &str = include_str!("../../assets/hooks/copilot/tss-hooks.json");
const DOCS_URL: &str = "https://docs.github.com/en/copilot/reference/hooks-reference";

pub struct CopilotIntegration;

impl AgentIntegration for CopilotIntegration {
    fn agent(&self) -> Agent {
        Agent::Copilot
    }

    fn detect(&self, scope: &Scope) -> Detection {
        let installed = file_exists(scope, ".github/hooks/tss.json");
        let rtk_conflict = copilot_rtk_conflict(scope);
        let mut notes = vec![String::from("Copilot CLI/cloud hook availability is surface-specific; check host docs at install time.")];
        if rtk_conflict {
            notes.push(String::from("RTK conflict detected: Copilot hook config references RTK; use one active command-rewriting hook per host."));
        }
        Detection {
            agent: Agent::Copilot,
            installed,
            active: installed && !rtk_conflict,
            version: None,
            notes,
        }
    }

    fn install(&self, scope: &Scope, dry_run: bool) -> InstallPlan {
        InstallPlan {
            agent: Agent::Copilot,
            scope: scope.kind,
            dry_run,
            mutation_mode: MutationMode::ToolArgsRewrite,
            actions: vec![
                version_check("Check Copilot CLI or cloud agent hook support for preToolUse/PreToolUse."),
                write_file(
                    scope.join(".github/hooks/tss.json"),
                    "Install Copilot preToolUse hook config for command-bearing tools.",
                ),
                write_file(
                    scope.join(".github/hooks/tss-pre-tool-use.py"),
                    "Install hook script that emits modifiedArgs only when a command argument is present.",
                ),
            ],
            rendered_files: vec![
                rendered_file(scope.join(".github/hooks/tss.json"), CONFIG),
                rendered_file(scope.join(".github/hooks/tss-pre-tool-use.py"), HOOK),
            ],
            commands_intercepted: vec![String::from("toolArgs.command"), String::from("tool_input.command")],
            blind_spots: vec![
                String::from("tools without a command argument are not mutated"),
                String::from("cloud agent is non-interactive; ask decisions are treated as deny"),
                String::from("repository hooks must be present under .github/hooks for cloud jobs"),
            ],
            warnings: copilot_install_warnings(scope),
            restart_required: false,
            docs_url: Some(String::from(DOCS_URL)),
        }
    }

    fn verify(&self, scope: &Scope) -> Verification {
        let detected = self.detect(scope);
        Verification {
            agent: Agent::Copilot,
            installed: detected.installed,
            active: detected.active,
            commands_intercepted: vec![
                String::from("toolArgs.command"),
                String::from("tool_input.command"),
            ],
            blind_spots: vec![String::from("non-command tools are docs-only")],
            notes: detected.notes,
        }
    }

    fn uninstall(&self, scope: &Scope, dry_run: bool) -> UninstallPlan {
        UninstallPlan {
            agent: Agent::Copilot,
            scope: scope.kind,
            dry_run,
            actions: vec![
                remove_file(
                    scope.join(".github/hooks/tss.json"),
                    "Remove Copilot TSS hook config.",
                ),
                remove_file(
                    scope.join(".github/hooks/tss-pre-tool-use.py"),
                    "Remove Copilot TSS hook script.",
                ),
            ],
        }
    }
}

fn copilot_rtk_conflict(scope: &Scope) -> bool {
    [
        ".github/hooks/tss.json",
        ".github/hooks/rtk.json",
        ".github/hooks.json",
        ".github/copilot/hooks.json",
    ]
    .iter()
    .any(|relative| {
        file_contains_any(
            scope,
            relative,
            &["rtk hook", " rtk ", "/rtk", "\"rtk", "RTK_AGENT"],
        )
    })
}

fn copilot_install_warnings(scope: &Scope) -> Vec<String> {
    let mut warnings = vec![String::from("Copilot hook payload casing differs by surface; the script accepts both camelCase and snake_case.")];
    if copilot_rtk_conflict(scope) {
        warnings.push(String::from(
            "RTK conflict detected: remove the Copilot RTK hook before making TSS the active Copilot command-rewrite hook.",
        ));
    }
    warnings
}
