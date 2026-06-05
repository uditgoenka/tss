use super::{CommandInput, FilterResult};

pub fn filter(command: &CommandInput, raw: &str) -> FilterResult {
    match command.subcommand() {
        Some("test") => filter_go_test(raw),
        _ => FilterResult::passthrough("go", raw, "unsupported go subcommand"),
    }
}

fn filter_go_test(raw: &str) -> FilterResult {
    let lines: Vec<&str> = raw.lines().filter(|line| !line.trim().is_empty()).collect();
    if lines.is_empty() {
        return FilterResult::filtered("go", raw.to_string(), 0);
    }

    let mut kept = Vec::new();
    for line in &lines {
        if should_keep_go_test_line(line) {
            kept.push(*line);
        }
    }

    if kept.is_empty() {
        return FilterResult::filtered("go", raw.to_string(), 0);
    }

    let omitted = lines.len().saturating_sub(kept.len());
    let mut output = String::from("go test failure summary\n");
    for line in kept {
        output.push_str(line);
        output.push('\n');
    }
    if omitted > 0 {
        output.push_str(&format!(
            "... omitted {omitted} lines; use tss raw <id> for full output\n"
        ));
    }

    FilterResult::filtered("go", output, omitted)
}

fn should_keep_go_test_line(line: &str) -> bool {
    let trimmed = line.trim_start();
    trimmed.starts_with('#')
        || trimmed.starts_with("FAIL")
        || trimmed.starts_with("--- FAIL:")
        || trimmed.starts_with("coverage:")
        || (trimmed.contains(".go:") && trimmed.contains(':'))
}
