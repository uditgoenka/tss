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
    emit(empty_context("TSS hook skipped: input was not valid JSON."))

tool_name = payload.get("tool_name") or payload.get("toolName") or payload.get("tool")
tool_input = payload.get("tool_input") or payload.get("toolInput") or payload.get("input") or {}

if tool_name != "Bash" or not isinstance(tool_input, dict):
    emit(empty_context("TSS hook skipped: only Claude Bash tool calls are command-mutable."))

command = tool_input.get("command")
if not isinstance(command, str) or not command.strip():
    emit(empty_context("TSS hook skipped: Bash payload had no command string."))

trimmed = command.strip()
if (
    already_owned(trimmed)
    or trimmed.startswith("env TSS_AGENT=")
    or trimmed.startswith("TSS_AGENT=")
    or trimmed.startswith("TSS_BYPASS=1")
    or trimmed.startswith("env TSS_BYPASS=1")
):
    emit({"hookSpecificOutput": {"hookEventName": "PreToolUse", "updatedInput": tool_input}})

updated_input = dict(tool_input)
updated_input["command"] = "env TSS_AGENT=claude-code tss run -- bash -lc " + shlex.quote(command)

emit({
    "hookSpecificOutput": {
        "hookEventName": "PreToolUse",
        "updatedInput": updated_input,
        "additionalContext": "TSS wrapped this Bash command as claude-code. If a child/sub-agent shell does not inherit this hook, run `eval \"$(tss shell-init --agent claude-code --subagent)\"` in that child shell or prefix commands with `TSS_AGENT=claude-code TSS_SUBAGENT=1 tss run --`.",
    }
})
