# 04 — Model Metadata, Reasoning Effort, Honest Budgets (local-first, D009)

## Goal
**Slimmed per D009 (2026-06-10): the first instance targets the user's local
model.** No vendored hosted-model catalog; model metadata comes from provider
config and dynamic discovery of local servers. Ship: per-model context limits
wired end-to-end, `/v1/models` discovery for local servers, reasoning effort as
a typed control with a local no-op clamp, window-relative compaction, and
honest context surfacing. Closes review §5.4 (per-model context limits are
declared but unused) and addresses §3.3 (token-estimate bias is user-visible).
Requires subjects 01–03 `DONE` (D001, D004).

## Boxes
> ID = `<subject-id>.<box-number>`. Stable — never renumber; mark retired boxes
> done and struck with `ABANDONED; see D###`, don't delete.
> Owner MUST be one of: agent · release-engineer · product-owner · tech-lead ·
> domain-sme.

- [x] ~~**04.1** (agent) Generated catalog: a vendored, pinned snapshot of
      models.dev regenerated via an `xtask` build step into data the binary
      embeds (offline + reproducible).~~ ABANDONED; see D009. Local-first:
      model metadata comes from config + discovery (04.3/04.4). The 00.5
      license research stays recorded for a follow-on hosted-catalog plan.
- [x] ~~**04.2** (product-owner) Sign off the models.dev license/attribution
      and the vendoring approach before the snapshot lands in-repo.~~
      ABANDONED; see D009 (no snapshot lands in this plan).
- [ ] **04.3** (agent) Per-model context limits wired end-to-end: the declared
      `max_context_tokens` plumbing is populated from provider config (a
      `context_window` per provider/model) and from 04.4 discovery where the
      server reports it, and consumed by the session budget, replacing the
      lone global default. (Review §5.4; re-sourced per D009.)
- [ ] **04.4** (agent) Dynamic local discovery: query OpenAI-compatible
      `/v1/models` on configured local servers (Ollama/llama.cpp/vLLM/local
      gateways) and surface what is actually loaded in `localpilot models`;
      merge reported metadata (context length where present) into the
      session's model info. Network effect goes through the permission
      engine. (Research §5.5.)
- [ ] **04.5** (agent) Reasoning effort as a typed control: an effort level on
      the request model and provider contract, mapped per provider
      (reasoning-effort field where the protocol shape supports it; explicit
      no-op clamp for models without it), switchable in the REPL, and
      overridable per harness step (e.g. high for planning, low for mechanical
      edits). Depends on 01.3's correct reasoning-stream handling. (Research
      §5.6; catalog-aware clamping dropped per D009.)
- [ ] **04.6** (agent) Window-relative, iterative compaction: trigger becomes
      `context_window − reserve` using the real window from config/discovery;
      the previous summary feeds into the next compaction. Token-estimator
      bias is documented, and the user-visible context-usage number states its
      basis. (Research §5.4; review §3.3; D004.)
- [ ] **04.7** (agent) Surfacing: `doctor` and the TUI footer show model and
      context usage against the real window. (Research §5.11; cost metadata
      dropped per D009 — no catalog to source it from.)

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

- 2026-06-10 · slice 0 · 04.1, 04.2 · Slimmed to local-first per product-owner
  direction (D009): catalog vendoring and its sign-off abandoned; remaining
  boxes re-sourced from config + local discovery. No code shipped this slice.
