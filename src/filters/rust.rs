use super::{CommandInput, FilterResult};

pub fn filter(command: &CommandInput, raw: &str) -> FilterResult {
    match command.subcommand() {
        Some("test") | Some("check") => filter_cargo_failure(raw),
        _ => FilterResult::passthrough("rust", raw, "unsupported cargo subcommand"),
    }
}

fn filter_cargo_failure(raw: &str) -> FilterResult {
    FilterResult::filtered("rust", raw.to_string(), 0)
}
