---
phase: 6
title: "Add analytics and privacy controls"
status: pending
effort: "1.5 days"
---

# Phase 6: Add analytics and privacy controls

## Overview

Add honest savings analytics and privacy controls. TSS must avoid familiar
over-counted savings and argument/secret leakage.

## Context Links

- Depends on: [Phase 3](./phase-03-implement-safe-filter-engine-and-raw-store.md)

## Key Insights

- Savings can be misleading when only part of a pipeline is wrapped.
- Raw output and command args may contain secrets.
- Local analytics are enough for MVP.

## Requirements

- Local-only savings ledger.
- Local-only command compatibility ledger.
- No remote telemetry by default.
- Argument redaction by default.
- Separate counts:
  - raw bytes/tokens observed
  - emitted bytes/tokens
  - omitted bytes/tokens
  - passthrough reason
  - provider-cache caveat
- Config commands:
  - `tss config privacy`
  - `tss gain`
  - `tss gain --json`
  - `tss compat`
  - `tss compat --legacy-cli`

## Architecture

```text
analytics/
  ledger.rs
  token_estimate.rs
  compat.rs
privacy/
  redaction.rs
  permissions.rs
```

Use simple local storage first. SQLite is acceptable if the implementation needs queries; JSONL is acceptable for MVP if file locking is robust enough.

## Related Code Files

- Create: `/Users/uditgoenka/Desktop/workspace/tss/src/analytics/*.rs`
- Create: `/Users/uditgoenka/Desktop/workspace/tss/src/privacy/*.rs`
- Create: `/Users/uditgoenka/Desktop/workspace/tss/docs/privacy.md`
- Create: `/Users/uditgoenka/Desktop/workspace/tss/tests/contracts/privacy.rs`

## Implementation Steps

1. RED: add privacy test proving full args are not stored by default.
2. GREEN: define ledger schema with redacted command preview.
3. RED: add behavior test for command category, filter name, safety decision, byte counts, token estimate.
4. GREEN: record analytics fields.
5. RED: add retention config test.
6. GREEN: add raw-store retention config.
7. RED: add env-toggle tests for `TSS_NO_STORE=1` and `TSS_NO_ANALYTICS=1`.
8. GREEN: implement env toggles.
9. RED: add CLI behavior test for `tss gain`.
10. GREEN: implement `tss gain` summary.
11. RED: add test for `tss gain --failures`.
12. GREEN: report passthrough/larger-output failures.
13. RED: add compatibility report test for known command states.
14. GREEN: implement `tss compat --legacy-cli`.
15. Write privacy doc after behavior is green.

## Todo List

- [ ] Add ledger.
- [ ] Add redaction.
- [ ] Add config/env toggles.
- [ ] Add gain reports.
- [ ] Add command compatibility reports.
- [ ] Add privacy tests.

## Success Criteria

- [ ] No full command args stored unless user opts in.
- [ ] Raw output can be disabled.
- [ ] Savings report distinguishes compression from truncation/omission.
- [ ] Token estimates are labeled estimates, not exact billing claims.
- [ ] `tss compat --legacy-cli` reports optimized/passthrough/planned/blocked/needs-research counts.

## Risk Assessment

- Risk: users distrust any local storage.
- Mitigation: visible config, doctor output, retention controls, no remote telemetry.

## Security Considerations

- Local files use restrictive permissions.
- Secrets are never sent remotely.
- Analytics do not include cwd unless opted in.

## Next Steps

Phase 7 proves the product with contracts and CI.
