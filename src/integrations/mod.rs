pub mod additional;
pub mod claude;
pub mod codex;
pub mod copilot;
pub mod cursor;
pub mod gemini;
pub mod installer;
pub mod opencode;

pub use installer::{
    doctor_integrations, ActionKind, Agent, AgentIntegration, Detection, DoctorEntry, DoctorReport,
    InstallPlan, MutationMode, PlanAction, RenderedFile, Scope, ScopeKind, UninstallPlan,
    Verification,
};

pub fn all_integrations() -> Vec<Box<dyn AgentIntegration>> {
    vec![
        Box::new(claude::ClaudeIntegration),
        Box::new(copilot::CopilotIntegration),
        Box::new(additional::AdditionalIntegration::new(
            &additional::COPILOT_CLI,
        )),
        Box::new(gemini::GeminiIntegration),
        Box::new(opencode::OpenCodeIntegration),
        Box::new(additional::AdditionalIntegration::new(
            &additional::OPENCLAW,
        )),
        Box::new(cursor::CursorIntegration),
        Box::new(codex::CodexIntegration),
        Box::new(additional::AdditionalIntegration::new(
            &additional::WINDSURF,
        )),
        Box::new(additional::AdditionalIntegration::new(&additional::CLINE)),
        Box::new(additional::AdditionalIntegration::new(
            &additional::ROO_CODE,
        )),
        Box::new(additional::AdditionalIntegration::new(&additional::PI_DEV)),
        Box::new(additional::AdditionalIntegration::new(&additional::HERMES)),
        Box::new(additional::AdditionalIntegration::new(
            &additional::MISTRAL_VIBE,
        )),
        Box::new(additional::AdditionalIntegration::new(
            &additional::KILO_CODE,
        )),
        Box::new(additional::AdditionalIntegration::new(
            &additional::ANTIGRAVITY,
        )),
    ]
}
