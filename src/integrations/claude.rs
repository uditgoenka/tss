use super::installer::{
    file_contains, file_exists, remove_file, rendered_file, version_check, write_file, Agent,
    AgentIntegration, Detection, InstallPlan, MutationMode, Scope, UninstallPlan, Verification,
};

const HOOK: &str = include_str!("../../assets/hooks/claude/tss-pre-tool-use.py");
const SETTINGS: &str = include_str!("../../assets/hooks/claude/settings.tss.json");
const DOCS_URL: &str = "https://code.claude.com/docs/en/hooks";

pub struct ClaudeIntegration;

impl AgentIntegration for ClaudeIntegration {
    fn agent(&self) -> Agent {
        Agent::Claude
    }

    fn detect(&self, scope: &Scope) -> Detection {
        let installed = file_exists(scope, ".claude/hooks/tss-pre-tool-use.py");
        let active = installed
            && file_contains(scope, ".claude/settings.json", "tss-pre-tool-use.py")
            && file_contains(scope, ".claude/settings.json", "PreToolUse");
        Detection {
            agent: Agent::Claude,
            installed,
            active,
            version: None,
            notes: vec![String::from(
                "Run `claude --version` during install and verify active Claude settings include the TSS PreToolUse hook.",
            )],
        }
    }

    fn install(&self, scope: &Scope, dry_run: bool) -> InstallPlan {
        InstallPlan {
            agent: Agent::Claude,
            scope: scope.kind,
            dry_run,
            mutation_mode: MutationMode::BashCommandRewrite,
            actions: vec![
                version_check("Check `claude --version`; updatedInput requires current PreToolUse hook support."),
                write_file(
                    scope.join(".claude/hooks/tss-pre-tool-use.py"),
                    "Install Bash-only PreToolUse hook that wraps commands with `tss run -- bash -lc`.",
                ),
                write_file(
                    scope.join(".claude/settings.tss.json"),
                    "Write mergeable hook settings snippet instead of overwriting existing Claude settings.",
                ),
            ],
            rendered_files: vec![
                rendered_file(scope.join(".claude/hooks/tss-pre-tool-use.py"), HOOK),
                rendered_file(scope.join(".claude/settings.tss.json"), SETTINGS),
            ],
            commands_intercepted: vec![String::from("Bash.command")],
            blind_spots: vec![
                String::from("non-Bash tools are never mutated"),
                String::from("TSS never grants command permission; Claude approval rules stay in control"),
                String::from("Claude deny/ask rules still take precedence over hook output"),
            ],
            warnings: vec![String::from(
                "Merge .claude/settings.tss.json into the active settings file after review.",
            )],
            restart_required: false,
            docs_url: Some(String::from(DOCS_URL)),
        }
    }

    fn verify(&self, scope: &Scope) -> Verification {
        let detected = self.detect(scope);
        Verification {
            agent: Agent::Claude,
            installed: detected.installed,
            active: detected.active,
            commands_intercepted: vec![String::from("Bash.command")],
            blind_spots: vec![String::from("non-Bash tools are not command-mutable")],
            notes: detected.notes,
        }
    }

    fn uninstall(&self, scope: &Scope, dry_run: bool) -> UninstallPlan {
        UninstallPlan {
            agent: Agent::Claude,
            scope: scope.kind,
            dry_run,
            actions: vec![
                remove_file(
                    scope.join(".claude/hooks/tss-pre-tool-use.py"),
                    "Remove TSS Claude hook script.",
                ),
                remove_file(
                    scope.join(".claude/settings.tss.json"),
                    "Remove mergeable TSS Claude settings snippet.",
                ),
            ],
        }
    }
}
