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

- [x] **05.1** (agent) `localpilot rpc`: newline-delimited JSON over
      stdin/stdout — typed commands in (prompt, cancel, permission reply,
      session select), streamed session events out, optional `id` for
      request/response correlation, explicit protocol version (§6.21). Framing
      contract as a hard requirement with tests: LF-only records, tolerate
      trailing CR, never split on non-LF Unicode separators (U+2028/U+2029
      inside a record stay inside it). (Research §5.2, §9 W2.)
- [x] **05.2** (agent) Permission asks over RPC: a pending ask is surfaced as
      an event and answered by a reply command; the decision logic stays in the
      permission engine, and a non-responding client degrades exactly like
      non-interactive mode (asks denied, recorded). Harness step + permission
      state are inspectable over the protocol — that is what makes this SERVES
      tier rather than "drive a chat agent headless."
- [x] **05.3** (agent) Input disposition: queued input is typed
      `steer` / `follow_up` / `immediate` and admitted at safe provider-turn
      boundaries (steer after the current turn's tool calls, before the next
      provider call; follow_up at idle). Available in REPL and RPC.
      (Research §5.7.)
- [x] **05.4** (agent) Embedding surface documented: the in-process
      `SessionRuntime` is the supported library API for embedding; doc page
      with a minimal host example and the stability caveats.
- [x] **05.5** (agent) ACP adapter over the same runtime, implemented against
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

- [x] Captain Hindsight review recorded
- [x] Verdict is `CLOSE`

**Captain Hindsight review (2026-06-10):**

1. **Keep:** one byte-level `LineFraming` shared by both stdio protocols —
   the LF-only/U+2028 contract is structural (no multi-byte UTF-8 sequence
   contains 0x0A) and pinned by tests. One `RpcApprover` serving both wires:
   the ACP adapter and the native protocol are thin views over the same
   ask registry, so the deny-on-silence semantics cannot diverge. The serve
   loops drive turns to completion in an inner select (the REPL's own
   pattern) instead of fighting the borrow checker with stored futures.
   Steering implemented in the runtime (`SteerQueue` drained at the safe
   boundary), so REPL, RPC, and ACP all get identical admission semantics.
2. **Fix before closing:** none — gate green; native protocol covered by 5
   duplex tests, ACP by 3 scripted-client conformance tests including the
   permission round trip and cancellation.
3. **Record:** ACP wire shapes were verified against the published docs
   (`sessionUpdate` discriminator, `session/request_permission`
   options/outcome shapes, protocolVersion negotiation) before
   implementation; provenance noted in the module header. Bounded stops
   (max turns/tool calls) map to ACP `end_turn` — the closest value in the
   published stop-reason vocabulary; revisit if a finer-grained reason is
   adopted by the spec.
4. **Risk:** the ACP adapter implements the core lifecycle (initialize,
   session/new, session/prompt, session/update, session/request_permission,
   session/cancel) and declines optional capabilities (loadSession, fs,
   images); a real editor integration may exercise spec corners the
   scripted client does not. The native RPC `status` reads PROGRESS.md
   directly rather than through the harness parser — fine for inspection,
   not authoritative.
5. **Verdict:** CLOSE.

## Progress log
> One line per slice. Date · slice number (per-subject, starting at 1) · box
> IDs touched · what shipped · how verified · checkpoint commit/push status.
> Append a `tasks/next-phase/lessons.md` line here too whenever the slice
> taught something.
- 2026-06-10 · slice 1 · 05.1–05.5 · New `localpilot-rpc` crate: versioned
  LF-framed JSON protocol (hello/prompt/cancel/permission_reply/status/
  shutdown in; streamed session events out) with byte-level framing tests
  (LF-only, trailing CR, U+2028/2029, split chunks); permission asks over the
  wire via `RpcApprover` (deny on disconnect/timeout, registry-resolved
  replies, outstanding asks inspectable via `status`); typed input
  dispositions (immediate/steer/follow_up) backed by a runtime `SteerQueue`
  admitted at safe provider-turn boundaries, wired into the REPL
  (submit-during-turn queues steering) and both wire protocols;
  `docs/embedding.md` documents the in-process embedding surface with a
  minimal host and stability caveats; ACP adapter (`localpilot acp`)
  implementing initialize/session lifecycle/streamed updates/permission
  requests/cancel against the published spec, exercised by a scripted
  client. Verified: fmt/clippy (both feature sets)/full workspace tests
  green. Checkpoint: committed + pushed.
