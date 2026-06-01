# 08 — Terminal UI

## Goal
> Phase 10 (`docs/03`) — the interactive REPL on the committed `ratatui` +
> `crossterm` + `tui-textarea` stack (ADR-0006, not a suggestion). Terminal-
> native, dense, quiet UI (`docs/12` UI Direction): header, large
> transcript/input area, always-visible footer stats, optional thinking panel,
> permission/mode indicators, narrow-terminal collapse. Renders the subject-05
> runtime event stream; drives subject-06 harness commands; shows subject-07
> quota state. Snapshot-tested with ratatui `TestBackend` (`docs/13` §10,
> `docs/08`). Cross-platform via crossterm (ADR-0007).

## Boxes
> ID = `08.<box-number>`. All agent-owned.

- [ ] **08.1** (agent) Add the TUI stack deps to the workspace table
      (`ratatui`, `crossterm`, `tui-textarea`, exact-pinned, `docs/13` §1) and a
      base app/event loop in `unshackled-tui` that consumes the subject-05
      runtime event channel and handles crossterm input. (Verified: app starts
      and quits cleanly under a scripted event source.)
- [ ] **08.2** (agent) Implement the message **viewport**/transcript area and
      streaming response rendering (text deltas appended live) (`docs/03` Phase
      10, `docs/12`). (Verified: `TestBackend` snapshot of a streamed turn.)
- [ ] **08.3** (agent) Implement the **prompt input** box using `tui-textarea`
      (multi-line) (`docs/02` §`unshackled-tui`). (Verified: snapshot of input
      with wrapped multi-line text.)
- [ ] **08.4** (agent) Implement the **header** (app, version, provider/model,
      workspace; short session IDs labelled explicitly) (`docs/12` UI
      Direction). (Verified: header snapshot.)
- [ ] **08.5** (agent) Implement the **always-visible footer stats**
      (`docs/12`): model/provider, mode, permission state, tokens in/out,
      tokens/sec, context usage, estimated cost/usage when known, quota/reset
      timer when paused or near a limit. Footer is never hidden by the thinking
      panel and stays visible during streaming + tool execution (`docs/03` Phase
      10 Done-when). (Verified: footer snapshot during streaming; a test that the
      thinking panel does not occlude it.)
- [ ] **08.6** (agent) Implement the **tool approval modal/dialog** wired to the
      subject-04 approval interface (`docs/03` Phase 10): shows tool, normalized
      path/command, risk class, and the active permission profile; `bypass` shows
      no prompt but the profile is visible in the footer (`docs/07`). (Verified:
      approval-modal snapshot; an approve/deny round-trip test against a scripted
      decision.)
- [ ] **08.7** (agent) Implement the **status line** and permission/mode
      indicators (`docs/12`). (Verified: status-line snapshot showing
      mode+profile.)
- [ ] **08.8** (agent) Implement the optional **thinking/reasoning side panel**
      rendering `ReasoningDelta` events as metadata (not final answer), toggleable
      at runtime (`docs/12`, `docs/04`). (Verified: snapshot with panel on/off;
      reasoning text routed to the panel, not the transcript.)
- [ ] **08.9** (agent) Implement **slash commands** in the REPL (run harness
      commands and mode/model switches inside the TUI, `docs/03` Phase 10
      Done-when, `docs/01` Interactive REPL). (Verified: a slash command triggers
      the matching action under a scripted input.)
- [ ] **08.10** (agent) Implement the **model/provider picker** and transcript
      **search** (`docs/03` Phase 10). (Verified: picker selection switches the
      active provider in runtime state; search highlights matches in a snapshot.)
- [ ] **08.11** (agent) Implement **responsive collapse** for narrow terminals:
      the right panel auto-collapses and the footer stats remain visible; no text
      overlap in common terminal sizes (`docs/03` Phase 10 Done-when, `docs/12`).
      (Verified: `TestBackend` snapshots at narrow + wide widths show no overlap
      and footer present at both.)

## Progress log
> One line per slice. Date · slice · box IDs · what shipped · how verified.
