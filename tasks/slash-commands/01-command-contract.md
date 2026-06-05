# 01 - Command Contract

## Goal

Define the observable behavior and parser shape for `/clear`, `/compact`, and
`/search` before implementation. The contract should be precise enough for
tests without over-specifying implementation internals.

## Boxes

- [x] **01.1** (agent) Extend `SlashAction` with explicit actions for clear,
      compact, and search.
- [x] **01.2** (agent) Decide and document argument behavior:
      `/search <query>` sets search, `/search` clears search, `/clear` and
      `/compact` reject or ignore trailing arguments consistently.
- [x] **01.3** (agent) Preserve exact existing aliases: `/thinking`,
      `/wait_resume`, and `/q`.
- [x] **01.4** (agent) Add parser tests for supported commands, aliases,
      argument parsing, and unknown commands.
- [x] **01.5** (agent) Run Captain Hindsight and record the verdict before
      closing this subject.

## Progress

| Date | Summary | Boxes |
|---|---|---|
| 2026-06-05 | Added explicit `SlashAction` variants for clear, compact, search, invalid argument errors, and unknown commands. `/clear` and `/compact` reject trailing arguments; `/search` accepts a trimmed query or clears with no query. Existing aliases remain covered. | 01.1-01.5 |

## Captain Hindsight

- Keep: Parser stays deterministic and terminal-free.
- Fix before closing: None.
- Record: Invalid known-command arguments are parsed separately from unknown commands so the host can give precise feedback.
- Risk: Search remains exact and case-sensitive by design.
- Verdict: CLOSE
