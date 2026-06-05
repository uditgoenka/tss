use super::installer::{
    file_exists, remove_file, rendered_file, version_check, write_file, Agent, AgentIntegration,
    Detection, InstallPlan, MutationMode, Scope, UninstallPlan, Verification,
};

const EXTENSION: &str = include_str!("../../assets/hooks/gemini/gemini-extension.json");
const GEMINI_MD: &str = include_str!("../../assets/hooks/gemini/GEMINI.tss.md");
const DOCS_URL: &str = "https://google-gemini.github.io/gemini-cli/docs/extensions/";

pub struct GeminiIntegration;

impl AgentIntegration for GeminiIntegration {
    fn agent(&self) -> Agent {
        Agent::Gemini
    }

    fn detect(&self, scope: &Scope) -> Detection {
        let installed = file_exists(scope, ".gemini/extensions/tss/gemini-extension.json");
        Detection {
            agent: Agent::Gemini,
            installed,
            active: installed,
            version: None,
            notes: vec![String::from(
                "Gemini CLI loads extensions on startup; restart the session after install.",
            )],
        }
    }

    fn install(&self, scope: &Scope, dry_run: bool) -> InstallPlan {
        InstallPlan {
            agent: Agent::Gemini,
            scope: scope.kind,
            dry_run,
            mutation_mode: MutationMode::InstructionOnly,
            actions: vec![
                version_check(
                    "Check `gemini --version`; extension support depends on the installed CLI.",
                ),
                write_file(
                    scope.join(".gemini/extensions/tss/gemini-extension.json"),
                    "Install local Gemini extension metadata for TSS guidance.",
                ),
                write_file(
                    scope.join("GEMINI.tss.md"),
                    "Write reviewable TSS guidance for Gemini; merge into GEMINI.md manually after review.",
                ),
            ],
            rendered_files: vec![
                rendered_file(
                    scope.join(".gemini/extensions/tss/gemini-extension.json"),
                    EXTENSION,
                ),
                rendered_file(scope.join("GEMINI.tss.md"), GEMINI_MD),
            ],
            commands_intercepted: Vec::new(),
            blind_spots: vec![
                String::from("no command mutation is claimed by this integration"),
                String::from(
                    "agents must choose `tss run --` from instructions or user-provided wrappers",
                ),
            ],
            warnings: vec![String::from(
                "Restart Gemini CLI after installing or updating the extension.",
            )],
            restart_required: true,
            docs_url: Some(String::from(DOCS_URL)),
        }
    }

    fn verify(&self, scope: &Scope) -> Verification {
        let detected = self.detect(scope);
        Verification {
            agent: Agent::Gemini,
            installed: detected.installed,
            active: detected.active,
            commands_intercepted: Vec::new(),
            blind_spots: vec![String::from(
                "instruction-only; commands are not intercepted automatically",
            )],
            notes: detected.notes,
        }
    }

    fn uninstall(&self, scope: &Scope, dry_run: bool) -> UninstallPlan {
        UninstallPlan {
            agent: Agent::Gemini,
            scope: scope.kind,
            dry_run,
            actions: vec![remove_file(
                scope.join(".gemini/extensions/tss/gemini-extension.json"),
                "Remove Gemini TSS extension metadata.",
            )],
        }
    }
}
