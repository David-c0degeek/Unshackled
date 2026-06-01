# 01 — Foundation: Workspace Hardening

## Goal
> Finish Phase 0 (`docs/03`) and the `docs/14` §4 repo conveniences so every
> later subject builds on a stable, lint-clean, supply-chain-gated, fully-crated
> workspace. Adds the four crates the architecture names but the workspace lacks
> (`unshackled-memory`, `unshackled-skills`, `unshackled-recovery`,
> `unshackled-quota`), centralizes lint policy, expands CI, and replaces the
> stub `doctor` with a real one. No product behaviour beyond `doctor`.

## Boxes
> ID = `01.<box-number>`. All agent-owned.

- [ ] **01.1** (agent) Add the four missing member crates —
      `crates/unshackled-memory`, `crates/unshackled-skills`,
      `crates/unshackled-recovery`, `crates/unshackled-quota` — to
      `Cargo.toml` `members`, each with a `Cargo.toml` inheriting
      `edition/license/rust-version` from the workspace and a `//!` module-doc
      `lib.rs` stating its responsibility + must-not-own (mirror `docs/02`
      §Crate Responsibilities). Dependency direction stays one-way (ADR-0001).
      (Verified: `cargo check --workspace` passes with 14 crates.)
- [ ] **01.2** (agent) Add `#![forbid(unsafe_code)]` to every library crate's
      `lib.rs` (`docs/13` §8/§12). (Verified: present in all 13 lib crates;
      `cargo check --workspace` clean.)
- [ ] **01.3** (agent) Add a workspace `[workspace.lints]` table centralizing
      the clippy policy from `docs/13` §9/§12: deny `clippy::unwrap_used`,
      `clippy::expect_used` (library crates; allow under `#[cfg(test)]`),
      `clippy::todo`, `clippy::dbg_macro` everywhere; `unsafe_code` forbidden.
      Each crate opts in with `[lints] workspace = true`. **Do NOT enable the
      `clippy::pedantic` group as a blanket gate yet** — the CI gate runs
      `-D warnings`, so pedantic-at-warn becomes a hard failure across the whole
      tree before the code that triggers it even exists. `docs/13` §9 / `docs/14`
      §4 both say to centralize pedantic "once it stabilizes"; defer it to its
      own later commit that triages and silences the noisy lints deliberately
      (record that deferral in `CHANGELOG.md` when it lands). (Verified:
      `cargo clippy --workspace --all-targets -- -D warnings` clean; the targeted
      denies fire on a planted `unwrap()` in library code.)
- [ ] **01.4** (agent) Add `.cargo/config.toml` aliases for the CI quartet so
      `cargo ci` runs `fmt --check` + `clippy --workspace --all-targets -D
      warnings` + test + `check --workspace` locally exactly as CI does
      (`docs/14` §4). (Verified: `cargo ci` runs the four steps.)
- [ ] **01.5** (agent) Add `.editorconfig` enforcing final newline, LF for
      tracked text, and `max_width`-friendly settings, supporting the
      cross-platform line-ending rule (`docs/13` §7, `docs/14` §4). (Verified:
      file present; `rustfmt.toml` `newline_style = "Auto"` unchanged.)
- [ ] **01.6** (agent) Expand `.github/workflows/ci.yml`: keep the 3-OS matrix
      and fmt/clippy/test/check; switch test to `cargo nextest run --workspace`
      with a fallback note; add a non-blocking `cargo deny check` + `cargo audit`
      job (to be promoted to blocking before release per `docs/14` §4). Pin the
      1.82 toolchain (already pinned). (Verified: CI config parses; jobs defined
      for all three OSes.)
- [ ] **01.7** (agent) Add a dependency policy note + ensure `deny.toml` covers
      `[licenses]` (present), and stub `[bans]`/`[sources]` awareness for when
      the tree grows (`docs/13` §1–2, §8). Confirm all current workspace deps are
      exact-pinned (they are) and pass `cargo deny check`. (Verified:
      `cargo deny check` exits 0.)
- [ ] **01.8** (agent) Add a git pre-commit hook (or `cargo-husky`) running
      `cargo fmt --check` and a fast `cargo clippy` so style failures are caught
      before push (`docs/14` §4). Keep it opt-in/documented so it does not block
      CI-less contributors. (Verified: hook script present; documented in
      `CONTRIBUTING.md`.)
- [ ] **01.9** (agent) Replace the stub `doctor` command with a real
      diagnostics command in `unshackled-cli`: report version, OS/platform,
      resolved config path(s), provider config status (without secrets),
      detected tool availability (e.g. ripgrep, git), and workspace trust state.
      Output goes through a command-output layer, not `println!` scattered
      (`docs/13` §11). (Verified: snapshot test of `unshackled doctor` output;
      no secrets printed.)
- [ ] **01.10** (agent) Update `CHANGELOG.md` `## Unreleased` with foundation
      changes (new crates, lint table, CI expansion) as its own note; raising
      MSRV/unpinning is never a side effect (`docs/13` §1). Confirm Phase 0
      "Done when": `cargo check --workspace` and `cargo test --workspace` pass,
      provenance docs intact. (Verified: changelog updated; both commands pass.)

## Progress log
> One line per slice. Date · slice · box IDs · what shipped · how verified.
