# the prior token-saving tool Issue Fixture Backlog

This directory is reserved for raw terminal fixtures that reproduce issue and
open PR classes before TSS v0.1 claims support for the same command surface.

The goal is not command coverage by breadth. The goal is a public, replayable corpus
that proves TSS preserves command truth before it saves tokens.

## TDD Rule

Add fixtures one vertical slice at a time:

1. Add one raw fixture for one observable failure class.
2. Add one public-interface contract test that consumes that fixture.
3. Confirm the test fails for the missing behavior.
4. Implement the smallest passthrough, validation, or filter behavior needed to
   make the test pass.
5. Refactor only while green.

Do not add a bulk fixture dump without tests. A fixture is accepted only when a
contract test proves the behavior through the TSS public interface.

## Fixture Naming

Use this shape for new fixtures:

```text
tests/fixtures/legacy-cli-issues/<category>/<short-issue-class>.raw.<ext>
```

Preferred extensions:

- `.txt` for terminal text.
- `.json` for a single JSON payload.
- `.jsonl` for line-delimited JSON.
- `.diff` or `.patch` for patch-like output that must stay applicable.

Every fixture PR should state:

- Raw command that produced the output.
- Expected exit code.
- Whether filtered output may be lossy.
- Required recovery behavior if lossy.
- Structured format, if any.
- Source issue or PR link when the fixture came from a public report.

## Backlog Summary

The initial backlog has 8 categories and 40 proposed fixture candidates. P0
fixtures are the v0.1 trust gate. P1 and P2 are queued behind the first passing
slice in each category.

| Category | Count | Why It Exists |
| --- | ---: | --- |
| Silent loss | 5 | Prevent dropped lines, paths, patches, matches, or diagnostics without omission markers and raw recovery. |
| Fake success | 5 | Prevent failed commands from being summarized as passed, fixed, green, or successful. |
| Invalid JSON | 5 | Preserve parseable structured output or pass through unchanged. |
| Shell rewrite | 5 | Prevent command semantics from changing during rewrite, especially with pipes, redirects, newlines, quoting, and flags. |
| Package managers | 5 | Keep package-manager and lifecycle-script output honest across npm, pnpm, yarn, bun, and brew-style commands. |
| Cloud/container | 5 | Default broad operational CLIs to passthrough unless exact structured/text contracts exist. |
| Windows/path | 5 | Preserve Windows, WSL, CRLF, PowerShell, UTF-16, and path-with-space behavior. |
| Integrations | 5 | Prove host-specific hook and instruction modes without claiming fake auto-rewrite parity. |

## Candidate Fixtures

| Category | Priority | Proposed Fixture | Public Behavior To Prove |
| --- | --- | --- | --- |
| Silent loss | P0 | `silent-loss/git-log-p-with-hidden-patch.raw.txt` | `git log -p` output is passed through or remains a valid patch with no hidden requested hunks. |
| Silent loss | P0 | `silent-loss/find-exec-results-dropped.raw.txt` | `find -exec` output is passed through without partial predicate parsing. |
| Silent loss | P0 | `silent-loss/rg-matches-rewritten-or-dropped.raw.txt` | Search matches preserve file, line, match text, and count. |
| Silent loss | P1 | `silent-loss/ls-recursive-files-omitted.raw.txt` | Recursive listings do not hide paths without explicit omission and raw recovery. |
| Silent loss | P2 | `silent-loss/cat-tail-middle-loss.raw.txt` | Long file display uses range markers and raw recovery for omitted content. |
| Fake success | P0 | `fake-success/go-test-compile-errors-success-banner.raw.txt` | Go compile, vet, and test failures keep non-zero status and failure markers. |
| Fake success | P0 | `fake-success/npm-test-failed-but-passed.raw.txt` | Package test failure cannot be summarized as passing. |
| Fake success | P0 | `fake-success/dotnet-binlog-only-suppressed-counts.raw.txt` | Suppressed test counts do not become successful test counts. |
| Fake success | P1 | `fake-success/cargo-test-panic-status-zero-summary.raw.txt` | Rust panic or failed test output remains visibly failed. |
| Fake success | P2 | `fake-success/next-build-failed-success-summary.raw.txt` | Next build or compile failure preserves route, file, line, message, and failed status. |
| Invalid JSON | P0 | `invalid-json/hook-malformed-stdin.raw.json` | Hook input that is not valid host JSON degrades without breaking command execution. |
| Invalid JSON | P0 | `invalid-json/rg-json-omission-marker-corruption.raw.jsonl` | JSONL search output is valid JSONL after filtering or exact passthrough. |
| Invalid JSON | P1 | `invalid-json/curl-json-truncated.raw.json` | JSON API output is not truncated into invalid JSON. |
| Invalid JSON | P1 | `invalid-json/kubectl-get-pods-json.raw.json` | Kubernetes JSON is preserved exactly until a parser-backed adapter exists. |
| Invalid JSON | P2 | `invalid-json/npm-audit-json.raw.json` | Package audit JSON stays parseable or passes through unchanged. |
| Shell rewrite | P0 | `shell-rewrite/pipeline-data-corruption.raw.txt` | Pipeline or redirect syntax is passthrough by default. |
| Shell rewrite | P0 | `shell-rewrite/newline-trailing-command.raw.txt` | Newline-separated trailing commands are not auto-allowed or collapsed. |
| Shell rewrite | P0 | `shell-rewrite/grep-rn-rewrite-collision.raw.txt` | Flag collisions do not rewrite grep semantics into ripgrep replacement or encoding modes. |
| Shell rewrite | P1 | `shell-rewrite/env-assignment-lost.raw.txt` | Environment assignment and command argv are preserved. |
| Shell rewrite | P2 | `shell-rewrite/find-bare-native-action.raw.txt` | Native `find` actions are rejected for filtering unless fully understood. |
| Package managers | P0 | `package-managers/npm-run-workspace-fail.raw.txt` | Workspace lifecycle failure preserves script name, workspace, stderr, and exit code. |
| Package managers | P0 | `package-managers/pnpm-filter-lifecycle-error.raw.txt` | `pnpm --filter` failure preserves selected package and underlying script failure. |
| Package managers | P1 | `package-managers/yarn-berry-immutable-install.raw.txt` | Install failures and lockfile diagnostics pass through or remain complete. |
| Package managers | P1 | `package-managers/bunx-prisma-rewrite.raw.txt` | Runtime launcher commands are not rewritten into a different package command. |
| Package managers | P2 | `package-managers/brew-install-warning-and-failure.raw.txt` | Homebrew warnings, caveats, and failed formula steps remain visible. |
| Cloud/container | P0 | `cloud-container/docker-logs-error-stack.raw.txt` | Container logs with errors are passthrough unless lossy output has raw recovery. |
| Cloud/container | P1 | `cloud-container/docker-ps-wide-table.raw.txt` | Wide container tables preserve IDs, names, image, ports, and status. |
| Cloud/container | P1 | `cloud-container/kubectl-get-pods-wide.raw.txt` | Cluster state tables preserve namespace, pod, readiness, restarts, age, and node. |
| Cloud/container | P2 | `cloud-container/terraform-plan-diff.raw.txt` | Plan diffs remain complete enough to distinguish create, update, replace, and destroy. |
| Cloud/container | P2 | `cloud-container/aws-cloudformation-events.raw.txt` | Cloud event streams preserve failed resource names, status reasons, and ordering. |
| Windows/path | P0 | `windows-path/powershell-utf16-hook-output.raw.txt` | PowerShell hook output is UTF-8 for host consumption or explicitly unsupported. |
| Windows/path | P0 | `windows-path/backslash-drive-path.raw.txt` | Windows drive paths and backslashes survive classification and display. |
| Windows/path | P1 | `windows-path/git-bash-to-powershell-path.raw.txt` | Git Bash paths passed to PowerShell are not silently mangled. |
| Windows/path | P1 | `windows-path/crlf-test-output.raw.txt` | CRLF output comparisons do not manufacture diffs or hide failures. |
| Windows/path | P2 | `windows-path/path-with-spaces.raw.txt` | Paths with spaces remain quoted or tokenized correctly. |
| Integrations | P0 | `integrations/claude-pretooluse-valid-rewrite.raw.json` | Claude hook output uses the current host JSON shape and does not emit stderr on success. |
| Integrations | P0 | `integrations/codex-instruction-only-init.raw.txt` | Codex integration reports instruction/wrapper limits instead of fake hook parity. |
| Integrations | P1 | `integrations/gemini-beforetool-command.raw.json` | Gemini hook payloads validate and preserve original command on passthrough. |
| Integrations | P1 | `integrations/copilot-deny-with-suggestion.raw.json` | Copilot CLI limitation is represented as suggestion/deny behavior, not silent mutation. |
| Integrations | P2 | `integrations/opencode-plugin-before-execute.raw.json` | OpenCode plugin fixtures preserve command, cwd, and rollback expectations. |

## Acceptance Gates

For v0.1, a category is accepted only when:

- At least one P0 tracer fixture has a failing-then-passing public contract test.
- Any lossy output includes an omission marker and raw recovery handle.
- Structured output is parser-checked or exact passthrough.
- Exit code and non-zero failure semantics are preserved.
- Unsupported or unsafe command shapes are treated as honest passthrough.

For release claims, cite the exact fixture set used. Do not claim generic the prior token-saving tool
parity or generic token savings outside the published fixture matrix.
