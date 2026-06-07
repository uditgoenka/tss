---
title: "Build TSS trust-first token saving scheme"
description: "Build TSS as a trust-first Apache-2.0 CLI that reduces agent terminal tokens without silent data loss."
status: ready_for_build
priority: P1
effort: 12-16 days
branch: "main"
tags: [feature, cli, security, agent-integrations, token-optimization, tdd]
blockedBy: []
blocks: []
created: 2026-06-05
createdBy: "ck:plan"
source: skill
---

# Build TSS trust-first token saving scheme

## Overview

Build TSS (Token Saving Scheme) as the safer alternative to the prior token-saving tool:
save tokens, expose familiar familiar commands, and never silently make the
agent less correct than raw command output.

Scope now targets familiar v0.1 command support. This does not mean every
command is optimized on day one. It means every known command path is classified
as optimized, passthrough-compatible, planned, blocked by the trust contract, or
needs research. Unsafe partial compression is forbidden.

Research inputs:
- [Research Summary](./reports/research-summary.md)
- [Red-Team Review](./reports/red-team-review.md)

## Product Principles

- Preserve exit code and failure semantics exactly.
- Unknown, complex, redirected, piped, or unparseable commands pass through by default.
- Every lossy output includes omission markers and a `tss raw <id>` recovery handle.
- Structured output stays valid when the caller requested structured output.
- Local-first privacy. Remote telemetry off by default.
- Apache-2.0 license, public repo: `https://github.com/uditgoenka/tss`.
- familiar commands should exist or route through compatibility handling in v0.1.
- "Covered" means honest behavior, not necessarily optimized output.

## TDD Operating Mode

This plan is `ck-plan --tdd`. Implementation must use vertical red-green-refactor slices:

- Write one behavior test first.
- Watch it fail for the expected reason.
- Write only enough production code to pass.
- Refactor only while green.
- Prefer public-interface contract tests over private implementation tests.
- Do not write all tests first, then all code.

Each phase must start with its first failing test or fixture before production code.

## Cross-Plan Dependencies

Follow-up plan `plans/260607-1912-v0.1.02-rtk-coexistence` handles the v0.1.02
release slice for RTK coexistence, stale-hook conflict detection, and updated
package surfaces.

## Execution Strategy

Parallel-capable after Phase 1. Phase 2 defines architecture. Phases 3-6 can then proceed with clear file ownership. Phase 7 must validate all previous work before Phase 8 launch docs.
The expanded v0.1 scope adds a command parity registry and issue/PR matrix
to Phases 4, 6, 7, and 8.

| Group | Phases | Can Run In Parallel | Notes |
|-------|--------|---------------------|-------|
| Decision lock | 1 | No | Trust contract and setup decisions gate the build. |
| Foundation | 2 | No | Creates CLI/module skeleton and contracts. |
| Build lanes | 3, 4, 5, 6 | Yes after 2 | Disjoint ownership: core, filters, integrations, analytics. |
| Quality gate | 7 | No | Contract tests, fixtures, CI, packaging. |
| Launch | 8 | No | Docs, README, migration guide, issue templates. |

## File Ownership Matrix

| Phase | Owns |
|-------|------|
| 1 | `docs/`, `plans/`, setup-skill docs after user choices |
| 2 | `Cargo.toml`, `src/main.rs`, `src/core/**`, `src/cli/**` |
| 3 | `src/core/filter_engine/**`, `src/core/raw_store/**`, `src/core/shell/**` |
| 4 | `src/filters/**`, `tests/fixtures/**`, `tests/contracts/filter_*` |
| 5 | `src/integrations/**`, `assets/hooks/**`, `tests/contracts/integrations_*` |
| 6 | `src/analytics/**`, `src/privacy/**`, `docs/privacy.md` |
| 7 | `.github/workflows/**`, `scripts/**`, `tests/**`, `benches/**`, issue fixtures |
| 8 | `README.md`, `docs/**`, `LICENSE`, `CONTRIBUTING.md`, `.github/ISSUE_TEMPLATE/**`, compatibility docs |

## Phases

| Phase | Name | Status |
|-------|------|--------|
| 1 | [Define trust contract and product scope](./phase-01-define-trust-contract-and-product-scope.md) | Complete; setup-matt-pocock files deferred pending user confirmation |
| 2 | [Scaffold core CLI architecture](./phase-02-scaffold-core-cli-architecture.md) | Complete |
| 3 | [Implement safe filter engine and raw store](./phase-03-implement-safe-filter-engine-and-raw-store.md) | Complete |
| 4 | [Build MVP command adapters](./phase-04-build-mvp-command-adapters.md) | Complete for v0.1 optimized and passthrough-compatible families |
| 5 | [Install agent integrations](./phase-05-install-agent-integrations.md) | Complete as integration plans/assets; mutating installers remain release follow-up |
| 6 | [Add analytics and privacy controls](./phase-06-add-analytics-and-privacy-controls.md) | Complete |
| 7 | [Create verification harness and release pipeline](./phase-07-create-verification-harness-and-release-pipeline.md) | Build verification complete; release publication workflow remains follow-up |
| 8 | [Write launch documentation](./phase-08-write-launch-documentation.md) | Complete for README, trust, privacy, and compatibility docs |

## Dependencies

- Rust CLI toolchain selected for single-binary distribution and low overhead.
- External command adapters must not add new command semantics beyond safe wrappers.
- Agent integration behavior must follow each host's current hook contract:
  Claude Code, GitHub Copilot, Gemini CLI, OpenCode, Cursor, Codex.
- command parity is implemented through a registry, not ad hoc unsafe rewrites.
- Open issue/PR classes feed fixtures and compatibility reporting before release.

## Validation Gates

- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
- Contract fixture suite against the prior token-saving tool-reported failure classes
- open issue/PR matrix reviewed before v0.1.0
- familiar command registry reports zero unknown mapped commands
- JSON/diff/patch round-trip checks
- Manual smoke test for at least Claude Code and Codex-style instruction mode

## Cook Handoff

Run after review:

```bash
/ck:cook --parallel /Users/uditgoenka/Desktop/workspace/tss/plans/260605-2118-build-tss/plan.md
```
