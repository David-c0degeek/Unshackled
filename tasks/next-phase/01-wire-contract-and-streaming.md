# 01 — Reliability Gate A: Wire-Contract And Streaming Correctness

## Goal
Make the session loop and both provider adapters honor the provider wire
contracts on every path, and make streaming decode correct for real network
conditions (multi-delta reasoning blocks, multibyte characters split across
chunks). Fixes review-technical §1.1–§1.3 and the adapter-level items in §3.
After this subject, an unattended harness run cannot 400 the provider with a
malformed history, leak hidden reasoning into the visible transcript, or
corrupt non-ASCII output. Blocking gate (D001): subjects 03–07 do not start
until this subject and subject 02 are `DONE`.

## Boxes
> ID = `<subject-id>.<box-number>`. Stable — never renumber; mark retired boxes
> done and struck with `ABANDONED; see D###`, don't delete.
> Owner MUST be one of: agent · release-engineer · product-owner · tech-lead ·
> domain-sme.

- [ ] **01.1** (agent) Every `tool_use` block persisted to history receives a
      `tool_result` on every exit path. On invalid-call rejection and on
      tool-budget exhaustion, synthesize an error `tool_result` per unexecuted
      call ("tool call rejected: <reason>" / "tool budget exhausted; call not
      executed") before continuing or stopping. (Review §1.1.)
- [ ] **01.2** (agent) Pairing-invariant property test: after any `run_turn`
      return — success, rejection, budget exhaustion, cancellation, stream
      error — every `tool_use` id in history has a matching `tool_result` id.
      Test lives with the harness tests and fails against the pre-fix loop.
      Record the test invocation in the plan's §2 Verification-commands table
      (plan-specific gate row). (Review §5.1; feeds the reliability contract
      in 02.6.)
- [ ] **01.3** (agent) Stateful inline-thinking stripper: per-stream state (in
      the existing stateful decoder) that routes `<think>`-style reasoning
      spanning many deltas to reasoning events, with holdback of potential
      partial tags at chunk tails. Shared by both adapters. Fixtures must
      include: tag split across deltas, block spanning 3+ deltas, text after
      close tag in the same delta, stream ending inside an open block.
      Re-derive cases clean-room; do not port sibling-project code.
      (Review §1.2, §5.2.)
- [ ] **01.4** (agent) Byte-buffered SSE decode: never `from_utf8_lossy` on a
      raw network chunk; buffer bytes and decode complete lines, holding back
      an undecodable tail (≤3 bytes) for the next push. Both adapters. Test
      with a multibyte character split across two pushes (CJK + emoji
      fixtures). (Review §1.3.)
- [ ] **01.5** (agent) OpenAI-compatible quota header parsing matches the real
      API: duration strings (`"1s"`, `"6m0s"`) on
      `x-ratelimit-reset-requests`/`-tokens`, and `retry-after` as either
      seconds or HTTP-date. Unparseable values degrade to absent metadata, not
      errors. (Review §3.6.)
- [ ] **01.6** (agent) Non-standard reasoning round-trip keys
      (`reasoning_content`/`reasoning_signature`) are sent only when a provider
      capability flag opts in; hosted-OpenAI-shaped endpoints get standard
      fields only. (Review §3.7.)
- [ ] **01.7** (agent) Adapter robustness pair: raise/make prominent the
      Anthropic `max_tokens` default (coding-agent appropriate, config-visible)
      and guard the tool-call accumulator against servers that omit `index` on
      parallel tool calls (fall back to id-keyed accumulation when ids are
      present). (Review §3.8, §3.9.)
- [ ] **01.8** (agent) Late system-message semantics are defined and uniform:
      decide (and document in docs/04) how a mid-conversation system injection
      is represented on each wire, so the Anthropic adapter no longer silently
      time-travels it to the front while the OpenAI adapter keeps it in place.
      Behavioral test per adapter. (Review §3.10.)

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
