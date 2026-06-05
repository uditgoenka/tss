---
created: 2026-06-05
plan: plans/260605-2118-build-tss
status: v0.1-build-baseline-verified
---

# Build TSS Plan Journal

## Context

Created implementation plan for TSS, an Apache-2.0 trust-first alternative to the prior token-saving tool.

## What Happened

- Scanned existing plan state: none found.
- Researched issue surface and competitor landscape.
- Created parallel-mode plan with 8 phases.
- Updated plan to `ck-plan --tdd` mode with red-green-refactor gates.
- Expanded v0.1 scope to familiar command support and open issue/PR class handling.
- Included setup-matt-pocock-skills findings but did not write setup files because that skill requires user confirmation.
- Implemented Phase 1 documentation foundation:
  - `docs/trust-contract.md`
  - `docs/mvp-scope.md`
  - `docs/development-rules.md`
- Deferred `AGENTS.md`, `CLAUDE.md`, and `docs/agents/*` because the user explicitly instructed not to create setup-matt-pocock-skills outputs without confirmation.
- Implemented the v0.1 Rust CLI baseline:
  - familiar direct aliases: `tss <cmd>`, `tss -- <cmd>`, and `tss proxy <cmd>`.
  - Raw output storage and `tss raw` recovery modes.
  - `tss doctor`, `tss compat`, `tss gain`, and `tss --version`.
  - Fixture-backed filters for Git, files, search, JS/TS diagnostics, Go, Rust, and Python tests.
  - Passthrough-compatible package-manager vocabulary including `npm`, `pnpm`, `yarn`, `npx`, `pnpx`, `bun`, `deno`, `corepack`, `brew`, `pip`, `uv`, `poetry`, and related wrappers.
  - Local privacy and analytics ledger controls.
  - Agent integration plans/assets for Claude, Copilot, Gemini, OpenCode, Cursor, and Codex instruction/wrapper mode.
- Added npm and Homebrew distribution scaffolding:
  - `package.json`
  - `npm/bin/tss`
  - `npm/postinstall.js`
  - `packaging/homebrew/tss.rb.template`
  - `README.md`
- Verified with `cargo fmt --check`, `cargo test`, `cargo clippy --all-targets -- -D warnings`, Node syntax checks, npm package dry-run, and Ruby syntax check for the Homebrew template.

## Decisions

- TSS differentiates on correctness and recoverability, not command-count breadth.
- Phase 1 gates implementation with trust contract and MVP scope.
- Filters require fixtures and contract tests before merge.
- Passthrough is documented as a successful safety outcome, not a product failure.
- Codex integration is documented as instruction/wrapper mode, not fake hook parity.

## Next

Prepare the first release build. Publication follow-ups:

- Create GitHub release assets matching npm and Homebrew expectations.
- Publish the npm package after release assets are attached.
- Publish or update the Homebrew tap formula with the final tarball checksum.
- Confirm setup-matt-pocock-skills choices before creating `AGENTS.md`,
  `CLAUDE.md`, or `docs/agents/*`.
