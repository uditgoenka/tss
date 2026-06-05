use super::{CommandInput, FilterResult};

pub fn filter(command: &CommandInput, raw: &str) -> FilterResult {
    match command.command_name() {
        "next" => filter_next(raw),
        "tsc" => filter_tsc(raw),
        "vitest" => filter_vitest(raw),
        "npm" | "pnpm" | "yarn" | "yarnpkg" | "npx" | "pnpx" | "bun" | "bunx" | "deno"
        | "corepack" | "brew" => {
            FilterResult::passthrough("js", raw, "package manager passthrough")
        }
        "node" | "jest" | "mocha" | "playwright" | "cypress" | "ava" | "tap" | "uvu" | "karma"
        | "wdio" => FilterResult::passthrough("js", raw, "test runner passthrough"),
        _ => FilterResult::passthrough("js", raw, "unsupported js command"),
    }
}

fn filter_next(raw: &str) -> FilterResult {
    if is_failure(raw) {
        return FilterResult::filtered("js", raw.to_string(), 0);
    }

    compact_success(raw)
}

fn compact_success(raw: &str) -> FilterResult {
    let lines: Vec<&str> = raw.lines().filter(|line| !line.trim().is_empty()).collect();
    if lines.len() <= 12 {
        return FilterResult::filtered("js", raw.to_string(), 0);
    }

    let keep = 12;
    let omitted = lines.len() - keep;
    let mut output = String::from("js command summary\n");
    for line in lines.iter().take(keep) {
        output.push_str(line);
        output.push('\n');
    }
    output.push_str(&format!(
        "... omitted {omitted} lines; use tss raw <id> for full output\n"
    ));
    FilterResult::filtered("js", output, omitted)
}

fn filter_vitest(raw: &str) -> FilterResult {
    if is_parser_failure(raw) {
        return FilterResult::passthrough("js", raw, "parser failure");
    }

    if !is_failure(raw) {
        return compact_success(raw);
    }

    let lines: Vec<&str> = raw.lines().filter(|line| !line.trim().is_empty()).collect();
    let mut kept = Vec::new();

    for line in &lines {
        if should_keep_vitest_failure_line(line) {
            kept.push(*line);
        }
    }

    if kept.is_empty() {
        return FilterResult::filtered("js", raw.to_string(), 0);
    }

    let omitted = lines.len().saturating_sub(kept.len());
    let mut output = String::from("vitest failure summary\n");
    for line in kept {
        output.push_str(line);
        output.push('\n');
    }
    if omitted > 0 {
        output.push_str(&format!(
            "... omitted {omitted} lines; use tss raw <id> for full output\n"
        ));
    }

    FilterResult::filtered("js", output, omitted)
}

fn filter_tsc(raw: &str) -> FilterResult {
    let output = strip_ansi(raw);
    FilterResult::filtered("js", output, 0)
}

fn strip_ansi(raw: &str) -> String {
    let mut output = String::with_capacity(raw.len());
    let mut chars = raw.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch != '\x1b' {
            output.push(ch);
            continue;
        }

        if chars.next_if_eq(&'[').is_some() {
            for code in chars.by_ref() {
                if ('@'..='~').contains(&code) {
                    break;
                }
            }
        }
    }

    output
}

fn is_parser_failure(raw: &str) -> bool {
    raw.contains("Transform failed")
        || raw.contains("Plugin: vite:")
        || raw.contains("Failed to parse")
        || raw.contains("ParseError")
}

fn is_failure(raw: &str) -> bool {
    raw.contains("Failed to compile")
        || raw.contains("Type error:")
        || raw.contains("SyntaxError:")
        || raw.contains(" FAIL ")
        || raw.contains("failed")
}

fn should_keep_vitest_failure_line(line: &str) -> bool {
    let trimmed = line.trim_start();
    trimmed.starts_with("FAIL ")
        || trimmed.starts_with("AssertionError")
        || trimmed.starts_with("Error:")
        || is_actionable_test_stack(trimmed)
        || trimmed.starts_with("Test Files")
        || trimmed.starts_with("Tests")
        || trimmed.starts_with("Start at")
        || trimmed.starts_with("Duration")
}

fn is_actionable_test_stack(trimmed: &str) -> bool {
    trimmed.starts_with("at ")
        && (trimmed.contains(".test.") || trimmed.contains(".spec."))
        && trimmed.contains(':')
}
