#[path = "../../src/filters/mod.rs"]
mod filters;

use filters::{filter_command, CommandInput};

fn fixture(path: &str) -> String {
    std::fs::read_to_string(path).unwrap()
}

#[test]
fn next_compile_error_preserves_route_file_line_message_and_dev_url() {
    let raw = fixture("tests/fixtures/js/next_compile_error.txt");
    let result = filter_command(CommandInput::new("next", ["dev"]), &raw);

    assert_eq!(result.filter_name, "js");
    assert!(!result.passthrough);
    assert!(result.output.contains("http://localhost:3000"));
    assert!(result.output.contains("Compiling /dashboard/settings"));
    assert!(result
        .output
        .contains("./app/dashboard/settings/page.tsx:42:17"));
    assert!(result.output.contains("Property 'usernmae' does not exist"));
    assert!(result
        .output
        .contains("> 42 |     <h1>{profile.usernmae}</h1>"));
    assert!(result.output.contains("Route (app)"));
}

#[test]
fn vitest_parser_error_passes_through_without_losing_transform_details() {
    let raw = fixture("tests/fixtures/js/vitest_parser_error.txt");
    let result = filter_command(CommandInput::new("vitest", ["run"]), &raw);

    assert_eq!(result.filter_name, "js");
    assert!(result.passthrough);
    assert_eq!(result.output, raw);
    assert!(result.output.contains("Transform failed with 1 error"));
    assert!(result
        .output
        .contains("Expected \";\" but found \"return\""));
    assert!(result
        .output
        .contains("/repo/src/components/BrokenWidget.tsx:17:12"));
}

#[test]
fn vitest_many_failures_preserves_every_failed_test_and_actionable_stack() {
    let raw = fixture("tests/fixtures/js/vitest_many_failures.txt");
    let result = filter_command(CommandInput::new("vitest", ["run"]), &raw);

    assert_eq!(result.filter_name, "js");
    assert!(!result.passthrough);
    assert!(result.omitted_lines > 0);
    assert!(result.output.contains("renders balance for paid invoice"));
    assert!(result
        .output
        .contains("blocks checkout when card is expired"));
    assert!(result.output.contains("surfaces API validation errors"));
    assert!(result
        .output
        .contains("keeps optimistic update after retry"));
    assert!(result.output.contains("src/billing/billing.test.ts:18:11"));
    assert!(result.output.contains("src/billing/billing.test.ts:39:9"));
    assert!(result.output.contains("src/billing/billing.test.ts:61:13"));
    assert!(result.output.contains("src/billing/billing.test.ts:88:15"));
    assert!(result.output.contains("Test Files  1 failed"));
    assert!(result.output.contains("Tests  4 failed"));
}

#[test]
fn tsc_ansi_errors_are_stripped_but_diagnostics_are_preserved() {
    let raw = fixture("tests/fixtures/js/tsc_ansi_errors.txt");
    let result = filter_command(CommandInput::new("tsc", ["--noEmit"]), &raw);

    assert_eq!(result.filter_name, "js");
    assert!(!result.passthrough);
    assert!(!result.output.contains('\u{1b}'));
    assert!(result.output.contains(
        "src/index.ts:12:7 - error TS2322: Type 'string' is not assignable to type 'number'."
    ));
    assert!(result.output.contains(
        "src/routes/user.ts:31:14 - error TS2339: Property 'emailAddress' does not exist on type 'User'."
    ));
    assert!(result.output.contains("Found 2 errors in 2 files."));
    assert!(result.output.contains("src/index.ts:12"));
    assert!(result.output.contains("src/routes/user.ts:31"));
}

#[test]
fn package_manager_test_commands_are_explicit_noop_passthroughs() {
    let raw = fixture("tests/fixtures/js/package_manager_test_script.txt");
    let commands = [
        CommandInput::new(
            "npm",
            ["run", "test", "--workspace", "@acme/web", "--", "--run"],
        ),
        CommandInput::new("pnpm", ["--filter", "@acme/web", "test", "--", "--run"]),
        CommandInput::new("yarn", ["workspace", "@acme/web", "test", "--run"]),
    ];

    for command in commands {
        let result = filter_command(command, &raw);

        assert_eq!(result.filter_name, "js");
        assert!(result.passthrough);
        assert_eq!(
            result.passthrough_reason,
            Some("package manager passthrough")
        );
        assert_eq!(result.output, raw);
        assert!(result
            .output
            .contains("> vitest run --config vitest.workspace.ts --run"));
        assert!(result.output.contains("Scope: @acme/web"));
    }
}

#[test]
fn js_package_and_test_compat_commands_are_honest_passthroughs() {
    let raw = fixture("tests/fixtures/js/compat_passthrough_output.txt");
    let commands = [
        (
            CommandInput::new("npx", ["vitest", "run"]),
            Some("package manager passthrough"),
        ),
        (
            CommandInput::new("pnpx", ["vitest", "run"]),
            Some("package manager passthrough"),
        ),
        (
            CommandInput::new("bun", ["test", "--filter", "billing"]),
            Some("package manager passthrough"),
        ),
        (
            CommandInput::new("deno", ["test", "--allow-env"]),
            Some("package manager passthrough"),
        ),
        (
            CommandInput::new("corepack", ["pnpm", "test"]),
            Some("package manager passthrough"),
        ),
        (
            CommandInput::new("brew", ["install", "node"]),
            Some("package manager passthrough"),
        ),
        (
            CommandInput::new("node", ["--test", "test/example.test.js"]),
            Some("test runner passthrough"),
        ),
        (
            CommandInput::new("jest", ["--runInBand"]),
            Some("test runner passthrough"),
        ),
        (
            CommandInput::new("mocha", ["test/**/*.spec.js"]),
            Some("test runner passthrough"),
        ),
        (
            CommandInput::new("playwright", ["test"]),
            Some("test runner passthrough"),
        ),
    ];

    for (command, reason) in commands {
        let result = filter_command(command, &raw);

        assert_eq!(result.filter_name, "js");
        assert!(result.passthrough);
        assert_eq!(result.passthrough_reason, reason);
        assert_eq!(result.output, raw);
        assert!(result.output.contains("workspace: @acme/web"));
        assert!(result.output.contains("/repo/example.spec.ts:11:7"));
    }
}

#[test]
fn python_package_tools_are_honest_passthroughs() {
    let raw = "Resolved 12 packages in 620ms\nDownloaded acme-1.2.3\n";
    let commands = [
        CommandInput::new("pip", ["install", "-r", "requirements.txt"]),
        CommandInput::new("pipx", ["run", "ruff", "check", "."]),
        CommandInput::new("uv", ["sync"]),
        CommandInput::new("uvx", ["pytest", "-q"]),
        CommandInput::new("poetry", ["install"]),
        CommandInput::new("rye", ["sync"]),
    ];

    for command in commands {
        let result = filter_command(command, raw);

        assert_eq!(result.filter_name, "python");
        assert!(result.passthrough);
        assert_eq!(
            result.passthrough_reason,
            Some("python package manager passthrough")
        );
        assert_eq!(result.output, raw);
    }
}

#[test]
fn go_test_failure_preserves_compile_vet_errors_and_coverage() {
    let raw = fixture("tests/fixtures/go/go_test_failure.txt");
    let result = filter_command(CommandInput::new("go", ["test", "./...", "-cover"]), &raw);

    assert_eq!(result.filter_name, "go");
    assert!(!result.passthrough);
    assert!(result
        .output
        .contains("# example.com/acme/internal/billing"));
    assert!(result
        .output
        .contains("internal/billing/service.go:42:13: undefined: missingSymbol"));
    assert!(result.output.contains(
        "internal/billing/service.go:58:20: cannot use amount (variable of type string) as int value"
    ));
    assert!(result.output.contains("# example.com/acme/internal/api"));
    assert!(result.output.contains(
        "internal/api/handler.go:91:2: fmt.Println arg user.ID is a func value, not called"
    ));
    assert!(result
        .output
        .contains("FAIL    example.com/acme/internal/billing [build failed]"));
    assert!(result
        .output
        .contains("coverage: 68.5% of statements in ./..."));
}

#[test]
fn cargo_test_failure_preserves_errors_failure_names_and_summary() {
    let raw = fixture("tests/fixtures/rust/cargo_test_failure.txt");
    let result = filter_command(CommandInput::new("cargo", ["test"]), &raw);

    assert_eq!(result.filter_name, "rust");
    assert!(!result.passthrough);
    assert!(result.output.contains("error[E0308]: mismatched types"));
    assert!(result.output.contains("--> src/lib.rs:27:19"));
    assert!(result
        .output
        .contains("error[E0599]: no method named `retry_now`"));
    assert!(result.output.contains("--> tests/payment_flow.rs:44:12"));
    assert!(result
        .output
        .contains("payment_flow_retries_failed_invoice"));
    assert!(result.output.contains("payment_flow_reports_declined_card"));
    assert!(result
        .output
        .contains("test result: FAILED. 1 passed; 2 failed"));
    assert!(result.output.contains("error: could not compile `acme`"));
}

#[test]
fn pytest_failure_preserves_traceback_collection_error_failure_and_summary() {
    let raw = fixture("tests/fixtures/python/pytest_failure.txt");
    let result = filter_command(CommandInput::new("pytest", ["tests"]), &raw);

    assert_eq!(result.filter_name, "python");
    assert!(!result.passthrough);
    assert!(result
        .output
        .contains("ERROR collecting tests/test_api_contract.py"));
    assert!(result.output.contains("Traceback:"));
    assert!(result
        .output
        .contains("ModuleNotFoundError: No module named 'acme.missing'"));
    assert!(result.output.contains("test_invoice_total_is_returned"));
    assert!(result.output.contains("tests/test_billing.py:21"));
    assert!(result
        .output
        .contains("AssertionError: assert '0.00' == '42.00'"));
    assert!(result
        .output
        .contains("FAILED tests/test_billing.py::test_invoice_total_is_returned"));
    assert!(result.output.contains("1 failed, 1 passed, 1 error"));
}
