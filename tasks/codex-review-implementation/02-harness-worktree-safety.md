# 02 - Resume Preflight And Scoped Staging

## Goal

Prevent harness resume from committing unrelated dirty worktree changes by
enforcing session-start rules before model work and by limiting staging to
known harness-owned paths.

## Boxes

- [x] **02.1** (agent) Trace resume start, rule engine triggers, git status
      parsing, tool-call path tracking, and commit staging code; identify the
      smallest safe staging contract.
- [x] **02.2** (agent) Evaluate `RuleEngine` with `Trigger::SessionStart` at
      the start of each resume step before model/tool execution.
- [x] **02.3** (agent) Populate `RuleContext::uncommitted_unrelated` from
      `git status --porcelain` using cross-platform path handling and existing
      workspace policy.
- [x] **02.4** (agent) Replace broad `git add -A` staging with scoped staging
      for harness-owned files and paths actually changed by approved tool calls,
      or block if the changed-path set cannot be trusted.
- [x] **02.5** (agent) Add regression tests with pre-existing dirty files that
      prove resume blocks before model/tool execution and does not stage or
      commit unrelated edits.
- [x] **02.6** (agent) Add regression tests proving legitimate harness changes,
      including runtime `PROGRESS.md`, are still committed when the worktree is
      otherwise clean.

## Hindsight checkpoint

- [x] Captain Hindsight review recorded
- [x] Verdict is `CLOSE`

## Progress log

> One line per slice. Date - slice number - box IDs touched - what shipped -
> how verified - checkpoint commit/push status.

- 2026-06-05 - 1 - 02.1-02.6 - Added session-start preflight using
  `RuleEngine` and porcelain git status, removed broad `git add -A`, scoped
  project staging away from runtime state, and committed `PROGRESS.md` as its
  own runtime update. Added dirty-worktree and legitimate harness commit
  regressions. Verified by focused harness tests and the final workspace gate.
  Checkpoint not committed/pushed by agent.

## Captain Hindsight

1. Keep: Blocking before provider work is clearer and safer than trying to
   distinguish unrelated changes after a model turn.
2. Fix before closing: None.
3. Record: Runtime state under `.unshackled/` is intentionally excluded from
   harness commits; `PROGRESS.md` is staged explicitly.
4. Risk: The scoped staging contract currently uses git status rather than a
   future tool-call changed-path ledger, which is acceptable for this closeout
   but may be refined later.
5. Verdict: CLOSE.
