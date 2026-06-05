# 01 - Permission-Mediated Harness Checks

## Goal

Ensure repository-controlled `harness.test_command` cannot execute during
resume without the same permission, approval, trust, and sandbox mediation used
by ratified quality checks.

## Boxes

- [x] **01.1** (agent) Trace the current `harness.test_command`, quality
      discovery, `CheckRunner`, `PermissionEngine`, and sandbox call paths;
      document the minimal owning modules and the current trust boundary.
- [x] **01.2** (agent) Replace the direct `std::process::Command` execution
      path with a synthesized quality check or equivalent mediated command
      path, preserving existing user-facing semantics for safe configured test
      commands.
- [x] **01.3** (agent) Ensure legacy `harness.test_command` is represented with
      a stable check identity and participates in approval/non-interactive
      denial behavior consistently with ratified checks.
- [x] **01.4** (agent) Add regression tests proving destructive, network, and
      unknown configured commands are denied in non-interactive default mode
      before execution.
- [x] **01.5** (agent) Add a positive regression test proving a ratified safe
      test command still runs through the mediated path and reports failures in
      the existing harness style.
- [x] **01.6** (agent) Run focused package tests for harness, sandbox, config,
      and quality modules, then the checkpoint gate subset needed for this
      security surface.
- [x] **01.7** (tech-lead) Review whether the final design changes ADR-0009 or
      only implements it; promote an ADR update only if the contract changes.

## Hindsight checkpoint

- [x] Captain Hindsight review recorded
- [x] Verdict is `CLOSE`

## Progress log

> One line per slice. Date - slice number - box IDs touched - what shipped -
> how verified - checkpoint commit/push status.

- 2026-06-05 - 1 - 01.1-01.7 - Replaced direct legacy
  `harness.test_command` execution with a synthesized mediated check named
  `test`, preserving the legacy config surface while routing through
  `CheckRunner`, command classification, permission policy, and sandbox
  execution. Added destructive/network/unknown denial regressions and a positive
  mediated legacy command test. Verified by focused harness tests and the final
  workspace gate. Checkpoint not committed/pushed by agent.

## Captain Hindsight

1. Keep: Synthesizing a normal check kept the change small and reused the
   ratified-check permission path.
2. Fix before closing: None.
3. Record: ADR-0009 did not need an update; the implementation now matches its
   existing rule that harness checks run through permission and sandbox paths.
4. Risk: Legacy `test_command` remains supported for compatibility, but users
   should prefer explicit `[[harness.checks]]` for new configs.
5. Verdict: CLOSE.
