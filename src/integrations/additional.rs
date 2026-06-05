use super::installer::{
    file_exists, merge_instructions, remove_file, rendered_file, version_check, write_file, Agent,
    AgentIntegration, Detection, InstallPlan, MutationMode, Scope, UninstallPlan, Verification,
};

pub struct AdditionalIntegration {
    spec: &'static IntegrationSpec,
}

impl AdditionalIntegration {
    pub const fn new(spec: &'static IntegrationSpec) -> Self {
        Self { spec }
    }
}

pub struct IntegrationSpec {
    pub agent: Agent,
    pub detect_path: &'static str,
    pub render_path: &'static str,
    pub global_render_path: Option<&'static str>,
    pub file_contents: &'static str,
    pub mutation_mode: MutationMode,
    pub version_check: &'static str,
    pub install_description: &'static str,
    pub detect_note: &'static str,
    pub commands_intercepted: &'static [&'static str],
    pub blind_spots: &'static [&'static str],
    pub warnings: &'static [&'static str],
    pub docs_url: Option<&'static str>,
    pub restart_required: bool,
    pub merge_instructions: bool,
}

impl AgentIntegration for AdditionalIntegration {
    fn agent(&self) -> Agent {
        self.spec.agent
    }

    fn detect(&self, scope: &Scope) -> Detection {
        let detect_path = render_path_for_scope(self.spec, scope);
        let installed = file_exists(scope, &detect_path);
        Detection {
            agent: self.spec.agent,
            installed,
            active: installed && self.spec.agent != Agent::MistralVibe,
            version: None,
            notes: vec![String::from(self.spec.detect_note)],
        }
    }

    fn install(&self, scope: &Scope, dry_run: bool) -> InstallPlan {
        let render_path = render_path_for_scope(self.spec, scope);
        let path = scope.join(&render_path);
        let file_action = if self.spec.merge_instructions {
            merge_instructions(path.clone(), self.spec.install_description)
        } else {
            write_file(path.clone(), self.spec.install_description)
        };

        InstallPlan {
            agent: self.spec.agent,
            scope: scope.kind,
            dry_run,
            mutation_mode: self.spec.mutation_mode,
            actions: vec![version_check(self.spec.version_check), file_action],
            rendered_files: vec![rendered_file(path, self.spec.file_contents)],
            commands_intercepted: strings(self.spec.commands_intercepted),
            blind_spots: strings(self.spec.blind_spots),
            warnings: strings(self.spec.warnings),
            restart_required: self.spec.restart_required,
            docs_url: self.spec.docs_url.map(String::from),
        }
    }

    fn verify(&self, scope: &Scope) -> Verification {
        let detected = self.detect(scope);
        Verification {
            agent: self.spec.agent,
            installed: detected.installed,
            active: detected.active,
            commands_intercepted: strings(self.spec.commands_intercepted),
            blind_spots: strings(self.spec.blind_spots),
            notes: detected.notes,
        }
    }

    fn uninstall(&self, scope: &Scope, dry_run: bool) -> UninstallPlan {
        let render_path = render_path_for_scope(self.spec, scope);
        UninstallPlan {
            agent: self.spec.agent,
            scope: scope.kind,
            dry_run,
            actions: vec![remove_file(
                scope.join(render_path),
                format!(
                    "Remove {} TSS integration file.",
                    self.spec.agent.display_name()
                ),
            )],
        }
    }
}

fn render_path_for_scope(spec: &IntegrationSpec, scope: &Scope) -> String {
    if scope.kind == super::installer::ScopeKind::User {
        spec.global_render_path
            .unwrap_or(spec.render_path)
            .to_string()
    } else {
        spec.render_path.to_string()
    }
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| String::from(*value)).collect()
}

pub static COPILOT_CLI: IntegrationSpec = IntegrationSpec {
    agent: Agent::CopilotCli,
    detect_path: ".github/hooks/tss-copilot-cli.json",
    render_path: ".github/hooks/tss-copilot-cli.json",
    global_render_path: Some(".copilot/hooks/tss-copilot-cli.json"),
    file_contents: include_str!("../../assets/hooks/additional/copilot-cli.json"),
    mutation_mode: MutationMode::SuggestionOnly,
    version_check:
        "Check `gh copilot --version` or the active Copilot CLI surface before installing.",
    install_description:
        "Install Copilot CLI suggestion-mode guidance for command-bearing tool calls.",
    detect_note:
        "Copilot CLI command mutation can be suggestion/deny based depending on host capability.",
    commands_intercepted: &["tool_input.command"],
    blind_spots: &["CLI hosts may require deny-with-suggestion instead of transparent mutation"],
    warnings: &["Review Copilot CLI host behavior before enabling automatic rewrites."],
    docs_url: Some("https://docs.github.com/en/copilot"),
    restart_required: false,
    merge_instructions: false,
};

pub static WINDSURF: IntegrationSpec = IntegrationSpec {
    agent: Agent::Windsurf,
    detect_path: ".windsurfrules",
    render_path: ".windsurfrules",
    global_render_path: None,
    file_contents: include_str!("../../assets/hooks/additional/windsurf-rules.md"),
    mutation_mode: MutationMode::InstructionOnly,
    version_check: "Check the installed Windsurf agent version and project-rule support.",
    install_description: "Write TSS terminal-wrapper guidance to Windsurf project rules.",
    detect_note:
        "Windsurf support is project-rule based unless a command hook is configured separately.",
    commands_intercepted: &[],
    blind_spots: &["instruction-only; terminal commands are not automatically rewritten"],
    warnings: &[],
    docs_url: Some("https://docs.windsurf.com/"),
    restart_required: false,
    merge_instructions: false,
};

pub static CLINE: IntegrationSpec = IntegrationSpec {
    agent: Agent::Cline,
    detect_path: ".clinerules/tss.md",
    render_path: ".clinerules/tss.md",
    global_render_path: None,
    file_contents: include_str!("../../assets/hooks/additional/cline-rules.md"),
    mutation_mode: MutationMode::InstructionOnly,
    version_check: "Check Cline or Roo Code rule-file support before installing.",
    install_description: "Install Cline rule guidance for using TSS in terminal commands.",
    detect_note: "Cline support is project-rule based.",
    commands_intercepted: &[],
    blind_spots: &["instruction-only; command adoption depends on agent rule following"],
    warnings: &[],
    docs_url: Some("https://cline.bot/"),
    restart_required: false,
    merge_instructions: false,
};

pub static ROO_CODE: IntegrationSpec = IntegrationSpec {
    agent: Agent::RooCode,
    detect_path: ".roo/rules/tss.md",
    render_path: ".roo/rules/tss.md",
    global_render_path: None,
    file_contents: include_str!("../../assets/hooks/additional/roo-code-rules.md"),
    mutation_mode: MutationMode::InstructionOnly,
    version_check: "Check Roo Code rule-file support before installing.",
    install_description: "Install Roo Code rule guidance for using TSS in terminal commands.",
    detect_note: "Roo Code support is project-rule based.",
    commands_intercepted: &[],
    blind_spots: &["instruction-only; command adoption depends on agent rule following"],
    warnings: &[],
    docs_url: Some("https://roocode.com/"),
    restart_required: false,
    merge_instructions: false,
};

pub static OPENCLAW: IntegrationSpec = IntegrationSpec {
    agent: Agent::OpenClaw,
    detect_path: ".openclaw/plugins/tss-plugin.js",
    render_path: ".openclaw/plugins/tss-plugin.js",
    global_render_path: None,
    file_contents: include_str!("../../assets/hooks/additional/openclaw-plugin.js"),
    mutation_mode: MutationMode::BashCommandRewrite,
    version_check: "Check OpenClaw plugin API support for before_tool_call before installing.",
    install_description: "Install OpenClaw plugin that wraps terminal command tool calls with TSS.",
    detect_note: "OpenClaw plugin support depends on the active runtime plugin API.",
    commands_intercepted: &["before_tool_call.command"],
    blind_spots: &["non-terminal tools are not mutated"],
    warnings: &["Restart OpenClaw after installing the plugin."],
    docs_url: None,
    restart_required: true,
    merge_instructions: false,
};

pub static PI_DEV: IntegrationSpec = IntegrationSpec {
    agent: Agent::PiDev,
    detect_path: ".pi/extensions/tss.ts",
    render_path: ".pi/extensions/tss.ts",
    global_render_path: Some(".pi/agent/extensions/tss.ts"),
    file_contents: include_str!("../../assets/hooks/additional/pi-extension.ts"),
    mutation_mode: MutationMode::ToolArgsRewrite,
    version_check: "Check Pi.dev extension API support for terminal tool calls before installing.",
    install_description: "Install Pi.dev TypeScript extension for command-bearing tool calls.",
    detect_note: "Pi.dev support depends on extension API availability in the active host.",
    commands_intercepted: &["tool_call.command"],
    blind_spots: &["tools without command fields are not mutated"],
    warnings: &["Restart Pi.dev after installing the extension."],
    docs_url: None,
    restart_required: true,
    merge_instructions: false,
};

pub static HERMES: IntegrationSpec = IntegrationSpec {
    agent: Agent::Hermes,
    detect_path: ".hermes/plugins/tss-rewrite.py",
    render_path: ".hermes/plugins/tss-rewrite.py",
    global_render_path: None,
    file_contents: include_str!("../../assets/hooks/additional/hermes-plugin.py"),
    mutation_mode: MutationMode::ToolArgsRewrite,
    version_check: "Check Hermes plugin adapter support before installing.",
    install_description: "Install Hermes Python plugin adapter for terminal command mutation.",
    detect_note: "Hermes support uses a Python plugin adapter when available.",
    commands_intercepted: &["terminal.command"],
    blind_spots: &["non-terminal tools are not mutated"],
    warnings: &["Restart Hermes after installing the plugin."],
    docs_url: None,
    restart_required: true,
    merge_instructions: false,
};

pub static MISTRAL_VIBE: IntegrationSpec = IntegrationSpec {
    agent: Agent::MistralVibe,
    detect_path: ".mistral-vibe/tss.md",
    render_path: ".mistral-vibe/tss.md",
    global_render_path: None,
    file_contents: include_str!("../../assets/hooks/additional/mistral-vibe.md"),
    mutation_mode: MutationMode::InstructionOnly,
    version_check: "Check whether Mistral Vibe exposes project instructions or terminal hooks.",
    install_description: "Install placeholder TSS instructions for future Mistral Vibe support.",
    detect_note: "Mistral Vibe support is planned and remains instruction-only until host APIs are available.",
    commands_intercepted: &[],
    blind_spots: &["planned integration; no command mutation is claimed"],
    warnings: &["Treat this as planned support, not active interception."],
    docs_url: None,
    restart_required: false,
    merge_instructions: false,
};

pub static KILO_CODE: IntegrationSpec = IntegrationSpec {
    agent: Agent::KiloCode,
    detect_path: ".kilocode/rules/tss-rules.md",
    render_path: ".kilocode/rules/tss-rules.md",
    global_render_path: None,
    file_contents: include_str!("../../assets/hooks/additional/kilo-code-rules.md"),
    mutation_mode: MutationMode::InstructionOnly,
    version_check: "Check Kilo Code project-rule support before installing.",
    install_description: "Install Kilo Code project rules for TSS terminal usage.",
    detect_note: "Kilo Code support is project-rule based.",
    commands_intercepted: &[],
    blind_spots: &["instruction-only; command adoption depends on agent rule following"],
    warnings: &[],
    docs_url: None,
    restart_required: false,
    merge_instructions: false,
};

pub static ANTIGRAVITY: IntegrationSpec = IntegrationSpec {
    agent: Agent::Antigravity,
    detect_path: ".agents/rules/antigravity-tss-rules.md",
    render_path: ".agents/rules/antigravity-tss-rules.md",
    global_render_path: None,
    file_contents: include_str!("../../assets/hooks/additional/antigravity-rules.md"),
    mutation_mode: MutationMode::InstructionOnly,
    version_check: "Check Google Antigravity project-rule support before installing.",
    install_description: "Install Google Antigravity project rules for TSS terminal usage.",
    detect_note: "Google Antigravity support is project-rule based.",
    commands_intercepted: &[],
    blind_spots: &["instruction-only; command adoption depends on agent rule following"],
    warnings: &[],
    docs_url: None,
    restart_required: false,
    merge_instructions: false,
};
