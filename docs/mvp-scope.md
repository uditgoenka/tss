---
title: "TSS MVP Scope"
status: "draft"
last_updated: 2026-06-05
license: "Apache-2.0"
---

# TSS v0.1 Scope

TSS v0.1 proves a trust-first product claim:

```text
TSS can offer familiar command support while preserving command truth,
failure semantics, structured-output validity, and raw-output recovery.
```

The scope is intentionally broader than the first draft. v0.1 should feel
familiar to users, including common package-manager commands such as `brew`,
`npm`, `pnpm`, `yarn`, and related dev-tool wrappers. However, familiarity does
not override the trust contract.

Command state vocabulary:

| State | Meaning |
|-------|---------|
| Optimized | TSS has a fixture-backed filter that emits compact, truthful output. |
| Passthrough-compatible | TSS recognizes the command and preserves raw behavior without unsafe compression. |
| Planned | Command is in the v0.1 registry but not yet optimized; doctor/docs show this clearly. |
| Blocked by trust contract | Command shape is too risky for automatic filtering, such as binary downloads piped to extractors. |
| Needs research | issue/PR data shows demand, but semantics are not understood enough for v0.1 optimization. |

The v0.1 release target is zero unmapped known command families, not 100%
optimized filters.

## In Scope

TSS v0.1 includes:

- A Rust CLI distributed as a single binary.
- Safe command classification with passthrough as the default.
- Raw-output recovery for lossy summaries.
- Contract-tested filters for high-value command families.
- familiar command compatibility registry.
- Honest reporting for every known command family.
- Local-first privacy defaults.
- Agent integration surfaces for documented host behavior.
- Fixture-backed measurement for savings claims.

## v0.1 Command Surface

Command adapters may be added only when backed by fixtures and the trust
contract. Other familiar commands should still be recognized and classified
as passthrough-compatible, planned, blocked, or needs-research.

| Family | Commands | v0.1 Intent |
|--------|----------|------------|
| Git state | `git status`, `git branch` | Reduce repetitive state listings while preserving branch, dirty state, staged/unstaged meaning, and untracked visibility. |
| Git history | `git log` | Summarize long history output while preserving commit identity, ordering, refs, and requested formatting. |
| Git diff | `git diff` | Reduce large diffs only when patch structure remains valid or raw recovery is available. |
| GitHub/GitLab | `gh`, `glab` | Preserve machine-readable modes; optimize list/view/run logs only with fixtures. |
| Search | `rg`, `grep` | Collapse repeated matches and long context while preserving filenames, line numbers, match counts, and failure/no-match semantics. |
| File listing | `ls`, `find` | Summarize large listings while preserving path identity and explicit omission counts. |
| File display | `cat`, `head`, `tail` | Omit long content only with clear range markers and raw recovery. |
| JS/TS package managers | `npm`, `pnpm`, `yarn`, `npx`, `pnpx`, `bun`, `bunx`, `uvx` | Preserve script/workspace semantics; optimize install/test noise only with fixtures. |
| JS/TS tests | `vitest`, `jest` | Preserve failing suite/test names, assertion summaries, locations, and final status. |
| Type/build tools | `tsc`, `next` | Preserve diagnostic locations, error codes, build failure state, and final status. |
| Go tests | `go test` | Preserve package status, failing test names, panic roots, and final status. |
| Rust checks | `cargo test`, `cargo check` | Preserve compiler/test diagnostics, failure status, and actionable spans. |
| Python tests | `pytest` | Preserve failing test names, assertion summaries, traceback roots, and final status. |
| Python package tools | `pip`, `pip3`, `uv`, `poetry`, `ruff`, `mypy` | Preserve install/failure semantics; optimize list/outdated/lint output with fixtures. |
| Ruby tools | `bundle`, `rake`, `rspec`, `rubocop` | Preserve failures and install result; optimize repetitive install/test output with fixtures. |
| JVM/.NET | `gradle`, `gradlew`, `mvn`, `dotnet` | Preserve compiler/test result counts and logs; structured reports pass through unless parsed. |
| Homebrew/packages | `brew`, `composer`, `pre-commit` | Preserve install/update/failure state; strip progress/noise only with fixtures. |
| Containers | `docker`, `docker compose`, `kubectl`, `helm` | Optimize listings/log noise cautiously; structured YAML/JSON and exec modes pass through by default. |
| Cloud/infra | `aws`, `az`, `gcloud`, `terraform`, `tofu`, `sops`, `ansible-playbook` | v0.1 registry support with selective optimized filters; sensitive/structured output defaults to passthrough. |
| System/network | `env`, `ps`, `df`, `du`, `stat`, `wc`, `curl`, `wget`, `ssh`, `scp`, `dig`, `lsof`, `journalctl` | Recognize and classify; optimize only safe text/list/log modes. |

Adapters for these commands are not automatically approved. Each adapter still
needs fixtures and public-interface contract tests before merge.

## Command Shapes

Supported command shapes are simple direct invocations where TSS can reason about
the command and output format.

Examples:

```bash
tss -- git status --short
tss -- rg "TokenStore" src tests
tss -- npm test
tss -- cargo test
```

The MVP must pass through by default for:

- Pipelines, such as `rg foo | head`.
- Redirects, such as `cargo test > out.txt`.
- Compound shell forms, such as `cmd1 && cmd2`.
- Command substitution.
- Aliases or shell functions that cannot be classified safely.
- Unknown flags that request machine-readable output unless a parser-backed
  adapter supports them.

## Agent Integrations

The MVP may provide integration guidance or installable wrappers for:

- Claude Code.
- Codex instruction/wrapper mode.
- Gemini CLI.
- GitHub Copilot.
- OpenCode.
- Cursor.

Agent integrations must respect each host's real extension mechanism. Codex must
not be documented as having hook parity when the correct integration shape is an
instruction or wrapper mode.

Each integration must have a contract test, fixture, or documented manual smoke
check before release.

## Explicit Non-Goals

TSS v0.1 does not include:

- Cloud telemetry.
- Remote raw-output storage.
- Project-local filters that are auto-trusted.
- Compression for pipelines, redirects, or complex shell syntax by default.
- A plugin framework.
- A multi-crate architecture before the core product is proven.
- Claims that TSS is a universal terminal compression layer.
- Claims that token savings are guaranteed outside the published fixture matrix.
- 100% optimized parity with the prior token-saving tool. v0.1 targets command recognition and honest behavior first.

## Security And Privacy Boundaries

The MVP must preserve these boundaries:

- Raw output is stored locally with restrictive permissions when storage is
  enabled.
- `TSS_NO_STORE=1` disables raw storage.
- Telemetry is absent or local-only until a later privacy document accepts more.
- Secrets in terminal output are treated as user data, not product analytics.
- Project-local configuration cannot alter output without explicit user approval.

## Acceptance Criteria

The MVP is ready to launch only when:

- The trust contract has public-interface tests for its critical invariants.
- Every enabled adapter has success, failure, and passthrough fixtures where
  applicable.
- Lossy output includes omission markers and raw recovery handles.
- Structured output is parser-checked or passed through.
- The verification harness includes familiar failure classes from the research
  and red-team notes.
- The open issue/PR matrix has no unclassified high-priority issue class.
- The command compatibility registry lists every known command family with a state.
- Savings claims cite the exact fixture matrix and command set.
- The repository remains Apache-2.0 compatible.

## Deferred Setup Work

The plan identified `setup-matt-pocock-skills` outputs for agent docs, issue
tracking, triage labels, and domain docs. Those files are deferred until the user
confirms the setup choices required by that skill.

Do not create these paths without that confirmation:

- `AGENTS.md`
- `CLAUDE.md`
- `docs/agents/issue-tracker.md`
- `docs/agents/triage-labels.md`
- `docs/agents/domain.md`
