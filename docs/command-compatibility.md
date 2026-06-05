---
title: "TSS Command Compatibility"
version: "0.1.0"
status: "draft"
last_updated: 2026-06-05
license: "Apache-2.0"
---

# TSS Command Compatibility v0.1.0

TSS v0.1.0 should feel familiar to users without inheriting the legacy tool's trust
risks. The command vocabulary is intentionally broad, but optimization claims are
intentionally narrow. Unknown, risky, structured, destructive, or unimplemented
command shapes must run as passthrough unless an adapter has trust-contract test
coverage.

## Compatibility Goals

- Accept familiar prefix usage for common shell commands.
- Keep `tss run -- <cmd> [args...]` as the canonical, least ambiguous form.
- Provide familiar meta commands where they fit TSS semantics.
- Recognize package managers, cloud/container tools, and developer tools even
  before optimizing them.
- Report compatibility honestly through `tss doctor` and `tss compat`.
- Never mark a command optimized because the vocabulary recognizes it.

## Invocation Vocabulary

The preferred form is:

```bash
tss run -- git status --short
tss run -- npm test
tss run -- cargo test
```

For the prior token-saving tool familiarity, v0.1.0 should also accept direct prefix aliases:

```bash
tss git status --short
tss npm test
tss cargo test
tss pytest -q
```

Direct prefix mode is an alias for `tss run --` only when the first argument is a
recognized external command or a command classified as passthrough-compatible.
It must not shadow TSS meta commands.

## the prior token-saving tool Meta Command Mapping

| the prior token-saving tool Command | TSS v0.1.0 Command | Status | Notes |
|-------------|--------------------|--------|-------|
| `legacy-cli <cmd> [args...]` | `tss <cmd> [args...]` | implemented | Direct prefix alias for `tss run -- <cmd> [args...]`. |
| `legacy-cli proxy <cmd>` | `tss proxy <cmd>` | implemented | Passthrough-only alias. Must not filter lossy summaries. |
| `legacy-cli gain` | `tss gain` | implemented | Shows local savings ledger summary, or a clear empty state. |
| `legacy-cli gain --history` | `tss gain --history` | planned | Shows recent local ledger rows with redacted command args. |
| `legacy-cli --version` | `tss --version` | implemented | Reports TSS version. |
| command matrix | `tss compat` | implemented | Prints the v0.1.0 the prior token-saving tool migration matrix. |
| `which legacy-cli` | `which tss` | passthrough-compatible | Shell-level check, not a TSS command. |

`tss proxy <cmd>` exists for muscle memory. It should be equivalent to a
passthrough policy decision and should still preserve exit code, stdout, and
stderr. It should not be described as "unsafe mode"; it is the compatibility
escape hatch for exact raw command behavior.

## Status Categories

`tss doctor` and public docs should use the same vocabulary.

| Status | Meaning | Runtime Behavior |
|--------|---------|------------------|
| optimized | TSS has an enabled adapter with fixture and contract coverage for this command shape. | May emit lossless or lossy summaries. Lossy output requires omission markers and raw recovery. |
| passthrough-compatible | TSS recognizes the command as safe to run through the wrapper, but no filter is enabled for this shape. | Executes raw command and emits raw output unchanged. |
| planned | TSS recognizes the command family as a parity target, but v0.1.0 does not yet optimize it. | Executes raw command and reports planned passthrough. |
| blocked | TSS refuses to filter, or optionally refuses to run, because the shape violates the trust contract. | Passes through for unsafe-to-filter shapes; denies only when an explicit guard blocks execution. |

Passthrough is a successful compatibility result. It means the wrapper preserved
truth over savings.

## Optimized v0.1.0 Targets

These command shapes may be optimized only when their adapter is enabled and
their tests pass.

| Family | Commands / Shapes | Required Preservation |
|--------|-------------------|-----------------------|
| Git state | `git status`, `git branch` | Branch, tracking, staged/unstaged/untracked categories, and exact verbose modes. |
| Git history | `git log` | Commit identity, ordering, merge visibility, and requested formatting. |
| Git diff/show | `git diff`, `git show` | Patch structure and machine-consumer modes. Pass through exact modes. |
| Search | `rg`, `grep`, `egrep`, `fgrep` | File paths, line numbers, match text, counts, no-match semantics, and structured modes. |
| Files | `ls`, `find`, `cat`, `head`, `tail` | Path identity, metadata flags, multi-file banners, and explicit omission counts. |
| JS/TS diagnostics | `next`, `tsc`, `vitest` | Failure status, failing tests, compiler locations, routes, stack roots, and final summaries. |
| Go tests | `go test` | Package status, build/vet errors, failing tests, panic roots, and coverage summaries. |
| Rust checks/tests | `cargo test`, `cargo check`, `rustc` diagnostics | Error codes, spans, failing test names, and final status. |
| Python tests | `pytest` | Collection errors, failing test names, assertion summaries, traceback roots, and final status. |

If an optimized command requests JSON, XML, JSONL, diff/patch, null-delimited, or
other machine-readable output and no parser-backed adapter supports that exact
mode, TSS must pass it through unchanged.

## Package Manager Compatibility

Package managers should be easy to remember from the prior token-saving tool usage. Most package-manager
commands are passthrough-compatible in v0.1.0 because package scripts can execute
arbitrary tools and workspace routing must not be rewritten.

| Command Family | Examples | Status | v0.1.0 Rule |
|----------------|----------|--------|-------------|
| npm | `npm test`, `npm run build`, `npm exec`, `npx vitest` | passthrough-compatible | Preserve package script banner, workspace flags, subprocess output, and exit code. |
| pnpm | `pnpm test`, `pnpm --filter web build`, `pnpm dlx` | passthrough-compatible | Do not rewrite filter/workspace selection. |
| Yarn | `yarn test`, `yarn workspace web build`, `yarn dlx` | passthrough-compatible | Preserve workspace command semantics. |
| Bun | `bun test`, `bun run build`, `bunx` | passthrough-compatible | Recognize as package/runtime vocabulary; pass through until fixtures exist. |
| Deno | `deno test`, `deno task build` | passthrough-compatible | Pass through until permission and diagnostic fixtures exist. |
| Corepack | `corepack pnpm test`, `corepack enable` | passthrough-compatible | Treat as package-manager dispatcher. |
| Homebrew | `brew install`, `brew upgrade`, `brew doctor`, `brew bundle` | passthrough-compatible | Pass through by default; do not summarize install/upgrade failures without fixtures. |

`brew` is included for familiar vocabulary, not because TSS should optimize
installer output in v0.1.0. Homebrew commands can be long-running, networked, and
system-mutating, so filtering must wait for dedicated fixtures.

## Cloud, Container, And Dev Tool Compatibility

TSS should recognize these commands so agents get predictable doctor output and
passthrough behavior instead of "unknown command" confusion.

| Family | Commands | Status | Trust Rule |
|--------|----------|--------|------------|
| Containers | `docker`, `docker compose`, `podman`, `nerdctl` | planned | Pass through by default. Do not compact build logs or container errors until failure fixtures exist. |
| Kubernetes | `kubectl`, `helm`, `k9s` command invocations | planned | Structured modes such as `-o json`, `-o yaml`, and watch streams must pass through. |
| Infrastructure | `terraform`, `tofu`, `pulumi`, `ansible` | planned | Plans, diffs, prompts, and state-changing output are high-risk. Pass through unless exact adapters exist. |
| Cloud CLIs | `aws`, `gcloud`, `az`, `flyctl`, `vercel`, `netlify`, `wrangler`, `supabase` | planned | Pass through. JSON/table modes and auth prompts must remain exact. |
| Build tools | `make`, `cmake`, `ninja`, `bazel`, `gradle`, `mvn` | planned | Pass through until target-specific failure fixtures exist. |
| Language tools | `python`, `python3`, `pip`, `pipx`, `uv`, `poetry`, `rye`, `go`, `cargo`, `rustc` | planned | Optimize only covered test/check shapes; package/install commands pass through. |
| Linters/formatters | `eslint`, `prettier`, `ruff`, `mypy`, `biome`, `shellcheck` | planned | Preserve file locations, rule IDs, fixability, and non-zero status before optimizing. |
| Shell utilities | `sed`, `awk`, `jq`, `yq`, `xargs`, `tar`, `curl`, `wget`, `ssh`, `scp`, `rsync` | blocked or passthrough-compatible | Treat transformations, network calls, and boundary-changing tools conservatively. `xargs` is unsafe to filter. |

Planned cloud/container/dev-tool recognition is about honest compatibility. It
must not be used for savings claims.

## Blocked Shapes

The following shapes are blocked from filtering even when the command family is
recognized:

- Pipelines, such as `tss rg token | head`.
- Redirection, such as `tss cargo test > out.txt`.
- Compound shell syntax, such as `cmd1 && cmd2`.
- Command substitution and backticks.
- Background jobs.
- `xargs` or command forms that reshape argument boundaries.
- Destructive commands when an explicit destructive guard is enabled, such as
  `rm -rf` or `git clean -f`.
- Structured output modes without exact adapter support.
- Watch/streaming modes until TSS has stream-aware semantics.
- Project-local filters or configs that have not been explicitly trusted.

The default blocked behavior is "unsafe to filter, so passthrough." Denial is
reserved for explicit execution guards.

## `tss doctor` Compatibility Report

`tss doctor` should report command compatibility as a matrix, not a binary health
check. It should answer four questions:

1. Which TSS meta commands and the prior token-saving tool aliases are available?
2. Which command families are optimized, passthrough-compatible, planned, or
   blocked?
3. Which trust-contract gates are active?
4. Which local privacy and raw-recovery settings affect compatibility?

Suggested output shape:

```text
tss doctor

version: 0.1.0
trust: ok
raw store: enabled, retention 30d, permissions ok
analytics: local, command args redacted

legacy-cli compatibility:
  tss <cmd>                 planned
  tss proxy <cmd>           planned
  tss gain                  planned
  tss gain --history        planned

commands:
  optimized:
    git status, git log, rg -n, cat, next, tsc, vitest, go test
  passthrough-compatible:
    npm, pnpm, yarn, npx, pnpx, bun, deno, corepack, brew, node, jest, mocha, playwright
  planned:
    docker, kubectl, terraform, aws, gcloud, az
  blocked:
    pipelines, redirects, xargs, untrusted project-local filters, unsupported structured modes

notes:
  passthrough-compatible commands preserve raw output and exit code.
  planned commands are recognized parity targets, not optimized filters.
  run `tss doctor --commands --format=json` for machine-readable status.
```

Current and planned flags:

| Command | Purpose |
|---------|---------|
| `tss doctor` | Implemented human-readable health and compatibility summary. |
| `tss compat` | Implemented the prior token-saving tool migration command matrix. |
| `tss doctor --commands` | Planned expanded command matrix with status, reason, and adapter name. |
| `tss doctor --format=json` | Planned machine-readable doctor output for agents and CI. |
| `tss doctor --command -- <cmd> [args...]` | Planned command-shape classifier without execution. |
| `tss doctor --legacy-cli` | Planned the prior token-saving tool migration aliases and unsupported the prior token-saving tool behavior report. |

The JSON format should include at least:

```json
{
  "version": "0.1.0",
  "trust": "ok",
  "raw_store": {
    "enabled": true,
    "retention_days": 30,
    "permissions": "ok"
  },
  "commands": [
    {
      "pattern": "git status",
      "status": "optimized",
      "adapter": "git",
      "reason": "fixture-backed status summary"
    },
    {
      "pattern": "docker build",
      "status": "planned",
      "adapter": null,
      "reason": "recognized parity target, passthrough in v0.1.0"
    }
  ],
  "blocked_shapes": [
    {
      "pattern": "pipeline",
      "status": "blocked",
      "reason": "pipe syntax is unsafe to filter"
    }
  ]
}
```

Doctor output must not imply that planned commands are optimized. It should
prefer phrases such as "recognized, passthrough" over "supported" when no filter
is enabled.

## Documentation And Release Rules

- The README may say TSS is familiar in command vocabulary.
- The README must not say TSS has command coverage unless the command is optimized or
  explicitly passthrough-compatible.
- Savings claims may mention only optimized command shapes with fixtures.
- Every new optimized entry requires tests for success, failure, structured or
  exact modes where applicable, omission markers, and raw recovery.
- Every new planned entry requires a doctor reason so agents understand why it
  passes through.
