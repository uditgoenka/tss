# Command Parity Design

## Summary

TSS v0.1.0 should adopt familiar command vocabulary while keeping TSS trust
semantics as the product boundary. The design is not "match every the prior token-saving tool filter." It
is "make the prior token-saving tool muscle memory work, classify the command honestly, and optimize only
when the trust contract is proven by fixtures."

Primary design artifact: [docs/command-compatibility.md](../../../docs/command-compatibility.md).

## Decisions

| Decision | Rationale |
|----------|-----------|
| Keep `tss run -- <cmd>` canonical. | It is explicit, avoids meta-command ambiguity, and matches the existing CLI skeleton. |
| Add `tss <cmd> [args...]` as an familiar alias. | users expect prefix mode, for example `legacy-cli cargo test` and `legacy-cli npm run build`. |
| Add `tss proxy <cmd>` as an compatible passthrough alias. | the prior token-saving tool exposes `legacy-cli proxy <cmd>`; TSS can map it to exact raw command behavior without weakening trust semantics. |
| Keep `tss gain` and add `tss gain --history`. | the prior token-saving tool has `gain` and `gain --history`; TSS should report local, redacted savings ledgers only. |
| Use four status categories: optimized, passthrough-compatible, planned, blocked. | This separates command vocabulary recognition from real filtering support. |
| Include `brew`, package managers, cloud/container tools, and dev tools as recognized vocabulary. | This reduces the prior token-saving tool migration friction while avoiding unsupported optimization claims. |
| Make `tss doctor` the source of truth for compatibility reporting. | Agents need a quick way to know whether a command will be optimized, passed through, planned, or blocked. |

## Compatibility Surface

### Meta Commands

| Command | Design Status | Notes |
|---------|---------------|-------|
| `tss run -- <cmd> [args...]` | existing/canonical | Primary execution form. |
| `tss <cmd> [args...]` | planned alias | Alias for `tss run --` when `<cmd>` is not a TSS meta command. |
| `tss proxy <cmd> [args...]` | planned alias | compatible exact passthrough. |
| `tss raw <id>` | existing/canonical | Raw recovery for lossy summaries. |
| `tss gain` | existing/planned behavior | Should read local analytics ledger once Phase 6 is complete. |
| `tss gain --history` | planned alias | compatible recent savings history. |
| `tss doctor` | existing/planned behavior | Should report compatibility matrix, trust gates, raw store, and integration status. |
| `tss verify` | existing/canonical | Local verification harness entrypoint. |
| `tss init <agent>` | existing/canonical | Agent integration installer entrypoint. |

### Command Families

| Family | Examples | v0.1.0 Status | Reason |
|--------|----------|---------------|--------|
| Git | `git status`, `git log`, `git diff`, `git show`, `git branch` | optimized where adapter-covered | Core high-value the prior token-saving tool migration surface. |
| Search/files | `rg`, `grep`, `ls`, `find`, `cat`, `head`, `tail` | optimized where adapter-covered | High-volume agent output with clear safe passthrough boundaries. |
| JS/TS diagnostics | `next`, `tsc`, `vitest` | optimized where adapter-covered | Directly addresses known issue classes. |
| JS package managers | `npm`, `pnpm`, `yarn`, `npx`, `corepack` | passthrough-compatible | Workspace and script dispatch must not be rewritten without deeper fixtures. |
| JS runtimes/testers | `node`, `jest`, `mocha`, `playwright`, `cypress` | passthrough-compatible or planned | Recognize vocabulary; optimize only specific fixture-backed outputs. |
| Modern package managers | `bun`, `bunx`, `deno` | planned | Recognized parity targets with passthrough behavior. |
| Homebrew | `brew install`, `brew upgrade`, `brew doctor`, `brew bundle` | planned | Networked/system-mutating output; passthrough until dedicated fixtures exist. |
| Go/Rust/Python | `go test`, `cargo test`, `cargo check`, `rustc`, `pytest` | planned or optimized where covered | Test/check diagnostics are useful optimization targets; installs remain passthrough. |
| Build tools | `make`, `cmake`, `ninja`, `bazel`, `mvn`, `gradle` | planned | Broad output shapes require target-specific fixtures. |
| Containers | `docker`, `docker compose`, `podman`, `nerdctl` | planned | Build logs, streams, and status tables are high-value but risky. |
| Kubernetes | `kubectl`, `helm` | planned | JSON/YAML/watch modes must stay exact until stream/structured support exists. |
| Infrastructure | `terraform`, `tofu`, `pulumi`, `ansible` | planned | Plan/diff/state semantics are too risky for generic compression. |
| Cloud CLIs | `aws`, `gcloud`, `az`, `flyctl`, `vercel`, `netlify`, `wrangler`, `supabase` | planned | Auth prompts, JSON output, and remote side effects require passthrough by default. |
| Boundary-changing utilities | `xargs`, complex `sed`/`awk`, command substitution | blocked from filtering | These reshape command/output semantics and violate conservative classification. |

## Trust Semantics

Compatibility does not grant optimization. Runtime classification should preserve
this order:

1. If the command shape is unsafe to filter, mark it `blocked` and pass through
   unless an explicit execution guard denies it.
2. If an exact adapter supports the command shape, mark it `optimized`.
3. If the command is recognized but not adapter-covered, mark it `planned` or
   `passthrough-compatible`.
4. If the command is unknown but plain text and simple, mark it
   `passthrough-compatible`.
5. If structured output is requested without exact adapter support, mark it
   `blocked` from filtering and pass through unchanged.

Lossy optimized output must include omission accounting and `tss raw <id>`.
Passthrough output must preserve stdout, stderr, and exit code.

## `tss doctor` Design

`tss doctor` should become the compatibility dashboard for both humans and
agents.

Required human output sections:

- Version and build metadata.
- Trust-contract status.
- Raw store enabled/disabled, retention, and permission checks.
- Analytics enabled/disabled and redaction status.
- the prior token-saving tool compatibility aliases.
- Command matrix grouped by optimized, passthrough-compatible, planned, blocked.
- Agent integrations and any host-specific blind spots.
- Notes explaining that planned commands are recognized passthrough targets.

Required machine output:

```bash
tss doctor --format=json
tss doctor --commands --format=json
tss doctor --command -- kubectl get pods -o json
```

The JSON schema should expose `pattern`, `status`, `adapter`, `reason`, and
`trust_gate` for each command entry. Agents should be able to classify a command
without running it.

Example doctor classification:

| Command | Doctor Status | Reason |
|---------|---------------|--------|
| `tss git status --short --branch` | optimized | Fixture-backed Git status adapter. |
| `tss npm test` | passthrough-compatible | Package-manager script dispatch preserved raw. |
| `tss brew install node` | planned | Recognized command coverage target; no v0.1.0 filter. |
| `tss kubectl get pods -o json` | blocked from filtering | Structured output requires exact support; run passthrough. |
| `tss rg token src \| head` | blocked from filtering | Pipe syntax is unsafe to filter. |
| `tss proxy cargo test` | passthrough-compatible | Explicit exact raw command behavior. |

## Implementation Implications

This report is documentation-only, but the design implies later code work:

- Extend CLI parsing so direct prefix mode dispatches external commands while
  preserving existing TSS meta commands.
- Add `proxy` as a passthrough-only meta command.
- Add `gain --history`.
- Replace stub `doctor` output with a command registry and trust-gate report.
- Keep command registry entries separate from filter implementations so planned
  vocabulary cannot accidentally become optimized.
- Add contract tests for direct prefix aliasing, `proxy`, doctor JSON, and
  command classification.

## Risks

| Risk | Mitigation |
|------|------------|
| Users read "recognized" as "optimized." | Doctor and docs must use exact status terms and avoid generic "supported" wording. |
| Direct prefix alias shadows TSS meta commands. | Reserve meta commands first; only dispatch external commands after meta lookup fails. |
| Broad vocabulary pressures the project into unsafe filters. | Treat planned commands as passthrough until fixtures and trust-contract tests exist. |
| Cloud/container commands contain prompts, secrets, or streams. | Pass through by default; do not store/filter lossy output without explicit coverage and privacy review. |
| command coverage claims exceed evidence. | Savings and parity claims may reference only optimized, fixture-backed command shapes. |

## Acceptance Criteria

- `docs/command-compatibility.md` exists and defines familiar aliases, status
  categories, command-family matrix, blocked shapes, and doctor reporting.
- `tss doctor` design distinguishes optimized, passthrough-compatible, planned,
  and blocked statuses.
- Package managers, `brew`, cloud/container tools, and common dev tools are
  included as recognized vocabulary without unsafe optimization claims.
- The design preserves TSS trust semantics: passthrough over risky compression,
  raw recovery for lossy summaries, and structured output passthrough unless
  exact support exists.
