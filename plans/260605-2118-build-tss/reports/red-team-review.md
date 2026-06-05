# Red-Team Review

## Verdict

Plan is viable if Phase 1 is enforced. If implementation jumps straight to many
filters, TSS will repeat the legacy tool's failure pattern.

## Critical Risks

| Risk | Severity | Mitigation |
|------|----------|------------|
| Too many filters before trust harness | High | No filter merges without fixture + contract tests |
| Shell parsing complexity | High | Conservative passthrough for complex syntax |
| Agent integration drift | High | Per-agent contract tests and `tss doctor` blind-spot reporting |
| Raw store secrets | High | `0600`, retention controls, `TSS_NO_STORE=1` |
| Marketing overclaims | Medium | Publish exact fixture matrix, not generic savings claims |
| Rust project over-abstraction | Medium | One crate first, modules over plugin framework |

## Required Plan Adjustments

- Keep Phase 1 as a hard gate before code.
- Make passthrough a success case, not a failure.
- Start with issue fixtures, especially Udit-reported issues.
- Treat Codex differently: instruction/wrapper mode, not fake hook parity.
- Do not auto-trust project-local filters in v0.1.
- Enforce TDD vertically: one failing behavior test, minimal implementation, then refactor.

## Accepted Trade-Off

TSS v0.1 will save fewer tokens than the prior token-saving tool on some workloads. That is acceptable.
The product promise is "correct first, compact second."
