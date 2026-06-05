## TSS Terminal Rules

Use `TSS_AGENT=kilo-code tss run -- <command>` for noisy terminal commands and `tss raw <id>` when a lossy summary omits details.

Treat package-manager and environment commands as raw unless TSS classifies the exact command shape as optimized.

For delegated or sub-agent shells, use:

```bash
TSS_AGENT=kilo-code TSS_SUBAGENT=1 tss run -- <command>
```
