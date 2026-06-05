use super::{CommandInput, FilterResult};

pub fn filter(command: &CommandInput, raw: &str) -> FilterResult {
    if command.command_name() == "pytest"
        || command.command_name() == "py.test"
        || is_python_pytest(command)
    {
        return filter_pytest_failure(raw);
    }

    if is_python_package_tool(command.command_name()) {
        return FilterResult::passthrough("python", raw, "python package manager passthrough");
    }

    FilterResult::passthrough("python", raw, "unsupported python command")
}

fn is_python_package_tool(command_name: &str) -> bool {
    matches!(
        command_name,
        "pip" | "pip3" | "pipx" | "uv" | "uvx" | "poetry" | "rye"
    )
}

fn is_python_pytest(command: &CommandInput) -> bool {
    command
        .args
        .windows(2)
        .any(|pair| pair[0] == "-m" && pair[1] == "pytest")
}

fn filter_pytest_failure(raw: &str) -> FilterResult {
    FilterResult::filtered("python", raw.to_string(), 0)
}
