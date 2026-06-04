# 02 — Toolchain profiles + discovery

## Goal
The fixed abstraction (built-in profiles) plus discovery that detects the stack,
probes available tools, and *proposes* a gate. No execution, no auto-adoption.

## Boxes

- [x] **02.1** (agent) Define the profile abstraction: a `ToolchainProfile`
      (data + small trait) declaring default checks, finding interpretation, and
      auto-fixability — original code, the stack-neutral seam (§6.6).
- [x] **02.2** (agent) Built-in **Rust** profile: fmt (step, auto_fix true),
      clippy (step, safe), test (phase), machete (phase), audit (phase, block).
      Commands are profile data, not engine literals.
- [x] **02.3** (agent) Second built-in profile to prove generality (PowerShell:
      PSScriptAnalyzer; or Node: prettier/eslint/test). Pick in 00.6; record why.
- [x] **02.4** (agent) Stack detection (marker files: `Cargo.toml`,
      `package.json`, `*.psd1`/`*.ps1`, …) + tool probing (is the tool on PATH?).
      Cross-platform PATH probe; no command executed beyond a version/help probe
      classified read-only.
- [x] **02.5** (agent) Produce a *proposed* `Vec<CheckConfig>` from detected
      profile ∩ available tools. Mark each proposed check's command class; surface
      `destructive`/`privileged`/`network` for ratification (§6.5). Tests:
      detection per marker; proposal excludes absent tools; nothing runs.

## Hindsight checkpoint
- [x] Captain Hindsight review recorded
- [x] Verdict is `CLOSE`

**Keep:** `ToolchainProfile` trait + `propose_gate_with(is_available)` seam makes
discovery deterministic without depending on the host PATH. Rust + PowerShell
profiles prove the abstraction is not Rust-shaped. Each proposal carries its
command class for ratification.

**Fix before closing:** none. Gate green workspace-wide.

**Record:** `classify()` is platform-dispatched — a test that fed it a POSIX `rm`
asserted `Destructive` but got `Unknown` on Windows. `needs_explicit_warning` is
class-only, so it is now tested against the class directly. Lesson logged for the
subject 05 cross-platform box.

**Risk:** detection is top-level marker files only (no recursive scan); a project
with sources in subdirs but no root marker is missed. Acceptable first cut;
revisit if real projects need it. PATH probing honors PATHEXT on Windows but does
not resolve shell aliases/functions (not executables anyway).

**Verdict:** CLOSE.

## Progress log
- 2026-06-04 · s5 · 02.1-02.5 · added quality module (profiles + discovery),
  Rust/PowerShell profiles, marker detection, PATH probing, class-tagged proposal
  · verified fmt/clippy --all-targets/test --workspace green · commit `493757f`.
  Lesson: classify() is OS-dispatched; cross-platform tests must not assume POSIX.
