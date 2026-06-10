# 02 — Reliability Gate B: Permission Engine Hardening + Reliability Contract

## Goal
Make the permission engine's behavior match the story the docs tell, close the
classification bypasses, and write the **reliability contract** — the explicit,
property-tested invariants that turn the research doc's identity contract from
aspiration into enforced behavior. Fixes review-technical §1.4–§1.5 and
§2.1–§2.4, plus §3.5. Blocking gate (D001) together with subject 01.

## Boxes
> ID = `<subject-id>.<box-number>`. Stable — never renumber; mark retired boxes
> done and struck with `ABANDONED; see D###`, don't delete.
> Owner MUST be one of: agent · release-engineer · product-owner · tech-lead ·
> domain-sme.

- [x] **02.1** (agent) `run_shell` approval prompts show what is being
      approved: program plus a joined args preview in the prompt detail. Tools
      provide their own detail string (registry stops key-guessing from input
      JSON); every builtin with side effects supplies one. A multi-effect call
      no longer prompts twice with empty detail. (Review §1.4, §2.4.)
- [x] **02.2** (agent) Allowlist is floor-aware: under the relaxed profile, an
      allowlisted tool may relax `Ask` to `Allow` only for ReadOnly /
      ProjectWrite / Network command classes; Destructive, Privileged, and
      Unknown still ask (or deny per profile). Regression test: allowlisting
      `run_shell` and invoking a privileged/destructive command still asks.
      Record the test invocation in the plan's §2 Verification-commands table
      (plan-specific gate row). (Review §1.5.)
- [x] **02.3** (agent) POSIX wrapper classification: `bash`/`sh`/`zsh`/`dash`/
      `ksh -c`, `env`-prefixed commands, and interpreter `-c`/`-e` invocations
      (python/node/perl/ruby) never classify below Unknown, on all platforms;
      docs/07 states that wrapper commands are never auto-allowed. Parity test
      mirroring the existing Windows wrapper cases. (Review §2.2.)
- [x] **02.4** (agent) Destructive git flags escalate: `git reset --hard`,
      `git clean -f`, and `checkout`/`restore` with pathspecs classify
      Destructive, matching the purpose-built `git_restore` tool's severity, so
      `run_shell` is never a weaker gate than the equivalent builtin.
      (Review §2.3.)
- [x] **02.5** (agent) Bypass workspace-boundary claim resolved: either scope
      the docs/comment honestly ("bypass keeps the boundary for file tools;
      commands are not path-contained") or implement a command-path containment
      story. Whichever way, the decision gets a §4 row and the doc comment, the
      docs/07 text, and the behavior agree. (Review §2.1.)
- [x] **02.6** (agent) Reliability contract written: a spec section (owning
      docs — 06 for loop invariants, 07 for permission invariants) stating at
      minimum: (a) every `tool_use` in history is answered by a `tool_result`
      (pinned by 01.2); (b) no command reachable via `run_shell` faces a weaker
      gate than the equivalent builtin tool; (c) allowlists never lift
      Destructive/Privileged gating; (d) the persisted transcript equals the
      model-visible history (delivered by 03.5). Each invariant cites its
      enforcing test. Draft the ADR for docs/10.
- [x] **02.7** (agent) `write_file` with `overwrite=false` refuses to clobber
      non-UTF-8 files: existence check via the path, not via lossy read;
      `read_to_string` used only for newline detection. Regression test with a
      binary target. (Review §3.5.)
- [ ] **02.8** (product-owner) Review and approve the reliability-contract ADR
      and the 02.5 bypass-scope decision. Mirrored in `manual-actions.md`.

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

- [x] Captain Hindsight review recorded (interim — agent boxes 02.1–02.7;
      subject stays open until product-owner box 02.8 resolves)
- [ ] Verdict is `CLOSE` (pending 02.8)

**Captain Hindsight review (2026-06-10, interim after 02.1–02.7):**

1. **Keep:** Floor-aware allowlist expressed as a single pure predicate
   (`allowlist_may_relax`) instead of scattered special cases. The opaque-
   wrapper guard shared by both platform classifiers — one list, tested for
   parity on both, plus a proptest that wrappers are never read-only.
   Tool-supplied approval detail as a trait method: the registry stopped
   guessing keys, and the prompt-content test reads the actual
   `PermissionRequest` the approver sees. Reliability contract written as
   named-invariant → named-test pairs in the owning specs.
2. **Fix before closing:** the first floor implementation only relaxed `Ask`,
   which broke the ratified quality gate's headless allowance (ADR-0009
   relies on the allowlist lifting a *non-interactive* low-risk run). Fixed:
   the allowlist auto-approves relaxable effects in any interactivity; the
   risky classes keep their gate in every mode. The pre-existing
   `ratification_allowance_lets_the_gate_run_headless_but_grants_nothing_else`
   test caught it — exactly what behavior tests are for.
3. **Record:** D007 (bypass scope: documented honestly rather than
   implementing command path-containment); lessons.md entry on the
   allowlist/ratification interplay. ADR-0010 added as `proposed` — awaiting
   02.8.
4. **Risk:** wrapper and interpreter lists are deny-lists; a wrapper not on
   the list still lands at `Unknown` only because nothing else claims it.
   Acceptable: Unknown asks/denies by default, and the proptest pins the
   listed set. `git checkout <branch>` with a single pathspec-looking branch
   name still classifies ProjectWrite; conservative cases (`.`/`--`/multiple
   args) escalate, and a wrong guess costs a prompt, not an escape.
5. **Verdict:** engineering work CLOSE; subject remains open solely on the
   human approval box 02.8.

## Progress log
> One line per slice. Date · slice number (per-subject, starting at 1) · box
> IDs touched · what shipped · how verified · checkpoint commit/push status.
> Append a `tasks/next-phase/lessons.md` line here too whenever the slice
> taught something.

- 2026-06-10 · slice 1 · 02.1–02.7 · Tool-supplied approval detail
  (`Tool::approval_detail`; run_shell shows full command line); floor-aware
  allowlist with regression tests; opaque-wrapper classification on both
  platforms (incl. removing `env` from the read-only list); destructive git
  flag escalation; bypass scope documented honestly (code comment + docs/07,
  D007); reliability-contract sections in docs/06 + docs/07 with named tests;
  ADR-0010 (proposed) in docs/10; write_file binary overwrite=false fix +
  regression test. Verified: fmt/clippy/full workspace tests green (one
  intermediate failure — ratification allowance — fixed before checkpoint).
  Checkpoint: committed + pushed. 02.8 (product-owner) remains open.
