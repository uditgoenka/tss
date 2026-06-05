# Research Summary

## Summary

the prior token-saving tool proves demand for command-output token reduction, but its open issue surface
shows the product risk: silent data loss and command drift. TSS should compete on
trust, not feature count.

## Evidence

- the prior token-saving tool repo checked on 2026-06-05: 595 open issues, 276 bugs, 132 high-priority, 109 filter-quality, 16 security.
- Udit-reported issues center on Next.js, Vitest, and subagent coverage.
- the prior token-saving tool architecture is command-wrapper heavy: many bespoke Rust modules plus TOML regex filters.
- Public security advisory CVE-2026-45792: project-local filter trust allowed silent output tampering before the prior token-saving tool 0.32.0.

## Gaps TSS Must Address

| Gap | TSS Response |
|-----|--------------|
| Silent truncation | Omission markers + raw handle |
| Fake success | Preserve exit code + non-zero failure validator |
| Invalid structured output | Parse check or passthrough |
| Rewrite drift | Conservative shell classifier |
| Agent hook drift | Per-host adapters + contract tests |
| Privacy concerns | Local-only, redacted analytics, opt-out raw store |
| Overstated savings | Fixture-backed benchmark methodology |

## Competitor Notes

- the prior token-saving tool: broad command coverage, major trust gap.
- Headroom/TokenPak/lessloss: broader prompt/proxy compression, not focused on deterministic terminal truth.
- chop/trs: terminal-compression competitors; TSS needs stronger trust story and open test corpus.

## Recommended MVP

Build a Rust CLI with:

- trust contract
- raw-output recovery
- safety-first filter engine
- high-value filters for Git, search/files, JS/TS, core test runners
- Claude/Codex/Gemini/Copilot/OpenCode/Cursor integration surfaces
- fixture-based verification harness

## References

- the prior token-saving tool: https://github.com/legacy-cli-ai/legacy-cli
- issues: https://github.com/legacy-cli-ai/legacy-cli/issues
- TSS repo: https://github.com/uditgoenka/tss
- CVE-2026-45792: https://advisories.gitlab.com/cargo/legacy-cli/CVE-2026-45792/
- Claude Code hooks: https://code.claude.com/docs/en/hooks
- GitHub Copilot hooks: https://docs.github.com/en/copilot/reference/hooks-reference
- Gemini CLI extensions: https://google-gemini.github.io/gemini-cli/docs/extensions/
