---
title: "TSS Development Rules"
status: "draft"
last_updated: 2026-06-05
license: "Apache-2.0"
---

# TSS Development Rules

These rules turn the Phase 1 trust contract into a TDD-ready implementation
foundation for later phases.

## Development Contract

- Correctness comes before token savings.
- Passthrough is an acceptable result for unsupported or risky commands.
- No adapter may merge without a fixture and a public-interface contract test.
- No lossy output may ship without omission markers and raw recovery behavior.
- No structured output may be filtered unless parseability is preserved.
- No project-local filter or config may be trusted silently.
- All contributions must remain compatible with Apache-2.0 distribution.

## TDD Loop

Use vertical red-green-refactor slices.

1. Pick one observable behavior from the trust contract or MVP scope.
2. Add one failing test or fixture that proves that behavior through the TSS
   public interface.
3. Run the focused test and confirm it fails for the expected reason.
4. Implement only enough production code to pass that test.
5. Run the focused test and confirm it passes.
6. Refactor only while green.
7. Run the focused test again, then the relevant wider suite.

Do not write a bulk set of imagined tests before implementation. Each test should
respond to behavior learned in the previous slice.

## Test Shape

Prefer tests that execute the public CLI or public library boundary that backs
the CLI.

Good test names describe behavior:

```text
passes_through_unknown_commands_without_filtering
preserves_exit_code_for_failed_cargo_test
emits_raw_handle_when_git_diff_is_lossy
keeps_json_output_parseable_or_passes_through
```

Avoid tests that assert private parser internals, helper function names, or exact
implementation structure unless the helper is itself a public contract.

## Required Fixture Classes

Every command adapter should start with fixtures before production filtering.

| Fixture Class | Purpose |
|---------------|---------|
| Success | Proves normal output remains truthful. |
| Failure | Proves failures stay visible and exit codes are preserved. |
| Noisy output | Proves token-heavy output can be reduced safely. |
| Structured output | Proves parseability or passthrough for machine-readable modes. |
| Unsupported shape | Proves passthrough for flags, syntax, or modes outside the adapter contract. |

Not every command has every class, but omissions must be deliberate and recorded
in the test or phase notes.

## Adapter Merge Gate

Before enabling a filter by default, verify:

- Command shape is listed in `docs/mvp-scope.md`.
- Behavior complies with `docs/trust-contract.md`.
- Contract tests cover success and failure behavior where applicable.
- Lossy behavior has an omission marker assertion.
- Lossy behavior has a raw recovery assertion.
- Structured output has a parser check or passthrough assertion.
- Unsupported flags and complex shell forms pass through.
- Test fixtures do not require non-Apache-compatible licensing.

## Verification Commands

When the Rust project exists, the default verification gate is:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

Phase 7 will add the full verification harness. Until then, each phase should run
the narrowest focused test that proves its slice plus any available project-wide
checks.

## Documentation Updates

When implementation changes the product contract, update docs in this order:

1. `docs/trust-contract.md` for invariant or safety-policy changes.
2. `docs/mvp-scope.md` for command surface or non-goal changes.
3. `docs/development-rules.md` for test or merge-gate changes.
4. The active plan or journal only when plan status changes.

Contract changes should be rare. If a filter needs a contract exception, prefer
passthrough until the exception is reviewed.
