---
phase: 7
title: "Create verification harness and release pipeline"
status: pending
effort: "2 days"
---

# Phase 7: Create verification harness and release pipeline

## Overview

Create the verification harness that makes the trust claim credible. This phase
is mandatory before public release.

## Context Links

- Depends on: phases 2-6

## Key Insights

- A token saver without adversarial tests will regress silently.
- Tests must use raw fixtures, not only happy-path handcrafted strings.
- Release artifacts must be reproducible enough to avoid malware false-positive panic.

## Requirements

- Unit tests.
- Golden fixture tests.
- Property-style invariants where practical.
- Integration contract tests.
- CI for macOS, Linux, Windows.
- Release checks and checksums.

## Architecture

```text
tests/
  fixtures/
  contracts/
  integration/
scripts/
  fixture-capture.sh
  verify-release.sh
.github/workflows/
  ci.yml
  release.yml
```

Contract classes:

- exit code preservation
- non-zero failure preservation
- raw recovery
- omission marker presence
- structured output validity
- patch/diff applicability
- hook JSON validity
- analytics privacy
- issue/PR class matrix
- familiar command registry completeness

## Related Code Files

- Create: `/Users/uditgoenka/Desktop/workspace/tss/tests/contracts/*.rs`
- Create: `/Users/uditgoenka/Desktop/workspace/tss/tests/fixtures/**`
- Create: `/Users/uditgoenka/Desktop/workspace/tss/.github/workflows/ci.yml`
- Create: `/Users/uditgoenka/Desktop/workspace/tss/.github/workflows/release.yml`
- Create: `/Users/uditgoenka/Desktop/workspace/tss/scripts/*.sh`

## Implementation Steps

1. RED: add a failing fixture-harness smoke test.
2. GREEN: implement minimal fixture harness.
3. Add CI workflow:
   - fmt
   - clippy
   - test
   - audit if available
   - package smoke test
4. RED/GREEN vertical loop for fixture suite from issue classes:
   - Next.js compile error
   - Vitest config error
   - Vitest many failures
   - TypeScript ANSI pretty errors
   - Go compiler/vet errors
   - git diff patch
   - grep incompatible flags
   - JSON response
   - brew/npm/package-manager install noise
   - cloud/container structured output passthrough
   - Windows/path command lookup
   - integration hook drift
5. Add cross-platform smoke tests.
6. Add release workflow:
   - build binaries
   - generate checksums
   - attach SBOM if practical
   - publish GitHub release draft
7. RED: add CLI test for `tss verify fixtures`.
8. GREEN: add `tss verify fixtures`.
9. Require all fixtures pass before adding a new filter.
10. Require issue/PR matrix review before v0.1.0 tag.

## Todo List

- [ ] Add CI workflow.
- [ ] Add release workflow.
- [ ] Add fixture harness.
- [ ] Add trust contract tests.
- [ ] Add integration JSON schema tests.
- [ ] Add platform smoke tests.
- [ ] Add issue/PR fixtures.
- [ ] Add command registry completeness tests.

## Success Criteria

- [ ] CI proves trust contract on every PR.
- [ ] No filter can merge without raw fixture coverage.
- [ ] Release artifacts include checksums.
- [ ] v0.1.0 has no unclassified the prior token-saving tool high-priority issue class.

## Risk Assessment

- Risk: fixtures become stale or too narrow.
- Mitigation: every bug report gets a fixture before a fix.

## Security Considerations

- Avoid `curl | sh` as primary install path.
- If install script exists, verify checksum/signature.
- Release workflow cannot embed secrets in binaries.

## Next Steps

Phase 8 prepares launch docs and contributor process.
