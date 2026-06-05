## TSS Command Output

Prefer `tss run -- <command>` for large human-readable terminal output when compacting output will not change the task result. Do not use TSS for structured output, patches, redirected or piped commands, or commands where exact stdout/stderr matters.

If TSS omits output, inspect the raw handle with `tss raw <id>` before making decisions from missing details. TSS is instruction-only for Codex unless the user explicitly configures a wrapper.
