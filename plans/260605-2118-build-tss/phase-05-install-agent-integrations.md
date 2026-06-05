---
phase: 5
title: "Install agent integrations"
status: pending
effort: "2 days"
---

# Phase 5: Install agent integrations

## Overview

Add agent-specific integrations without pretending all agents support the same hook model.
Each integration must have a version check, a dry-run, and a rollback path.

## Context Links

- Depends on: [Phase 3](./phase-03-implement-safe-filter-engine-and-raw-store.md)
- Claude Code hooks: `hookSpecificOutput.permissionDecision` + `updatedInput`
- GitHub Copilot hooks: `preToolUse` lifecycle
- Gemini CLI: extensions and hooks
- OpenCode: plugin/tool execution hooks
- Codex: no general command-mutation hook; use instructions/wrapper mode

## Key Insights

- Integration drift caused many the prior token-saving tool failures.
- `tss init` must explain what is installed and what cannot be intercepted.
- Where command mutation is unsupported, use suggestion/instruction mode.

## Requirements

- `tss init claude`
- `tss init copilot`
- `tss init gemini`
- `tss init opencode`
- `tss init cursor`
- `tss init codex`
- `tss doctor integrations`
- `tss uninstall <agent>`

## Architecture

```text
integrations/
  claude.rs
  copilot.rs
  gemini.rs
  opencode.rs
  cursor.rs
  codex.rs
  installer.rs
```

Each adapter implements:

```rust
trait AgentIntegration {
    fn detect(&self) -> Detection;
    fn install(&self, scope: Scope, dry_run: bool) -> InstallPlan;
    fn verify(&self) -> Verification;
    fn uninstall(&self) -> UninstallPlan;
}
```

## Related Code Files

- Create: `/Users/uditgoenka/Desktop/workspace/tss/src/integrations/*.rs`
- Create: `/Users/uditgoenka/Desktop/workspace/tss/assets/hooks/**`
- Create: `/Users/uditgoenka/Desktop/workspace/tss/tests/contracts/integrations_*.rs`
- Modify after setup confirmation: `/Users/uditgoenka/Desktop/workspace/tss/AGENTS.md` or `CLAUDE.md`

## Implementation Steps

1. RED: add adapter contract test for detect/install/verify/uninstall plan shape.
2. GREEN: implement installer abstraction.
3. RED/GREEN vertical loop for Claude Code:
   - PreToolUse hook for Bash only.
   - Return valid JSON on every path.
   - Include `permissionDecision` only when host policy allows.
   - Never mutate non-Bash tool payloads.
4. RED/GREEN vertical loop for Copilot:
   - Generate hook config for CLI/cloud-supported formats.
   - If mutation unsupported, deny-with-suggestion or docs-only mode.
5. RED/GREEN vertical loop for Gemini:
   - Install extension/hook config.
   - Require restart note.
6. RED/GREEN vertical loop for OpenCode:
   - Install plugin adapter.
   - Handle non-zero rewrite codes correctly.
7. RED/GREEN vertical loop for Cursor:
   - Detect supported surfaces.
   - Prefer docs/instruction mode where hook mutation is unavailable.
8. RED/GREEN vertical loop for Codex:
   - Generate concise `AGENTS.md` guidance.
   - Add optional shell wrapper mode for users who explicitly opt in.
9. RED: add `tss doctor` coverage tests:
   - installed
   - active
   - commands intercepted
   - known blind spots
10. GREEN: implement doctor output.
11. REFACTOR: de-duplicate host config writers while tests remain green.

## Todo List

- [ ] Implement integration trait.
- [ ] Add Claude installer.
- [ ] Add Copilot installer.
- [ ] Add Gemini installer.
- [ ] Add OpenCode installer.
- [ ] Add Cursor/Codex safe modes.
- [ ] Add uninstall and doctor checks.

## Success Criteria

- [ ] Every hook emits valid host-specific JSON.
- [ ] Unsupported host capabilities are stated plainly.
- [ ] `tss doctor` identifies blind spots instead of implying full coverage.

## Risk Assessment

- Risk: host APIs change.
- Mitigation: version checks, contract fixtures, no shared JSON format assumptions.

## Security Considerations

- Installer never overwrites existing instructions without merge/diff.
- Project-local hooks require explicit trust.
- Auto-allow must be subset of host permission model.

## Next Steps

Phase 6 adds analytics without privacy regression.
