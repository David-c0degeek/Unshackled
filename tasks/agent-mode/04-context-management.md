# 04 — Context, Compaction, and Long-Session Handling

## Goal
Keep multi-step sessions coherent within the model's context window: compact
older turns without losing task state, and surface context pressure to the user.

## Boxes

- [x] **04.1** (agent) Tune compaction so it preserves the task goal, the most
      recent tool results, and pending work while trimming old turns; verify it
      keeps a long tool-using session under the configured token limit. Artefact:
      a compaction test on a long synthetic session.
- [x] **04.2** (agent) Add an original conversation-summary step that condenses
      trimmed history into a compact "what happened so far" note retained in
      context, so the model does not lose earlier decisions across compaction.
      Artefact: a test that the summary is injected and bounded.
- [x] **04.3** (agent) Estimate and surface context usage accurately in the
      footer (used/limit) so the user sees pressure; wire real token accounting
      from provider usage where available. Artefact: footer/usage test.
- [x] **04.4** (agent) Verify behavior at the boundary: a session that exceeds
      the window compacts and continues rather than failing. Artefact: a
      boundary test.

## Hindsight checkpoint
- [x] Captain Hindsight review recorded
- [x] Verdict is `CLOSE`

## Progress log
> One line per slice. Date · slice · box IDs · what shipped · how verified.

- 2026-06-03 · context management · 04.1-04.3 · added bounded summary compaction and context usage events surfaced to the TUI · verified with compaction and session tests.
- 2026-06-03 · boundary behavior · 04.4 · added a full session regression that exceeds the configured context limit, compacts, executes a tool, and continues the turn · verified with `cargo test -p unshackled-harness --test session context_boundary_compacts_and_continues_the_turn`.
- 2026-06-03 · hindsight close · 04.1-04.4 · boundary, summary retention, usage surfacing, and continuation behavior are covered by focused tests; no open context-management box remains · verdict `CLOSE`.
