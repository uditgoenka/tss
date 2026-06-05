# TSS v0.1.01 Evals

Tested on June 5, 2026.

These evals are local regression checks for TSS v0.1.01. They are not billing
claims. Token counts are estimated with the project estimator
(`ceil(bytes / 4)`), and actual provider billing can differ by tokenizer,
cache behavior, agent context policy, and model.

## Summary

| Eval | Iterations | Result | Measured savings | What it proves |
|------|------------|--------|------------------|----------------|
| Mixed fixture regression | 50 | 50/50 passed | 5 estimated tokens saved, 6.6% on final iteration | filtered output, passthrough output, raw recovery, gain dashboard shape |
| Large synthetic output | 50 | 50/50 passed | 139.7K estimated tokens saved, 100.0% rounded on final iteration | high-savings behavior on deterministic long terminal output with raw recovery |
| Agent/sub-agent gain regression | 50 | 50/50 passed | 10 estimated tokens saved, 6.7% on final iteration | `TSS_AGENT`, `TSS_SUBAGENT`, shell-wrapped inner-command filtering, `By Agent`, `Sub-Agent Usage`, `Top Sub-Agents` |

## What These Evals Prove

TSS v0.1.01 is evaluated against a trust-first contract:

1. Filtered output must keep enough actionable context to continue the task.
2. Lossy summaries must include a raw recovery handle.
3. Raw recovery must return the exact omitted output without re-running the
   original command.
4. Passthrough commands must preserve stdout/stderr and exit behavior.
5. `tss gain` must report savings without claiming provider billing accuracy.
6. Agent and sub-agent analytics must remain local and command-argument
   redacted.
7. Hook-style `bash -lc "<simple command>"` wrappers must filter and record the
   inner command, not a meaningless `bash` row.

## Environment

| Field | Value |
|-------|-------|
| Date tested | June 5, 2026 |
| Repository | `uditgoenka/tss` |
| Version under test | `0.1.01` |
| Binary source | local Rust source tree, built with Cargo |
| Operating system | macOS on Apple Silicon in the maintainer workspace |
| Analytics scope | temporary `TSS_HOME`, `TSS_ANALYTICS_FILE`, and `TSS_RAW_DIR` per iteration |
| Token estimator | `ceil(bytes / 4)` |
| Network dependency | none for command-output evals |

The evals intentionally use temporary state directories so previous local usage
cannot inflate savings numbers. Each iteration starts with a fresh analytics
ledger and raw-output store.

## Mixed Fixture Regression

Final iteration dashboard:

```text
TSS Token Savings (Local Scope)
============================================================

Total commands:                     2
Input tokens:                      76
Output tokens:                     71
Tokens saved:                       5 (  6.6%)
Safety fallbacks:                   1
Total exec time:                 12ms (avg 6ms)
Efficiency meter:        [█░░░░░░░░░░░░░░░░░░░]   6.6%

Command Coverage
------------------------------------------------------------
optimized                               1
passthrough-compatible                  1
planned                                 0
blocked                                 0

By Agent
------------------------------------------------------------
 #  Agent              Count    Sub     Saved    Avg%    Time  Impact
------------------------------------------------------------
 1. Manual / Unknown       2      0         5    6.6%     6ms  █░░░░░░░░░

Sub-Agent Usage
------------------------------------------------------------
sub-agent commands                      0
sub-agent input tokens                  0
sub-agent output tokens                 0
sub-agent tokens saved                  0 (  0.0%)

By Command
------------------------------------------------------------
 #  Command                         Count     Saved    Avg%    Time  Impact
------------------------------------------------------------
 1. cat [args redacted: 1]              1         5    6.8%     6ms  █░░░░░░░░░
 2. printf [args redacted: 1]           1         0    0.0%     6ms  ░░░░░░░░░░

estimated from bytes; actual billing depends on tokenizer and provider cache behavior.
```

Checks performed on every iteration:

- `tss run -- cat tests/fixtures/files/cat_long_single_file.txt`
- Output includes a `tss raw <id>` recovery handle.
- `tss raw <id> --combined` recovers the omitted `line 10: release verification`.
- `tss proxy printf 'hello\n'` remains exact passthrough.
- `tss gain` includes `TSS Token Savings (Local Scope)`, `Command Coverage`,
  `By Agent`, `Sub-Agent Usage`, `By Command`, `cat [args redacted: 1]`, and
  `printf [args redacted: 1]`.
- `tss gain` does not contain stale attribution or stale parity wording.

## Large Synthetic Output

The large-output eval generates a deterministic 30,000-line file in `/tmp`, then
runs `tss run -- cat <file>` and verifies raw recovery on every iteration.

Final iteration dashboard:

```text
TSS Token Savings (Local Scope)
============================================================

Total commands:                     1
Input tokens:                  139.7K
Output tokens:                     48
Tokens saved:                  139.7K (100.0%)
Safety fallbacks:                   0
Total exec time:                  3ms (avg 3ms)
Efficiency meter:        [████████████████████] 100.0%

Command Coverage
------------------------------------------------------------
optimized                               1
passthrough-compatible                  0
planned                                 0
blocked                                 0

By Agent
------------------------------------------------------------
 #  Agent              Count    Sub     Saved    Avg%    Time  Impact
------------------------------------------------------------
 1. Manual / Unknown       1      0    139.7K  100.0%     3ms  ██████████

Sub-Agent Usage
------------------------------------------------------------
sub-agent commands                      0
sub-agent input tokens                  0
sub-agent output tokens                 0
sub-agent tokens saved                  0 (  0.0%)

By Command
------------------------------------------------------------
 #  Command                         Count     Saved    Avg%    Time  Impact
------------------------------------------------------------
 1. cat [args redacted: 1]              1    139.7K  100.0%     3ms  ██████████

estimated from bytes; actual billing depends on tokenizer and provider cache behavior.
```

Checks performed on every iteration:

- Generate a 30,000-line deterministic text file under `/tmp`.
- Run `tss run -- cat <generated-file>`.
- Verify the filtered output includes a raw recovery handle.
- Verify `tss raw <id> --combined` recovers `release line 30000`.
- Verify `tss gain` renders the dashboard and by-command table.
- Verify the gain output does not contain stale attribution or stale parity
  wording.

## Agent/Sub-Agent Gain Regression

This eval is the v0.1.01 proof for the sub-agent leakage class reported in
multi-agent workflows. It verifies that TSS can measure and show child-agent
usage when a sub-agent actually invokes TSS.

Final iteration dashboard:

```text
TSS Token Savings (Local Scope)
============================================================

Total commands:                     3
Input tokens:                     150
Output tokens:                    140
Tokens saved:                      10 (  6.7%)
Safety fallbacks:                   1
Total exec time:                258ms (avg 86ms)
Efficiency meter:        [█░░░░░░░░░░░░░░░░░░░]   6.7%

Command Coverage
------------------------------------------------------------
optimized                               2
passthrough-compatible                  1
planned                                 0
blocked                                 0

By Agent
------------------------------------------------------------
 #  Agent              Count    Sub     Saved    Avg%    Time  Impact
------------------------------------------------------------
 1. Codex                  1      1         5    6.8%   234ms  █░░░░░░░░░
 2. OpenCode               1      0         5    6.8%    15ms  █░░░░░░░░░
 3. Claude Code            1      1         0    0.0%     9ms  ░░░░░░░░░░

Sub-Agent Usage
------------------------------------------------------------
sub-agent commands                      2
sub-agent input tokens                 76
sub-agent output tokens                71
sub-agent tokens saved                  5 (  6.6%)

Top Sub-Agents
 #  Agent              Name             Count     Saved    Avg%  Impact
------------------------------------------------------------
 1. Codex              scenario-scanner     1         5    6.8%  █░░░░░░░░░
 2. Claude Code        db-scanner           1         0    0.0%  ░░░░░░░░░░

By Command
------------------------------------------------------------
 #  Command                         Count     Saved    Avg%    Time  Impact
------------------------------------------------------------
 1. cat [args redacted: 1]              2        10    6.8%   124ms  █░░░░░░░░░
 2. printf [args redacted: 1]           1         0    0.0%     9ms  ░░░░░░░░░░

estimated from bytes; actual billing depends on tokenizer and provider cache behavior.
```

Checks performed on every iteration:

- Run a hook-style shell wrapper:
  `TSS_AGENT=codex TSS_AGENT_ROLE=sub-agent TSS_SUBAGENT=1 TSS_SUBAGENT_NAME=scenario-scanner tss run -- bash -lc 'cat tests/fixtures/files/cat_long_single_file.txt'`.
- Verify TSS filters the inner `cat` command and emits `file output`, not a raw
  `bash` passthrough.
- Verify the filtered output includes a `tss raw <id>` recovery handle.
- Verify `tss raw <id> --combined` recovers the omitted
  `line 10: release verification`.
- Run a main-agent `OpenCode` tagged filtered command.
- Run a `Claude Code` sub-agent passthrough command to prove fallbacks still
  appear in agent and sub-agent rows.
- Verify `tss gain` contains `By Agent`, `Sub-Agent Usage`, `Top Sub-Agents`,
  `Codex`, `OpenCode`, `Claude Code`, `scenario-scanner`, and `db-scanner`.
- Verify the command table contains `cat [args redacted: 1]` and does not
  collapse shell-wrapped savings into a misleading `bash [args redacted: ...]`
  row.

### Issue #1820 Boundary

This eval proves TSS records and reports sub-agent savings when the sub-agent
routes commands through TSS. It does not prove that Claude Code or any other
host inherits parent hooks into child-agent contexts, because that behavior is
controlled by the host. The supported mitigation is explicit:

```bash
eval "$(tss shell-init --agent claude-code --subagent)"
```

or per command:

```bash
TSS_AGENT=claude-code TSS_SUBAGENT=1 tss run -- <command>
```

If a child-agent shell runs raw commands and does not source wrappers or prefix
commands, TSS cannot measure or save those tokens.

## Reproduction Commands

The dated proof run used the built release binary from the current v0.1.01
source tree. The commands below use `cargo run --quiet --` as a source-tree
reproduction path; replacing it with an installed `tss` binary should produce
the same command-output behavior.

```bash
# Mixed fixture loop: 50 iterations
for i in $(seq 1 50); do
  state_dir="$(mktemp -d /tmp/tss-mixed-state.XXXXXX)"
  analytics_file="$state_dir/analytics.jsonl"
  raw_dir="$state_dir/raw"

  TSS_HOME="$state_dir" TSS_ANALYTICS_FILE="$analytics_file" TSS_RAW_DIR="$raw_dir" \
    cargo run --quiet -- run -- cat tests/fixtures/files/cat_long_single_file.txt

  TSS_HOME="$state_dir" TSS_ANALYTICS_FILE="$analytics_file" TSS_RAW_DIR="$raw_dir" \
    cargo run --quiet -- proxy printf 'hello\n'

  TSS_HOME="$state_dir" TSS_ANALYTICS_FILE="$analytics_file" TSS_RAW_DIR="$raw_dir" \
    cargo run --quiet -- gain
done
```

```bash
# Agent/sub-agent loop: 50 iterations
for i in $(seq 1 50); do
  state_dir="$(mktemp -d /tmp/tss-agent-eval-state.XXXXXX)"
  analytics_file="$state_dir/analytics.jsonl"
  raw_dir="$state_dir/raw"

  TSS_HOME="$state_dir" TSS_ANALYTICS_FILE="$analytics_file" TSS_RAW_DIR="$raw_dir" \
    TSS_AGENT=codex TSS_AGENT_ROLE=sub-agent TSS_SUBAGENT=1 \
    TSS_SUBAGENT_NAME=scenario-scanner \
    cargo run --quiet -- run -- bash -lc 'cat tests/fixtures/files/cat_long_single_file.txt'

  TSS_HOME="$state_dir" TSS_ANALYTICS_FILE="$analytics_file" TSS_RAW_DIR="$raw_dir" \
    TSS_AGENT=opencode \
    cargo run --quiet -- run -- cat tests/fixtures/files/cat_long_single_file.txt

  TSS_HOME="$state_dir" TSS_ANALYTICS_FILE="$analytics_file" TSS_RAW_DIR="$raw_dir" \
    TSS_AGENT=claude-code TSS_SUBAGENT=1 TSS_SUBAGENT_NAME=db-scanner \
    cargo run --quiet -- proxy printf 'hello\n'

  TSS_HOME="$state_dir" TSS_ANALYTICS_FILE="$analytics_file" TSS_RAW_DIR="$raw_dir" \
    cargo run --quiet -- gain
done
```

## Negative And Passthrough Cases

The evals intentionally include passthrough behavior because safe token saving
requires knowing when not to filter.

| Case | Expected behavior |
|------|-------------------|
| `tss proxy printf 'hello\n'` | exact passthrough and analytics fallback count |
| `git diff` and `git show` patch output | raw passthrough in v0.1.01 |
| package managers such as `npm`, `pnpm`, `brew`, `pip` | recognized but raw until fixture-backed |
| structured or exact output flags | pass through unless a parser-backed adapter exists |
| missing host hook inheritance in child agents | not assumed; use `tss shell-init --subagent` or explicit env prefix |

## Claims Not Made

- These are not billing guarantees.
- These are not claims that every supported agent host automatically mutates
  every terminal command.
- These are not claims that cloud, container, process-table, ESLint, or patch
  reducers are implemented in v0.1.01.
- These are not remote telemetry. All eval analytics are local JSONL files in
  temporary state directories.

```bash
# Large-output loop: 50 iterations
eval_dir="$(mktemp -d /tmp/tss-evals-large.XXXXXX)"
fixture="$eval_dir/long-output.txt"
seq 1 30000 | sed 's/^/release line /' > "$fixture"

for i in $(seq 1 50); do
  state_dir="$eval_dir/state-$i"
  analytics_file="$eval_dir/analytics-$i.jsonl"
  raw_dir="$eval_dir/raw-$i"

  TSS_HOME="$state_dir" TSS_ANALYTICS_FILE="$analytics_file" TSS_RAW_DIR="$raw_dir" \
    cargo run --quiet -- run -- cat "$fixture"

  TSS_HOME="$state_dir" TSS_ANALYTICS_FILE="$analytics_file" TSS_RAW_DIR="$raw_dir" \
    cargo run --quiet -- gain
done
```
