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

- [ ] **02.1** (agent) `run_shell` approval prompts show what is being
      approved: program plus a joined args preview in the prompt detail. Tools
      provide their own detail string (registry stops key-guessing from input
      JSON); every builtin with side effects supplies one. A multi-effect call
      no longer prompts twice with empty detail. (Review §1.4, §2.4.)
- [ ] **02.2** (agent) Allowlist is floor-aware: under the relaxed profile, an
      allowlisted tool may relax `Ask` to `Allow` only for ReadOnly /
      ProjectWrite / Network command classes; Destructive, Privileged, and
      Unknown still ask (or deny per profile). Regression test: allowlisting
      `run_shell` and invoking a privileged/destructive command still asks.
      Record the test invocation in the plan's §2 Verification-commands table
      (plan-specific gate row). (Review §1.5.)
- [ ] **02.3** (agent) POSIX wrapper classification: `bash`/`sh`/`zsh`/`dash`/
      `ksh -c`, `env`-prefixed commands, and interpreter `-c`/`-e` invocations
      (python/node/perl/ruby) never classify below Unknown, on all platforms;
      docs/07 states that wrapper commands are never auto-allowed. Parity test
      mirroring the existing Windows wrapper cases. (Review §2.2.)
- [ ] **02.4** (agent) Destructive git flags escalate: `git reset --hard`,
      `git clean -f`, and `checkout`/`restore` with pathspecs classify
      Destructive, matching the purpose-built `git_restore` tool's severity, so
      `run_shell` is never a weaker gate than the equivalent builtin.
      (Review §2.3.)
- [ ] **02.5** (agent) Bypass workspace-boundary claim resolved: either scope
      the docs/comment honestly ("bypass keeps the boundary for file tools;
      commands are not path-contained") or implement a command-path containment
      story. Whichever way, the decision gets a §4 row and the doc comment, the
      docs/07 text, and the behavior agree. (Review §2.1.)
- [ ] **02.6** (agent) Reliability contract written: a spec section (owning
      docs — 06 for loop invariants, 07 for permission invariants) stating at
      minimum: (a) every `tool_use` in history is answered by a `tool_result`
      (pinned by 01.2); (b) no command reachable via `run_shell` faces a weaker
      gate than the equivalent builtin tool; (c) allowlists never lift
      Destructive/Privileged gating; (d) the persisted transcript equals the
      model-visible history (delivered by 03.5). Each invariant cites its
      enforcing test. Draft the ADR for docs/10.
- [ ] **02.7** (agent) `write_file` with `overwrite=false` refuses to clobber
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

- [ ] Captain Hindsight review recorded
- [ ] Verdict is `CLOSE`

## Progress log
> One line per slice. Date · slice number (per-subject, starting at 1) · box
> IDs touched · what shipped · how verified · checkpoint commit/push status.
> Append a `tasks/next-phase/lessons.md` line here too whenever the slice
> taught something.
