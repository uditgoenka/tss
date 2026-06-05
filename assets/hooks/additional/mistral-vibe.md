## TSS Terminal Rules

Mistral Vibe support is planned. Until a stable command-interception API is available, use explicit commands:

```bash
TSS_AGENT=mistral-vibe tss run -- <command>
tss proxy <command>
tss raw <id>
```

For delegated or sub-agent shells, use:

```bash
TSS_AGENT=mistral-vibe TSS_SUBAGENT=1 tss run -- <command>
```
