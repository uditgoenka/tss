use std::path::{Path, PathBuf};

use crate::analytics::{command_coverage_counts, issue_class_coverage_counts, CoverageCounts};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Agent {
    Claude,
    Copilot,
    CopilotCli,
    Gemini,
    OpenCode,
    OpenClaw,
    Cursor,
    Codex,
    Windsurf,
    Cline,
    RooCode,
    PiDev,
    Hermes,
    MistralVibe,
    KiloCode,
    Antigravity,
}

impl Agent {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Claude => "claude",
            Self::Copilot => "copilot",
            Self::CopilotCli => "copilot-cli",
            Self::Gemini => "gemini",
            Self::OpenCode => "opencode",
            Self::OpenClaw => "openclaw",
            Self::Cursor => "cursor",
            Self::Codex => "codex",
            Self::Windsurf => "windsurf",
            Self::Cline => "cline",
            Self::RooCode => "roo-code",
            Self::PiDev => "pi-dev",
            Self::Hermes => "hermes",
            Self::MistralVibe => "mistral-vibe",
            Self::KiloCode => "kilo-code",
            Self::Antigravity => "antigravity",
        }
    }

    pub fn display_name(self) -> &'static str {
        match self {
            Self::Claude => "Claude Code",
            Self::Copilot => "GitHub Copilot CLI / Cloud Agent",
            Self::CopilotCli => "GitHub Copilot CLI",
            Self::Gemini => "Gemini CLI",
            Self::OpenCode => "OpenCode",
            Self::OpenClaw => "OpenClaw",
            Self::Cursor => "Cursor",
            Self::Codex => "Codex",
            Self::Windsurf => "Windsurf",
            Self::Cline => "Cline",
            Self::RooCode => "Roo Code",
            Self::PiDev => "Pi.dev",
            Self::Hermes => "Hermes",
            Self::MistralVibe => "Mistral Vibe",
            Self::KiloCode => "Kilo Code",
            Self::Antigravity => "Google Antigravity",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScopeKind {
    Project,
    User,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scope {
    pub kind: ScopeKind,
    pub root: PathBuf,
}

impl Scope {
    pub fn project(root: impl Into<PathBuf>) -> Self {
        Self {
            kind: ScopeKind::Project,
            root: root.into(),
        }
    }

    pub fn user(root: impl Into<PathBuf>) -> Self {
        Self {
            kind: ScopeKind::User,
            root: root.into(),
        }
    }

    pub fn join(&self, relative: impl AsRef<Path>) -> String {
        self.root
            .join(relative)
            .to_string_lossy()
            .replace('\\', "/")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MutationMode {
    BashCommandRewrite,
    ToolArgsRewrite,
    InstructionOnly,
    SuggestionOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionKind {
    VersionCheck,
    WriteFile,
    MergeInstructions,
    VerifyFile,
    RemoveFile,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanAction {
    pub kind: ActionKind,
    pub path: Option<String>,
    pub description: String,
    pub rollback: Option<String>,
}

impl PlanAction {
    pub fn is_version_check(&self) -> bool {
        self.kind == ActionKind::VersionCheck
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderedFile {
    pub path: String,
    pub contents: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Detection {
    pub agent: Agent,
    pub installed: bool,
    pub active: bool,
    pub version: Option<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Verification {
    pub agent: Agent,
    pub installed: bool,
    pub active: bool,
    pub commands_intercepted: Vec<String>,
    pub blind_spots: Vec<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstallPlan {
    pub agent: Agent,
    pub scope: ScopeKind,
    pub dry_run: bool,
    pub mutation_mode: MutationMode,
    pub actions: Vec<PlanAction>,
    pub rendered_files: Vec<RenderedFile>,
    pub commands_intercepted: Vec<String>,
    pub blind_spots: Vec<String>,
    pub warnings: Vec<String>,
    pub restart_required: bool,
    pub docs_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UninstallPlan {
    pub agent: Agent,
    pub scope: ScopeKind,
    pub dry_run: bool,
    pub actions: Vec<PlanAction>,
}

pub trait AgentIntegration {
    fn agent(&self) -> Agent;
    fn detect(&self, scope: &Scope) -> Detection;
    fn install(&self, scope: &Scope, dry_run: bool) -> InstallPlan;
    fn verify(&self, scope: &Scope) -> Verification;
    fn uninstall(&self, scope: &Scope, dry_run: bool) -> UninstallPlan;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoctorEntry {
    pub agent: Agent,
    pub installed: bool,
    pub active: bool,
    pub commands_intercepted: Vec<String>,
    pub blind_spots: Vec<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoctorReport {
    pub entries: Vec<DoctorEntry>,
    pub parity: CoverageCounts,
    pub issue_classes: CoverageCounts,
    pub summary: String,
}

pub fn doctor_integrations(
    scope: &Scope,
    integrations: &[Box<dyn AgentIntegration>],
) -> DoctorReport {
    let entries = integrations
        .iter()
        .map(|integration| {
            let verification = integration.verify(scope);
            DoctorEntry {
                agent: verification.agent,
                installed: verification.installed,
                active: verification.active,
                commands_intercepted: verification.commands_intercepted,
                blind_spots: verification.blind_spots,
                notes: verification.notes,
            }
        })
        .collect::<Vec<_>>();

    let installed = entries.iter().filter(|entry| entry.installed).count();
    let blind_spots = entries
        .iter()
        .filter(|entry| !entry.blind_spots.is_empty())
        .count();

    let parity = command_coverage_counts();
    let issue_classes = issue_class_coverage_counts();

    DoctorReport {
        summary: format!(
            "{} integrations checked; {} installed; {} report blind spots. Command coverage: {} optimized, {} passthrough-compatible, {} planned, {} blocked. Issue classes: {} optimized, {} passthrough-compatible, {} planned, {} blocked.",
            entries.len(),
            installed,
            blind_spots,
            parity.optimized,
            parity.passthrough_compatible,
            parity.planned,
            parity.blocked,
            issue_classes.optimized,
            issue_classes.passthrough_compatible,
            issue_classes.planned,
            issue_classes.blocked
        ),
        entries,
        parity,
        issue_classes,
    }
}

pub fn version_check(description: impl Into<String>) -> PlanAction {
    PlanAction {
        kind: ActionKind::VersionCheck,
        path: None,
        description: description.into(),
        rollback: Some(String::from("No filesystem change.")),
    }
}

pub fn write_file(path: String, description: impl Into<String>) -> PlanAction {
    PlanAction {
        kind: ActionKind::WriteFile,
        path: Some(path.clone()),
        description: description.into(),
        rollback: Some(format!(
            "Remove {} or restore the previous file from backup.",
            path
        )),
    }
}

pub fn merge_instructions(path: String, description: impl Into<String>) -> PlanAction {
    PlanAction {
        kind: ActionKind::MergeInstructions,
        path: Some(path.clone()),
        description: description.into(),
        rollback: Some(format!("Remove the TSS instruction block from {}.", path)),
    }
}

pub fn remove_file(path: String, description: impl Into<String>) -> PlanAction {
    PlanAction {
        kind: ActionKind::RemoveFile,
        path: Some(path.clone()),
        description: description.into(),
        rollback: Some(format!(
            "Recreate {} from the install plan if needed.",
            path
        )),
    }
}

pub fn rendered_file(path: String, contents: &str) -> RenderedFile {
    RenderedFile {
        path,
        contents: contents.to_string(),
    }
}

pub fn file_exists(scope: &Scope, relative: &str) -> bool {
    scope.root.join(relative).exists()
}
