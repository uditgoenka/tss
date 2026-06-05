<h1 align="center">Token Saving Scheme (TSS)</h1>

---

<p align="center">
  <strong>A trust-first Rust CLI that reduces AI-agent terminal tokens without hiding command truth.</strong>
</p>

<p align="center">
  Built for familiar terminal workflows — command parity + raw recovery + local privacy + fixture-backed filtering.
</p>

<p align="center">
  <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/Rust-CLI-orange?logo=rust&logoColor=white" alt="Rust CLI"></a>
  <a href="https://github.com/uditgoenka/tss/releases"><img src="https://img.shields.io/badge/version-0.1.0-blue.svg" alt="Version 0.1.0"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-Apache--2.0-green.svg" alt="License Apache-2.0"></a>
  <a href="package.json"><img src="https://img.shields.io/badge/npm-@uditgoenka/tss-CB3837?logo=npm&logoColor=white" alt="npm @uditgoenka/tss"></a>
  <a href="packaging/homebrew/tss.rb.template"><img src="https://img.shields.io/badge/Homebrew-ready-FBB040?logo=homebrew&logoColor=black" alt="Homebrew ready"></a>
</p>

<p align="center">
  <a href="#commands"><img src="https://img.shields.io/badge/Commands-familiar-555555" alt="Familiar commands"></a>
  <a href="https://x.com/intent/follow?screen_name=iuditg"><img src="https://img.shields.io/badge/Follow-@iuditg-000000?style=flat&logo=x&logoColor=white" alt="Follow @iuditg"></a>
  <a href="https://paypal.me/uditgoenka"><img src="https://img.shields.io/badge/Support-PayPal-003087?logo=paypal&logoColor=white" alt="Support on PayPal"></a>
</p>

<p align="center">
  <em>"Run the command → TSS keeps the truth → The agent sees fewer tokens"</em>
</p>

<p align="center">
  <em>You don't need silent truncation. You need raw recovery, mechanical verification, and a trust contract.</em>
</p>

<p align="center">
  <strong>Familiar commands. Rust single binary. npm + Homebrew ready. Local-only privacy. Honest passthrough when compression is unsafe.</strong>
</p>

<p align="center">
  <a href="#why-this-exists">Why</a> ·
  <a href="#how-it-works">How It Works</a> ·
  <a href="#installation">Installation</a> ·
  <a href="#commands">Commands</a> ·
  <a href="#quick-start">Quick Start</a> ·
  <a href="#tss-vs-rtk">TSS vs RTK</a> ·
  <a href="#migration">Migration</a> ·
  <a href="#faq">FAQ</a>
</p>

---

```text
              RUN                 CLASSIFY              FILTER               RECOVER
        +------------+        +-------------+       +-------------+      +-------------+
        | tss <cmd>  | -----> | Trust gate  | ----> | Adapter     | ---> | tss raw <id>|
        | tss run -- |        | Shape check |       | Omit safely |      | Full output |
        | tss proxy  |        | Exact modes |       | Mark losses |      | stdout/err  |
        +------------+        +-------------+       +-------------+      +-------------+

              DOCTOR              ANALYTICS             PACKAGING            AGENTS
        +------------+        +-------------+       +-------------+      +-------------+
        | tss compat |        | tss gain    |       | Cargo       |      | Claude      |
        | tss doctor |        | Local JSONL |       | npm         |      | Codex       |
        | Matrix     |        | Redacted    |       | Homebrew    |      | OpenCode    |
        +------------+        +-------------+       +-------------+      +-------------+
```

---

## Why This Exists

Agent terminals are noisy. Large command outputs burn context, hide the useful
lines, and slow down coding loops.

Terminal-output compression is useful only when the compressed output stays
true. TSS is built around the gap that matters most for production use:
correctness. A token-saving tool must not silently remove the line that explains
a failing test, corrupt structured output, rewrite shell semantics, or make the
model believe a failed command succeeded.

TSS starts from a stricter contract:

- Preserve exit codes and failure semantics.
- Keep raw output recoverable after every lossy summary.
- Pass through risky command shapes instead of guessing.
- Treat package managers, installers, cloud tools, and shell pipelines with
  caution.
- Make compatibility status visible with `tss doctor` and `tss compat`.

v0.1.0 is a working Rust baseline, not only a plan. It is intentionally honest:
some command families are optimized, many familiar commands are recognized as
passthrough-compatible, and dangerous shapes are blocked from filtering.

---

## How It Works

### Terminal Flow

```text
Without TSS:                         With TSS:

Agent  -- git status --> shell --> git        Agent  -- git status --> TSS --> git
  ^                              |              ^                         |
  |                              |              |                         |
  +------ raw terminal output ---+              +--- compact, verified ---+
          high token cost                              fewer tokens
                                                       raw handle when lossy
```

Four strategies are applied only when the adapter can preserve command truth:

1. **Smart filtering** - removes boilerplate while preserving failures.
2. **Grouping** - aggregates similar files, matches, and diagnostics.
3. **Bounded truncation** - keeps actionable context and emits omission markers.
4. **Deduplication** - collapses repeated lines with counts when safe.

### Pipeline

```text
1. Run a command through TSS
2. Parse the command shape without rewriting shell semantics
3. Classify it as optimized, passthrough-compatible, planned, or blocked
4. Store raw stdout/stderr locally when storage is enabled
5. Apply a fixture-backed adapter only if it can preserve the command truth
6. Emit compact output with omission markers and a raw recovery handle
7. Record local, redacted analytics for `tss gain`
```

Every adapter follows the trust contract. If TSS cannot prove a safe summary, it
emits the raw command output unchanged.

### Trust Rules

| # | Rule |
|---|------|
| 1 | **Passthrough by default** — unknown, complex, or risky shapes stay raw. |
| 2 | **Exit codes are sacred** — summaries must preserve success/failure state. |
| 3 | **No fake success** — failed test/build output cannot be summarized as green. |
| 4 | **Raw recovery is mandatory** — lossy output includes `tss raw <id>`. |
| 5 | **Structured output stays exact** — JSON, diffs, and null-delimited modes pass through unless exactly supported. |
| 6 | **No shell rewriting drift** — pipes, redirects, compound forms, and `xargs` are unsafe to filter. |
| 7 | **Local privacy first** — raw output and analytics are local; command args are redacted. |
| 8 | **Compatibility is honest** — recognized does not mean optimized. |

---

## Commands

| Command | What it does | Status |
|---------|--------------|--------|
| `tss run -- <cmd>` | Canonical wrapper form | implemented |
| `tss <cmd>` | direct prefix alias | implemented |
| `tss -- <cmd>` | Explicit direct command separator | implemented |
| `tss proxy <cmd>` | Exact passthrough escape hatch | implemented |
| `tss raw <id>` | Recover stored raw output | implemented |
| `tss raw <id> --stdout` | Recover only stdout | implemented |
| `tss raw <id> --stderr` | Recover only stderr | implemented |
| `tss raw <id> --combined` | Recover combined output | implemented |
| `tss gain` | Local savings summary | implemented |
| `tss gain --json` | Machine-readable savings summary | implemented |
| `tss doctor` | Health, trust, compatibility summary | implemented |
| `tss compat` | command migration matrix | implemented |
| `tss init [agent|--agent <agent>] [--dry-run] [-g]` | Install or preview agent integration assets | implemented |
| `tss verify` | Basic verification command | implemented |
| `tss --version` | Print version | implemented |

### Command Status Vocabulary

| Status | Meaning | Runtime behavior |
|--------|---------|------------------|
| `optimized` | Fixture-backed filter exists for this command shape. | May emit compact output with raw recovery. |
| `passthrough-compatible` | TSS recognizes the command but should not compress it in v0.1.0. | Runs raw and preserves stdout/stderr/exit code. |
| `planned` | Known parity target without a v0.1.0 adapter. | Runs raw and appears in compatibility reports. |
| `blocked` | Shape violates the trust contract for filtering. | Passes through or denies only under explicit guards. |

---

## Examples

### Directory And Search Output

```bash
tss ls -la
tss find . -name "*.rs"
tss rg -n "TokenStore" src tests
```

TSS keeps path identity, line numbers, match text, and omission markers. Exact
or structured modes pass through unless the adapter validates them.

### Git Output

```bash
tss git status --short --branch
tss git log -n 10 --oneline
tss git diff
```

Safe status and log shapes are compacted. Patch output stays raw in v0.1.0
unless TSS can preserve the patch exactly.

### Test Output

```bash
tss cargo test
tss go test ./...
tss pytest
tss vitest run
```

TSS keeps failed tests, compiler diagnostics, traceback context, and non-zero
exit states visible.

### Package Managers

```bash
tss npm test
tss pnpm install
tss brew install node
```

Package-manager output is recognized but raw in v0.1.0. Scripts, installers,
and resolver output are too risky to summarize without dedicated fixtures.

### Gain Dashboard

```bash
tss gain
```

```text
TSS Token Savings (Local Scope)
============================================================

Total commands:                     42
Input tokens:                    18.4K
Output tokens:                    5.1K
Tokens saved:                    13.3K ( 72.3%)
Safety fallbacks:                   9
Efficiency meter:        [##############------]  72.3%

Command Coverage
------------------------------------------------------------
optimized                              18
passthrough-compatible                 19
planned                                 4
blocked                                 1

By Command
------------------------------------------------------------
 #  Command                         Count     Saved    Avg%  Impact
------------------------------------------------------------
 1. git [args redacted: 2]             12      5.4K   81.2%  [################----]
 2. cargo [args redacted: 1]            5      3.2K   74.8%  [###############-----]
```

---

## Installation

### Cargo / Source

From source:

```bash
git clone https://github.com/uditgoenka/tss.git
cd tss
cargo build --release
cargo install --path .
```

From npm, after release publication:

```bash
npm install -g @uditgoenka/tss
```

From Homebrew, after tap publication:

```bash
brew install uditgoenka/tap/tss
```

The npm wrapper and Homebrew formula template are included in this repository as
v0.1.0 packaging scaffolding. Publish the GitHub release assets first, then the
npm package and Homebrew tap can point at those binaries.

### Pre-built Binaries

v0.1.0 release binaries are intended for GitHub Releases:

- macOS arm64 / x64
- Linux arm64 / x64
- Windows x64

The npm and Homebrew templates install the same Rust binary; they do not
reimplement the CLI in JavaScript or Ruby.

---

## Quick Start

### 1. Verify

```bash
tss --version
tss doctor
tss compat
```

### 2. Use It With Familiar Commands

```bash
tss git status --short --branch
tss rg -n "TokenStore" src tests
tss cargo test
tss npm test
tss proxy /bin/sh -c 'printf out; printf err >&2'
```

### 3. Recover Raw Output

```bash
tss raw <id>
tss raw <id> --combined
tss raw <id> --stdout
tss raw <id> --stderr
```

---

## Optimized v0.1.0 Families

These families have contract tests and fixtures in v0.1.0:

| Family | Commands / shapes | What TSS preserves |
|--------|-------------------|--------------------|
| Git state | `git status`, safe `git branch` | branch, tracking, staged/unstaged/untracked categories |
| Git history | `git log` | commit identity, ordering, merge visibility |
| Git exact modes | `git diff`, `git show`, verbose branch | raw patch/metadata output |
| Search | `rg`, `grep`, `egrep`, `fgrep` | file paths, line numbers, match text, structured modes |
| Files | `ls`, `find`, `cat`, `head`, `tail` | path identity, metadata flags, multi-file banners |
| JS/TS | `next`, `tsc`, `vitest` | routes, diagnostics, failed tests, parser failures |
| Go | `go test` | package status, build/vet errors, summaries |
| Rust | `cargo test`, `cargo check` | compiler errors, spans, failed tests |
| Python | `pytest` | collection errors, tracebacks, failures, summaries |

### Passthrough-Compatible v0.1.0 Families

These commands are recognized for familiar terminal workflows but preserved raw
in v0.1.0:

```bash
npm pnpm yarn yarnpkg npx pnpx bun bunx deno corepack brew
node jest mocha playwright cypress ava tap uvu karma wdio
pip pip3 pipx uv uvx poetry rye
```

Package managers can run arbitrary scripts and installers. TSS does not compress
them until a fixture proves the output can be safely summarized.

---

## Token Savings

TSS reports estimates, not billing guarantees. Provider-side caching, model
tokenizers, and agent context policies change actual savings.

<h3 align="center">v0.1.0 Regression Proof</h3>

<p align="center">
  <strong>100/100 local eval iterations passed on June 5, 2026.</strong><br>
  The regression loop verifies filtered output, passthrough output, raw recovery,
  and the dashboard-style <code>tss gain</code> report. A deterministic 500-line
  synthetic output eval measured <strong>98.5% estimated token reduction</strong>
  with raw recovery intact. See <a href="evals.md">evals.md</a> for the dated
  commands, criteria, caveats, and exact result.
</p>

| Operation | Raw behavior | TSS behavior | v0.1.0 status |
|-----------|--------------|--------------|---------------|
| `git status --short --branch` | repeated file/status lines | branch + counts + changed files | optimized |
| long `git log` | many commits | keeps leading commits and merge visibility | optimized |
| `cat` long file | full file body | keeps leading lines + raw handle | optimized |
| `rg -n` many matches | every match line | groups by file, keeps line numbers | optimized |
| `tsc --noEmit` | ANSI-heavy diagnostics | strips ANSI, preserves codes/locations | optimized |
| `vitest run` many failures | verbose runner output | keeps failed tests + actionable stacks | optimized |
| `cargo test` failure | full compiler/test output | preserves errors, spans, failed tests | optimized |
| `npm test` | package script output | raw passthrough | passthrough-compatible |
| `brew install` | installer/progress output | raw passthrough | passthrough-compatible |

Run:

```bash
tss gain
tss gain --json
```

---

## TSS vs RTK

| Area | TSS v0.1.0 | RTK |
|------|------------|-----|
| Core promise | Save tokens only when output remains truthful and recoverable. | Broad terminal-output compression for agent workflows. |
| Default behavior | Passthrough for unknown, risky, structured, or state-changing command shapes. | Broader command filtering surface with more aggressive savings targets. |
| Raw recovery | Lossy summaries include `tss raw <id>` when local storage is enabled. | Recovery behavior depends on the specific command path and setup. |
| Failure handling | Non-zero exits, failed tests, compiler errors, and omission markers are contract-tested. | Users should verify failure summaries carefully on risky command shapes. |
| Structured output | JSON, diffs, null-delimited output, and exact modes pass through unless parser-backed. | Structured output can be fragile if filtering is not parser-backed. |
| Package managers | `npm`, `pnpm`, `yarn`, `bun`, `deno`, `pip`, and `brew` are recognized but raw in v0.1.0. | More package-manager convenience may be available depending on command coverage. |
| Agent support | 16 tool surfaces are represented; several are instruction-only because host APIs differ. | Established integrations and auto-rewrite flows for many tools. |
| Privacy | Local raw store and local analytics; args redacted; no remote telemetry in v0.1.0. | Review telemetry/config behavior for the installed version and setup. |
| Distribution | Rust binary with Cargo, npm wrapper, and Homebrew formula template for v0.1.0. | Existing package-manager distribution and release assets. |
| Verification posture | Fixture-backed filters first; passthrough is treated as a valid safety result. | Optimized breadth is higher, but users should audit correctness-sensitive cases. |

TSS is intentionally conservative in v0.1.0. Lower savings on some commands is
acceptable when the alternative is hiding the line that explains a failure.

---

## Migration

TSS keeps familiar command-wrapper habits while avoiding unsafe compression
claims.

| Existing habit | TSS command | Notes |
|-----------|-------------|-------|
| `<wrapper> <cmd> [args...]` | `tss <cmd> [args...]` | direct prefix alias |
| `<wrapper> proxy <cmd>` | `tss proxy <cmd>` | raw passthrough |
| `<wrapper> gain` | `tss gain` | local savings summary |
| `<wrapper> --version` | `tss --version` | version output |
| command matrix | `tss compat` | migration status |

Passthrough-compatible is not a failure. It means TSS chose correctness over
unsafe savings for that command shape.

### Failure Classes Covered

| Issue class | TSS response |
|-------------|--------------|
| Silent truncation/data loss | omission markers + raw handles |
| Fake success after failure | validator rejects success-only summaries on non-zero exits |
| Structured output corruption | exact-mode passthrough unless parser-backed |
| Shell rewrite drift | complex shell syntax is unsafe to filter |
| Agent hook drift | integration plans state blind spots per agent |
| Telemetry/privacy overclaim | local ledger, redacted args, no remote telemetry |
| Package-manager ambiguity | passthrough-compatible until fixture-backed |
| Secret-bearing output | env-style commands blocked from optimization |

---

## Supported AI Tools

TSS v0.1.0 includes integration plans and assets for common coding agents. The
status column is intentionally explicit: not every host exposes a safe terminal
command mutation API.

| Tool | Install command | Method | v0.1.0 status |
|------|-----------------|--------|---------------|
| Claude Code | `tss init -g` | PreToolUse Bash hook | command rewrite asset; TSS never grants command permission |
| GitHub Copilot (VS Code) | manual guidance | editor instructions | instruction-only in v0.1.0; no VS Code auto-rewrite claim |
| GitHub Copilot CLI / Cloud Agent | `tss init -g --copilot` | `.github/hooks` preToolUse hook | command-argument rewrite plan |
| GitHub Copilot CLI suggestion mode | `tss init -g --copilot-cli` | suggestion/deny mode | CLI host limitations are represented explicitly |
| Cursor | `tss init --agent cursor` | project rule | instruction-only terminal guidance |
| Gemini CLI | `tss init -g --gemini` | extension + memory file | instruction-only in v0.1.0 |
| Codex | `tss init -g --codex` | `AGENTS.md` + optional wrapper | instruction/wrapper mode; no fake hook parity |
| Windsurf | `tss init --agent windsurf` | `.windsurfrules` | project-scoped instruction mode |
| Cline | `tss init --agent cline` | `.clinerules` | project-scoped instruction mode |
| Roo Code | `tss init --agent roo` | `.roo/rules` | project-scoped instruction mode |
| OpenCode | `tss init -g --opencode` | plugin JS | bash-command plugin plan |
| OpenClaw | `tss init --agent openclaw` | plugin JS | terminal command plugin plan |
| Pi.dev | `tss init --agent pi.dev` | `.pi/extensions` TypeScript extension | command-field rewrite when the host extension API is available |
| Hermes | `tss init --agent hermes` | Python plugin adapter | terminal-command mutation plan |
| Mistral Vibe | `tss init --agent mistral-vibe` | instruction placeholder | planned; no command mutation claim |
| Kilo Code | `tss init --agent kilocode` | project rules | project-scoped instruction mode |
| Google Antigravity | `tss init --agent antigravity` | project rules | project-scoped instruction mode |

Files live under `assets/hooks/`. Run `tss init <agent> --dry-run` to inspect
the planned writes before installing.

---

## Configuration

TSS configuration is environment-variable based in v0.1.0.

| Setting | Meaning |
|---------|---------|
| `TSS_NO_STORE=1` | disables raw-output storage |
| `TSS_NO_ANALYTICS=1` | disables local analytics ledger writes |
| `TSS_HOME` | changes the local TSS state directory |
| `TSS_RAW_DIR` | changes raw-output storage directory |
| `TSS_ANALYTICS_FILE` | changes analytics ledger path |

Default analytics redact command arguments. Savings reports are based on byte
estimates and include a provider-cache caveat.

---

## Privacy & Storage

TSS is local-first.

| Setting | Meaning |
|---------|---------|
| `TSS_NO_STORE=1` | disables raw-output storage |
| `TSS_NO_ANALYTICS=1` | disables local analytics ledger writes |
| `TSS_HOME` | changes the local TSS state directory |
| `TSS_RAW_DIR` | changes raw-output storage directory |
| `TSS_ANALYTICS_FILE` | changes analytics ledger path |

No remote telemetry is emitted in v0.1.0.

---

## Development

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets -- -D warnings
node --check npm/bin/tss
node --check npm/postinstall.js
npm pack --dry-run --ignore-scripts
ruby -c packaging/homebrew/tss.rb.template
```

## Uninstall

Remove the package through the installer you used:

```bash
npm uninstall -g @uditgoenka/tss
cargo uninstall tss
brew uninstall tss
```

Agent integration cleanup is file-based in v0.1.0. Run a dry run to inspect the
installed paths, then remove generated TSS files for that agent:

```bash
tss init --agent codex --dry-run
rm -f AGENTS.tss.md .codex/tss-wrapper.sh
```

Raw output and analytics are local. Remove the local state directory if you want
to delete all saved raw output:

```bash
rm -rf "${TSS_HOME:-$HOME/.local/share/tss}"
```

---

### Repository Structure

```text
tss/
├── README.md
├── Cargo.toml
├── package.json
├── src/
│   ├── cli/              # user-facing command surface
│   ├── core/             # command model, runner, raw store, safety gate
│   ├── filters/          # fixture-backed command adapters
│   ├── analytics/        # local gain ledger and compatibility matrix
│   ├── integrations/     # agent integration plans
│   └── privacy/          # local storage and redaction policy
├── tests/
│   ├── contracts/        # public-interface contract tests
│   └── fixtures/         # raw command-output fixtures
├── assets/hooks/         # Claude, Copilot, Gemini, OpenCode, Cursor, Codex assets
├── npm/                  # npm wrapper and installer
├── packaging/homebrew/   # formula template
├── docs/                 # trust, scope, privacy, compatibility
└── plans/                # build plan and research reports
```

### Test Coverage

The contract suite covers:

- CLI aliases and familiar commands
- raw recovery modes
- safety gates and structured-output validation
- git/files/search filters
- JS/TS, Go, Rust, and Python test runners
- npm, brew, Python package-tool passthrough behavior
- privacy and local analytics
- agent integration plans
- npm/Homebrew packaging scaffolding

---

## FAQ

**Q: Is TSS a generic terminal compressor?**<br>
A: No. TSS is a trust-first command wrapper. Unsafe output stays raw.

**Q: Why Rust?**<br>
A: TSS is a terminal proxy. Rust gives a small single binary, fast startup,
predictable distribution, and strict control over command execution and local
file permissions.

**Q: Does npm mean a JavaScript implementation?**<br>
A: No. The npm package is a distribution wrapper around the Rust binary.

**Q: Does Homebrew install the same binary?**<br>
A: Yes. The Homebrew formula template installs the release binary.

**Q: Why are `npm`, `brew`, and `pip` mostly passthrough?**<br>
A: They can run scripts, installers, resolvers, and network operations. TSS
recognizes them for workflow familiarity but does not compress them without
fixtures.

**Q: How do I know what TSS will optimize?**<br>
A: Run `tss compat` for the migration matrix and `tss doctor` for local health.

**Q: What if TSS removes something important?**<br>
A: Lossy summaries include a `tss raw <id>` handle. Use `tss raw <id>` to recover
the exact stored output.

**Q: Does TSS send telemetry?**<br>
A: No remote telemetry in v0.1.0. The analytics ledger is local and redacts args.

**Q: Can I disable storage?**<br>
A: Yes. Set `TSS_NO_STORE=1`.

---

## Roadmap

- Release automation for GitHub assets, npm, and Homebrew checksums.
- `tss doctor --commands` and `tss doctor --format=json`.
- More fixture-backed adapters for cloud, container, package-manager, and
  language-tool output.
- Stream-aware filtering for long-running logs.
- Safer install/uninstall flows for agent integrations.
- More Windows-native smoke testing.

---

## Contributing

Contributions are welcome. The bar is simple: add a raw fixture, add a contract
test, prove the unsafe case passes through, then implement the smallest safe
filter.

Before submitting changes:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets -- -D warnings
```

---

## License

Apache-2.0 — see [LICENSE](LICENSE).

---

## Acknowledgements

- Rust — for making a small, fast, portable CLI practical.
- Claude Code, OpenCode, Codex, Gemini, Copilot, and Cursor — for pushing the
  terminal-agent workflow forward.

---

<h2 align="center">Contributor</h2>

<p align="center">
  <a href="https://github.com/uditgoenka">uditgoenka</a>
</p>

---

<h2 align="center">Support</h2>

<p align="center">
  Support development through <a href="https://paypal.me/uditgoenka">PayPal</a>.
</p>
