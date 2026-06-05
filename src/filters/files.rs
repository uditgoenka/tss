use super::{truncate_lines, CommandInput, FilterResult};

pub fn filter(command: &CommandInput, raw: &str) -> FilterResult {
    match command.command_name() {
        "ls" => filter_ls(command, raw),
        "find" => filter_find(command, raw),
        "cat" | "head" | "tail" => filter_read(command, raw),
        _ => FilterResult::passthrough("files", raw, "unsupported files command"),
    }
}

fn filter_ls(command: &CommandInput, raw: &str) -> FilterResult {
    if ls_requires_exact_output(command) {
        return FilterResult::passthrough("files", raw, "files exact output mode");
    }

    truncate_lines("files", raw, 40, "ls entries")
}

fn ls_requires_exact_output(command: &CommandInput) -> bool {
    command.args.iter().any(|arg| {
        if arg.starts_with("--") {
            matches!(
                arg.as_str(),
                "--all"
                    | "--almost-all"
                    | "--classify"
                    | "--directory"
                    | "--full-time"
                    | "--human-readable"
                    | "--inode"
                    | "--long"
                    | "--numeric-uid-gid"
                    | "--recursive"
                    | "--size"
                    | "--time-style"
            ) || arg.starts_with("--format=")
                || arg.starts_with("--time-style=")
        } else {
            arg.starts_with('-')
                && arg
                    .chars()
                    .skip(1)
                    .any(|flag| matches!(flag, 'a' | 'A' | 'F' | 'i' | 'l' | 'n' | 'R' | 's'))
        }
    })
}

fn filter_find(command: &CommandInput, raw: &str) -> FilterResult {
    if find_requires_exact_output(command) {
        return FilterResult::passthrough("files", raw, "files exact output mode");
    }

    truncate_lines("files", raw, 80, "find results")
}

fn find_requires_exact_output(command: &CommandInput) -> bool {
    command.args.iter().any(|arg| {
        matches!(
            arg.as_str(),
            "!" | "("
                | ")"
                | "-a"
                | "-and"
                | "-delete"
                | "-exec"
                | "-execdir"
                | "-fls"
                | "-fprint"
                | "-fprint0"
                | "-ls"
                | "-o"
                | "-ok"
                | "-okdir"
                | "-or"
                | "-print0"
                | "-printf"
                | "-prune"
                | "-quit"
        )
    }) || has_unknown_find_predicate(command)
}

fn has_unknown_find_predicate(command: &CommandInput) -> bool {
    let mut args = command.args.iter().peekable();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-maxdepth" | "-mindepth" | "-name" | "-path" | "-type" => {
                args.next();
            }
            "-print" => {}
            value if value.starts_with('-') => return true,
            _ => {}
        }
    }
    false
}

fn filter_read(command: &CommandInput, raw: &str) -> FilterResult {
    if read_requires_exact_output(command, raw) {
        return FilterResult::passthrough("files", raw, "files exact output mode");
    }

    let keep = if command.command_name() == "cat" {
        6
    } else {
        120
    };
    truncate_lines("files", raw, keep, "file output")
}

fn read_requires_exact_output(command: &CommandInput, raw: &str) -> bool {
    raw.lines()
        .any(|line| line.starts_with("==> ") && line.ends_with(" <=="))
        || command.args.iter().any(|arg| {
            matches!(
                arg.as_str(),
                "-A" | "--show-all"
                    | "-E"
                    | "--show-ends"
                    | "-T"
                    | "--show-tabs"
                    | "-n"
                    | "--number"
            )
        })
}
