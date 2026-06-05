---
phase: 2
title: "Scaffold core CLI architecture"
status: pending
effort: "1.5 days"
---

# Phase 2: Scaffold core CLI architecture

## Overview

Create the initial Rust CLI architecture without implementing broad filters.
The goal is stable interfaces: command execution, safety classification, filter trait,
raw-output store, and output envelope.

## Context Links

- Depends on: [Phase 1](./phase-01-define-trust-contract-and-product-scope.md)

## Key Insights

- Keep one Rust crate initially. A workspace is premature until integrations grow.
- Use modules and traits, not a plugin framework.
- Make passthrough the default, not an exception.

## Requirements

- Apache-2.0 project metadata.
- CLI command skeleton:
  - `tss run -- <cmd>`
  - `tss raw <id>`
  - `tss doctor`
  - `tss gain`
  - `tss init <agent>`
  - `tss verify`
- Clear module boundaries.
- No remote network calls.

## Architecture

```text
src/
  main.rs
  cli/
  core/
    command.rs
    runner.rs
    shell.rs
    filter.rs
    envelope.rs
    raw_store.rs
    policy.rs
  filters/
  integrations/
  analytics/
  privacy/
```

Core contracts:

```rust
trait OutputFilter {
    fn name(&self) -> &'static str;
    fn supports(&self, cmd: &CommandSpec) -> Support;
    fn filter(&self, raw: RawOutput, ctx: FilterContext) -> FilterResult;
}

enum Support {
    Exact,
    UnsafeReason(&'static str),
    Unsupported,
}
```

## Related Code Files

- Create: `/Users/uditgoenka/Desktop/workspace/tss/Cargo.toml`
- Create: `/Users/uditgoenka/Desktop/workspace/tss/src/main.rs`
- Create: `/Users/uditgoenka/Desktop/workspace/tss/src/cli/mod.rs`
- Create: `/Users/uditgoenka/Desktop/workspace/tss/src/core/*.rs`
- Create: `/Users/uditgoenka/Desktop/workspace/tss/LICENSE`

## Implementation Steps

1. RED: add CLI smoke test for `tss run -- echo hello` preserving stdout and exit code.
2. GREEN: initialize Rust binary crate metadata for `tss`.
3. RED: add test for Apache-2.0 package metadata/license presence.
4. GREEN: add Apache-2.0 license.
5. RED: add public-interface tests for CLI subcommand parsing.
6. GREEN: create CLI subcommands and help text.
7. RED: add tests for `CommandSpec`, `RawOutput`, `FilterResult`, and `OutputEnvelope` behavior.
8. GREEN: implement core types.
9. RED: add runner test for stdout, stderr, combined stream, duration, and exit code.
10. GREEN: implement passthrough runner.
11. RED: add envelope/footer behavior test.
12. GREEN: implement output footer format:
   - filter name
   - raw id
   - bytes before/after
   - omissions count
   - safety status
13. REFACTOR: remove duplication only after all tests pass.

## Todo List

- [ ] Create Rust crate.
- [ ] Add CLI skeleton.
- [ ] Add core types.
- [ ] Add passthrough runner.
- [ ] Add unit tests.

## Success Criteria

- [ ] `tss run -- echo hello` preserves output and exit code.
- [ ] `tss raw <id>` returns stored raw output for any filtered run.
- [ ] CLI has no broad filter behavior yet.

## Risk Assessment

- Risk: abstracting too early.
- Mitigation: one crate, simple modules, traits only where filters need a stable boundary.

## Security Considerations

- Raw store path under user cache/data dir.
- File mode `0600` for raw artifacts and index.
- No project-local executable hook installed by default.

## Next Steps

Phase 3 builds filtering and raw recovery on this foundation.
