# 01 — `[[harness.checks]]` config schema + parsing

## Goal
Add the ratified quality-gate config to `unshackled-config`: a `CheckConfig`
list on `HarnessConfig`, parsing `.unshackled.toml`'s `[[harness.checks]]`, with
defaults, validation, and serde round-trip.

## Boxes

- [x] **01.1** (agent) Add `checks: Vec<CheckConfig>` to `HarnessConfig`
      (`crates/unshackled-config/src/schema.rs`), defaulting empty. `CheckConfig`
      fields (D002 — program+args, not a shell string): `name: String`,
      `program: String`, `args: Vec<String>`, `fix_program: Option<String>`,
      `fix_args: Vec<String>`, `cadence: Cadence` (`Step`/`Phase`),
      `auto_fix: AutoFix` (`No`/`Safe`/`Full`), `severity: Option<RuleSeverity>`.
- [x] **01.2** (agent) Serde: `cadence`/`auto_fix` snake_case; `auto_fix = true`
      maps to `Full`, `"safe"` to `Safe`, `false`/absent to `No`. `test_command`
      back-compat: an unset `checks` with a set `test_command` yields one
      synthesized `Phase` test check (document the equivalence).
- [x] **01.3** (agent) Validate: unique non-empty `name`; non-empty `command`;
      typed error in `unshackled-config` error enum for duplicate/empty.
- [x] **01.4** (agent) Tests: round-trip a `[[harness.checks]]` fixture
      (parse→serialize→equal); `auto_fix` bool/"safe" variants; `test_command`
      synthesis; duplicate-name rejection.

## Hindsight checkpoint
- [x] Captain Hindsight review recorded
- [x] Verdict is `CLOSE`

**Keep:** program+args modelling (D002) matches `run_shell` exactly; `AutoFix`
custom serde accepts `true`/`false`/`"safe"` and round-trips; `resolved_checks`
keeps `test_command` working without a migration.

**Fix before closing:** none. Gate green (fmt/clippy --all-targets/test
workspace).

**Record:** `from_test_command` whitespace-splits the legacy string — fine for a
back-compat shim, but real checks are structured program+args (already enforced).
No new ADR needed; ADR-0009 covers it.

**Risk:** `auto_fix` accepts a few string aliases (`"full"`, `"no"`, …) beyond
the documented `true`/`false`/`"safe"`; harmless leniency, documented in code.

**Verdict:** CLOSE.

## Progress log
- 2026-06-04 · s4 · 01.1-01.4 · added CheckConfig/Cadence/AutoFix + checks field,
  validation (InvalidCheck), resolved_checks back-compat, exports, unit + TOML
  tests · verified fmt/clippy --all-targets/test --workspace green · commit
  `eaa4729`.
