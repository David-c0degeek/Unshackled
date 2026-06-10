# 06 — Hook Fabric + Session Lifecycle UX

## Goal
Define the typed internal hook/event surface (the Rust analogue of an
extension event bus, research §5.1) and route LocalPilot's own cross-cutting
behaviors through it — with the permission engine as the first, always-on
hook, so extensibility *is* the safety model. On top of the subject-03 event
log, ship the session lifecycle UX (list/resume/continue/fork/tree) that makes
the audit trail something a user can pick back up (research §5.12). Third-party
plugin *packaging* is the follow-on plan (D002); this subject builds the fabric
and proves it internally. Requires subject 03 `DONE`.

## Boxes
> ID = `<subject-id>.<box-number>`. Stable — never renumber; mark retired boxes
> done and struck with `ABANDONED; see D###`, don't delete.
> Owner MUST be one of: agent · release-engineer · product-owner · tech-lead ·
> domain-sme.

- [ ] **06.1** (agent) Typed hook surface, notify-only first: lifecycle
      notifications (turn start/end, tool execution start/end, model selected,
      compaction, recovery, quota transitions) with a registration API used
      in-process. Hook activity lands in the session event log so hooks are
      auditable like everything else.
- [ ] **06.2** (agent) Permission engine as the first hook: the engine's
      verdict (`Allow`/`Ask`/`Deny`/`Block{reason}`) is delivered through the
      same hook chain every effect passes; recovery, quota, LocalMind
      injection, and the quality gate are re-routed as hooks with behavior
      pinned by existing tests (no behavior change, structural change only).
- [ ] **06.3** (agent) Mutating hooks (block/modify tool calls, rewrite
      context) exist behind the permission engine and are exercised by at
      least one internal consumer; the stance for third-party hook code is
      documented now (out-of-process or trusted-only; no in-process arbitrary
      code), so the follow-on plugin plan has a fixed boundary.
- [ ] **06.4** (agent) Session lifecycle UX on the event log: CLI
      `session list` / `session resume <id>` / `session export <id>`,
      `--continue` (most recent for this workspace) / `--resume <id>`; REPL
      `/resume` (picker), `/new`, `/fork`, `/clone`, `/tree`. Resume rebuilds
      state from the event log (resume, replay, and audit are one mechanism),
      records an open-reason, and **re-applies the current permission profile
      and trust state — never inherits stale elevated permissions** (test
      pins this). (Research §5.12.)
- [ ] **06.5** (agent) Cancellation reaches running tools: a cancel races the
      executing tool future and kills spawned child processes instead of
      waiting out their timeout; the event log records the aborted execution.
      (Review §5.3.)

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
