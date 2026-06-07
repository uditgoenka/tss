#!/usr/bin/env python3
import json
import shlex
import sys


def emit(value):
    sys.stdout.write(json.dumps(value, separators=(",", ":")))
    sys.exit(0)


def empty_context(message):
    return {
        "hookSpecificOutput": {
            "hookEventName": "PreToolUse",
            "additionalContext": message,
        }
    }


try:
    payload = json.load(sys.stdin)
except Exception:
    emit(empty_context("TSS Codex hook skipped: input was not valid JSON."))

tool_input = payload.get("tool_input") or payload.get("toolInput") or payload.get("input") or {}
if not isinstance(tool_input, dict):
    emit(empty_context("TSS Codex hook skipped: tool input was not an object."))

command_key = None
for candidate in ("cmd", "command"):
    if isinstance(tool_input.get(candidate), str):
        command_key = candidate
        break

if command_key is None:
    emit(empty_context("TSS Codex hook skipped: no shell command field was present."))

command = tool_input[command_key]
trimmed = command.strip()
if not trimmed:
    emit(empty_context("TSS Codex hook skipped: shell command was empty."))

if (
    trimmed.startswith("tss ")
    or trimmed.startswith("env TSS_AGENT=")
    or trimmed.startswith("TSS_AGENT=")
    or trimmed.startswith("TSS_BYPASS=1")
    or trimmed.startswith("env TSS_BYPASS=1")
    or trimmed.startswith("command tss ")
):
    emit({"hookSpecificOutput": {"hookEventName": "PreToolUse", "updatedInput": tool_input}})

updated_input = dict(tool_input)
updated_input[command_key] = "env TSS_AGENT=codex tss run -- bash -lc " + shlex.quote(command)

emit(
    {
        "hookSpecificOutput": {
            "hookEventName": "PreToolUse",
            "updatedInput": updated_input,
            "additionalContext": "TSS wrapped this shell command as Codex. Use `TSS_BYPASS=1` or start the command with `tss` when exact raw execution is required.",
        }
    }
)
