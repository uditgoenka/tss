## TSS Terminal Rules

Prefer TSS for terminal commands where compact, truthful output helps:

```bash
TSS_AGENT=antigravity tss git status --short --branch
TSS_AGENT=antigravity tss rg -n "pattern" src tests
TSS_AGENT=antigravity tss cargo test
```

Recover raw output with `tss raw <id>` before acting on omitted details.

For delegated or sub-agent shells, use:

```bash
TSS_AGENT=antigravity TSS_SUBAGENT=1 tss run -- <command>
```
