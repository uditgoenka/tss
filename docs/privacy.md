# TSS Privacy Model

TSS is local-first. The MVP does not send analytics, raw output, command text, cwd, or errors to a remote service.

## What Is Stored Locally

TSS may store two local data classes:

- Raw-output recovery artifacts, used by `tss raw <id>`.
- Analytics ledger rows, used by `tss gain`.

Analytics rows store aggregate counts for raw bytes/tokens observed, emitted bytes/tokens, omitted bytes/tokens, filter name, command category, safety decision, passthrough reason, and a provider-cache caveat. Token counts are estimates based on byte size; they are not billing claims.

Analytics rows also store a local the prior token-saving tool migration coverage status for the command family:

- `optimized`: TSS has a wired, fixture-backed optimizer for the safe command shape.
- `passthrough-compatible`: TSS recognizes the migration vocabulary but keeps output raw for correctness.
- `planned`: the command or issue class is tracked for v0.1.x/vNext work but is not optimized yet.
- `blocked`: TSS should not optimize this class because it is unsafe, secret-bearing, or depends on untrusted project-local behavior.

`blocked` is a coverage-reporting status, not a claim that TSS blocks command execution. Runtime policy should still prefer raw passthrough unless an explicit safety guard denies a command.

## Command Argument Redaction

Command arguments are redacted by default. A command such as:

```text
deploy --token super-secret --prod
```

is stored as:

```text
deploy [args redacted: 3]
```

TSS does not store cwd by default. Full command storage and cwd storage require explicit opt-in configuration.

## Environment Toggles

- `TSS_NO_STORE=1` disables raw-output storage.
- `TSS_NO_ANALYTICS=1` disables analytics ledger writes.

Both toggles are local process controls and should be honored before any file is created.

## Retention

The default retention target is 30 days for local TSS artifacts. Installers and doctor output should report the active policy so users can decide whether local storage is acceptable for a project.

## Permissions

On Unix platforms, ledger files are created with `0600` permissions. Raw output may contain secrets, so users handling sensitive repositories should prefer `TSS_NO_STORE=1` and rely on raw passthrough when recovery storage is not acceptable.

## Remote Telemetry

Remote telemetry is off by default and is not part of the MVP skeleton. Any future remote telemetry must be opt-in, documented separately, and must preserve command argument redaction unless the user explicitly chooses otherwise.

command coverage and issue-class coverage tables are static local metadata. TSS does not contact the prior token-saving tool, GitHub, or any remote service when producing `doctor` or `gain` reports.
