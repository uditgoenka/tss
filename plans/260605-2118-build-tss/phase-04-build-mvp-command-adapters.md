---
phase: 4
title: "Build v0.1 command adapters and the prior token-saving tool compatibility registry"
status: pending
effort: "3 days"
---

# Phase 4: Build v0.1 command adapters and the prior token-saving tool compatibility registry

## Overview

Build the v0.1 filters that directly address the most painful issue classes:
Next.js, Vitest, TypeScript, Git, grep/rg, filesystem discovery, core test runners,
package managers, and familiar compatibility commands.

## Context Links

- Depends on: [Phase 3](./phase-03-implement-safe-filter-engine-and-raw-store.md)
- Udit-reported issues: #2013, #1836, #1820, #1813

## Key Insights

- Filter count is less important than correctness under edge flags.
- Test/build failures need more detail, not less.
- For patches/diffs, output must remain programmatically useful.

## Requirements

- Each filter has:
  - explicit command matcher
  - flag allowlist/denylist
  - fixture tests
  - contract tests
  - raw fallback path
- First command groups:
  - Git
  - Files/search
  - JS/TS build/test
  - Package managers: `brew`, `npm`, `pnpm`, `yarn`, `bun`, `pip`, `uv`, `poetry`, `bundle`
  - Dev/build tools from the prior token-saving tool registry
  - Go/Rust/Python tests
- Every known command family must have a v0.1 registry status:
  optimized, passthrough-compatible, planned, blocked-by-trust-contract, needs-research.

## Architecture

```text
filters/
  git.rs
  files.rs
  search.rs
  js.rs
  packages.rs
  registry.rs
  go.rs
  rust.rs
  python.rs
```

Filter output policy:

- Success output: compact aggressively.
- Failure output: preserve diagnostic blocks.
- Large lists: group and mark omitted counts.
- User-requested exact modes: pass through.

## Related Code Files

- Create: `/Users/uditgoenka/Desktop/workspace/tss/src/filters/git.rs`
- Create: `/Users/uditgoenka/Desktop/workspace/tss/src/filters/files.rs`
- Create: `/Users/uditgoenka/Desktop/workspace/tss/src/filters/search.rs`
- Create: `/Users/uditgoenka/Desktop/workspace/tss/src/filters/js.rs`
- Create: `/Users/uditgoenka/Desktop/workspace/tss/src/filters/packages.rs`
- Create: `/Users/uditgoenka/Desktop/workspace/tss/src/filters/registry.rs`
- Create: `/Users/uditgoenka/Desktop/workspace/tss/src/filters/go.rs`
- Create: `/Users/uditgoenka/Desktop/workspace/tss/src/filters/rust.rs`
- Create: `/Users/uditgoenka/Desktop/workspace/tss/src/filters/python.rs`
- Create: `/Users/uditgoenka/Desktop/workspace/tss/tests/fixtures/**`

## Implementation Steps

1. RED/GREEN vertical loop for Git filters:
   - `status`: preserve branch, merge/rebase state, staged/unstaged/untracked.
   - `diff/show`: compact only when not patch-consumer mode; preserve `--name-only`, `--stat`, `-p`.
   - `log`: do not hide merges unless explicitly requested.
   - `branch`: preserve remote/tracking/hash metadata when flags request it.
2. RED/GREEN vertical loop for files/search:
   - `ls`: preserve hierarchy and key metadata; no fake empty dirs.
   - `find`: pass through complex predicates/actions.
   - `rg/grep`: do not translate incompatible flags blindly.
   - `cat/head/tail`: safe read only; multi-file banners preserved.
3. RED/GREEN vertical loop for JS/TS:
   - `vitest/jest`: preserve every failing test name and first actionable stack frame; no parser-error swallow.
   - `tsc`: strip ANSI, preserve file/line/error code/message.
   - `next`: preserve compile error file, line, message, route, port/dev-server URL.
   - `npm/pnpm/yarn run`: never discard package script body or workspace flags.
4. RED/GREEN vertical loop for package managers:
   - `brew install/upgrade`: strip progress only; preserve already-installed, failure, formula name.
   - `npm/pnpm/yarn/bun install`: preserve lockfile/workspace/failure semantics.
   - `pip/uv/poetry/bundle`: preserve install result, dependency conflict, and command failure.
   - Pass through unrecognized lifecycle/script modes.
5. RED/GREEN vertical loop for Go/Rust/Python:
   - `go test`: preserve compiler/vet errors and coverage lines.
   - `cargo test/check`: preserve first error per file plus summary.
   - `pytest`: preserve failures, tracebacks, collection errors.
6. RED/GREEN vertical loop for compatibility registry:
   - Register every known family from docs/research.
   - Return honest status for each command.
   - `unknown` is a test failure for known command families.
7. For each filter, create one failing fixture test before adding production logic.
8. Refactor shared formatter helpers only after the domain's fixture tests are green.

## Todo List

- [ ] Add Git filters and fixtures.
- [ ] Add files/search filters and fixtures.
- [ ] Add JS/TS filters and fixtures.
- [ ] Add package-manager filters and fixtures.
- [ ] Add Go/Rust/Python test filters and fixtures.
- [ ] Add familiar command registry.
- [ ] Add contract tests for requested raw/structured flags.

## Success Criteria

- [ ] TSS handles all four Udit-reported issues by design.
- [ ] Any unsupported flag path passes through, not partially rewritten.
- [ ] Every known command family is classified in v0.1.
- [ ] Package-manager commands preserve install/script/workspace semantics.
- [ ] Fixture suite catches silent loss, fake success, invalid JSON, and diff corruption.

## Risk Assessment

- Risk: command-specific parsing grows messy.
- Mitigation: one filter per domain, shared helpers, no generic regex mutation of args.

## Security Considerations

- Do not redact user-visible failure output by default.
- Redaction applies only to analytics/logs, not command output, unless user enables it.

## Next Steps

Phase 5 wires filters into agents.
