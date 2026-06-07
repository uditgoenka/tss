#!/usr/bin/env python3
import json
import shlex
import sys


def emit(value):
    sys.stdout.write(json.dumps(value, separators=(",", ":")))
    sys.exit(0)


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
    emit({})

tool_args = (
    payload.get("toolArgs")
    or payload.get("tool_args")
    or payload.get("tool_input")
    or payload.get("toolInput")
    or {}
)

if not isinstance(tool_args, dict):
    emit({})

command_key = None
for candidate in ("command", "cmd", "shellCommand"):
    if isinstance(tool_args.get(candidate), str) and tool_args[candidate].strip():
        command_key = candidate
        break

if command_key is None:
    emit({})

command = tool_args[command_key]
trimmed = command.strip()
if (
    already_owned(trimmed)
    or trimmed.startswith("env TSS_AGENT=")
    or trimmed.startswith("TSS_AGENT=")
    or trimmed.startswith("TSS_BYPASS=1")
    or trimmed.startswith("env TSS_BYPASS=1")
):
    emit({})

modified = dict(tool_args)
modified[command_key] = "env TSS_AGENT=copilot tss run -- bash -lc " + shlex.quote(command)
emit({"modifiedArgs": modified})
