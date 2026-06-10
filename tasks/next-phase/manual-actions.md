# Manual Actions — NextPhase Plan

> Mirror of every human-owned box from the subject files (plan §5). One row per
> human action; keep in sync with the owning subject file. Status is one of
> `TODO`, `DONE`, `DEFERRED` (a deferral needs a rationale). Owner is a
> non-`agent` role from the §5 enum. M-rows are plan-level actions with no
> owning subject box.

| Box ID | Owner | Action | Source subject | Status | Deferral rationale |
|---|---|---|---|---|---|
| 02.8 | product-owner | Review and approve the reliability-contract ADR and the bypass-scope decision (02.5) | 02 | DONE | Approved 2026-06-10; ADR-0010 accepted. |
| 03.2 | product-owner | Approve the memory/store convergence ADR (LocalPilot store vs LocalMind ownership; single redaction stack) | 03 | TODO | |
| 04.2 | product-owner | Sign off models.dev license/attribution and the vendoring approach before the snapshot lands in-repo | 04 | DEFERRED | Box abandoned per D009 (local-first scope; no snapshot lands in this plan). Revisit with a follow-on hosted-catalog plan. |
| M1 | product-owner | §8 acceptance sign-off after the §7 gate passes | plan | TODO | |
| M2 | product-owner | `review-technical.2026-06-09.md` and `review.2026-06-09.md` are untracked at the repo root; commit them (suggested: move under `tasks/`) so the plan's authoritative inputs are durable across machines and resets | plan | DONE | Approved 2026-06-10; moved under `tasks/` and committed. |
| M3 | product-owner | Repo's bundled plan template (`.agents/skills/plan-large-task/plan-template.md`, 2026-06-06) predates the canonical 2026-06-09 revision (`D:\repos\c0degeek-ai\templates\plan-template.md`: risks-and-rollback, verification-commands table, depends-on column, per-subject slice numbering). Decide whether to port the update into the repo skill | plan | DONE | Approved 2026-06-10; canonical revision merged into the repo template and SKILL.md, keeping repo adaptations (cargo defaults, ADR promotion, clean-room, name-clash, tier trigger). |
