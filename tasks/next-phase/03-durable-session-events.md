# 03 — Durable Sessions: Store Convergence + Event-Log Tree

## Goal
Replace final-message-only persistence with a durable, versioned, tree-shaped
session event log (research W1 core), after first deciding which store owns
what (the two-memory-systems problem, review §6.2). The event log is designed
as a tree (`parent_id`) from day one so the harness's anti-sunk-cost replan
loop maps onto fork/branch and becomes replayable and auditable. Also lands the
transcript-fidelity and compaction-correctness fixes that belong to the session
loop. Requires subjects 01–02 `DONE` (D001).

## Boxes
> ID = `<subject-id>.<box-number>`. Stable — never renumber; mark retired boxes
> done and struck with `ABANDONED; see D###`, don't delete.
> Owner MUST be one of: agent · release-engineer · product-owner · tech-lead ·
> domain-sme.

- [ ] **03.1** (agent) Memory/store convergence ADR drafted: decide ownership
      between the LocalPilot store and embedded LocalMind (e.g. LocalMind as
      the only memory backend; LocalPilot store as transcript/event-only), and
      a single redaction stack (today two stacks with different pattern sets).
      The event-log schema in 03.3 depends on this decision. (Review §6.2.)
- [ ] **03.2** (product-owner) Approve the convergence ADR. Mirrored in
      `manual-actions.md`.
- [ ] **03.3** (agent) Session event log in the store: typed event enum
      covering at minimum session open/close (with open reason), user input
      admitted, provider turn start/end, text/reasoning/usage deltas or
      summaries, tool call recorded / permission decision / tool execution
      start/end, recovery diagnostic, quota pause/resume, compaction, harness
      step transitions, cancellation. Every entry carries `id`, `parent_id`,
      and a session format version; loading an older version migrates on load
      (§6.21). Roundtrip + migration tests. (Research §5.3, §9 W1; survey
      Priority 2 for the event inventory.)
- [ ] **03.4** (agent) `SessionRuntime` emits events without behavior change;
      the transcript is derivable from the event log, and a derivation test
      proves rebuilt-transcript == stored-transcript for a representative
      session (tool calls, denial, recovery, compaction).
- [ ] **03.5** (agent) Transcript fidelity: synthetic conversation-shaping
      messages (repair prompts, invalid-tool-call feedback) are persisted and
      marked as synthetic, so a resumed/replayed session reconstructs exactly
      the history the model saw. Pins reliability-contract invariant (d) from
      02.6. (Review §3.1.)
- [ ] **03.6** (agent) Compaction correctness: a truncation pass shrinks
      oversized kept exchanges (oldest tool-result outputs first) before
      giving up, so one huge tool result cannot exceed the budget with nothing
      left to drop; the degenerate-output flood check scans only the new tail
      with bounded look-back; the per-iteration compaction result is cached
      until history changes. (Review §3.2, §3.4; trigger math stays in subject
      04 per D004.)
- [ ] **03.7** (agent) Replan-as-branch: a discarded harness step attempt
      closes its branch with a structured branch summary; a replan forks from
      the last good step. One structured summary format shared between
      compaction and branch summaries. Demonstrated by a harness test that
      replays a replanned run from the event log. (Research §5.3, §5.4.)

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
