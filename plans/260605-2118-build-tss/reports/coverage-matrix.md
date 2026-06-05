# the prior token-saving tool Issue/PR Coverage Matrix for TSS v0.1

Generated: 2026-06-05

## Inputs

- `/tmp/legacy-cli-open-issues.json`: 595 open issues.
- `/tmp/legacy-cli-open-prs.json`: 510 open the prior token-saving tool PRs.
- TSS scope and plan docs:
  - `docs/trust-contract.md`
  - `docs/mvp-scope.md`
  - `docs/privacy.md`
  - `docs/development-rules.md`
  - `plans/260605-2118-build-tss/plan.md`
  - phase docs 01-08
  - existing research/red-team reports

This matrix classifies v0.1 scope coverage, not implementation completion. The current
TSS repo has a CLI skeleton, while the plan still marks most build phases pending.

## Coverage Definitions

| Classification | Meaning |
|---|---|
| optimized | TSS v0.1 should actively reduce output for this class with fixture-backed filters and trust-contract validation. |
| passthrough-compatible | TSS v0.1 should address the class by preserving raw behavior and declining risky optimization. |
| planned | TSS v0.1 docs/phase plan include the feature, but it still needs implementation and verification. |
| blocked-by-trust-contract | TSS should deliberately reject this familiar behavior for v0.1 because it conflicts with the trust contract. |
| needs-research | The class is too broad, host-specific, stale, or under-specified from titles/labels alone. |

## Source Snapshot

Exact label counts from the issue export show the open the prior token-saving tool surface is mostly CLI and
filter correctness work:

| Signal | Count |
|---|---:|
| `area:cli` issues | 460 |
| `bug` issues | 276 |
| `enhancement` issues | 292 |
| `priority:high` issues | 132 |
| `filter-quality` issues | 109 |
| `area:config` issues | 44 |
| `area:docs` issues | 32 |
| `area:ci` issues | 25 |
| `area:security` issues | 16 |
| platform issues: macOS / Windows / Linux | 66 / 46 / 28 |
| PR labels: `enhancement` / `bug` / `filter-quality` | 56 / 36 / 22 |
| PR labels: `wrong-base` / `awaiting-changes` | 22 / 13 |

Dominant title-class counts below are heuristic and mutually exclusive. They are
intended for planning, not as a replacement for per-ticket triage.

| Dominant the prior token-saving tool class | Issues | PRs | Total |
|---|---:|---:|---:|
| Agent integrations and install drift | 116 | 90 | 206 |
| Misc / needs-info / cross-cutting | 74 | 49 | 123 |
| Non-MVP command ecosystem expansion | 58 | 47 | 105 |
| Analytics, discovery, savings claims | 46 | 50 | 96 |
| Platform, install, and release packaging | 45 | 31 | 76 |
| Git and VCS fidelity | 39 | 34 | 73 |
| JS/TS package, build, and test fidelity | 39 | 26 | 65 |
| Config, customization, and policy controls | 30 | 26 | 56 |
| Go/Rust/Python runner fidelity | 27 | 22 | 49 |
| Filesystem/read/list fidelity | 22 | 25 | 47 |
| Search output fidelity | 23 | 21 | 44 |
| Structured/API output preservation | 21 | 21 | 42 |
| Security/privacy/trust hardening | 19 | 12 | 31 |
| Rewrite/proxy/shell-shape safety | 18 | 12 | 30 |
| Docs/onboarding/support | 4 | 24 | 28 |
| CI/tests/contributor workflow | 3 | 11 | 14 |
| Output polish/compression quality | 5 | 9 | 14 |
| API/library/plugin architecture | 6 | 0 | 6 |

## Coverage Matrix

| issue/PR class | the prior token-saving tool examples | TSS v0.1 classification | TSS v0.1 coverage | Migration implication |
|---|---|---|---|---|
| Silent loss, fake success, misleading summaries | #2281, #2203, #2204, #2271, #2230, #2013 | optimized | Trust contract requires exact exit-code preservation, non-zero failure visibility, omission markers, raw recovery handles, and validator tests for fake success/silent truncation. | users should expect fewer aggressive summaries but a safer failure signal. |
| Structured output, JSON/YAML/XML, diff/patch validity | #2139, #2140, #1981, #2275, #863 | optimized | Structured output must be parser-checked or passed through. Diff/patch output must remain applicable or raw. | Machine consumers should prefer TSS for supported structured modes; unsupported structured flags should remain raw. |
| Git and VCS fidelity | #2275, #2211, #2148, #2137, #2021, PRs #2276/#2272/#2164 | optimized | MVP includes `git status`, `git branch`, `git log`, and `git diff`, with requested patch/stat/ref semantics preserved. | High-value the prior token-saving tool git cases are in scope, but exact or unsupported flag combinations should pass through. |
| Search output fidelity | #2253, #2167, #2064, #2060, #2120, PRs #2254/#2183/#2174 | optimized | MVP includes `rg` and `grep`, with dialect-aware flag handling, filenames, line numbers, counts, and no-match semantics preserved. | Users get safe search compaction only for supported shapes; incompatible flag translation should not happen. |
| Filesystem/read/list fidelity | #2271, #2058, #2102, #714, #664, PRs #2265/#2160/#2014 | optimized | MVP includes `ls`, `find`, `cat`, `head`, and `tail`, with hierarchy/path identity, multi-file banners, and explicit omission counts. | TSS should avoid familiar fake empty listings and unsafe find action rewrites. |
| JS/TS package/build/test fidelity | #2233, #2154, #2098, #2094, #2023, #2010, #2013, PRs #2232/#2223/#1951 | optimized | MVP includes `npm`/`pnpm` scripts, `vitest`, `jest`, `tsc`, and `next`, preserving script body, workspace flags, compile errors, ports, and failing test names. | This is the biggest direct migration win for agent coding workflows; add fixtures before each adapter. |
| Go/Rust/Python core runner fidelity | #2281, #2203, #2096, #1958, #660, PRs #2284/#2006/#1969/#679 | optimized | MVP includes `go test`, `cargo test`, `cargo check`, and `pytest`, preserving compiler/vet/test failure details and final status. | Core language runners are in scope; adjacent tools like nextest/golangci-lint are only safe when explicitly fixture-backed. |
| Rewrite/proxy shell safety | #2262, #2191, #2163, #2145, #2032, PRs #2274/#2225/#2165 | passthrough-compatible | Safety gate should pass through pipelines, redirects, heredocs, command substitution, compound commands, background jobs, loops, and `xargs` by default. | Users lose some the prior token-saving tool auto-rewrite magic, but TSS should not corrupt shell intent. |
| Unknown commands and broad command requests | #2259, #2245, #2238, #2237, #2158, #2057, #1995, #756 | passthrough-compatible | Unknown/non-MVP command families pass through unchanged. Broad AWS, `kubectl`, Docker, Terraform, DB, JVM, Ruby, PHP, Swift, C/C++, mobile, and infra coverage is outside v0.1 optimization. | Migration from the prior token-saving tool broad coverage must be honest: safe no-op first, future adapters by fixture demand. |
| Long-tail structured/cloud CLIs | #2139, #1209, #896, #896, #645, #402 | passthrough-compatible | TSS should preserve raw output unless a parser-backed adapter exists for a specific CLI and output mode. | Cloud and infra users keep correctness but may see lower savings than the prior token-saving tool. |
| MVP agent integrations | #2258, #2242, #2216, #2198, #2033, #899, PRs #2269/#2234/#2198 | planned | Phase 5 covers Claude Code, GitHub Copilot, Gemini CLI, OpenCode, Cursor, and Codex, with host-specific detection/install/verify/uninstall and `tss doctor integrations`. | Migration docs must say which host surfaces can mutate commands and which are wrapper/instruction only. |
| Analytics, discovery, gain, savings claims | #2241, #2208, #1973, #2001, #486, PRs #1978/#1987/#1176 | planned | Phase 6 includes local-only ledger fields, `tss gain`, `tss gain --json`, passthrough reasons, and provider-cache caveats. | familiar savings dashboards should become more conservative and auditable. |
| Privacy/security/local raw store | #2070, #2085, #2106, #640, PRs #2072/#2050/#1988/#655/#656 | planned | Trust/privacy docs require local-first storage, restrictive permissions, `TSS_NO_STORE=1`, `TSS_NO_ANALYTICS=1`, argument redaction, and no remote telemetry. | Security-sensitive users can disable raw storage and rely on passthrough instead of hidden upload/reporting. |
| Platform/install/release packaging | #2235, #2132, #1945, #615, #383, PRs #2089/#2196/#1168/#1165 | planned | Phase 7 calls for CI on macOS/Linux/Windows, release workflows, checksums, and package smoke tests. | v0.1 launch must reduce false-positive and broken-binary panic with checksums and clear platform support. |
| Docs/onboarding/migration | #2244, #2115, #2067, #2034, #590, PRs #2266/#2257/#2156/#1170 | planned | Phase 8 includes README, the prior token-saving tool migration guide, filter behavior matrix, benchmark methodology, troubleshooting, contributing, and issue templates. | Docs must explain passthrough as a successful safety decision, not a missing feature. |
| CI/tests/contributor workflow | #208, #348, #805, #943, PRs #901/#1169 | planned | Phase 7 adds contract fixtures, CI, release checks, and `tss verify fixtures`. Development rules require one fixture/contract test before any enabled adapter. | issue fixes should become TSS regression fixtures before implementation. |
| Output polish and compression quality | #2230, #795, #639, #972, #1174 | planned | MVP allows compaction only where output remains truthful; formatting polish follows after domain fixtures. | Cosmetic compression should not outrank failure or path fidelity. |
| familiar command vocabulary | the prior token-saving tool patterns: `legacy-cli proxy`, `legacy-cli raw`, `legacy-cli gain`, `legacy-cli init`, `legacy-cli git`, `legacy-cli grep`, `legacy-cli find` | planned | Current plan/code expose `tss run -- <cmd>`, `tss raw <id>`, `tss doctor`, `tss gain`, `tss init <agent>`, and `tss verify`; Phase 6 adds `tss config privacy`; Phase 7 adds `tss verify fixtures`. | Add or document familiar aliases/shorthands before v0.1 docs freeze. |
| Project-local/custom filters and untrusted TOML | #2179, #820, #707, #587, #488 | blocked-by-trust-contract | v0.1 explicitly rejects project-local filters/config that alter output without explicit user approval. | users with local custom filters need a manual trust workflow or must stay raw in v0.1. |
| Remote telemetry/raw-output sharing | telemetry/privacy issue class, CVE/security research | blocked-by-trust-contract | Cloud telemetry and remote raw-output storage are explicit MVP non-goals. | Enterprise/privacy users get a clearer trust story; community analytics must wait. |
| General plugin/framework/library API | #2037, #818, #1442, #758 | blocked-by-trust-contract | Plugin framework and multi-crate/library API are explicit v0.1 non-goals until core trust is proven. | Third-party extension authors should contribute fixture-backed built-ins first. |
| Long-tail agent integrations beyond MVP six | #2248, #2207, #2190, #2136, #2099, #1975, #1966, #1944 | needs-research | TSS should not promise parity for every the prior token-saving tool agent/plugin until host APIs and trust implications are reviewed. | Provide issue templates and docs-only guidance rather than silent unsupported installs. |
| Vague, support, or title-only reports | needs-info/question labels, title-only exports | needs-research | The JSON exports lack bodies for many items, so exact failure shape cannot always be inferred. | Preserve a research backlog and require repro fixtures before adding adapters. |
| Open PR lifecycle/status classes | `wrong-base`, `awaiting-changes`, duplicate, DONE_REVIEW | needs-research | These PR states say more about contribution hygiene than product scope. | TSS needs contributor docs and triage labels, but should not blindly port stale PR logic. |

## Classification Counts

Counts are by matrix row, not by individual the prior token-saving tool ticket.

| Classification | Matrix rows |
|---|---:|
| optimized | 7 |
| passthrough-compatible | 3 |
| planned | 8 |
| blocked-by-trust-contract | 3 |
| needs-research | 3 |

## the prior token-saving tool-Familiar Command Vocabulary

TSS should make migration feel familiar without copying the legacy tool's unsafe rewrite posture.

| familiar concept | TSS v0.1 command shape | Status / note |
|---|---|---|
| Wrap a native command | `tss run -- <cmd> [args...]` | Present in CLI skeleton and Phase 2. |
| Short wrapper form | `tss -- <cmd> [args...]` | Used in `docs/mvp-scope.md` examples but not in current CLI help; align docs or add alias before v0.1. |
| Recover raw output | `tss raw <id>` | Core trust-contract command. Phase 3 plans raw/full/stdout/stderr modes. |
| Savings report | `tss gain`, `tss gain --json`, `tss gain --failures` | Planned in Phase 6. |
| Agent setup | `tss init claude|copilot|gemini|opencode|cursor|codex` | Planned in Phase 5. |
| Integration health | `tss doctor integrations` | Planned in Phase 5; generic `tss doctor` exists in skeleton. |
| Uninstall integration | `tss uninstall <agent>` | Planned in Phase 5, not in current CLI skeleton. |
| Verify fixtures | `tss verify fixtures` | Planned in Phase 7. |
| Privacy config | `tss config privacy` | Planned in Phase 6, not in current CLI skeleton. |
| the prior token-saving tool direct subcommands | `legacy-cli git`, `legacy-cli grep`, `legacy-cli find`, `legacy-cli npm`, etc. | TSS docs currently prefer wrapping native commands. Consider familiar aliases only for MVP families if they can share the same trust gate. |
| the prior token-saving tool discovery/learn | `legacy-cli discover`, `legacy-cli learn` | No direct v0.1 equivalent. Keep as future research; `tss gain` can report observed categories after local ledger exists. |

## Migration Implications

- TSS v0.1 should optimize fewer command families than the prior token-saving tool, but every optimized path
  must be backed by fixtures and trust-contract validation.
- Unsupported commands should be explained as safe passthrough, not failures.
- Users depending on the legacy tool's broad ecosystem filters should expect correctness parity
  through raw passthrough, not token-saving parity.
- Users depending on the prior token-saving tool custom TOML/project-local filters need a new explicit-trust
  story; v0.1 should not silently honor those filters.
- Agent setup docs must be host-specific. Codex should be instruction/wrapper mode,
  not described as hook-equivalent.
- Savings claims should cite the fixture matrix and command set, not global percent
  claims.
- Every migrated issue/PR should first become a raw fixture and public-interface
  contract test.

## Unresolved Gaps

- Align `tss run -- <cmd>` vs `tss -- <cmd>` before publishing v0.1 docs.
- Decide whether familiar direct aliases such as `tss git`, `tss grep`, and `tss find`
  are migration helpers or unnecessary surface area.
- Add `tss uninstall <agent>` and `tss config privacy` to the CLI skeleton if they
  remain v0.1 commitments.
- Prioritize the non-MVP command backlog by fixture demand; current open items
  include cloud/infra, JVM, Ruby, PHP, C/C++, mobile, database, and OS-admin CLIs.
- Research long-tail agent APIs before promising installation support.
- Convert title-only/vague the prior token-saving tool reports into reproducible fixtures before classifying
  them as optimized.
