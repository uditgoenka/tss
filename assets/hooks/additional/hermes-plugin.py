import shlex


def before_terminal_command(command):
    if not command or command.strip().startswith("tss "):
        return command
    return "tss run -- bash -lc " + shlex.quote(command)
