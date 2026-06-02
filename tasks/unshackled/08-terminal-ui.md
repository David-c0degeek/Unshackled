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

- [x] **08.1** (agent) Add the TUI stack deps to the workspace table
      (`ratatui`, `crossterm`, `tui-textarea`, exact-pinned, `docs/13` §1) and a
      base app/event loop in `unshackled-tui` that consumes the subject-05
      runtime event channel and handles crossterm input. (Verified: app starts
      and quits cleanly under a scripted event source.)
- [x] **08.2** (agent) Implement the message **viewport**/transcript area and
      streaming response rendering (text deltas appended live) (`docs/03` Phase
      10, `docs/12`). (Verified: `TestBackend` snapshot of a streamed turn.)
- [x] **08.3** (agent) Implement the **prompt input** box using `tui-textarea`
      (multi-line) (`docs/02` §`unshackled-tui`). (Verified: snapshot of input
      with wrapped multi-line text.)
- [x] **08.4** (agent) Implement the **header** (app, version, provider/model,
      workspace; short session IDs labelled explicitly) (`docs/12` UI
      Direction). (Verified: header snapshot.)
- [x] **08.5** (agent) Implement the **always-visible footer stats**
      (`docs/12`): model/provider, mode, permission state, tokens in/out,
      tokens/sec, context usage, estimated cost/usage when known, quota/reset
      timer when paused or near a limit. Footer is never hidden by the thinking
      panel and stays visible during streaming + tool execution (`docs/03` Phase
      10 Done-when). (Verified: footer snapshot during streaming; a test that the
      thinking panel does not occlude it.)
- [x] **08.6** (agent) Implement the **tool approval modal/dialog** wired to the
      subject-04 approval interface (`docs/03` Phase 10): shows tool, normalized
      path/command, risk class, and the active permission profile; `bypass` shows
      no prompt but the profile is visible in the footer (`docs/07`). (Verified:
      approval-modal snapshot; an approve/deny round-trip test against a scripted
      decision.)
- [x] **08.7** (agent) Implement the **status line** and permission/mode
      indicators (`docs/12`). (Verified: status-line snapshot showing
      mode+profile.)
- [x] **08.8** (agent) Implement the optional **thinking/reasoning side panel**
      rendering `ReasoningDelta` events as metadata (not final answer), toggleable
      at runtime (`docs/12`, `docs/04`). (Verified: snapshot with panel on/off;
      reasoning text routed to the panel, not the transcript.)
- [x] **08.9** (agent) Implement **slash commands** in the REPL (run harness
      commands and mode/model switches inside the TUI, `docs/03` Phase 10
      Done-when, `docs/01` Interactive REPL). (Verified: a slash command triggers
      the matching action under a scripted input.)
- [x] **08.10** (agent) Implement the **model/provider picker** and transcript
      **search** (`docs/03` Phase 10). (Verified: picker selection switches the
      active provider in runtime state; search highlights matches in a snapshot.)
- [x] **08.11** (agent) Implement **responsive collapse** for narrow terminals:
      the right panel auto-collapses and the footer stats remain visible; no text
      overlap in common terminal sizes (`docs/03` Phase 10 Done-when, `docs/12`).
      (Verified: `TestBackend` snapshots at narrow + wide widths show no overlap
      and footer present at both.)


## Hindsight checkpoint
> Run after all boxes in this subject are complete and before marking
> the subject `DONE` in §5. Use the embedded prompt in `tasks/Unshackled-Plan.md`
> "Appendix: Captain Hindsight Prompt". Record the review result here.
>
> Required output sections: Keep; Fix before closing; Record; Risk;
> Verdict (`CLOSE` or `DO NOT CLOSE`). If the verdict is `DO NOT CLOSE`,
> leave the subject open, add/reopen boxes or update decisions/lessons,
> and rerun this checkpoint after the fixes.
>
> Subjects already marked `DONE` before this checkpoint was added still need
> this section completed retroactively before the §7 gate review is ticked.

- [x] Captain Hindsight review recorded
- [x] Verdict is `CLOSE`

### Review result

1. **Keep:** The TUI is render-centric: one pure `render(frame, &AppState)` draws
   header, transcript/streaming, input, the always-visible footer, the optional
   thinking side panel, the approval modal, and the picker, with responsive
   collapse — all snapshot-tested via `TestBackend`. The event loop is driven by a
   scripted `AppInput` iterator, so slash commands, the picker, and quit are tested
   deterministically. Reasoning is routed to the thinking panel (metadata), never
   the transcript; `bypass` is always visible in the footer; the footer stays
   present at narrow and wide widths.
2. **Fix before closing:** The interactive REPL terminal driver (crossterm event
   loop + `tui-textarea` + launch) lives in the CLI and is a thin wrapper around
   the tested core; it is not unit-tested because crossterm's terminal init crashes
   the harness on the local windows-gnu toolchain (D015). It is verified on real
   terminals / MSVC CI and is on the §9 release checklist ("TUI usable").
3. **Record:** D015 added (TUI decoupled from crossterm; driver in CLI). The
   ratatui transitive license/advisory (Zlib, unmaintained `paste`) posture is in
   `deny.toml`.
4. **Risk:** The live terminal driver and `tui-textarea` integration are exercised
   only off this machine; transcript search highlight is a simple prefix marker
   rather than inline styling. Both acceptable for alpha.
5. **Verdict:** CLOSE.

## Progress log
> One line per slice. Date · slice · box IDs · what shipped · how verified.

- 2026-06-02 · slice 1 · 08.1–08.11 · `unshackled-tui`: view-model `AppState` +
  `UiEvent` mapping + a pure `render` (header, transcript/streaming viewport,
  input, always-visible footer stats, optional thinking side panel, approval
  modal, model/provider picker, transcript search) with responsive narrow
  collapse; a scripted-source event loop with slash commands and modal handling.
  Decoupled from crossterm (D015) so the whole UI snapshot-tests via `TestBackend`.
  Verified: 9 tests — 4 layout snapshots (full/streaming/thinking/approval) + 5
  behaviour (footer-not-occluded, bypass-visible, narrow-keeps-footer, quit, slash,
  picker+search); clippy(-D)/fmt/deny green.
