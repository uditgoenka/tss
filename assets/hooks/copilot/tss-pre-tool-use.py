#!/usr/bin/env python3
import json
import shlex
import sys


def emit(value):
    sys.stdout.write(json.dumps(value, separators=(",", ":")))
    sys.exit(0)


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
if command.strip().startswith("tss "):
    emit({})

modified = dict(tool_args)
modified[command_key] = "tss run -- bash -lc " + shlex.quote(command)
emit({"modifiedArgs": modified})
