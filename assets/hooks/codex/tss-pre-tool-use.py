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


def first_executable(command):
    try:
        parts = shlex.split(command)
    except Exception:
        parts = command.strip().split()

    index = 0
    if parts[:1] == ["env"]:
        index = 1
        while index < len(parts):
            part = parts[index]
            if part == "--":
                index += 1
                break
            if part in ("-S", "--split-string"):
                return first_executable(parts[index + 1] if index + 1 < len(parts) else "")
            if part in ("-u", "-C", "--unset", "--chdir"):
                index += 2
                continue
            if part.startswith("-"):
                index += 1
                continue
            if "=" in part:
                index += 1
                continue
            break
    while index < len(parts) and "=" in parts[index] and not parts[index].startswith("-"):
        index += 1
    if parts[index:index + 1] == ["command"]:
        index += 1
    executable = parts[index] if index < len(parts) else ""
    return executable.replace("\\", "/").rsplit("/", 1)[-1]


def already_owned(command):
    executable = first_executable(command)
    return executable in ("tss", "rtk", "tss-wrapper.sh")


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
    already_owned(trimmed)
    or trimmed.startswith("env TSS_AGENT=")
    or trimmed.startswith("TSS_AGENT=")
    or trimmed.startswith("TSS_BYPASS=1")
    or trimmed.startswith("env TSS_BYPASS=1")
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
