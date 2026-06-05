# 03 - Compact Command

## Goal

Implement `/compact` as a manual trigger for the existing context compaction
rules, preserving tool-call/result pairing and bounded summary behavior.

## Boxes

- [x] **03.1** (agent) Add a `SessionRuntime` API that compacts the stored
      message history using the configured context token limit.
- [x] **03.2** (agent) Ensure manual compaction reports whether anything was
      compacted and the resulting estimated context usage.
- [x] **03.3** (agent) Keep automatic compaction behavior unchanged in
      `run_turn`.
- [x] **03.4** (agent) Wire `/compact` in the CLI host and update footer context
      usage after a successful manual compaction.
- [x] **03.5** (agent) Add tests for manual compaction no-op, manual compaction
      with dropped exchanges, and preservation of tool-call/result pairing.
- [x] **03.6** (agent) Add a user-visible notice that distinguishes "already
      compact enough" from "compacted history".
- [x] **03.7** (agent) Run Captain Hindsight and record the verdict before
      closing this subject.

## Progress

| Date | Summary | Boxes |
|---|---|---|
| 2026-06-05 | Added `SessionRuntime::compact_conversation`, `ManualCompaction`, host footer/notice wiring, and runtime tests for no-op, summary storage, clear-after-compact, and tool pair invariants. | 03.1-03.7 |

## Captain Hindsight

- Keep: Manual compaction reuses `compact_with_summary`; automatic `run_turn` compaction remains unchanged.
- Fix before closing: None.
- Record: Host notices distinguish no-op from actual compaction and include resulting context usage.
- Risk: Manual compaction uses deterministic local summaries, matching the existing automatic compaction contract.
- Verdict: CLOSE
