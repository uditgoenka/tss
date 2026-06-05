---
phase: 8
title: "Write launch documentation"
status: pending
effort: "1 day"
---

# Phase 8: Write launch documentation

## Overview

Write launch docs that sell the correct promise: lower token usage with visible,
recoverable, contract-tested compression.

## Context Links

- Depends on: Phase 7

## Key Insights

- Do not claim "60-90%" globally.
- Explain when TSS passes through and why.
- Make migration from the prior token-saving tool simple but honest.

## Requirements

- README.
- Installation docs.
- Trust contract docs.
- Integration docs.
- Filter behavior matrix.
- command compatibility matrix.
- issue/PR matrix.
- Troubleshooting docs.
- Contributing docs.
- Issue templates.

## Architecture

Docs structure:

```text
docs/
  trust-contract.md
  mvp-scope.md
  integrations/
  filters/
  command-compatibility.md
  legacy-cli-issue-matrix.md
  privacy.md
  troubleshooting.md
  benchmark-methodology.md
```

## Related Code Files

- Create/modify: `/Users/uditgoenka/Desktop/workspace/tss/README.md`
- Create: `/Users/uditgoenka/Desktop/workspace/tss/CONTRIBUTING.md`
- Create: `/Users/uditgoenka/Desktop/workspace/tss/docs/**`
- Create: `/Users/uditgoenka/Desktop/workspace/tss/.github/ISSUE_TEMPLATE/**`

## Implementation Steps

1. Write README with:
   - product promise
   - trust contract summary
   - install
   - quick start
   - raw recovery
   - agent integrations
   - supported commands
2. Write the prior token-saving tool migration guide:
   - what TSS covers
   - what TSS intentionally skips
   - how to handle unsupported commands
3. Write filter behavior matrix.
4. Write command compatibility matrix.
5. Write issue/PR matrix:
   - optimized
   - passthrough-compatible
   - planned
   - blocked by trust contract
   - needs research
6. Write benchmark methodology:
   - fixtures
   - tokenizer assumptions
   - provider-cache caveat
   - correctness gates
7. Write contributing rules:
   - new filter requires fixture
   - lossy output must mark omissions
   - no silent project-local filter trust
8. Add issue templates:
   - silent loss
   - bad rewrite
   - integration drift
   - filter request
   - security report pointer

## Todo List

- [ ] Write README.
- [ ] Write installation docs.
- [ ] Write migration guide.
- [ ] Write filter matrix.
- [ ] Write command compatibility matrix.
- [ ] Write issue/PR matrix.
- [ ] Write benchmark methodology.
- [ ] Add issue templates.

## Success Criteria

- [ ] A user understands exactly when TSS saves tokens and when it refuses.
- [ ] Contributors know how to add a filter safely.
- [ ] Public claims match verified fixtures.

## Risk Assessment

- Risk: marketing pressure overstates savings.
- Mitigation: publish fixture methodology and exact command matrix.

## Security Considerations

- Security policy directs private disclosure.
- Docs warn raw store may contain secrets and show retention controls.

## Next Steps

After docs, ship v0.1.0-alpha and collect fixture-backed bug reports.
