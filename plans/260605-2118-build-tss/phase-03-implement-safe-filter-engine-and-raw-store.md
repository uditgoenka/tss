---
phase: 3
title: "Implement safe filter engine and raw store"
status: pending
effort: "2 days"
---

# Phase 3: Implement safe filter engine and raw store

## Overview

Implement the safety gate, filter engine, and raw-output store. This is the
heart of TSS. Filters must prove they did not violate the trust contract.

## Context Links

- Depends on: [Phase 2](./phase-02-scaffold-core-cli-architecture.md)

## Key Insights

- Lossy compression is acceptable only when disclosed.
- Parse errors should fail open to raw output.
- Raw recovery must be first-class, not a debugging side file.

## Requirements

- Conservative shell classifier.
- Raw-output storage with retrieval by id.
- Filter validation hooks.
- Omission accounting.
- Passthrough on unsafe command shapes.

## Architecture

```text
SafetyGate
  -> command shape check
  -> requested output mode check
  -> filter support check
  -> raw storage
  -> filter
  -> contract validator
  -> envelope emit or passthrough
```

Unsafe by default:

- pipes
- redirections
- heredocs
- command substitution
- background jobs
- loops
- `xargs`
- unknown value-taking flags
- structured-output flags unless filter has exact support

## Related Code Files

- Create: `/Users/uditgoenka/Desktop/workspace/tss/src/core/shell.rs`
- Create: `/Users/uditgoenka/Desktop/workspace/tss/src/core/policy.rs`
- Create: `/Users/uditgoenka/Desktop/workspace/tss/src/core/filter_engine.rs`
- Create: `/Users/uditgoenka/Desktop/workspace/tss/src/core/raw_store.rs`
- Create: `/Users/uditgoenka/Desktop/workspace/tss/tests/contracts/safety_gate.rs`

## Implementation Steps

1. RED: add contract test proving complex shell syntax passes through.
2. GREEN: implement shell classifier.
3. RED: add public test for `SafetyDecision` outcomes.
4. GREEN: add `SafetyDecision`:
   - `FilterAllowed`
   - `PassthroughUnsafe(reason)`
   - `PassthroughUnsupported`
   - `Deny(reason)` only for explicit destructive guard mode.
5. RED: add raw-store retrieval test using a real command output fixture.
6. GREEN: store raw stdout, stderr, combined output, exit code, cwd, command hash, timestamp.
7. RED: add test proving raw ids do not leak command text.
8. GREEN: generate raw ids independent of command text.
9. RED: add CLI test for `tss raw <id>` modes.
10. GREEN: add `tss raw <id>` with raw/full/stdout/stderr modes.
11. RED: add contract validator tests:
   - exit code unchanged
   - non-zero exit cannot emit success-only output
   - output marked lossy if bytes removed
   - structured output parse check when required
12. GREEN: implement validators.
13. RED: add golden tests for familiar failures:
   - fake success on compiler failure
   - silent truncation
   - invalid JSON
   - diff/patch corruption
   - swallowed test output
14. GREEN: wire filter engine to pass those tests.
15. REFACTOR: simplify policy names and shared helpers while green.

## Todo List

- [ ] Implement safety gate.
- [ ] Implement raw store.
- [ ] Implement filter validator.
- [ ] Add unsafe command passthrough tests.
- [ ] Add raw retrieval tests.

## Success Criteria

- [ ] Unsafe shell shapes pass through raw.
- [ ] Every lossy filter prints a raw id.
- [ ] Non-zero exits never become fake success.
- [ ] Structured output is either valid or unfiltered.

## Risk Assessment

- Risk: conservative passthrough reduces headline savings.
- Mitigation: market TSS around correctness; optimize coverage after trust.

## Security Considerations

- Raw output can contain secrets.
- Never store raw output when `TSS_NO_STORE=1`.
- Redact command args in analytics by default.

## Next Steps

Phase 4 adds high-value filters on top of the engine.
