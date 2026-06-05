---
phase: 1
title: "Define trust contract and product scope"
status: docs-complete-setup-deferred
effort: "1 day"
---

# Phase 1: Define trust contract and product scope

## Overview

Lock the product contract before code. This phase prevents TSS from becoming
"the prior token-saving tool but broader." The MVP must optimize only where correctness remains auditable.

## Context Links

- Plan: [Build TSS](./plan.md)
- Research: [Research Summary](./reports/research-summary.md)
- Red-team: [Red-Team Review](./reports/red-team-review.md)

## Key Insights

- the prior token-saving tool has many bugs because command output filters drift from real command semantics.
- The market gap is trust, not raw command count.
- The first build must deliberately skip unsupported command shapes.

## Requirements

- Define TSS trust contract as a versioned document.
- Define MVP command surface and explicit non-goals.
- Record Apache-2.0 licensing requirement.
- Complete `$setup-matt-pocock-skills` setup after required user choices.

## Architecture

Trust contract becomes the root policy consumed by filters, integrations, tests, docs.

```text
command -> classify -> safety gate -> execute -> filter -> validate -> emit
                         |                            |
                         v                            v
                    passthrough                 raw recovery
```

## Related Code Files

- Create: `/Users/uditgoenka/Desktop/workspace/tss/docs/trust-contract.md`
- Create: `/Users/uditgoenka/Desktop/workspace/tss/docs/mvp-scope.md`
- Create after setup confirmation: `/Users/uditgoenka/Desktop/workspace/tss/AGENTS.md` or `CLAUDE.md`
- Create after setup confirmation: `/Users/uditgoenka/Desktop/workspace/tss/docs/agents/issue-tracker.md`
- Create after setup confirmation: `/Users/uditgoenka/Desktop/workspace/tss/docs/agents/triage-labels.md`
- Create after setup confirmation: `/Users/uditgoenka/Desktop/workspace/tss/docs/agents/domain.md`

## Implementation Steps

1. Write `docs/trust-contract.md`.
2. Include invariants:
   - Preserve process exit code.
   - Preserve failure signal on non-zero exit.
   - Never print fake success.
   - Never silently truncate.
   - Keep requested structured output parseable or pass through.
   - Always attach raw recovery handle when lossy.
3. Write `docs/mvp-scope.md`.
4. Define MVP commands:
   - `git status`, `git diff`, `git log`, `git branch`
   - `rg`/`grep`, `ls`, `find`, `cat`/`head`/`tail`
   - `npm`/`pnpm` scripts, `vitest`, `jest`, `tsc`, `next`
   - `go test`, `cargo test/check`, `pytest`
5. Define non-goals:
   - No cloud telemetry.
   - No project-local filters auto-trusted.
   - No compression for pipelines/redirects by default.
   - No broad AWS/kubectl/docker coverage until core is proven.
6. Finish `$setup-matt-pocock-skills` once user confirms:
   - issue tracker
   - triage labels
   - domain-doc layout
7. Commit docs before code work starts.

## Todo List

- [x] Write trust contract.
- [x] Write MVP scope doc.
- [x] Record non-goals.
- [ ] Complete setup-matt-pocock-skills repo config. Deferred because the user explicitly instructed not to create `AGENTS.md`, `CLAUDE.md`, or `docs/agents/*` until setup choices are confirmed.
- [x] Review contract against issue failure classes.

## Success Criteria

- [x] Team can reject any future filter by checking it against trust contract.
- [x] MVP scope is small enough to ship and broad enough to prove value.
- [x] Setup-skill docs exist or are explicitly deferred with user confirmation.

## Risk Assessment

- Risk: over-scoping to match the prior token-saving tool.
- Mitigation: only add filters when contract tests exist.

## Security Considerations

- Project-local config must never be trusted silently.
- Raw-output store may contain secrets; secure permissions required.

## Next Steps

Proceed to Phase 2 only after trust contract is accepted.
