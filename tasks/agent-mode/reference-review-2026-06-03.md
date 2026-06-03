# Read-Only Behavior Reference Review — 2026-06-03

## Scope

Reviewed committed changes in `D:\repos\unshackled` from 2026-05-27 through
2026-06-03 as a read-only behavior reference. This review used commit metadata
and high-level behavior notes only. It did not copy, translate, paraphrase, or
derive code, prompts, tests, identifiers, schemas, file layout, UI copy, or
private endpoint behavior from the reference.

The reference worktree had local uncommitted changes in `README.md`,
`plan.telemetry-egress.md`, and `src/services/egressGuard.ts`; those were not
used.

## Findings

| Reference behavior lesson | Rust status | Action |
|---|---|---|
| Context compaction must not cut through an API/tool round; a kept tool result must keep its owning tool call. | Rust already had pairing tests for normal compaction, but final tight-budget trimming could remove individual messages. | Adopted now: `fix(harness): preserve tool rounds during compaction` adds whole-exchange final trimming, adaptive summaries, and a regression test. |
| Local/non-first-party providers are more likely to produce runaway or degenerate output from uncapped helper/fork calls. | Rust does not have prompt-suggestion or cache-sharing fork calls. It already has provider request timeouts, loop caps, and bad-output recovery. | No immediate port. Keep as a live-eval scenario for local providers. |
| Local vision backends can degrade when too many images are sent; omitted media should be explicit. | Multimodal/image input is out of scope for the current Rust agent-mode plan. | Defer until image input exists. |
| Harness completion gates should check real `PROGRESS.md` state, not a placeholder signal. | Rust harness parses `PROGRESS.md`, updates it on resume, and blocks commit when progress does not reflect completion. | Already covered; no action. |
| Post-edit quality checks must run on every edit-like tool path. | Rust has `edit_file` and `multi_edit`; harness post-edit rules are currently coarse and not wired to diff-scoped edited content. | Consider later if test-first or post-edit quality rules become stricter. |
| Arbitrary outbound network calls need a chokepoint/allowlist when the runtime has generic fetch. | Rust has no global fetch equivalent; provider clients are explicit and shell/network tools are permission-gated. | Prefer a release/audit gate over an app-level wrapper for now. |
| Private/undocumented endpoints should be blocked or require explicit user-owned configuration. | Rust ADRs and provider contracts already prohibit private endpoints; providers support official APIs, local servers, and custom user endpoints. | Already covered; keep final clean-room grep in plan gate. |
| Telemetry absence should be verifiable, not just asserted. | Rust docs say no remote telemetry by default; no telemetry subsystem exists. | Add to final release audit if needed: artifact scan for telemetry/vendor callback strings and endpoint inventory. |
| Build packaging should avoid fragile eager bundling modes. | The Rust project does not use JS bytecode bundling. | Not applicable. |
| Plans need close-out review before being marked finished. | Agent-mode subjects already use Captain Hindsight close checkpoints; final plan gate includes reviewer sign-off. | Already covered. |

## Carry-Forward

- Keep the compaction regression in the harness suite.
- Add local-provider runaway/helper-call scenarios to subject 05 when the live
  eval runner is implemented.
- When release hardening starts, include a no-unexpected-egress artifact audit
  and private-endpoint grep in the final gate.
