# 04 — Provider/Model Catalog, Reasoning Effort, Honest Budgets

## Goal
Ship the generated provider/model catalog (research W3): generated data from
the models.dev public dataset, keyed by API protocol shape, with a harness-safe
flag; dynamic discovery of local servers; reasoning-effort as a first-class,
catalog-aware control; and context budgets driven by real per-model windows
instead of one global default. Closes review §5.4 (per-model context limits are
declared but unused) and addresses §3.3 (token-estimate bias is user-visible).
Requires subjects 01–03 `DONE` (D001, D004); 00.5 must have verified models.dev
licensing first.

## Boxes
> ID = `<subject-id>.<box-number>`. Stable — never renumber; mark retired boxes
> done and struck with `ABANDONED; see D###`, don't delete.
> Owner MUST be one of: agent · release-engineer · product-owner · tech-lead ·
> domain-sme.

- [ ] **04.1** (agent) Generated catalog: a vendored, pinned snapshot of
      models.dev regenerated via an `xtask` build step into data the binary
      embeds (offline + reproducible). Fields: capabilities, context window,
      max output, reasoning support, cost, modality, and a **harness-safe**
      flag (tool-capable + deterministic enough for unattended harness use).
      Dispatch stays keyed on API protocol shape; vendors are data. Attribution
      recorded per the license verified in 00.5. (Research §5.5, §9 W3.)
- [ ] **04.2** (product-owner) Sign off the models.dev license/attribution and
      the vendoring approach before the snapshot lands in-repo. Mirrored in
      `manual-actions.md`.
- [ ] **04.3** (agent) Per-model context limits wired end-to-end: the declared
      `max_context_tokens` plumbing is populated from the catalog and consumed
      by the session budget, replacing the lone global default. (Review §5.4.)
- [ ] **04.4** (agent) Dynamic local discovery: query OpenAI-compatible
      `/v1/models` on configured local servers (LocalBox/Ollama/llama.cpp/
      vLLM) and merge into the catalog at runtime so `localpilot models` lists
      what is actually loaded. Network effect goes through the permission
      engine. (Research §5.5.)
- [ ] **04.5** (agent) Reasoning effort as a first-class control: a typed
      effort level on the request model and provider contract, mapped per
      provider (thinking budget / reasoning effort / no-op clamp for local
      models), catalog-aware clamping, switchable in the REPL, and overridable
      per harness step (e.g. high for planning, low for mechanical edits).
      Depends on 01.3's correct reasoning-stream handling. (Research §5.6.)
- [ ] **04.6** (agent) Window-relative, iterative compaction: trigger becomes
      `context_window − reserve` using real catalog windows; the previous
      summary feeds into the next compaction. Token-estimator bias is either
      corrected per provider or documented, and the user-visible context-usage
      number states its basis. (Research §5.4; review §3.3; D004.)
- [ ] **04.7** (agent) Surfacing: `doctor` and the TUI footer show model,
      context usage against the real window, and cost metadata from the
      catalog. (Research §5.11.)

## Hindsight checkpoint
> Run after all boxes in this subject are complete and before marking the
> subject `DONE` in §5 of `tasks/NextPhase-Plan.md`. Use the plan's embedded
> "Appendix: Captain Hindsight Prompt". Record the review result here. An
> interim run after a large or risky box is allowed and recorded the same way;
> it does not replace the closing run.
>
> Required output sections: Keep; Fix before closing; Record; Risk; Verdict
> (`CLOSE` or `DO NOT CLOSE`). If the verdict is `DO NOT CLOSE`, leave the
> subject open, add/reopen boxes or update decisions/lessons, and rerun this
> checkpoint after the fixes.

- [ ] Captain Hindsight review recorded
- [ ] Verdict is `CLOSE`

## Progress log
> One line per slice. Date · slice number (per-subject, starting at 1) · box
> IDs touched · what shipped · how verified · checkpoint commit/push status.
> Append a `tasks/next-phase/lessons.md` line here too whenever the slice
> taught something.
