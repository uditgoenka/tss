use super::command::CommandSpec;
use std::hash::{Hash, Hasher};

pub trait ShellCommandExt {
    fn program(&self) -> &str;

    fn args(&self) -> &[String];

    fn command_name(&self) -> &str {
        self.program().rsplit('/').next().unwrap_or(self.program())
    }

    fn command_hash(&self) -> String {
        let mut hasher = StableHasher::default();
        self.program().hash(&mut hasher);
        self.args().hash(&mut hasher);
        format!("{:016x}", hasher.finish())
    }

    fn requested_output_mode(&self) -> RequestedOutputMode {
        if self.command_name() == "git" && self.args().first().map(String::as_str) == Some("diff") {
            return RequestedOutputMode::Diff;
        }

        if self.args().iter().any(|arg| {
            matches!(arg.as_str(), "--json" | "-json")
                || arg.ends_with("=json")
                || arg.ends_with(":json")
        }) {
            return RequestedOutputMode::Json;
        }

        if self.args().iter().any(|arg| {
            matches!(arg.as_str(), "--xml" | "-xml")
                || arg.ends_with("=xml")
                || arg.ends_with(":xml")
        }) {
            return RequestedOutputMode::Xml;
        }

        RequestedOutputMode::PlainText
    }

    fn classify_shape(&self) -> CommandShape {
        if self.command_name() == "xargs" || self.args().iter().any(|arg| arg == "xargs") {
            return CommandShape::Unsafe("xargs can reshape command boundaries");
        }

        for token in std::iter::once(self.program()).chain(self.args().iter().map(String::as_str)) {
            if token == "|" || token == "||" || token.contains('|') {
                return CommandShape::Unsafe("pipe syntax is unsafe to filter");
            }
            if token == ">" || token == ">>" || token == "<" || token.starts_with("2>") {
                return CommandShape::Unsafe("redirection syntax is unsafe to filter");
            }
            if token.contains("<<") {
                return CommandShape::Unsafe("heredoc syntax is unsafe to filter");
            }
            if token.contains("$(") || token.contains('`') {
                return CommandShape::Unsafe("command substitution is unsafe to filter");
            }
            if token == "&" || token.ends_with('&') {
                return CommandShape::Unsafe("background jobs are unsafe to filter");
            }
            if matches!(
                token,
                "for"
                    | "while"
                    | "until"
                    | "do"
                    | "done"
                    | "if"
                    | "then"
                    | "else"
                    | "elif"
                    | "fi"
                    | "case"
                    | "esac"
            ) {
                return CommandShape::Unsafe("shell control flow is unsafe to filter");
            }
            if token == ";" || token == "&&" {
                return CommandShape::Unsafe("compound shell syntax is unsafe to filter");
            }
        }

        CommandShape::Simple
    }
}

impl ShellCommandExt for CommandSpec {
    fn program(&self) -> &str {
        &self.program
    }

    fn args(&self) -> &[String] {
        &self.args
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandShape {
    Simple,
    Unsafe(&'static str),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestedOutputMode {
    PlainText,
    Json,
    Xml,
    Diff,
}

#[derive(Default)]
struct StableHasher {
    state: u64,
}

impl Hasher for StableHasher {
    fn write(&mut self, bytes: &[u8]) {
        if self.state == 0 {
            self.state = 0xcbf29ce484222325;
        }
        for byte in bytes {
            self.state ^= u64::from(*byte);
            self.state = self.state.wrapping_mul(0x100000001b3);
        }
    }

    fn finish(&self) -> u64 {
        self.state
    }
}
