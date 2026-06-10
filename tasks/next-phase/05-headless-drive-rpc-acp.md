# 05 — Headless Drive: RPC Over Stdio + ACP Adapter

## Goal
Expose the existing runtime to IDEs, automation, and tests without building a
server or product surface (research W2, reordered per Pi's lesson: RPC first,
HTTP later, never a product SDK). The session event types from subject 03 are
the wire format. The standards-based IDE path is an ACP (Agent Client
Protocol) adapter over the same runtime — preferred over any bespoke editor
extension. Requires subject 03 `DONE`.

## Boxes
> ID = `<subject-id>.<box-number>`. Stable — never renumber; mark retired boxes
> done and struck with `ABANDONED; see D###`, don't delete.
> Owner MUST be one of: agent · release-engineer · product-owner · tech-lead ·
> domain-sme.

- [ ] **05.1** (agent) `localpilot rpc`: newline-delimited JSON over
      stdin/stdout — typed commands in (prompt, cancel, permission reply,
      session select), streamed session events out, optional `id` for
      request/response correlation, explicit protocol version (§6.21). Framing
      contract as a hard requirement with tests: LF-only records, tolerate
      trailing CR, never split on non-LF Unicode separators (U+2028/U+2029
      inside a record stay inside it). (Research §5.2, §9 W2.)
- [ ] **05.2** (agent) Permission asks over RPC: a pending ask is surfaced as
      an event and answered by a reply command; the decision logic stays in the
      permission engine, and a non-responding client degrades exactly like
      non-interactive mode (asks denied, recorded). Harness step + permission
      state are inspectable over the protocol — that is what makes this SERVES
      tier rather than "drive a chat agent headless."
- [ ] **05.3** (agent) Input disposition: queued input is typed
      `steer` / `follow_up` / `immediate` and admitted at safe provider-turn
      boundaries (steer after the current turn's tool calls, before the next
      provider call; follow_up at idle). Available in REPL and RPC.
      (Research §5.7.)
- [ ] **05.4** (agent) Embedding surface documented: the in-process
      `SessionRuntime` is the supported library API for embedding; doc page
      with a minimal host example and the stability caveats.
- [ ] **05.5** (agent) ACP adapter over the same runtime, implemented against
      the public ACP spec (version pinned per 00.5): session lifecycle, tool
      activity, and — centrally — ACP permission requests mapped onto
      LocalPilot permission verdicts (the editor renders the prompt; LocalPilot
      owns the decision). Conformance exercised against the spec's published
      message fixtures or a minimal scripted client. Clean-room: implement from
      the spec, not from any other agent's implementation. (Research §5.2.)

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
