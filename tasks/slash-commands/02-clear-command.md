# 02 - Clear Command

## Goal

Implement `/clear` so an interactive user can reset the visible transcript and
runtime conversation history without losing session configuration or trust.

## Boxes

- [x] **02.1** (agent) Add a focused `AppState` method or event to clear
      transcript, streaming output, thinking text if appropriate, plan display
      if appropriate, search state, footer per-turn stats if appropriate, and
      pending input-independent UI state.
- [x] **02.2** (agent) Add a `SessionRuntime` API to clear conversation
      messages while preserving the leading system/context setup required for
      future turns.
- [x] **02.3** (agent) Wire `/clear` in the CLI host so UI and runtime state are
      reset together.
- [x] **02.4** (agent) Add a concise user-visible notice after clear.
- [x] **02.5** (agent) Test that `/clear` does not change mode, profile, trust
      state, provider/model, session ID, or working directory.
- [x] **02.6** (agent) Run Captain Hindsight and record the verdict before
      closing this subject.

## Progress

| Date | Summary | Boxes |
|---|---|---|
| 2026-06-05 | Implemented `AppState::clear_conversation_view` and `SessionRuntime::clear_conversation`, wired `/clear` in the host, and tested UI/session preservation plus provider-context reset. | 02.1-02.6 |

## Captain Hindsight

- Keep: Clear resets transient UI and runtime history without touching mode, profile, trust, provider/model, session id, or workspace.
- Fix before closing: None.
- Record: Runtime clear drops generated compaction summaries so a prior `/compact` cannot leak old history into the next request.
- Risk: Persisted transcript files are not rewritten; `/clear` targets current view and in-memory runtime history.
- Verdict: CLOSE
