#[path = "../../src/filters/mod.rs"]
mod filters;

use filters::{filter_command, CommandInput};

fn fixture(path: &str) -> String {
    std::fs::read_to_string(path).unwrap()
}

#[test]
fn git_status_short_branch_preserves_branch_and_change_categories() {
    let raw = fixture("tests/fixtures/git/status_short_branch.txt");
    let result = filter_command(
        CommandInput::new("git", ["status", "--short", "--branch"]),
        &raw,
    );

    assert_eq!(result.filter_name, "git");
    assert!(result
        .output
        .contains("main...origin/main [ahead 1, behind 2]"));
    assert!(result.output.contains("staged: 2"));
    assert!(result.output.contains("unstaged: 1"));
    assert!(result.output.contains("untracked: 1"));
    assert!(result.output.contains("M  src/lib.rs"));
    assert!(result.output.contains(" M src/main.rs"));
    assert!(result
        .output
        .contains("?? tests/fixtures/git/status_short_branch.txt"));
    assert!(result.omitted_lines == 0);
    assert!(!result.passthrough);
}

#[test]
fn git_log_compacts_without_hiding_merge_commits() {
    let raw = fixture("tests/fixtures/git/log_oneline_with_merge.txt");
    let result = filter_command(CommandInput::new("git", ["log", "--oneline"]), &raw);

    assert_eq!(result.filter_name, "git");
    assert!(!result.passthrough);
    assert!(result.omitted_lines > 0);
    assert!(result.output.contains("Merge branch 'feature/raw-store'"));
    assert!(result.output.contains("omitted"));
}

#[test]
fn git_diff_name_only_is_preserved_as_exact_consumer_output() {
    let raw = fixture("tests/fixtures/git/diff_name_only.txt");
    let result = filter_command(CommandInput::new("git", ["diff", "--name-only"]), &raw);

    assert_eq!(result.filter_name, "git");
    assert_eq!(result.output, raw);
    assert!(result.passthrough);
    assert_eq!(result.passthrough_reason, Some("git exact output mode"));
}

#[test]
fn git_show_patch_output_is_preserved_without_diff_corruption() {
    let raw = fixture("tests/fixtures/git/show_patch.txt");
    let result = filter_command(CommandInput::new("git", ["show", "-p", "HEAD"]), &raw);

    assert_eq!(result.filter_name, "git");
    assert_eq!(result.output, raw);
    assert!(result.passthrough);
    assert!(result.output.contains("@@ -1,3 +1,4 @@"));
    assert_eq!(result.passthrough_reason, Some("git exact output mode"));
}

#[test]
fn git_branch_verbose_preserves_tracking_hash_and_remote_metadata() {
    let raw = fixture("tests/fixtures/git/branch_verbose.txt");
    let result = filter_command(CommandInput::new("git", ["branch", "-vv", "-a"]), &raw);

    assert_eq!(result.filter_name, "git");
    assert_eq!(result.output, raw);
    assert!(result.output.contains("[origin/main: ahead 1]"));
    assert!(result.output.contains("remotes/origin/main"));
    assert!(result.passthrough);
    assert_eq!(result.passthrough_reason, Some("git exact output mode"));
}

#[test]
fn ls_recursive_metadata_output_is_preserved_exactly() {
    let raw = fixture("tests/fixtures/files/ls_recursive_metadata.txt");
    let result = filter_command(CommandInput::new("ls", ["-laR"]), &raw);

    assert_eq!(result.filter_name, "files");
    assert_eq!(result.output, raw);
    assert!(result.passthrough);
    assert_eq!(result.passthrough_reason, Some("files exact output mode"));
}

#[test]
fn find_with_exec_action_passes_through_without_partial_predicate_parsing() {
    let raw = fixture("tests/fixtures/files/find_with_exec.txt");
    let result = filter_command(
        CommandInput::new("find", [".", "-type", "f", "-exec", "wc", "-l", "{}", ";"]),
        &raw,
    );

    assert_eq!(result.filter_name, "files");
    assert_eq!(result.output, raw);
    assert!(result.passthrough);
    assert_eq!(result.passthrough_reason, Some("files exact output mode"));
}

#[test]
fn cat_single_file_output_can_be_compacted_with_omission_marker() {
    let raw = fixture("tests/fixtures/files/cat_long_single_file.txt");
    let result = filter_command(CommandInput::new("cat", ["docs/trust-contract.txt"]), &raw);

    assert_eq!(result.filter_name, "files");
    assert!(!result.passthrough);
    assert_eq!(result.omitted_lines, 4);
    assert!(result.output.contains("line 01: trust contract"));
    assert!(result.output.contains("line 06: structured output guard"));
    assert!(!result.output.contains("line 10: release verification"));
    assert!(result.output.contains("use tss raw <id>"));
}

#[test]
fn head_multi_file_banners_are_preserved_exactly() {
    let raw = fixture("tests/fixtures/files/head_multi_file_banners.txt");
    let result = filter_command(
        CommandInput::new("head", ["src/lib.rs", "src/main.rs"]),
        &raw,
    );

    assert_eq!(result.filter_name, "files");
    assert_eq!(result.output, raw);
    assert!(result.passthrough);
    assert_eq!(result.passthrough_reason, Some("files exact output mode"));
}

#[test]
fn rg_json_output_is_preserved_as_structured_exact_mode() {
    let raw = fixture("tests/fixtures/search/rg_json.txt");
    let result = filter_command(CommandInput::new("rg", ["--json", "trust", "src"]), &raw);

    assert_eq!(result.filter_name, "search");
    assert_eq!(result.output, raw);
    assert!(result.passthrough);
    assert_eq!(result.passthrough_reason, Some("search exact output mode"));
}

#[test]
fn rg_line_matches_group_by_file_without_losing_line_numbers() {
    let raw = fixture("tests/fixtures/search/rg_line_matches.txt");
    let result = filter_command(
        CommandInput::new("rg", ["-n", "trust", "src", "tests"]),
        &raw,
    );

    assert_eq!(result.filter_name, "search");
    assert!(!result.passthrough);
    assert_eq!(result.omitted_lines, 1);
    assert!(result.output.contains("src/lib.rs"));
    assert!(result.output.contains("10:pub fn trust_contract() {}"));
    assert!(result.output.contains("42:assert_trust();"));
    assert!(result.output.contains("tests/filter.rs"));
    assert!(result.output.contains("14:trust contract fixture"));
    assert!(result.output.contains("22:raw trust recovery"));
    assert!(!result.output.contains("30:trust marker"));
    assert!(result.output.contains("omitted 1 matches"));
}

#[test]
fn grep_perl_regexp_mode_passes_through_without_dialect_translation() {
    let raw = fixture("tests/fixtures/search/grep_perl_regexp.txt");
    let result = filter_command(
        CommandInput::new("grep", ["-P", "-n", "trust\\w+", "src"]),
        &raw,
    );

    assert_eq!(result.filter_name, "search");
    assert_eq!(result.output, raw);
    assert!(result.passthrough);
    assert_eq!(result.passthrough_reason, Some("search exact output mode"));
}
