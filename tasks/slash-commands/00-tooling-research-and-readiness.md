# 00 - Tooling Research And Readiness

## Goal

Confirm the existing command, TUI, CLI host, and harness runtime surfaces before
implementation starts. Convert findings into concrete constraints and verify
the current baseline so later subjects do not guess at behavior.

## Boxes

- [x] **00.1** (agent) Read `AGENTS.md`, `docs/00-clean-room.md`, and relevant
      product docs; list applicable constraints for this feature.
- [x] **00.2** (agent) Inventory the current slash command parser, host routing,
      tests, and render state.
- [x] **00.3** (agent) Inventory harness message-history and compaction APIs;
      identify the smallest public API needed for `/clear` and `/compact`.
- [x] **00.4** (agent) Check whether any existing docs already promise
      `/clear`, `/compact`, or `/search`.
- [x] **00.5** (agent) Run or record baseline verification:
      `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
      `cargo test --workspace`, and `cargo check --workspace`.
- [x] **00.6** (agent) Record any baseline failures exactly, including whether
      they are pre-existing.
- [x] **00.7** (agent) Confirm no read-only behavior reference is needed; if it
      becomes necessary, apply clean-room provenance rules first.
- [x] **00.8** (agent) Run Captain Hindsight and record the verdict before
      closing this subject.

## Progress

| Date | Summary | Boxes |
|---|---|---|
| 2026-06-05 | Inventoried parser, host routing, TUI search/render state, runtime compaction, and product docs. No read-only behavior reference was needed. Gate commands passed after implementation with no baseline failures observed. | 00.1-00.8 |

## Captain Hindsight

- Keep: Existing parser/render/runtime boundaries were sufficient; no reference implementation was needed.
- Fix before closing: None.
- Record: `cargo fmt --check`, clippy, workspace tests, and workspace check passed after implementation.
- Risk: Terminal-host behavior is covered through parser/runtime/TUI tests rather than a terminal I/O unit test.
- Verdict: CLOSE
