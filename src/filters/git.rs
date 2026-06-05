use super::{non_empty_lines, truncate_lines, CommandInput, FilterResult};

const LOG_KEEP_LINES: usize = 6;

pub fn filter(command: &CommandInput, raw: &str) -> FilterResult {
    match command.subcommand() {
        Some("status") => filter_status(command, raw),
        Some("diff") | Some("show") => filter_diff_or_show(raw),
        Some("log") => filter_log(command, raw),
        Some("branch") => filter_branch(command, raw),
        _ => FilterResult::passthrough("git", raw, "unsupported git subcommand"),
    }
}

fn filter_diff_or_show(raw: &str) -> FilterResult {
    FilterResult::passthrough("git", raw, "git exact output mode")
}

fn filter_status(command: &CommandInput, raw: &str) -> FilterResult {
    if command.has_arg("-z") || command.has_arg("--porcelain=v2") {
        return FilterResult::passthrough("git", raw, "exact git status mode");
    }

    let mut branch = None;
    let mut staged = 0;
    let mut unstaged = 0;
    let mut untracked = 0;
    let mut ignored = 0;
    let mut changes = Vec::new();

    for line in raw.lines() {
        if let Some(rest) = line.strip_prefix("## ") {
            branch = Some(rest);
            continue;
        }

        if line.trim().is_empty() {
            continue;
        }

        if line.starts_with("??") {
            untracked += 1;
        } else if line.starts_with("!!") {
            ignored += 1;
        } else {
            let mut chars = line.chars();
            let index = chars.next().unwrap_or(' ');
            let worktree = chars.next().unwrap_or(' ');
            if index != ' ' {
                staged += 1;
            }
            if worktree != ' ' {
                unstaged += 1;
            }
        }

        changes.push(line);
    }

    let mut output = String::from("git status summary\n");
    if let Some(branch) = branch {
        output.push_str("branch: ");
        output.push_str(branch);
        output.push('\n');
    }
    output.push_str(&format!("staged: {staged}\n"));
    output.push_str(&format!("unstaged: {unstaged}\n"));
    output.push_str(&format!("untracked: {untracked}\n"));
    output.push_str(&format!("ignored: {ignored}\n"));

    if !changes.is_empty() {
        output.push_str("\nchanges:\n");
        for line in changes {
            output.push_str(line);
            output.push('\n');
        }
    }

    FilterResult::filtered("git", output, 0)
}

fn filter_log(command: &CommandInput, raw: &str) -> FilterResult {
    if command.has_arg("-z") || has_exact_format_arg(command) {
        return FilterResult::passthrough("git", raw, "git exact log format");
    }

    let lines = non_empty_lines(raw);
    if lines.len() <= LOG_KEEP_LINES {
        return FilterResult::passthrough("git", raw, "git log already compact");
    }

    let keep_merges = !command.has_arg("--no-merges");
    let mut selected: Vec<&str> = lines.iter().take(LOG_KEEP_LINES).copied().collect();

    if keep_merges {
        for line in lines.iter().skip(LOG_KEEP_LINES).copied() {
            if looks_like_merge_line(line) && !selected.contains(&line) {
                selected.push(line);
            }
        }
    }

    let omitted = lines.len().saturating_sub(selected.len());
    let mut output = String::from("git log summary\n");
    for line in selected {
        output.push_str(line);
        output.push('\n');
    }
    if omitted > 0 {
        output.push_str(&format!(
            "... omitted {omitted} log entries; use tss raw <id> for full output\n"
        ));
    }

    FilterResult::filtered("git", output, omitted)
}

fn filter_branch(command: &CommandInput, raw: &str) -> FilterResult {
    if branch_requires_exact_output(command) {
        return FilterResult::passthrough("git", raw, "git exact output mode");
    }

    truncate_lines("git", raw, 20, "git branches")
}

fn branch_requires_exact_output(command: &CommandInput) -> bool {
    command.args.iter().skip(1).any(|arg| {
        matches!(
            arg.as_str(),
            "-v" | "-vv"
                | "--verbose"
                | "-a"
                | "--all"
                | "-r"
                | "--remotes"
                | "--contains"
                | "--merged"
                | "--no-merged"
                | "--points-at"
                | "--format"
                | "--show-current"
        ) || arg.starts_with("--format=")
            || (arg.starts_with('-') && arg.contains('v'))
            || (arg.starts_with('-') && arg.contains('a'))
            || (arg.starts_with('-') && arg.contains('r'))
    })
}

fn has_exact_format_arg(command: &CommandInput) -> bool {
    command.args.iter().any(|arg| {
        (arg.starts_with("--format=") && arg != "--format=oneline")
            || (arg.starts_with("--pretty=") && arg != "--pretty=oneline")
    })
}

fn looks_like_merge_line(line: &str) -> bool {
    let lower = line.to_ascii_lowercase();
    lower.starts_with("merge:") || lower.contains(" merge ")
}
