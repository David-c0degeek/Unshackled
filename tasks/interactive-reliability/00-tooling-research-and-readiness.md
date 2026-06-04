# 00 - Tooling Research And Readiness

## Goal

Confirm the affected boundaries, reproduction evidence, and repository rules.

## Boxes

- [x] **00.1** (agent) Read repository instructions and applicable skills/docs.
- [x] **00.2** (agent) Trace TUI input, terminal key mapping, and session streaming.
- [x] **00.3** (agent) Inspect the recorded failing session and preserve unrelated changes.

## Hindsight checkpoint

Keep: direct evidence identified two independent defects.
Fix before closing: none.
Record: the provider endpoint was offline during diagnosis.
Risk: raw provider SSE could not be replayed.
Verdict: CLOSE.

## Progress log

2026-06-04 - diagnosis completed; no code edits; checkpoint not committed because the shared worktree contains unrelated in-progress changes.
