use super::installer::{
    file_exists, remove_file, rendered_file, version_check, write_file, Agent, AgentIntegration,
    Detection, InstallPlan, MutationMode, Scope, UninstallPlan, Verification,
};

const INSTRUCTIONS: &str = include_str!("../../assets/hooks/codex/AGENTS.tss.md");
const WRAPPER: &str = include_str!("../../assets/hooks/codex/tss-wrapper.sh");
const DOCS_URL: &str = "https://openai.com/index/unrolling-the-codex-agent-loop/";

pub struct CodexIntegration;

impl AgentIntegration for CodexIntegration {
    fn agent(&self) -> Agent {
        Agent::Codex
    }

    fn detect(&self, scope: &Scope) -> Detection {
        let installed = file_exists(scope, "AGENTS.tss.md") || file_exists(scope, "AGENTS.md");
        Detection {
            agent: Agent::Codex,
            installed,
            active: installed,
            version: None,
            notes: vec![String::from("Codex reads AGENTS.md-style project instructions; TSS does not assume a command mutation hook.")],
        }
    }

    fn install(&self, scope: &Scope, dry_run: bool) -> InstallPlan {
        InstallPlan {
            agent: Agent::Codex,
            scope: scope.kind,
            dry_run,
            mutation_mode: MutationMode::InstructionOnly,
            actions: vec![
                version_check("Check `codex --version`; install only instructions unless the user opts into a shell wrapper."),
                write_file(
                    scope.join("AGENTS.tss.md"),
                    "Write reviewable TSS guidance for Codex; merge into AGENTS.md manually after review.",
                ),
                write_file(
                    scope.join(".codex/tss-wrapper.sh"),
                    "Optional shell wrapper for users who explicitly opt in; not enabled automatically.",
                ),
            ],
            rendered_files: vec![
                rendered_file(scope.join("AGENTS.tss.md"), INSTRUCTIONS),
                rendered_file(scope.join(".codex/tss-wrapper.sh"), WRAPPER),
            ],
            commands_intercepted: Vec::new(),
            blind_spots: vec![
                String::from("no general command-mutation hook is assumed"),
                String::from("wrapper mode only works when the user explicitly invokes or configures it"),
            ],
            warnings: Vec::new(),
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
            commands_intercepted: Vec::new(),
            blind_spots: vec![String::from("no general command-mutation hook is assumed")],
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
