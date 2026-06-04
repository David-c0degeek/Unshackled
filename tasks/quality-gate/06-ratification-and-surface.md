# 06 — Ratification, CLI surface, security, docs/eval sync

## Goal
Close the loop the user actually touches: discovery proposes → user ratifies into
`.unshackled.toml`; non-interactive runs use only the ratified gate; results are
surfaced. Sync docs and add an eval.

## Boxes

- [x] **06.1** (agent) Ratification: `quality/ratify.rs` (`ratify_gate`,
      `render_check`, `summarize_proposal`) + CLI `harness gate propose`/`ratify`.
      Writes `[[harness.checks]]` by textual append (preserves existing config);
      re-probe adds only checks not already ratified (additions-only).
- [x] **06.2** (agent) Security: `resume` grants the gate identity a relaxed
      allowance only when ratified checks exist (D005, refined to runtime-derived);
      `resume_one_step` runs only the checks it is handed, so an unratified check
      never runs (test `an_unratified_check_never_runs`). `summarize_proposal`
      surfaces the class and warns on destructive/privileged/network. Allowance +
      scope tested (`ratification_allowance_lets_the_gate_run_headless_…`).
- [x] **06.3** (agent) Surface: `harness status` lists the ratified gate;
      `ResumeOutcome.gate` carries the deciding attempt's outcomes and the CLI
      prints a bounded per-check line (ran/pass/fail/auto-fixed).
- [~] **06.4** (product-owner) DEFERRED to `manual-actions.md`: non-interactive
      surface shipped; interactive accept/skip UX wording + default-gate sign-off
      are the product-owner's call.
- [x] **06.5** (agent) Sync: docs/06 config example rewritten to the shipped
      `program`/`args` structured form (D009) + a `harness gate` command section;
      golden eval `discovered_gate_auto_fixes_and_commits` exercises
      discovered→ratified→loaded→run→auto-fixed→commit. docs/14 §6 unchanged (it
      is the dev gate, not product commands). Full gate run.

## Hindsight checkpoint
- [x] Captain Hindsight review recorded
- [x] Verdict is `CLOSE`

**Keep:** Ratification is a thin, pure core (`ratify_gate` returns the new config
text + an added/skipped split) wrapped by a trivial CLI — easy to test, no
provider needed. Writing `[[harness.checks]]` by textual append preserves the
user's existing config and comments with zero new dependency. The D005 allowance
is *runtime-derived* (grant `quality_check` only when ratified checks exist),
which avoids a config-schema allowlist field and keeps the allowance scoped to
the gate identity — it can never authorize arbitrary shell. The golden eval
proves the whole chain end to end and is deterministic/offline.

**Fix before closing:** none. Full workspace gate green (fmt/clippy/test/check).

**Record:** D009 — docs/06's `[[harness.checks]]` example now uses the structured
`program`/`args`/`fix_program`/`fix_args` form that `gate ratify` actually emits,
replacing the `command = "…"` shorthand D002 had called presentational. The
shorthand never parsed (the loader needs `program`), so a copy-paste would fail;
the docs now match shipped reality.

**Risk:** The allowance applies under `relaxed`; under `default` a project-write
check still prompts (interactive) or is denied (non-interactive) — intended, and
the surfaced `status`/run output makes a denied check visible. The interactive
per-check ratify UX is deferred (06.4); the shipped `gate ratify` writes the full
available proposal, which is itself the explicit user act of ratifying.

**Verdict:** CLOSE.

## Progress log
- 2026-06-04 · s9 · 06.1-06.3,06.5 · ratify.rs (ratify_gate/render_check/
  summarize_proposal) + `harness gate propose|ratify`; D005 runtime allowance in
  build_runtime; status + per-check resume output; docs/06 structured example +
  gate command; golden eval discovered→ratified→run→fixed; 06.4 deferred to
  product-owner · fmt/clippy/test/check `--workspace` green · commits `411bf59`,
  `a07b29f`.
