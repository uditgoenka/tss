import shlex


def before_terminal_command(command):
    if not command or command.strip().startswith("tss ") or command.strip().startswith("env TSS_AGENT="):
        return command
    return "env TSS_AGENT=hermes tss run -- bash -lc " + shlex.quote(command)
