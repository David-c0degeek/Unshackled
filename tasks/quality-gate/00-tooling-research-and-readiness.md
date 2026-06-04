# 00 — Tooling Research And Readiness

## Goal
Map the real seams the quality gate plugs into (rule engine, config schema,
sandbox/permission path, run_shell, harness loop), run the baseline gate, and
record the profile/finding-parsing strategy so subjects 01-06 execute against
known structures.

## Boxes

- [x] **00.1** (agent) Read repo instructions + ADR-0009 and the spec sections
      (docs/06 §Quality Gate / §Rule Engine, docs/05 §quality-gate checks, docs/07
      §Discovered Tooling). Constraints recorded in the plan §2/§6.
- [x] **00.2** (agent) Inventory crates touched: `unshackled-config` (schema),
      `unshackled-harness` (rules + loop: worker/resume/session), `unshackled-tools`
      (run_shell builtins/registry), `unshackled-sandbox` (command/permission/path).
- [x] **00.3** (agent) Baseline gate now GREEN. Was RED (pre-existing clippy
      failure in `unshackled-config/tests/config.rs`); fixed under D006 by handling
      the helper Results properly (commit `b6b7791`). `fmt`/`clippy --all-targets`/
      `test`/`check` all pass.
- [x] **00.4** (agent) Execution path: `run_shell` (builtins.rs) runs program+args
      with **no shell**, `classify()`→`CommandClass`→`Effect::RunCommand` via
      `Tool::effects()`; `PermissionEngine::decide` (permission.rs) maps effect→
      decision. A check must build a `PermissionRequest` and route through
      `decide` — there is no spawn that skips it. See D002/D003.
- [x] **00.5** (agent) `step_complete` fires in the rule engine on
      `Trigger::StepComplete`. No explicit `phase` boundary exists yet — "phase"
      must be introduced (group of PROGRESS steps / milestone). Exact seam scoped
      to subject 04 (read `worker.rs`/`resume.rs` there); not a blocker for 01-03.
- [x] **00.6** (agent) Placement: profiles + discovery + gate-runner live in
      `unshackled-harness`, reusing `unshackled-sandbox` (`classify`,
      `PermissionEngine`) and the spawn pattern from `unshackled-tools` run_shell.
      See D004.
- [x] **00.7** (agent) Finding parse: first cut = exit-code + bounded/redacted
      stdout+stderr as one finding; structured per-tool parsing deferred. Rust
      auto_fix: fmt=`true`, clippy=`safe`, others `false`.
- [x] **00.8** (agent) Findings baked into D002-D006 and subjects 01/03.
      **Readiness summary:** baseline green; gate-runner + profiles + discovery go
      in `unshackled-harness` (D004); checks are program+args presenting a distinct
      tool identity (D002/D003) routed through `classify`+`PermissionEngine::decide`;
      ratification grants that identity an allowance so `ProjectWrite` cargo checks
      run headless (D005); findings are exit-code + bounded output first cut.
      Subjects 01-03 are unblocked; the phase-boundary seam is scoped to 04.

## Hindsight checkpoint
- [x] Captain Hindsight review recorded
- [x] Verdict is `CLOSE`

**Keep:** the program+args / distinct-tool-identity / ratification-allowance
findings (D002/D003/D005) are load-bearing and were caught before any code —
exactly the point of subject 00. Module placement (D004) avoids a needless crate.

**Fix before closing:** none blocking. The baseline blocker (D006) is resolved
and committed.

**Record:** D002-D006 captured the non-obvious seams; lessons.md mirrors them.
The phase-boundary definition is an open design question carried into subject 04.

**Risk:** "phase" has no engine concept yet; subject 04 must define it without
overcomplicating the loop. Per-tool finding parsing is deferred — the first cut
may surface noisy output; acceptable for now.

**Verdict:** CLOSE.

## Progress log
- 2026-06-04 · s1 · 00.1 00.2 · read ADR-0009 + specs, mapped crates/seams from
  rules.rs and config/schema.rs · verified by reading source · baseline gate
  launched in background, scaffolding committed next.
- 2026-06-04 · s2 · 00.3 00.4 00.6 00.7 · read command.rs/permission.rs/
  builtins.rs; recorded execution path, classification, module placement;
  ran baseline gate (clippy red, pre-existing — D006) · verified by source +
  gate output · checkpoint commit `384b4b1`.
- 2026-06-04 · s3 · 00.3 00.5 00.8 · fixed baseline (D006, commit `b6b7791`),
  verified green gate (fmt/clippy --all-targets/test), wrote readiness summary +
  Hindsight · subject 00 CLOSE.
