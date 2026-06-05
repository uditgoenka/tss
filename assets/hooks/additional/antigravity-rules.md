## TSS Terminal Rules

Prefer TSS for terminal commands where compact, truthful output helps:

```bash
tss git status --short --branch
tss rg -n "pattern" src tests
tss cargo test
```

Recover raw output with `tss raw <id>` before acting on omitted details.
