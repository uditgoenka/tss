use super::installer::{
    file_exists, remove_file, rendered_file, version_check, write_file, Agent, AgentIntegration,
    Detection, InstallPlan, MutationMode, Scope, UninstallPlan, Verification,
};

const PLUGIN: &str = include_str!("../../assets/hooks/opencode/tss-plugin.js");
const DOCS_URL: &str = "https://dev.opencode.ai/docs/plugins/";

pub struct OpenCodeIntegration;

impl AgentIntegration for OpenCodeIntegration {
    fn agent(&self) -> Agent {
        Agent::OpenCode
    }

    fn detect(&self, scope: &Scope) -> Detection {
        let installed = file_exists(scope, ".opencode/plugins/tss-plugin.js");
        Detection {
            agent: Agent::OpenCode,
            installed,
            active: installed,
            version: None,
            notes: vec![String::from(
                "OpenCode plugin hooks run inside the OpenCode runtime.",
            )],
        }
    }

    fn install(&self, scope: &Scope, dry_run: bool) -> InstallPlan {
        InstallPlan {
            agent: Agent::OpenCode,
            scope: scope.kind,
            dry_run,
            mutation_mode: MutationMode::BashCommandRewrite,
            actions: vec![
                version_check(
                    "Check `opencode --version`; plugin API must support tool.execute.before.",
                ),
                write_file(
                    scope.join(".opencode/plugins/tss-plugin.js"),
                    "Install OpenCode plugin that rewrites only bash tool commands through TSS.",
                ),
            ],
            rendered_files: vec![rendered_file(
                scope.join(".opencode/plugins/tss-plugin.js"),
                PLUGIN,
            )],
            commands_intercepted: vec![String::from("bash.command")],
            blind_spots: vec![
                String::from("non-bash tools are not mutated"),
                String::from(
                    "plugin API drift can disable interception until doctor detects the file",
                ),
            ],
            warnings: Vec::new(),
            restart_required: true,
            docs_url: Some(String::from(DOCS_URL)),
        }
    }

    fn verify(&self, scope: &Scope) -> Verification {
        let detected = self.detect(scope);
        Verification {
            agent: Agent::OpenCode,
            installed: detected.installed,
            active: detected.active,
            commands_intercepted: vec![String::from("bash.command")],
            blind_spots: vec![String::from("only the bash tool is wrapped")],
            notes: detected.notes,
        }
    }

    fn uninstall(&self, scope: &Scope, dry_run: bool) -> UninstallPlan {
        UninstallPlan {
            agent: Agent::OpenCode,
            scope: scope.kind,
            dry_run,
            actions: vec![remove_file(
                scope.join(".opencode/plugins/tss-plugin.js"),
                "Remove OpenCode TSS plugin.",
            )],
        }
    }
}
