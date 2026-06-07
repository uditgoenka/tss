use super::installer::{
    file_contains, file_contains_any, file_exists, remove_file, rendered_file, version_check,
    write_file, Agent, AgentIntegration, Detection, InstallPlan, MutationMode, Scope,
    UninstallPlan, Verification,
};

const INSTRUCTIONS: &str = include_str!("../../assets/hooks/codex/AGENTS.tss.md");
const WRAPPER: &str = include_str!("../../assets/hooks/codex/tss-wrapper.sh");
const HOOK: &str = include_str!("../../assets/hooks/codex/tss-pre-tool-use.py");
const HOOKS: &str = include_str!("../../assets/hooks/codex/hooks.tss.json");
const DOCS_URL: &str = "https://openai.com/index/unrolling-the-codex-agent-loop/";

pub struct CodexIntegration;

impl AgentIntegration for CodexIntegration {
    fn agent(&self) -> Agent {
        Agent::Codex
    }

    fn detect(&self, scope: &Scope) -> Detection {
        let installed = file_exists(scope, ".codex/hooks/tss-pre-tool-use.py")
            || file_exists(scope, "AGENTS.tss.md")
            || file_contains(scope, "AGENTS.md", "TSS_AGENT=codex")
            || file_contains(scope, "AGENTS.md", "TSS Command Output");
        let rtk_conflict = file_contains_any(
            scope,
            ".codex/hooks.json",
            &["rtk hook", " rtk ", "/rtk", "\"rtk"],
        );
        let active = file_exists(scope, ".codex/hooks/tss-pre-tool-use.py")
            && file_contains(scope, ".codex/hooks.json", "tss-pre-tool-use.py")
            && file_contains(scope, ".codex/hooks.json", "PreToolUse");
        let mut notes = vec![String::from("Codex hook support is enabled when .codex/hooks.json references .codex/hooks/tss-pre-tool-use.py; AGENTS instructions remain a fallback.")];
        if rtk_conflict {
            notes.push(String::from(
                "RTK conflict detected: Codex hooks reference RTK; use one active shell command-rewriting hook per host.",
            ));
        }
        Detection {
            agent: Agent::Codex,
            installed,
            active,
            version: None,
            notes,
        }
    }

    fn install(&self, scope: &Scope, dry_run: bool) -> InstallPlan {
        InstallPlan {
            agent: Agent::Codex,
            scope: scope.kind,
            dry_run,
            mutation_mode: MutationMode::BashCommandRewrite,
            actions: vec![
                version_check("Check `codex --version`; PreToolUse hook support requires Codex hooks to be enabled and trusted."),
                write_file(
                    scope.join(".codex/hooks/tss-pre-tool-use.py"),
                    "Install Codex PreToolUse hook that wraps shell command fields through TSS.",
                ),
                write_file(
                    scope.join(".codex/hooks.tss.json"),
                    "Write mergeable Codex hook settings; merge into .codex/hooks.json to activate.",
                ),
                write_file(
                    scope.join("AGENTS.tss.md"),
                    "Write reviewable TSS guidance for Codex as a fallback and sub-agent reminder.",
                ),
                write_file(
                    scope.join(".codex/tss-wrapper.sh"),
                    "Optional shell wrapper for users who explicitly opt in; not enabled automatically.",
                ),
            ],
            rendered_files: vec![
                rendered_file(scope.join(".codex/hooks/tss-pre-tool-use.py"), HOOK),
                rendered_file(scope.join(".codex/hooks.tss.json"), HOOKS),
                rendered_file(scope.join("AGENTS.tss.md"), INSTRUCTIONS),
                rendered_file(scope.join(".codex/tss-wrapper.sh"), WRAPPER),
            ],
            commands_intercepted: vec![String::from("tool_input.cmd"), String::from("tool_input.command")],
            blind_spots: vec![
                String::from("hook config must be merged into .codex/hooks.json and trusted by Codex"),
                String::from("non-shell tools are not mutated"),
            ],
            warnings: codex_warnings(scope),
            restart_required: false,
            docs_url: Some(String::from(DOCS_URL)),
        }
    }

    fn verify(&self, scope: &Scope) -> Verification {
        let detected = self.detect(scope);
        Verification {
            agent: Agent::Codex,
            installed: detected.installed,
            active: detected.active,
            commands_intercepted: vec![
                String::from("tool_input.cmd"),
                String::from("tool_input.command"),
            ],
            blind_spots: vec![String::from("non-shell tools are not command-mutable")],
            notes: detected.notes,
        }
    }

    fn uninstall(&self, scope: &Scope, dry_run: bool) -> UninstallPlan {
        UninstallPlan {
            agent: Agent::Codex,
            scope: scope.kind,
            dry_run,
            actions: vec![
                remove_file(
                    scope.join(".codex/hooks/tss-pre-tool-use.py"),
                    "Remove Codex TSS hook script.",
                ),
                remove_file(
                    scope.join(".codex/hooks.tss.json"),
                    "Remove mergeable Codex TSS hook snippet.",
                ),
                remove_file(
                    scope.join("AGENTS.tss.md"),
                    "Remove generated Codex TSS instruction fragment.",
                ),
                remove_file(
                    scope.join(".codex/tss-wrapper.sh"),
                    "Remove optional Codex TSS wrapper.",
                ),
            ],
        }
    }
}

fn codex_warnings(scope: &Scope) -> Vec<String> {
    let mut warnings = vec![String::from(
        "Merge .codex/hooks.tss.json into .codex/hooks.json to activate command interception.",
    )];
    if file_contains_any(
        scope,
        ".codex/hooks.json",
        &["rtk hook", " rtk ", "/rtk", "\"rtk"],
    ) {
        warnings.push(String::from(
            "RTK hook reference detected. Keep TSS in coexist mode or remove the RTK hook before TSS owns Codex command rewriting.",
        ));
    }
    warnings
}
