## TSS Terminal Rules

Use `TSS_AGENT=roo-code tss run -- <command>` for terminal commands that benefit from compact, truthful output.

Prefer passthrough or native commands for secrets, environment dumps, and machine-readable output unless TSS says the command shape is optimized.

For delegated or sub-agent shells, use:

```bash
TSS_AGENT=roo-code TSS_SUBAGENT=1 tss run -- <command>
```
