# 07 — Tool/Skills Upgrades + Supply-Chain Posture

## Goal
Land the NEUTRAL-tier table stakes and the SERVES-tier skills alignment from
research W4, plus the W6 supply-chain posture: `apply_patch`, bounded tool
output with spill-to-id, a context-excludable shell transcript role,
agentskills.io-aligned skill loading, prompt templates, and dependency-policy
hardening. Everything here routes through the (now hardened) permission engine.
Requires subject 02 `DONE` (D001).

## Boxes
> ID = `<subject-id>.<box-number>`. Stable — never renumber; mark retired boxes
> done and struck with `ABANDONED; see D###`, don't delete.
> Owner MUST be one of: agent · release-engineer · product-owner · tech-lead ·
> domain-sme.

- [ ] **07.1** (agent) `apply_patch` tool: strict, original patch grammar;
      workspace-boundary-checked writes through the existing path containment;
      diff preview in interactive approval; rejected hunks fail the call with a
      model-actionable error. (Research §8; survey Priority 6.)
- [ ] **07.2** (agent) Tool-output bounding: cap with head+tail retention and
      spill of the full output to the existing retention store
      (`put_tool_output`), referenced by opaque id; a companion read-back tool
      fetches retained output by id. Caps respect char boundaries (consistent
      with existing output capping). (Research §5.11; survey Priority 6.)
- [ ] **07.3** (agent) Context-excludable shell role: a first-class transcript
      role for user-initiated shell runs with an `exclude_from_context` option,
      so a user can run commands without polluting the model's context. Runs
      still pass the permission engine and land in the session event log.
      (Research §5.11.)
- [ ] **07.4** (agent) agentskills.io-aligned skill loading: load standard
      `SKILL.md` skills (including cross-harness skill directories), with
      project-local skills behind the existing trust gate; LocalMind remains
      the authoring/review path that promotes a reviewed lesson into a standard
      skill file. (Research §5.8; spec verified in 00.5.)
- [ ] **07.5** (agent) Prompt templates: parameterized reusable prompts, user-
      and project-scoped, project ones trust-gated. Deliberately small —
      distinct from skills. (Research §5.8.)
- [ ] **07.6** (agent) Supply-chain posture: a minimum-release-age policy for
      newly adopted crate versions (CI check or `cargo deny` policy), a CI
      guard on unexpected `Cargo.lock` changes, a documented build-script
      (`build.rs`) audit rule for new deps, and a clean-environment install
      smoke test of the produced binary. (Research §5.9, §9 W6.)

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
