# 02 - Response Completion

## Goal

Prevent obvious mid-word response fragments from being persisted as complete.

## Boxes

- [x] **02.1** (agent) Require provider protocol completion markers.
- [x] **02.2** (agent) Route incomplete streams through recovery without persisting fragments.
- [x] **02.3** (agent) Add provider-decoder and session-runtime regression tests.

## Hindsight checkpoint

Keep: completion is decided from protocol events, not guessed from prose.
Fix before closing: none.
Record: compatible servers that close after an explicit finish/stop reason remain supported.
Risk: the reported local endpoint was offline, so its raw SSE could not be replayed.
Verdict: CLOSE.

## Progress log

2026-06-04 - implemented and verified with `cargo test -p unshackled-llm` and
`cargo test -p unshackled-harness --test session`; not committed due unrelated
shared-worktree changes.
