use super::installer::{
    file_exists, remove_file, rendered_file, version_check, write_file, Agent, AgentIntegration,
    Detection, InstallPlan, MutationMode, Scope, UninstallPlan, Verification,
};

const RULE: &str = include_str!("../../assets/hooks/cursor/tss.mdc");
const DOCS_URL: &str = "https://docs.cursor.com/";

pub struct CursorIntegration;

impl AgentIntegration for CursorIntegration {
    fn agent(&self) -> Agent {
        Agent::Cursor
    }

    fn detect(&self, scope: &Scope) -> Detection {
        let installed = file_exists(scope, ".cursor/rules/tss.mdc");
        Detection {
            agent: Agent::Cursor,
            installed,
            active: installed,
            version: None,
            notes: vec![String::from(
                "Cursor exposes terminal and rules surfaces; TSS uses rules only by default.",
            )],
        }
    }

    fn install(&self, scope: &Scope, dry_run: bool) -> InstallPlan {
        InstallPlan {
            agent: Agent::Cursor,
            scope: scope.kind,
            dry_run,
            mutation_mode: MutationMode::InstructionOnly,
            actions: vec![
                version_check("Check `cursor-agent --version` when available; no command mutation hook is assumed."),
                write_file(
                    scope.join(".cursor/rules/tss.mdc"),
                    "Install Cursor rule that asks the agent to use TSS for terminal commands.",
                ),
            ],
            rendered_files: vec![rendered_file(scope.join(".cursor/rules/tss.mdc"), RULE)],
            commands_intercepted: Vec::new(),
            blind_spots: vec![
                String::from("Terminal commands are not automatically rewritten"),
                String::from("rule following depends on the agent context window"),
            ],
            warnings: Vec::new(),
            restart_required: false,
            docs_url: Some(String::from(DOCS_URL)),
        }
    }

    fn verify(&self, scope: &Scope) -> Verification {
        let detected = self.detect(scope);
        Verification {
            agent: Agent::Cursor,
            installed: detected.installed,
            active: detected.active,
            commands_intercepted: Vec::new(),
            blind_spots: vec![String::from(
                "Terminal commands are not intercepted automatically",
            )],
            notes: detected.notes,
        }
    }

    fn uninstall(&self, scope: &Scope, dry_run: bool) -> UninstallPlan {
        UninstallPlan {
            agent: Agent::Cursor,
            scope: scope.kind,
            dry_run,
            actions: vec![remove_file(
                scope.join(".cursor/rules/tss.mdc"),
                "Remove Cursor TSS rule.",
            )],
        }
    }
}
