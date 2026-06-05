---
title: "TSS Trust Contract"
version: "0.1.01"
status: "draft"
last_updated: 2026-06-05
license: "Apache-2.0"
---

# TSS Trust Contract v0.1.01

TSS exists to reduce agent terminal tokens without making the agent less correct
than it would have been with raw command output. Correctness, auditability, and
recovery always outrank token savings.

This document is the root policy for filters, integrations, tests, documentation,
and release claims. Any future behavior that conflicts with this contract must be
rejected or handled as a contract change.

## Product Promise

TSS may compress command output only when the transformed output remains truthful,
recoverable, and safe for an agent to act on.

TSS must prefer passthrough over risky compression. Passthrough is a successful
outcome, not a failure.

## Required Invariants

Every TSS command path must preserve these invariants:

- Preserve the wrapped process exit code exactly.
- Preserve non-zero failure semantics. A failed command must still look failed.
- Never print, imply, or synthesize success when the wrapped command failed.
- Never silently truncate, hide, reorder, or rewrite material output.
- Keep requested structured output parseable, or pass it through unchanged.
- Emit an explicit omission marker for every lossy transformation.
- Attach a raw recovery handle to every lossy transformation.
- Make `tss raw <id>` capable of recovering the unmodified captured output when
  raw storage is enabled.
- Treat unclassified, complex, redirected, piped, or unparseable commands as
  passthrough by default.
- Treat security-sensitive uncertainty as passthrough by default.

## Output Classes

TSS output is one of three classes.

| Class | Meaning | Required Behavior |
|-------|---------|-------------------|
| Passthrough | Raw output is emitted unchanged. | Preserve stdout, stderr, exit code, and ordering as closely as the host shell allows. |
| Lossless summary | Output is shortened without removing user-relevant facts. | Must be validated by a command-specific contract test. No raw handle is required unless raw was captured for consistency. |
| Lossy summary | Some raw bytes are omitted from the emitted output. | Must include omission markers and a raw recovery handle. |

If a filter cannot prove which class it is producing, it must return passthrough.

## Command Pipeline Policy

The canonical execution model is:

```text
command -> classify -> safety gate -> execute -> filter -> validate -> emit
                         |                            |
                         v                            v
                    passthrough                 raw recovery
```

Each stage has a narrow responsibility.

| Stage | Responsibility |
|-------|----------------|
| Classify | Identify whether the command shape and requested output format are supported. |
| Safety gate | Reject risky shapes before filtering, including pipelines, redirects, shell expansions, project-local untrusted filters, and unknown structured-output modes. |
| Execute | Run the user command without changing its command semantics. |
| Filter | Apply only an approved command adapter with fixture-backed behavior tests. |
| Validate | Check process semantics, structured-output parseability, and failure preservation. |
| Emit | Print truthful output with omission markers and raw recovery handles when required. |

## Failure Semantics

Failure output is high-risk. Filters must not smooth over failures.

For every non-zero command result, TSS must:

- Preserve the exact exit code.
- Preserve enough stderr/stdout context for the agent to identify that the command
  failed and why.
- Preserve command-specific failure markers such as failing test names, compiler
  diagnostic locations, panic summaries, and stack roots when available.
- Add omission markers before removing repeated or low-signal failure detail.
- Include a raw recovery handle for lossy failure output.

A command adapter must pass through if it cannot distinguish failure context from
low-value noise.

## Structured Output

If a caller requested structured output, TSS must not break the format.

Structured output includes JSON, JSONL, XML, unified diff, patch output, machine
readable test output, and any command mode documented as parseable by tools.

For structured output, TSS must either:

- Emit valid output in the requested format after filtering, verified by a parser
  or round-trip contract test.
- Emit the original raw output unchanged.

TSS must not insert human-readable omission markers inside structured data unless
the marker itself is valid for that format and covered by a contract test.

## Raw Recovery

Raw recovery is the safety valve for lossy output.

When raw storage is enabled and TSS emits a lossy summary, the output must include
a stable handle in this form:

```text
[tss: omitted N lines, raw: tss raw <id>]
```

The exact wording may evolve, but every lossy output must clearly communicate:

- Something was omitted.
- How much was omitted when known.
- Which command retrieves the raw output.

Raw store requirements:

- Store unmodified stdout and stderr with metadata sufficient to reproduce the
  wrapped command result.
- Use restrictive local permissions, expected to be `0600` for files and `0700`
  for directories on Unix-like systems.
- Support `TSS_NO_STORE=1` to disable raw storage.
- Support retention controls before launch.
- Avoid remote upload by default.

If raw storage is disabled, lossy filtering must either be disabled or the output
must clearly state that raw recovery is unavailable.

## Privacy And Local Trust

TSS is local-first.

- Remote telemetry is off by default.
- Cloud telemetry is not part of the MVP.
- Project-local filters, config, or adapters are not trusted silently.
- User approval is required before any project-local behavior can affect command
  output.
- Analytics must be redacted, local by default, and documented before release.

Raw output may contain secrets. Any feature that stores, indexes, syncs, reports,
or shares raw output must be reviewed against this contract before implementation.

## Supported Filter Gate

A command adapter may be enabled only when all of the following are true:

- The command shape is inside the MVP scope or a later accepted scope document.
- The adapter has fixture coverage for success, failure, and noisy output where
  those states apply.
- The adapter has at least one public-interface contract test.
- The adapter preserves exit code, stderr/stdout meaning, and structured-output
  validity.
- The adapter passes through unsupported flags and unknown modes.
- The emitted output includes omission markers and raw handles for lossy cases.

No filter may merge on implementation logic alone.

## Contract Test Requirements

Future tests must verify observable behavior through the TSS public interface.

Required behavior classes:

- Passthrough for unknown commands.
- Passthrough for complex shell syntax, pipelines, and redirects.
- Exit-code preservation for success and failure.
- Non-zero failure visibility.
- JSON/diff/patch parseability or passthrough.
- Omission marker presence for lossy output.
- Raw recovery handle presence for lossy output.
- Raw recovery returns unmodified captured output when enabled.
- Raw storage can be disabled.
- Project-local filters are not auto-trusted.

Test names should describe user-observable behavior, for example:

```text
tss_preserves_exit_code_when_wrapped_command_fails
tss_passes_through_json_when_filter_would_break_parseability
tss_emits_raw_handle_for_lossy_test_output
```

## Release Claims

TSS must not claim generic savings without evidence. Any published savings claim
must identify the fixture matrix, command set, and measurement method.

Acceptable claim shape:

```text
On the v0.1 fixture matrix, TSS reduced emitted terminal tokens by X% while all
contract tests passed.
```

Unacceptable claim shape:

```text
TSS saves X% on terminal output.
```

## License Constraint

TSS is intended for an Apache-2.0 public repository at:

```text
https://github.com/uditgoenka/tss
```

All project code, docs, test fixtures, generated files, and vendored examples
must be compatible with Apache-2.0 distribution unless a later written decision
records a narrower exception.
