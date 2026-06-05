use super::command::CommandSpec;
use super::shell::{CommandShape, RequestedOutputMode, ShellCommandExt};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Support {
    Exact,
    UnsafeReason(&'static str),
    Unsupported,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SafetyDecision {
    FilterAllowed,
    PassthroughUnsafe(&'static str),
    PassthroughPlanned(&'static str),
    PassthroughUnsupported,
    Deny(&'static str),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandClass {
    Optimized,
    PassthroughCompatible,
    Planned(&'static str),
    BlockedByTrustContract(&'static str),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SafetyGate {
    destructive_guard: bool,
}

impl SafetyGate {
    pub fn with_destructive_guard() -> Self {
        Self {
            destructive_guard: true,
        }
    }

    pub fn decide(&self, command: &CommandSpec, support: Support) -> SafetyDecision {
        if self.destructive_guard && is_destructive(command) {
            return SafetyDecision::Deny("destructive guard blocked command");
        }

        match self.classify(command, support) {
            CommandClass::Optimized => SafetyDecision::FilterAllowed,
            CommandClass::PassthroughCompatible => SafetyDecision::PassthroughUnsupported,
            CommandClass::Planned(command_name) => SafetyDecision::PassthroughPlanned(command_name),
            CommandClass::BlockedByTrustContract(reason) => {
                SafetyDecision::PassthroughUnsafe(reason)
            }
        }
    }

    pub fn classify(&self, command: &CommandSpec, support: Support) -> CommandClass {
        if self.destructive_guard && is_destructive(command) {
            return CommandClass::BlockedByTrustContract("destructive guard blocked command");
        }

        if let CommandShape::Unsafe(reason) = command.classify_shape() {
            return CommandClass::BlockedByTrustContract(reason);
        }

        match support {
            Support::Exact => CommandClass::Optimized,
            Support::UnsafeReason(reason) => CommandClass::BlockedByTrustContract(reason),
            Support::Unsupported => {
                if command.requested_output_mode() != RequestedOutputMode::PlainText {
                    CommandClass::BlockedByTrustContract(
                        "structured output requires exact filter support",
                    )
                } else if let Some(command_name) = planned_parity_command(command.command_name()) {
                    CommandClass::Planned(command_name)
                } else {
                    CommandClass::PassthroughCompatible
                }
            }
        }
    }
}

fn is_destructive(command: &CommandSpec) -> bool {
    matches!(command.command_name(), "rm" | "shred")
        || (command.command_name() == "git"
            && command.args().first().map(String::as_str) == Some("clean")
            && command.args().iter().any(|arg| arg.contains('f')))
}

fn planned_parity_command(command_name: &str) -> Option<&'static str> {
    match command_name {
        "git" => Some("git"),
        "cargo" => Some("cargo"),
        "rustc" => Some("rustc"),
        "go" => Some("go"),
        "npm" => Some("npm"),
        "pnpm" => Some("pnpm"),
        "yarn" => Some("yarn"),
        "bun" => Some("bun"),
        "deno" => Some("deno"),
        "node" => Some("node"),
        "python" | "python3" => Some("python"),
        "pytest" => Some("pytest"),
        "make" => Some("make"),
        "cmake" => Some("cmake"),
        "mvn" => Some("mvn"),
        "gradle" => Some("gradle"),
        "docker" => Some("docker"),
        "kubectl" => Some("kubectl"),
        "rg" => Some("rg"),
        "grep" | "egrep" | "fgrep" => Some("grep"),
        "find" => Some("find"),
        "ls" => Some("ls"),
        "cat" => Some("cat"),
        "head" => Some("head"),
        "tail" => Some("tail"),
        _ => None,
    }
}
