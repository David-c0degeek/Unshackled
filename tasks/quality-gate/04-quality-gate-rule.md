# 04 — `quality_gate` rule, `PhaseComplete` trigger, cadence dispatch

## Goal
Wire the gate into the deterministic rule engine: a `quality_gate` rule that runs
the right checks at the right cadence and maps outcomes to verdicts. Generalize
the relationship to `suite_green`.

## Boxes

- [x] **04.1** (agent) Add `Trigger::PhaseComplete` to `rules.rs` (the enum is
      `#[non_exhaustive]`); `Step` checks evaluate on `StepComplete`, `Phase`
      checks on `PhaseComplete`.
- [x] **04.2** (agent) Extend `RuleContext` with the gate outcomes for the
      current trigger (e.g. `gate_outcomes: Vec<CheckOutcome>`), set by the loop
      before evaluation. Keep `RuleContext` cheap/cloneable.
- [x] **04.3** (agent) Implement the `quality_gate` rule: critical, default
      `Block`; per-check `severity` override; a failed check with no fix maps to
      the act-on-findings verdict (subject 05 supplies the mapping helper). Keep
      `suite_green` as the named `test` check for back-compat.
- [x] **04.4** (agent) Register in `RuleEngine::with_baseline`; tests: step-cadence
      check fires on `StepComplete` not `PhaseComplete` and vice versa; per-check
      `severity` override respected; critical clamp (cannot be `Off`).

## Hindsight checkpoint
- [x] Captain Hindsight review recorded
- [x] Verdict is `CLOSE`

**Keep:** the verdict mapping (pass→Allow, denied/errored→Block, failed→
severity: block→Block, warn→Warn, none→Retry) cleanly separates the actionable
(Retry) path from the human-decision (Block) path the spec wants. Rule-severity
ceiling gives a soft-gate option. `trigger_for_cadence` is the dispatch the loop
needs.

**Fix before closing:** none. Workspace gate green (56 lib tests).

**Record:** D007 — the finding→verdict mapping (`gate_verdict`) landed in subject
04, earlier than the plan put it (05.1), because the rule cannot compile without
it. `CheckOutcome` gained a `severity` field (carried from the check) so the rule
can apply per-check overrides. Subject 05 now only wires the loop + DECISIONS.md
on top of the rule's Retry/Block verdicts, not the mapping itself.

**Risk:** `RuleSeverity` has no `Retry`, so the actionable default is encoded as
"severity None on the check" rather than a severity value — slightly implicit.
Documented in `gate_verdict`. The loop (05) must populate `gate_outcomes` per
cadence using `trigger_for_cadence`.

**Verdict:** CLOSE.

## Progress log
- 2026-06-04 · s7 · 04.1-04.4 · added PhaseComplete trigger, gate_outcomes ctx
  field, quality_gate critical rule + gate_verdict mapping, trigger_for_cadence;
  carried severity on CheckOutcome (D007) · fmt/clippy/test --workspace green ·
  commit `3b93748`.
