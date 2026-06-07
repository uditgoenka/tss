---
title: "Ship TSS v0.1.02 RTK coexistence"
description: "Prevent double-wrapping and stale-hook confusion when RTK and TSS are both installed, then ship v0.1.02 through PR, npm, release assets, and Homebrew."
status: in_progress
priority: P0
effort: 1 day
branch: "release/v0.1.02-rtk-coexistence"
tags: [release, tdd, integrations, packaging, rtk-coexistence]
blockedBy: []
blocks: []
created: 2026-06-07
createdBy: "ck:plan"
source: "ck-plan+tdd+ck-cook"
---

# Ship TSS v0.1.02 RTK Coexistence

## Overview

Users can reasonably have both TSS and RTK installed during migration. TSS must
avoid double-wrapping commands, must not falsely claim active integration when
RTK owns the live hook, and must ship a clean v0.1.02 package/release path
instead of mutating immutable v0.1.01 npm assets.

## Scope

- Add hook skip guards for commands already owned by RTK.
- Surface RTK/TSS coexistence conflicts in integration detection and doctor
  output.
- Add clear dry-run/coexist/takeover language for users choosing one active
  command-rewrite owner.
- Bump release surfaces to v0.1.02 / npm 0.1.2.
- Build release binary and checksums for v0.1.02.
- Raise a PR, merge it, publish GitHub release, and confirm npm publish.

## Non-Goals

- Do not delete RTK binaries, stats, caches, or historical config.
- Do not rewrite historical TSS analytics rows.
- Do not overwrite v0.1.01 release assets or npm 0.1.1 checksums.
- Do not claim package-manager output is optimized if it still runs raw.

## TDD Contract

Use vertical red-green-refactor slices:

1. Add one behavior test for RTK-owned command skip behavior.
2. Implement the smallest hook-script change to pass.
3. Add one behavior test for active RTK conflict detection.
4. Implement conflict status/warnings.
5. Add one release-surface/package test for v0.1.02.
6. Bump release metadata and verify packaging.

## Behavior Requirements

| Area | Required behavior |
|------|-------------------|
| Hook rewrite | Claude/Codex hooks skip `rtk`, `command rtk`, and environment-prefixed RTK commands instead of wrapping them with TSS. |
| Active detection | Claude is not active for TSS if active settings still point to `rtk hook claude`. |
| Conflict visibility | Doctor/integration notes mention RTK conflict when RTK owns the same host command rewrite path. |
| User control | Docs describe coexistence and takeover as separate migration choices. |
| Release | v0.1.02 uses new release assets and npm 0.1.2; v0.1.01 assets stay untouched. |

## File Ownership

| Slice | Files |
|-------|-------|
| Hook guards | `assets/hooks/claude/tss-pre-tool-use.py`, `assets/hooks/codex/tss-pre-tool-use.py`, integration tests |
| Conflict detection | `src/integrations/**`, `tests/contracts/integrations_contract.rs`, CLI doctor tests |
| Release bump | `Cargo.toml`, `package.json`, `npm/checksums.json`, `src/cli/mod.rs`, README/evals/docs/tests |
| Packaging | `.github/workflows/**`, `packaging/homebrew/tss.rb.template`, release assets |

## Validation Gates

- Focused red/green tests for each TDD slice.
- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
- Hook source compile check without writing `.pyc`.
- `node --check npm/postinstall.js`
- `npm audit --omit=dev`
- `npm pack --dry-run`
- Release binary SHA matches `npm/checksums.json`.
- Fresh install check for npm package after publish.
- Homebrew formula check after tap update.

## Cook Handoff

```bash
/ck:cook /Users/uditgoenka/Desktop/workspace/tss/plans/260607-1912-v0.1.02-rtk-coexistence/plan.md --auto --parallel --tdd
```
