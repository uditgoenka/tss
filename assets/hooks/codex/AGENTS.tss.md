## TSS Command Output

Prefer `TSS_AGENT=codex tss run -- <command>` for large human-readable terminal output when compacting output will not change the task result. Do not use TSS for structured output, patches, redirected or piped commands, or commands where exact stdout/stderr matters.

If TSS omits output, inspect the raw handle with `tss raw <id>` before making decisions from missing details. TSS is instruction-only for Codex unless the user explicitly configures a wrapper.

For spawned sub-agents or delegated worker shells, mark the command so `tss gain` can show sub-agent savings:

```bash
TSS_AGENT=codex TSS_SUBAGENT=1 tss run -- <command>
```

For a dedicated sub-agent shell, source wrappers explicitly:

```bash
eval "$(tss shell-init --agent codex --subagent)"
```
