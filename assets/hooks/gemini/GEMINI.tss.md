## TSS Command Output

Prefer `TSS_AGENT=gemini tss run -- <command>` for terminal commands where compact output is useful. Treat TSS as a wrapper, not as a permission or correctness layer. If a command is complex, structured, redirected, piped, or the exact output matters, run the raw command.

When TSS reports omitted output, use `tss raw <id>` before drawing conclusions from missing details.

For delegated or sub-agent shells, mark the usage:

```bash
TSS_AGENT=gemini TSS_SUBAGENT=1 tss run -- <command>
```
