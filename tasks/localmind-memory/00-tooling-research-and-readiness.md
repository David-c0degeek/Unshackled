# 00 — Tooling Research And Readiness

## Goal

Capture the repo context and constraints already gathered for the LocalMind
memory replacement.

## Boxes

- [x] **00.1** (agent) Read repo instructions and clean-room policy.
- [x] **00.2** (agent) Inventory current memory/learning crate wiring.
- [x] **00.3** (agent) Inspect LocalMind store capabilities and test coverage.
- [x] **00.4** (agent) Record plan constraints and implementation direction.

## Hindsight checkpoint

- [x] Captain Hindsight review recorded
- [x] Verdict is `CLOSE`

Keep: The dependency direction is clear: LocalPilot adapts LocalMind, LocalMind
does not depend back.
Fix before closing: None for tooling readiness.
Record: LocalMind needs first-class list/delete APIs so LocalPilot does not
recreate persistence behavior.
Risk: Full workspace verification still pending.
Verdict: CLOSE

## Progress log

- 2026-06-05 · Initial context gathered · verified by file reads and focused
  searches · no checkpoint commit yet.
