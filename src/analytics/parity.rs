#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandParityStatus {
    Optimized,
    PassthroughCompatible,
    Planned,
    Blocked,
}

impl CommandParityStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Optimized => "optimized",
            Self::PassthroughCompatible => "passthrough-compatible",
            Self::Planned => "planned",
            Self::Blocked => "blocked",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CoverageCounts {
    pub optimized: u16,
    pub passthrough_compatible: u16,
    pub planned: u16,
    pub blocked: u16,
}

impl CoverageCounts {
    pub fn add(&mut self, status: CommandParityStatus) {
        match status {
            CommandParityStatus::Optimized => self.optimized += 1,
            CommandParityStatus::PassthroughCompatible => self.passthrough_compatible += 1,
            CommandParityStatus::Planned => self.planned += 1,
            CommandParityStatus::Blocked => self.blocked += 1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandCoverage {
    pub family: &'static str,
    pub command: &'static str,
    pub status: CommandParityStatus,
    pub note: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IssueClassCoverage {
    pub class: &'static str,
    pub status: CommandParityStatus,
    pub source_examples: &'static str,
    pub tss_response: &'static str,
}

pub fn classify_command_parity<I, S>(command: I) -> CommandParityStatus
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let parts = command
        .into_iter()
        .map(|part| part.as_ref().to_ascii_lowercase())
        .collect::<Vec<_>>();
    let Some(program) = parts.first().map(String::as_str) else {
        return CommandParityStatus::PassthroughCompatible;
    };
    let subcommand = parts.get(1).map(String::as_str);

    match (program, subcommand) {
        ("git", Some("status" | "branch" | "log")) => CommandParityStatus::Optimized,
        ("git", Some("diff" | "show" | "add" | "commit" | "push")) => {
            CommandParityStatus::PassthroughCompatible
        }
        ("rg" | "grep" | "egrep" | "fgrep", _) => CommandParityStatus::Optimized,
        ("ls" | "find" | "cat" | "head" | "tail", _) => CommandParityStatus::Optimized,
        ("tree" | "read", _) => CommandParityStatus::PassthroughCompatible,
        ("next" | "tsc" | "vitest", _) => CommandParityStatus::Optimized,
        ("jest" | "node" | "mocha" | "playwright" | "cypress", _) => {
            CommandParityStatus::PassthroughCompatible
        }
        (
            "npm" | "pnpm" | "yarn" | "yarnpkg" | "npx" | "pnpx" | "bun" | "bunx" | "deno"
            | "corepack" | "brew",
            _,
        ) => CommandParityStatus::PassthroughCompatible,
        ("go", Some("test")) => CommandParityStatus::Optimized,
        ("cargo", Some("test" | "check")) => CommandParityStatus::Optimized,
        ("pytest", _) => CommandParityStatus::Optimized,
        ("pip" | "pip3" | "pipx" | "uv" | "uvx" | "poetry" | "rye", _) => {
            CommandParityStatus::PassthroughCompatible
        }
        ("ruff" | "eslint" | "oxlint" | "dotnet" | "ctest" | "cmake", _) => {
            CommandParityStatus::Planned
        }
        ("docker", Some("ps")) => CommandParityStatus::Planned,
        (
            "kubectl" | "oc" | "gh" | "jj" | "ssh" | "scp" | "dig" | "lsof" | "journalctl"
            | "fs_cli",
            _,
        ) => CommandParityStatus::Planned,
        ("env" | "printenv" | "set", _) => CommandParityStatus::Blocked,
        _ => CommandParityStatus::PassthroughCompatible,
    }
}

pub fn known_command_coverage() -> Vec<CommandCoverage> {
    vec![
        command(
            "git",
            "git status",
            CommandParityStatus::Optimized,
            "wired MVP filter",
        ),
        command(
            "git",
            "git branch",
            CommandParityStatus::Optimized,
            "wired MVP filter for safe simple modes",
        ),
        command(
            "git",
            "git log",
            CommandParityStatus::Optimized,
            "wired MVP filter; patch/topology exact modes must pass through",
        ),
        command(
            "git",
            "git diff",
            CommandParityStatus::PassthroughCompatible,
            "patch output stays raw until validator-backed compaction exists",
        ),
        command(
            "git",
            "git show",
            CommandParityStatus::PassthroughCompatible,
            "patch output stays raw until validator-backed compaction exists",
        ),
        command(
            "git",
            "git add/commit/push",
            CommandParityStatus::PassthroughCompatible,
            "state-changing commands must preserve native semantics",
        ),
        command(
            "search",
            "rg/grep",
            CommandParityStatus::Optimized,
            "simple line matches are optimized; exact/structured modes pass through",
        ),
        command(
            "files",
            "ls/find",
            CommandParityStatus::Optimized,
            "safe simple listings are optimized; metadata/actions pass through",
        ),
        command(
            "files",
            "cat/head/tail",
            CommandParityStatus::Optimized,
            "long plain text can be summarized; exact banners/numbering pass through",
        ),
        command(
            "files",
            "tree/read",
            CommandParityStatus::PassthroughCompatible,
            "familiar command names remain raw-compatible until TSS adapters exist",
        ),
        command(
            "js",
            "next/tsc",
            CommandParityStatus::Optimized,
            "wired diagnostics filters",
        ),
        command(
            "js",
            "vitest",
            CommandParityStatus::Optimized,
            "wired test-output filter with parser-error passthrough",
        ),
        command(
            "js",
            "node/jest/mocha/playwright/cypress",
            CommandParityStatus::PassthroughCompatible,
            "recognized test runner vocabulary; v0.1.0 preserves native output unless a safe adapter exists",
        ),
        command(
            "js",
            "npm/pnpm/yarn/npx/pnpx/bun/deno/corepack/brew scripts",
            CommandParityStatus::PassthroughCompatible,
            "recognized migration vocabulary; package manager output remains raw in v0.1.0",
        ),
        command(
            "go",
            "go test",
            CommandParityStatus::Optimized,
            "wired v0.1.0 filter preserves compile, vet, failure, and summary lines",
        ),
        command(
            "rust",
            "cargo test/check",
            CommandParityStatus::Optimized,
            "wired v0.1.0 filter preserves compiler diagnostics, failed tests, and summaries",
        ),
        command(
            "python",
            "pytest",
            CommandParityStatus::Optimized,
            "wired v0.1.0 filter preserves collection errors, tracebacks, failures, and summaries",
        ),
        command(
            "python",
            "pip/pipx/uv/uvx/poetry/rye",
            CommandParityStatus::PassthroughCompatible,
            "recognized package-tool vocabulary; install and resolver output remains raw in v0.1.0",
        ),
        command(
            "python",
            "ruff",
            CommandParityStatus::Planned,
            "recognized command family without a v0.1.0 adapter yet",
        ),
        command(
            "dotnet",
            "dotnet test",
            CommandParityStatus::Planned,
            "open issue class around test result counts",
        ),
        command(
            "cpp",
            "ctest/cmake/clang-format",
            CommandParityStatus::Planned,
            "open command-request class",
        ),
        command(
            "containers",
            "docker ps",
            CommandParityStatus::Planned,
            "recognized README command; no TSS filter yet",
        ),
        command(
            "kubernetes",
            "kubectl/oc",
            CommandParityStatus::Planned,
            "open command-request class",
        ),
        command(
            "vcs",
            "gh/jj",
            CommandParityStatus::Planned,
            "open PR/request class",
        ),
        command(
            "system",
            "ssh/scp/dig/lsof/journalctl/fs_cli",
            CommandParityStatus::Planned,
            "open command-request class",
        ),
        command(
            "environment",
            "env/printenv/set",
            CommandParityStatus::Blocked,
            "too likely to expose secrets; TSS should pass through and avoid analytics args",
        ),
    ]
}

pub fn issue_class_coverage() -> Vec<IssueClassCoverage> {
    vec![
        issue_class(
            "filter-quality-data-loss",
            CommandParityStatus::Optimized,
            "silent truncation, missing test/build diagnostics, misleading success",
            "validator, raw handles, and fixture-backed filters for wired families",
        ),
        issue_class(
            "structured-output-corruption",
            CommandParityStatus::PassthroughCompatible,
            "rg --json, git patch/log -p, machine-readable modes",
            "pass through unless a parser-backed adapter proves output remains valid",
        ),
        issue_class(
            "shell-rewrite-drift",
            CommandParityStatus::PassthroughCompatible,
            "pipes, redirects, compound commands, value-taking flag confusion",
            "wrap shell commands without parsing them in integrations; core safety gate passes through risky shapes",
        ),
        issue_class(
            "agent-hook-drift",
            CommandParityStatus::Optimized,
            "Claude/Copilot/OpenCode hook changes and Codex parity confusion",
            "per-host integration plans with blind spots and instruction-only modes where mutation is unsupported",
        ),
        issue_class(
            "privacy-telemetry-overclaim",
            CommandParityStatus::Optimized,
            "saved token miscounts, telemetry opt-out, provider cache mismatch",
            "local ledger, redacted args, provider-cache caveat, gain failure counts",
        ),
        issue_class(
            "install-config-paths",
            CommandParityStatus::Planned,
            "global path drift, platform-specific init paths, version resolution",
            "install plans include version checks and rollbacks; CLI apply path remains future work",
        ),
        issue_class(
            "platform-specific-hooks",
            CommandParityStatus::Planned,
            "Windows PowerShell hooks, macOS/Linux path differences",
            "surface-specific assets are represented, native platform installers are not complete",
        ),
        issue_class(
            "project-local-filter-trust",
            CommandParityStatus::Blocked,
            "project-local filters and prompt-injection command filters",
            "v0.1.0 must not auto-trust project-local executable filters or remote telemetry",
        ),
        issue_class(
            "secret-bearing-environment-output",
            CommandParityStatus::Blocked,
            "env/printenv filtering and command argument leakage",
            "no optimization claims; default analytics redacts args and does not store cwd",
        ),
    ]
}

pub fn command_coverage_counts() -> CoverageCounts {
    count_statuses(known_command_coverage().iter().map(|entry| entry.status))
}

pub fn issue_class_coverage_counts() -> CoverageCounts {
    count_statuses(issue_class_coverage().iter().map(|entry| entry.status))
}

fn count_statuses<I>(statuses: I) -> CoverageCounts
where
    I: IntoIterator<Item = CommandParityStatus>,
{
    let mut counts = CoverageCounts::default();
    for status in statuses {
        counts.add(status);
    }
    counts
}

fn command(
    family: &'static str,
    command: &'static str,
    status: CommandParityStatus,
    note: &'static str,
) -> CommandCoverage {
    CommandCoverage {
        family,
        command,
        status,
        note,
    }
}

fn issue_class(
    class: &'static str,
    status: CommandParityStatus,
    source_examples: &'static str,
    tss_response: &'static str,
) -> IssueClassCoverage {
    IssueClassCoverage {
        class,
        status,
        source_examples,
        tss_response,
    }
}
