# 03 — Check execution through the permission/sandbox path

## Goal
Run a ratified check command through the *existing* `run_shell`/sandbox path so
it is classified, permissioned, timed, and captured — then turn its result into
findings. No side channel (docs/05).

## Boxes

- [x] **03.1** (agent) Add a check-runner that builds a `PermissionRequest` for a
      `CheckConfig` (program+args) presenting a **distinct tool identity** (D003,
      e.g. `quality_check`), calls `classify()` + `PermissionEngine::decide`, and
      only spawns on `Allow`. Reuse classification/permission; do not re-implement.
- [x] **03.2** (agent) Capture exit code + stdout/stderr (bounded, redacted per
      the result model) into a `CheckOutcome { check, passed, findings }`.
      First-cut `findings` = exit-code + captured output (per 00.7).
- [x] **03.3** (agent) Auto-fix execution: when a check fails and `auto_fix` is
      `Full`/`Safe`, run `fix_command` (also through the permission path,
      project-write class) and re-run the check once; record both outcomes.
- [x] **03.4** (agent) Tests: a passing check → `passed`; a failing check →
      finding with captured output; fix-then-pass path; assert the command went
      through the classifier (permission decision observed), not a raw spawn.

## Hindsight checkpoint
- [x] Captain Hindsight review recorded
- [x] Verdict is `CLOSE`

**Keep:** the runner has no spawn path that skips `PermissionEngine::decide`;
`Denied` short-circuits before spawn (proven by the nonexistent-program test that
would Error if it had run). `CheckStatus` (Passed/Failed/Denied/Errored) gives
subject 05 the verdict inputs it needs. `RunResult` keeps decide+spawn separate
from fix orchestration.

**Fix before closing:** none. Workspace gate green.

**Record:** fixer result is intentionally ignored — the check re-run decides the
outcome (a fixer that "succeeds" but doesn't fix still Fails). Output is
redacted via `unshackled_config::redact` then size-bounded.

**Risk:** findings are coarse (exit + full captured output); per-tool structured
parsing is deferred (ADR-0009 first cut). The 300s default timeout may be short
for very large suites; overridable via `with_timeout`.

**Verdict:** CLOSE.

## Progress log
- 2026-06-04 · s6 · 03.1-03.4 · added CheckRunner (classify → decide → spawn),
  CheckOutcome/CheckStatus, redacted+bounded detail, auto-fix-then-re-run · cross-
  platform spawn tests (cmd/sh) · verified fmt/clippy/test --workspace green ·
  commit `9240bd5`.
