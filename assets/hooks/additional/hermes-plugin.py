import shlex


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
    return first_executable(command) in ("tss", "rtk", "tss-wrapper.sh")


def before_terminal_command(command):
    if not command:
        return command
    trimmed = command.strip()
    if (
        already_owned(trimmed)
        or trimmed.startswith("env TSS_AGENT=")
        or trimmed.startswith("TSS_AGENT=")
        or trimmed.startswith("TSS_BYPASS=1")
        or trimmed.startswith("env TSS_BYPASS=1")
    ):
        return command
    return "env TSS_AGENT=hermes tss run -- bash -lc " + shlex.quote(command)
