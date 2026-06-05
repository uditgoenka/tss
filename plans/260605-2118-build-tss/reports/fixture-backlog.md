# the prior token-saving tool Issue Fixture Backlog And TDD Acceptance Matrix

Date: 2026-06-05

## Purpose

TSS v0.1 should address open issue and open PR classes with a test-first
fixture backlog. The backlog is class-based so it remains stable as individual
issues and PRs open, close, or get retitled.

The acceptance standard is the TSS trust contract: correctness, auditability, and
recovery outrank token savings. Passthrough is an acceptable green result when a
command shape is unsupported, unsafe, or too broad for a fixture-backed adapter.

## Source Snapshot

Inputs used for this report:

- Local TSS trust contract and MVP scope.
- Local red-team and research reports.
- the prior token-saving tool public issue/PR sample checked on 2026-06-05 through the GitHub issues
  API: <https://api.github.com/repos/legacy-cli-ai/legacy-cli/issues?state=open&per_page=100>
- the prior token-saving tool public PR page: <https://github.com/legacy-cli-ai/legacy-cli/pulls>
- the prior token-saving tool supported-agent page checked for hook, Windows, package,
  cloud/container, and integration classes:
  <https://github.com/legacy-cli-ai/legacy-cli/blob/master/docs/guide/getting-started/supported-agents.md>

The live the prior token-saving tool sample showed active issue and PR titles around hidden Go compiler
errors, misleading success, `git log -p` patch loss, `find`/`ls -R`/`env` data
loss, command lexer/newline risk, grep/ripgrep rewrite collisions, PowerShell and
Codex docs, package/runtime support, and container/cloud command passthrough.

## Backlog Counts

| Category | P0 | P1 | P2 | Total |
| --- | ---: | ---: | ---: | ---: |
| Silent loss | 3 | 1 | 1 | 5 |
| Fake success | 3 | 1 | 1 | 5 |
| Invalid JSON | 2 | 2 | 1 | 5 |
| Shell rewrite | 3 | 1 | 1 | 5 |
| Package managers | 2 | 2 | 1 | 5 |
| Cloud/container | 1 | 2 | 2 | 5 |
| Windows/path | 2 | 2 | 1 | 5 |
| Integrations | 2 | 2 | 1 | 5 |
| **Total** | **18** | **13** | **9** | **40** |

P0 is the v0.1 trust gate. P1 and P2 are parity/backlog expansion after the first
vertical slice in each category proves the harness shape.

## TDD Acceptance Matrix

| Issue Class | First Red Test | Fixture Entry Point | Required Green Behavior | Do Not Implement Until |
| --- | --- | --- | --- | --- |
| Silent loss | `tss_preserves_requested_git_log_patch_or_passes_through` | `tests/fixtures/legacy-cli-issues/silent-loss/git-log-p-with-hidden-patch.raw.txt` | Patch-bearing output is exact passthrough or a valid patch with omission marker and raw handle. | Fixture contains real patch hunk, expected exit code, and assertion target. |
| Silent loss | `tss_does_not_drop_find_exec_output` | `tests/fixtures/legacy-cli-issues/silent-loss/find-exec-results-dropped.raw.txt` | `find -exec` passes through unless the whole predicate/action grammar is supported. | Test proves output equality and passthrough reason. |
| Silent loss | `tss_preserves_search_match_identity` | `tests/fixtures/legacy-cli-issues/silent-loss/rg-matches-rewritten-or-dropped.raw.txt` | File path, line number, matched text, and match count are retained. | Raw fixture includes multiple files and similar-looking matches. |
| Fake success | `tss_keeps_go_failure_visible` | `tests/fixtures/legacy-cli-issues/fake-success/go-test-compile-errors-success-banner.raw.txt` | Exit code and compile/vet/test failure markers are preserved. | Test captures non-zero status and specific diagnostic lines. |
| Fake success | `tss_rejects_package_test_success_rewrite` | `tests/fixtures/legacy-cli-issues/fake-success/npm-test-failed-but-passed.raw.txt` | A failed lifecycle script never emits passing status language. | Test asserts failure words remain and success-only wording is absent. |
| Fake success | `tss_does_not_invent_dotnet_test_counts` | `tests/fixtures/legacy-cli-issues/fake-success/dotnet-binlog-only-suppressed-counts.raw.txt` | Suppressed or unavailable counts stay unknown, not green. | Fixture documents requested mode and original exit code. |
| Invalid JSON | `tss_hook_invalid_json_degrades_without_mutation` | `tests/fixtures/legacy-cli-issues/invalid-json/hook-malformed-stdin.raw.json` | Invalid host JSON does not produce a fake rewrite or broken hook response. | Host integration output schema is identified. |
| Invalid JSON | `tss_preserves_rg_jsonl_parseability` | `tests/fixtures/legacy-cli-issues/invalid-json/rg-json-omission-marker-corruption.raw.jsonl` | Every emitted JSONL line parses, or the raw JSONL is exact passthrough. | Test parses emitted output, not just string-matches braces. |
| Invalid JSON | `tss_does_not_truncate_json_api_response` | `tests/fixtures/legacy-cli-issues/invalid-json/curl-json-truncated.raw.json` | JSON stays parseable or exact passthrough. | A parser-backed assertion is available. |
| Shell rewrite | `tss_passes_through_pipeline_and_redirect_shapes` | `tests/fixtures/legacy-cli-issues/shell-rewrite/pipeline-data-corruption.raw.txt` | Pipes and redirects do not get partially rewritten. | Safety-gate public behavior is asserted. |
| Shell rewrite | `tss_rejects_newline_trailing_command_rewrite` | `tests/fixtures/legacy-cli-issues/shell-rewrite/newline-trailing-command.raw.txt` | Newline-separated command text is blocked or passthrough, never auto-allowed. | Fixture captures both visible and trailing command text. |
| Shell rewrite | `tss_preserves_grep_flag_semantics` | `tests/fixtures/legacy-cli-issues/shell-rewrite/grep-rn-rewrite-collision.raw.txt` | `grep -rn` and `grep -E` are not translated into different ripgrep semantics. | Test describes argv and output, not internal classifier details. |
| Package managers | `tss_preserves_npm_workspace_lifecycle_failure` | `tests/fixtures/legacy-cli-issues/package-managers/npm-run-workspace-fail.raw.txt` | Workspace, script name, stderr, and exit code remain visible. | Fixture includes command, workspace, and nested runner output. |
| Package managers | `tss_preserves_pnpm_filter_failure` | `tests/fixtures/legacy-cli-issues/package-managers/pnpm-filter-lifecycle-error.raw.txt` | Selected package and underlying runner failure are retained. | Test proves passthrough or lossless summary. |
| Package managers | `tss_preserves_brew_failure_caveats` | `tests/fixtures/legacy-cli-issues/package-managers/brew-install-warning-and-failure.raw.txt` | Brew warnings, caveats, and failed formula steps are not collapsed into success. | Fixture separates warning noise from failure lines. |
| Cloud/container | `tss_passes_through_docker_logs_until_adapter_exists` | `tests/fixtures/legacy-cli-issues/cloud-container/docker-logs-error-stack.raw.txt` | Logs with error stacks pass through unless lossy output has raw recovery. | Product scope accepts the command or marks it planned passthrough. |
| Cloud/container | `tss_preserves_kubectl_wide_table_identity` | `tests/fixtures/legacy-cli-issues/cloud-container/kubectl-get-pods-wide.raw.txt` | Namespace, pod, readiness, restarts, age, IP, and node stay visible. | Fixture has enough rows to tempt lossy compaction. |
| Cloud/container | `tss_preserves_terraform_plan_semantics` | `tests/fixtures/legacy-cli-issues/cloud-container/terraform-plan-diff.raw.txt` | Create, update, replace, and destroy actions remain distinguishable. | Test can identify resource action markers. |
| Windows/path | `tss_handles_powershell_utf16_hook_output` | `tests/fixtures/legacy-cli-issues/windows-path/powershell-utf16-hook-output.raw.txt` | Hook output is UTF-8 for the host or explicitly unsupported with passthrough. | Fixture records original encoding expectation. |
| Windows/path | `tss_preserves_windows_backslash_paths` | `tests/fixtures/legacy-cli-issues/windows-path/backslash-drive-path.raw.txt` | Drive letters, backslashes, UNC prefixes, and spaces survive display and classification. | Test runs in a platform-neutral way. |
| Windows/path | `tss_preserves_crlf_failure_output` | `tests/fixtures/legacy-cli-issues/windows-path/crlf-test-output.raw.txt` | CRLF output does not create false diffs or hide failed lines. | Fixture includes meaningful CRLF-only differences. |
| Integrations | `tss_claude_hook_response_is_valid_and_quiet` | `tests/fixtures/legacy-cli-issues/integrations/claude-pretooluse-valid-rewrite.raw.json` | Valid Claude hook response has no success-path stderr and preserves command on passthrough. | Current host JSON shape is captured. |
| Integrations | `tss_codex_reports_instruction_only_limits` | `tests/fixtures/legacy-cli-issues/integrations/codex-instruction-only-init.raw.txt` | Codex integration states instruction/wrapper behavior and blind spots, not fake hooks. | Test asserts user-facing install/doctor output. |
| Integrations | `tss_gemini_beforetool_payload_roundtrips` | `tests/fixtures/legacy-cli-issues/integrations/gemini-beforetool-command.raw.json` | Gemini payload validates and preserves original command if passthrough. | Fixture includes representative host payload. |

## Vertical Order For v0.1

1. Silent loss tracer: `git log -p` patch fixture.
2. Fake success tracer: Go or npm failure fixture.
3. Invalid JSON tracer: hook malformed input fixture.
4. Shell rewrite tracer: pipeline or newline trailing command fixture.
5. Package manager tracer: npm workspace failure fixture.
6. Windows/path tracer: PowerShell UTF-16 or drive path fixture.
7. Integration tracer: Claude valid hook plus Codex instruction-only output.
8. Cloud/container tracer: Docker logs or kubectl wide table passthrough.

This order keeps the first four slices focused on the highest trust risks:
silent data loss, fake success, invalid structured output, and command rewrite
drift. Broader parity comes only after those safeguards are green.

## Test Shape

Each acceptance test should use the public TSS surface for the behavior under
test:

- CLI behavior through `tss run -- <cmd>` when exit code, stdout, stderr, or raw
  recovery matters.
- Filter contract behavior through existing contract-test helpers when the
  fixture is testing an adapter decision.
- Integration plan or hook-rendering behavior through integration public APIs
  when no shell command should execute.
- Parser-backed assertions for JSON, JSONL, diff, patch, or host hook payloads.

Avoid implementation-coupled tests. Test names should read like trust claims,
for example:

```text
tss_preserves_exit_code_when_wrapped_go_test_fails
tss_passes_through_kubectl_json_until_parser_backed_adapter_exists
tss_codex_init_reports_instruction_only_blind_spots
```

## Release Gate

For v0.1, do not enable or advertise a filter class unless all related P0
fixtures are green. For planned-but-unsupported surfaces such as broad Docker,
kubectl, Terraform, cloud CLIs, or native Windows hook rewrite, green passthrough
with plain blind-spot reporting satisfies the trust contract.

Savings claims must cite the exact fixture matrix and command set. The expected
claim shape is:

```text
On the v0.1 issue fixture matrix, TSS reduced emitted terminal tokens by X%
while all P0 trust fixtures passed.
```

Do not claim universal command coverage.
