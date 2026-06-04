# 01 - Input Editing And Caret

## Goal

Support editing at the current cursor position and display the terminal caret.

## Boxes

- [x] **01.1** (agent) Add UTF-8-safe cursor state and editing operations.
- [x] **01.2** (agent) Map terminal navigation/delete keys and cursor-aware paste/newline.
- [x] **01.3** (agent) Render and test the caret at the active input position.

## Hindsight checkpoint

Keep: cursor ownership is centralized in `AppState`; edits preserve UTF-8 boundaries.
Fix before closing: none.
Record: caret placement follows the existing character-count wrapping behavior.
Risk: combining/wide-character display width remains an existing TUI limitation.
Verdict: CLOSE.

## Progress log

2026-06-04 - implemented and verified with `cargo test -p unshackled-tui` and
`cargo test -p unshackled --test key_input --features tui`; not committed due
unrelated shared-worktree changes.
