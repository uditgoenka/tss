use super::{truncate_lines, CommandInput, FilterResult};

const MATCHES_PER_FILE: usize = 2;

pub fn filter(command: &CommandInput, raw: &str) -> FilterResult {
    if search_requires_exact_output(command) {
        return FilterResult::passthrough("search", raw, "search exact output mode");
    }

    if let Some(result) = filter_line_matches(raw) {
        return result;
    }

    truncate_lines("search", raw, 80, "search output")
}

fn search_requires_exact_output(command: &CommandInput) -> bool {
    command.args.iter().any(|arg| {
        matches!(
            arg.as_str(),
            "--json"
                | "--vimgrep"
                | "--files"
                | "--files-with-matches"
                | "--files-without-match"
                | "--count"
                | "--count-matches"
                | "--stats"
                | "--passthru"
                | "--only-matching"
                | "--null"
                | "--null-data"
                | "--replace"
                | "--multiline"
                | "--perl-regexp"
                | "-0"
                | "-A"
                | "-B"
                | "-C"
                | "-L"
                | "-P"
                | "-U"
                | "-Z"
                | "-c"
                | "-l"
                | "-o"
                | "-z"
        ) || arg.starts_with("--context=")
            || arg.starts_with("--after-context=")
            || arg.starts_with("--before-context=")
            || arg.starts_with("--replace=")
            || arg.starts_with("-A")
            || arg.starts_with("-B")
            || arg.starts_with("-C")
    })
}

fn filter_line_matches(raw: &str) -> Option<FilterResult> {
    let mut groups: Vec<(String, Vec<String>)> = Vec::new();
    let mut omitted = 0;
    let mut parsed = 0;

    for line in raw.lines().filter(|line| !line.trim().is_empty()) {
        let (file, rest) = parse_match_line(line)?;
        parsed += 1;
        let index = match groups.iter().position(|(path, _)| path == file) {
            Some(index) => index,
            None => {
                groups.push((file.to_string(), Vec::new()));
                groups.len() - 1
            }
        };

        if groups[index].1.len() < MATCHES_PER_FILE {
            groups[index].1.push(rest.to_string());
        } else {
            omitted += 1;
        }
    }

    if parsed == 0 {
        return None;
    }

    let mut output = String::from("search matches\n");
    for (file, matches) in groups {
        output.push_str(&file);
        output.push('\n');
        for line in matches {
            output.push_str("  ");
            output.push_str(&line);
            output.push('\n');
        }
    }
    if omitted > 0 {
        output.push_str(&format!(
            "... omitted {omitted} matches; use tss raw <id> for full output\n"
        ));
    }

    Some(FilterResult::filtered("search", output, omitted))
}

fn parse_match_line(line: &str) -> Option<(&str, &str)> {
    let (file, rest) = line.split_once(':')?;
    let (line_number, _match_text) = rest.split_once(':')?;
    if file.is_empty() || line_number.parse::<usize>().is_err() {
        return None;
    }

    Some((file, rest))
}
