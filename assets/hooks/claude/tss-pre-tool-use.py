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
    emit(empty_context("TSS hook skipped: input was not valid JSON."))

tool_name = payload.get("tool_name") or payload.get("toolName") or payload.get("tool")
tool_input = payload.get("tool_input") or payload.get("toolInput") or payload.get("input") or {}

if tool_name != "Bash" or not isinstance(tool_input, dict):
    emit(empty_context("TSS hook skipped: only Claude Bash tool calls are command-mutable."))

command = tool_input.get("command")
if not isinstance(command, str) or not command.strip():
    emit(empty_context("TSS hook skipped: Bash payload had no command string."))

if command.strip().startswith("tss ") or command.strip().startswith("env TSS_AGENT="):
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
