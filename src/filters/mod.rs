pub mod files;
pub mod git;
pub mod go;
pub mod js;
pub mod python;
pub mod rust;
pub mod search;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandInput {
    pub program: String,
    pub args: Vec<String>,
}

impl CommandInput {
    pub fn new<I, S>(program: impl Into<String>, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            program: program.into(),
            args: args.into_iter().map(Into::into).collect(),
        }
    }

    pub fn command_name(&self) -> &str {
        self.program.rsplit('/').next().unwrap_or(&self.program)
    }

    pub fn subcommand(&self) -> Option<&str> {
        self.args.first().map(String::as_str)
    }

    pub fn has_arg(&self, needle: &str) -> bool {
        self.args.iter().any(|arg| arg == needle)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilterResult {
    pub filter_name: &'static str,
    pub output: String,
    pub omitted_lines: usize,
    pub passthrough: bool,
    pub passthrough_reason: Option<&'static str>,
}

impl FilterResult {
    pub fn filtered(filter_name: &'static str, output: String, omitted_lines: usize) -> Self {
        Self {
            filter_name,
            output,
            omitted_lines,
            passthrough: false,
            passthrough_reason: None,
        }
    }

    pub fn passthrough(filter_name: &'static str, raw: &str, reason: &'static str) -> Self {
        Self {
            filter_name,
            output: raw.to_string(),
            omitted_lines: 0,
            passthrough: true,
            passthrough_reason: Some(reason),
        }
    }
}

pub fn filter_command(command: CommandInput, raw: &str) -> FilterResult {
    match command.command_name() {
        "git" => git::filter(&command, raw),
        "cargo" => rust::filter(&command, raw),
        "go" => go::filter(&command, raw),
        "pytest" | "py.test" | "python" | "python3" | "pip" | "pip3" | "pipx" | "uv" | "uvx"
        | "poetry" | "rye" => python::filter(&command, raw),
        "ls" | "find" | "cat" | "head" | "tail" => files::filter(&command, raw),
        "next" | "vitest" | "tsc" | "npm" | "pnpm" | "yarn" | "yarnpkg" | "npx" | "pnpx"
        | "bun" | "bunx" | "deno" | "corepack" | "brew" | "node" | "jest" | "mocha"
        | "playwright" | "cypress" | "ava" | "tap" | "uvu" | "karma" | "wdio" => {
            js::filter(&command, raw)
        }
        "rg" | "grep" | "egrep" | "fgrep" => search::filter(&command, raw),
        _ => FilterResult::passthrough("unsupported", raw, "unsupported command"),
    }
}

fn non_empty_lines(raw: &str) -> Vec<&str> {
    raw.lines().filter(|line| !line.trim().is_empty()).collect()
}

fn truncate_lines(filter_name: &'static str, raw: &str, keep: usize, header: &str) -> FilterResult {
    let lines = non_empty_lines(raw);
    if lines.len() <= keep {
        return FilterResult::passthrough(filter_name, raw, "already compact");
    }

    let omitted = lines.len() - keep;
    let mut output = String::new();
    output.push_str(header);
    output.push('\n');
    for line in lines.iter().take(keep) {
        output.push_str(line);
        output.push('\n');
    }
    output.push_str(&format!(
        "... omitted {omitted} lines; use tss raw <id> for full output\n"
    ));
    FilterResult::filtered(filter_name, output, omitted)
}
