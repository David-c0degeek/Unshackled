# 05 — Session Runtime (Agent Mode) and Bad-Output Recovery

## Goal
> Phase 4 (`docs/03`) + Phase 9 (`docs/03`, `docs/12`). Build the conversational
> agent-mode loop: stream model events, execute tool calls through the
> permission engine, append results, persist the transcript, and support
> cancellation, loop limits, print mode, and context compaction. Then build the
> recovery engine that detects degraded model/backend states and recovers
> without corrupting the session. This is the shared loop both operating modes
> use (`docs/02`); harness mode (subject 06) wraps it.

## Boxes
> ID = `05.<box-number>`. All agent-owned.

- [ ] **05.1** (agent) Implement the conversation **state machine** for one
      turn (`docs/02` §Normal Chat Turn): build provider-neutral messages →
      expose allowed tool schemas → stream provider events → route tool calls
      through permission checks → append tool results → loop until `Done`.
      (Verified: integration test with the fake provider performing a file read
      then a final answer.)
- [ ] **05.2** (agent) Stream model events into **UI-agnostic** runtime events
      (text deltas, reasoning deltas as metadata not final answer, tool calls,
      usage, warnings) consumed via a channel so the TUI (subject 08) and print
      mode share one event source (`docs/13` §5 channels: `broadcast`/`watch`).
      (Verified: event-stream test asserts reasoning is tagged metadata.)
- [ ] **05.3** (agent) Execute tool calls via subject-04 registry + permission
      engine; a denied/failed tool call is represented as **data** (tool result
      with `is_error`), never a crash (`docs/05` safety invariants). (Verified:
      denied-permission path returns a model-visible error result.)
- [ ] **05.4** (agent) Persist the transcript through `unshackled-store` with
      redaction before persistence; the transcript is supporting context, not
      source of truth (ADR-0003, `docs/02`). (Verified: transcript written;
      redaction test.)
- [ ] **05.5** (agent) Implement **cancellation** (`docs/13` §5,
      `tokio_util::CancellationToken`/`select!`): user interrupt or shutdown
      stops the stream and tool execution and leaves persisted state consistent
      (no half-written files/sessions). (Verified: `docs/08` Integration test —
      cancellation during streaming/tool execution leaves consistent persisted
      state.)
- [ ] **05.6** (agent) Implement loop **limits**: max turns and max tool calls,
      configurable, enforced deterministically. (Verified: limit tests — loop
      stops at the cap with a clear status.)
- [ ] **05.7** (agent) Implement **print mode** (`docs/01` Interfaces): single
      prompt in, answer out; no workspace mutation unless explicitly enabled;
      useful in pipelines. (Verified: `assert_cmd` test — print mode emits an
      answer and makes no writes by default.)
- [ ] **05.8** (agent) Implement **agent-mode** entry wiring the loop to config
      mode (`--mode agent`, default) and permission profile
      (`--permission`/`--bypass`), with the active profile surfaced for the
      footer (`docs/06` Mode and Permission Flags). Tools still pass through the
      permission engine in agent mode (`docs/01`). (Verified: flag-parsing tests;
      agent-mode loop runs with `default` profile prompting on risky actions.)
- [ ] **05.9** (agent) Implement **context compaction** before overflow
      (`docs/03` Phase 8, `docs/08` Context): preserve tool-result pairing and,
      under harness mode, the current step contract. (Verified: `docs/08`
      Context tests — compaction preserves tool-result pairing; compaction
      preserves current step contract.)
- [x] **05.10** (agent) Implement the recovery model in `unshackled-recovery`:
      `ModelHealth` + `RecoveryAction` types; detectors for empty assistant
      turn, repeated-token loop (only after a threshold), slash flood
      (`/////////`), malformed tool call, malformed structured output, repeated
      provider transient error (`docs/03` Phase 9, `docs/12`). (Verified:
      `docs/08` Recovery tests — repeated-token loop detected only after
      threshold; malformed tool calls trigger recovery.)
- [x] **05.11** (agent) Make detection **context-aware**: slash-like / repeated
      punctuation inside fenced code, quoted logs, base64, or explicit
      user-requested output does NOT trigger recovery unless a degenerate
      threshold is exceeded (`docs/12`, `docs/11`). (Verified: `docs/08` Recovery
      tests — slash flood outside code detected; slash-like content inside fenced
      code is not.)
- [x] **05.12** (agent) Implement the recovery **ladder** (`docs/12`): abort
      stream → save diagnostic → retry once with a short repair prompt → reduce
      risky context (drop/summarize oversized tool results, lower local image
      count) → mark provider/model degraded if recovery fails → stop harness
      progress until a clean turn. Repair prompt has a **hard token/turn budget**
      (`docs/11`). (Verified: ladder test with a fake that emits a bad class then
      recovers; budget-exhaustion marks degraded.)
- [ ] **05.13** (agent) Persist recovery diagnostics and expose degraded status
      to CLI/TUI; enforce the invariant that a recovered turn may continue but a
      **bad turn may not complete a harness step** (`docs/12`, `docs/03` Phase 9
      Done-when). (Verified: `docs/08` Recovery test — exhausted recovery cannot
      complete a harness step; degraded status surfaces in `doctor`/status
      output.)

## Progress log
> One line per slice. Date · slice · box IDs · what shipped · how verified.

- 2026-06-02 · slice 1 · 05.10–05.12 · `unshackled-recovery`: `BadOutputKind`,
  `ModelHealth`, `RecoveryAction`, persistable `RecoveryDiagnostic`; context-aware
  detectors (empty turn, repeated-token loop past threshold, slash/punctuation
  flood that tolerates fenced code until a high threshold); `RecoveryEngine`
  driving the `docs/12` ladder (abort→diagnostic→repair→reduce-context→
  summarize→degraded→stop-progress) under a hard repair budget, with the
  `step_completable` invariant (a bad/unrecovered turn or a degraded model cannot
  complete a harness step). Verified: 8 tests — slash flood in/out of code,
  threshold loop, malformed-tool-call repair, exhausted-recovery degraded;
  clippy(-D)/fmt clean. (05.13 CLI/status surfacing lands with the session loop.)
