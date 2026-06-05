## TSS Terminal Rules

Use `TSS_AGENT=windsurf tss run -- <command>` or `tss <command>` for terminal commands when compact, truthful output helps the coding task.

If TSS emits a raw handle, recover details with `tss raw <id>` before making decisions from omitted output.

Do not route secrets, environment dumps, or unsupported structured output through lossy summaries.

For delegated or sub-agent shells, use:

```bash
TSS_AGENT=windsurf TSS_SUBAGENT=1 tss run -- <command>
```
