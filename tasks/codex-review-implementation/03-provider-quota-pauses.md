# 03 - Mid-Stream Quota Pause Behavior

## Goal

Make quota and rate-limit errors that occur after a stream starts return the
same resumable provider-error path as pre-stream quota failures, instead of
being treated as malformed structured output.

## Boxes

- [x] **03.1** (agent) Trace provider stream error types, quota metadata capture,
      runtime events, bad-output classification, and resume pause persistence.
- [x] **03.2** (agent) Define the observable contract for stream errors:
      quota/rate-limit/provider transport failures return provider-error stop
      reasons, while malformed model output remains in bad-output recovery.
- [x] **03.3** (agent) Implement typed classification that preserves quota
      metadata and emits the existing quota pause event/state when a stream
      fails mid-turn.
- [x] **03.4** (agent) Ensure partial text before a quota failure is handled
      consistently and does not get committed as a successful turn.
- [x] **03.5** (agent) Add session/resume tests where a fake stream yields no
      content, partial content, then a quota error; assert resumable pause state
      and no bad-output retry churn.
- [x] **03.6** (agent) Add a malformed/decode stream regression proving real
      structured-output errors still use recovery and are not misclassified as
      quota pauses.

## Hindsight checkpoint

- [x] Captain Hindsight review recorded
- [x] Verdict is `CLOSE`

## Progress log

> One line per slice. Date - slice number - box IDs touched - what shipped -
> how verified - checkpoint commit/push status.

- 2026-06-05 - 1 - 03.1-03.6 - Changed mid-stream provider errors to stop as
  provider errors, persist quota metadata, and emit quota pause state when quota
  metadata is present; preserved stream decode failures as structured-output
  recovery. Added quota and decode regressions. Verified by focused session and
  harness tests plus the final workspace gate. Checkpoint not committed/pushed
  by agent.

## Captain Hindsight

1. Keep: Classifying provider stream failures before bad-output recovery keeps
   quota pauses resumable and avoids retry churn.
2. Fix before closing: None.
3. Record: Partial assistant text preceding a provider error is not persisted as
   a successful turn.
4. Risk: Live provider-specific quota metadata still depends on each adapter's
   error mapping, which remains covered by adapter-level contract tests rather
   than live credentials.
5. Verdict: CLOSE.
