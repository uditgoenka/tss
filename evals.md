# TSS v0.1.0 Evals

Tested on June 5, 2026.

These evals are local regression checks for TSS v0.1.0. They are not billing
claims. Token counts are estimated with the project estimator
(`ceil(bytes / 4)`), and actual provider billing can differ by tokenizer,
cache behavior, agent context policy, and model.

## Summary

| Eval | Iterations | Result | Measured savings | What it proves |
|------|------------|--------|------------------|----------------|
| Mixed fixture regression | 50 | 50/50 passed | 5 estimated tokens saved, 6.6% on final iteration | filtered output, passthrough output, raw recovery, gain dashboard shape |
| Large synthetic output | 50 | 50/50 passed | 7.1K estimated tokens saved, 98.5% on final iteration | high-savings behavior on deterministic long terminal output with raw recovery |

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
Efficiency meter:        [#-------------------]   6.6%

Command Coverage
------------------------------------------------------------
optimized                               1
passthrough-compatible                  1
planned                                 0
blocked                                 0

By Command
------------------------------------------------------------
 #  Command                         Count     Saved    Avg%  Impact
------------------------------------------------------------
 1. cat [args redacted: 1]              1         5    6.8%  [#-------------------]
 2. printf [args redacted: 1]           1         0    0.0%  [--------------------]

estimated from bytes; actual billing depends on tokenizer and provider cache behavior.
```

Checks performed on every iteration:

- `tss run -- cat tests/fixtures/files/cat_long_single_file.txt`
- Output includes a `tss raw <id>` recovery handle.
- `tss raw <id> --combined` recovers the omitted `line 10: release verification`.
- `tss proxy printf 'hello\n'` remains exact passthrough.
- `tss gain` includes `TSS Token Savings (Local Scope)`, `Command Coverage`,
  `By Command`, `cat [args redacted: 1]`, and `printf [args redacted: 1]`.
- `tss gain` does not contain stale attribution or stale parity wording.

## Large Synthetic Output

The large-output eval generates a deterministic 500-line file in `/tmp`, then
runs `tss run -- cat <file>` and verifies raw recovery on every iteration.

Final iteration dashboard:

```text
TSS Token Savings (Local Scope)
============================================================

Total commands:                     1
Input tokens:                    7.2K
Output tokens:                    112
Tokens saved:                    7.1K ( 98.5%)
Safety fallbacks:                   0
Efficiency meter:        [####################]  98.5%

Command Coverage
------------------------------------------------------------
optimized                               1
passthrough-compatible                  0
planned                                 0
blocked                                 0

By Command
------------------------------------------------------------
 #  Command                         Count     Saved    Avg%  Impact
------------------------------------------------------------
 1. cat [args redacted: 1]              1      7.1K   98.5%  [####################]

estimated from bytes; actual billing depends on tokenizer and provider cache behavior.
```

Checks performed on every iteration:

- Generate a 500-line deterministic text file under `/tmp`.
- Run `tss run -- cat <generated-file>`.
- Verify the filtered output includes a raw recovery handle.
- Verify `tss raw <id> --combined` recovers `line 500`.
- Verify `tss gain` renders the dashboard and by-command table.
- Verify the gain output does not contain stale attribution or stale parity
  wording.

## Reproduction Commands

The local run used the current v0.1.0 source tree and `cargo run --quiet --` so
the tested binary matched the code in this repository.

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
# Large-output loop: 50 iterations
eval_dir="$(mktemp -d /tmp/tss-evals-large.XXXXXX)"
fixture="$eval_dir/long-output.txt"
for line in $(seq 1 500); do
  printf 'line %03d: deterministic long output for token-saving eval\n' "$line"
done > "$fixture"

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

