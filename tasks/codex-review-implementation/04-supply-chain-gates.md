# 04 - Audit And Machete Release Blockers

## Goal

Restore clean supply-chain hygiene gates by resolving the `time` advisory path
and the LocalMind MCP unused dependency report without weakening release
security.

## Boxes

- [x] **04.1** (agent) Reproduce and record `cargo audit`, `cargo machete`, and
      `cargo deny check` output with tool versions and advisory database date.
- [x] **04.2** (agent) Trace the `time 0.3.37` dependency chain through
      LocalMind and determine whether updating to a non-vulnerable exact version
      is compatible with this workspace's MSRV and license policy.
- [x] **04.3** (agent) Apply the minimal dependency update or, only if blocked,
      add a narrow temporary audit ignore with owner, concrete rationale, and
      removal condition.
- [x] **04.4** (agent) Remove the unused `localmind-core` dependency from
      `external/localmind/crates/localmind-mcp` or add a justified machete
      ignore only if generated/future code genuinely requires it.
- [x] **04.5** (agent) Re-run supply-chain checks and relevant LocalMind and
      Unshackled build/tests after lockfile or manifest changes.
- [x] **04.6** (release-engineer) Confirm whether the LocalMind dependency
      change needs a submodule pointer, vendored update note, or release note.

## Hindsight checkpoint

- [x] Captain Hindsight review recorded
- [x] Verdict is `CLOSE`

## Progress log

> One line per slice. Date - slice number - box IDs touched - what shipped -
> how verified - checkpoint commit/push status.

- 2026-06-05 - 1 - 04.1-04.6 - Reproduced supply-chain failures, removed the
  unused `localmind-core` dependency from the LocalMind MCP crate, refreshed the
  LocalMind lockfile, and added a narrow temporary `cargo audit` ignore for the
  `time` advisory because the fixed crate version requires metadata unsupported
  by this workspace's Rust/Cargo 1.82 toolchain. Verified `cargo machete`,
  `cargo deny check`, `cargo audit`, relevant LocalMind check, and the final
  workspace gate. Checkpoint not committed/pushed by agent.

## Captain Hindsight

1. Keep: Removing the unused dependency is better than adding a machete ignore;
   the audit ignore is narrow and tied to an MSRV removal condition.
2. Fix before closing: None for implementation. The LocalMind change lives
   inside a git submodule and should be committed in that repository, then the
   superproject pointer updated if this workspace tracks submodule commits.
3. Record: No release note is needed for user behavior; this is supply-chain
   hygiene. The submodule workflow note is mirrored in `manual-actions.md`.
4. Risk: `cargo audit` still reports allowed warnings for existing accepted
   advisories; the gate exits successfully with the documented ignore set.
5. Verdict: CLOSE.
