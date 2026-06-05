## TSS Terminal Rules

Prefer `TSS_AGENT=cline tss run -- <command>` for noisy terminal commands.

Use `tss proxy <command>` when exact passthrough is required.

When output contains `tss raw <id>`, inspect the raw output before relying on missing lines.

For delegated or sub-agent shells, use:

```bash
TSS_AGENT=cline TSS_SUBAGENT=1 tss run -- <command>
```
