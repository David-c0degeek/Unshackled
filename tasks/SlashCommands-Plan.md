# SlashCommands-Plan.md

> Disposable build plan for implementing `/clear`, `/compact`, and `/search`
> in the interactive REPL. Delete or archive this plan once the feature ships.
> Shipped code, tests, comments, identifiers, and commit messages must not
> reference this plan or its subject IDs.

## 1. Subject

Implement three interactive slash commands:

- `/clear` clears the current interactive conversation view and runtime chat
  history while keeping the session configuration, trust state, selected
  provider/model, permission profile, and working directory.
- `/compact` manually compacts the current runtime message history using the
  same compaction semantics as automatic context compaction, then updates the
  UI with a clear notice and context usage.
- `/search <query>` highlights matching transcript lines; `/search` with no
  query clears the active search.

Out of scope: adding a full command palette, non-interactive CLI variants,
changing provider contracts, changing automatic compaction policy, or adding
regex/fuzzy search in the first slice.

## 2. Authoritative Inputs

| Source | Contribution |
|---|---|
| `AGENTS.md` | Planning weight, clean-room boundaries, and repository workflow constraints. |
| `crates/unshackled-tui/src/app.rs` | Current slash parser and UI-only slash behavior. |
| `crates/unshackled-cli/src/repl.rs` | Interactive host behavior for slash commands that affect runtime state. |
| `crates/unshackled-harness/src/session.rs` | Runtime turn loop, automatic compaction, and context usage events. |
| `crates/unshackled-harness/src/compaction.rs` | Existing compaction contract and tests. |
| `crates/unshackled-tui/src/state.rs` and `render.rs` | Transcript, search state, footer context usage, and notice rendering. |
| `docs/01-product-spec.md` | Interactive REPL expectations, including slash commands and transcript search. |

## 3. Subject File Index

| # | File | Subject |
|---|---|---|
| 00 | `tasks/slash-commands/00-tooling-research-and-readiness.md` | Tooling research and readiness |
| 01 | `tasks/slash-commands/01-command-contract.md` | User-facing command contract |
| 02 | `tasks/slash-commands/02-clear-command.md` | `/clear` implementation |
| 03 | `tasks/slash-commands/03-compact-command.md` | `/compact` implementation |
| 04 | `tasks/slash-commands/04-search-command.md` | `/search` implementation |
| 05 | `tasks/slash-commands/05-tests-docs-and-gate.md` | Tests, docs, and final gate |

## 4. Decision Log

| ID | Date | Title | Decision | Rationale | Refs |
|---|---|---|---|---|---|
| D001 | 2026-06-05 | Slash commands stay REPL-scoped | Implement `/clear`, `/compact`, and `/search` as interactive REPL slash commands only. | The request is about slash commands; existing non-interactive CLI commands have separate subcommand surfaces. | 01.1, 01.2 |
| D002 | 2026-06-05 | Plain substring search first | `/search <query>` uses case-sensitive substring matching initially, matching current `AppState.search` rendering behavior. | The current UI already supports exact highlight state; regex/fuzzy matching would add avoidable scope. | 04.1, 04.2 |

## 5. Master Progress Tracker

| Done | # | File | Status | Owner Summary | Human Actions Mirrored? |
|---|---|---|---|---|---|
| [x] | 00 | `tasks/slash-commands/00-tooling-research-and-readiness.md` | DONE | Inventoried TUI parser/render state, CLI host routing, harness compaction APIs, docs, and baseline constraints; no read-only reference needed. | n/a |
| [x] | 01 | `tasks/slash-commands/01-command-contract.md` | DONE | Added explicit parser actions for `/clear`, `/compact`, `/search`, invalid arguments, aliases, and unknown commands with tests. | n/a |
| [x] | 02 | `tasks/slash-commands/02-clear-command.md` | DONE | Implemented UI and runtime clear while preserving session identity, mode/profile, trust, provider/model, and workspace. | n/a |
| [x] | 03 | `tasks/slash-commands/03-compact-command.md` | DONE | Added manual runtime compaction using existing compaction semantics, usage reporting, host wiring, and invariant tests. | n/a |
| [x] | 04 | `tasks/slash-commands/04-search-command.md` | DONE | Wired exact substring transcript search and no-argument search clearing without transcript mutation. | n/a |
| [x] | 05 | `tasks/slash-commands/05-tests-docs-and-gate.md` | DONE | Updated product docs and passed fmt, clippy, workspace tests, and workspace check. | n/a |

## 6. Cross-Cutting Principles

1. Keep code modular and locally understandable; command parsing belongs in
   `unshackled-tui`, host-only runtime effects belong in `unshackled-cli`, and
   message-history behavior belongs in `unshackled-harness`.
2. Preserve existing automatic compaction semantics; manual compaction must use
   the same tool-call/result pairing rules and bounded summary behavior.
3. Do not clear trust state, permission profile, provider/model selection, or
   working directory with `/clear`.
4. Keep slash parsing deterministic and testable without terminal I/O.
5. Unknown slash command behavior remains explicit and user-visible in the CLI.
6. Search is a UI filter/highlight state, not a mutation of transcript or
   runtime message history.
7. Code, tests, comments, and commit messages must be plan-agnostic.
8. Run the Captain Hindsight checkpoint before marking each subject `DONE`.

## 7. Gate Review

- [x] All §5 subjects done or explicitly abandoned with a §4 row.
- [x] Subject 00 completed or explicitly waived with a §4 row.
- [x] `cargo fmt --check`
- [x] `cargo clippy --workspace --all-targets -- -D warnings`
- [x] `cargo test --workspace`
- [x] `cargo check --workspace`
- [x] Slash-command tests cover known commands, command arguments, unknown
      commands, and no-argument `/search`.
- [x] Runtime tests cover `/clear` history reset and `/compact` preserving
      compaction invariants.
- [x] TUI render/input tests cover search state and clearing search.
- [x] User-facing docs mention the new commands and their exact behavior.
- [x] Every non-abandoned subject has a Captain Hindsight verdict of `CLOSE`.
- [x] `tasks/slash-commands/manual-actions.md` has no unresolved required
      human actions.
- [x] `tasks/slash-commands/lessons.md` reconciled into `tasks/lessons.md` if
      any durable lesson was learned.

## 8. Acceptance / Sign-Off

| Date | Reviewer | Result | Notes |
|---|---|---|---|
| 2026-06-05 | agent | PASS | Full gate passed; no read-only behavior reference used. |
