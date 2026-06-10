# LocalPilot — Full Repository Review

**Date:** 2026-06-09
**Reviewer:** automated deep review (Claude)
**Scope:** the full Rust workspace (`crates/`, 14 crates + `xtask`, ~23.4k lines of Rust including tests), the docs set (`docs/`, 23 files), CI (`.github/workflows/`), supply-chain policy (`deny.toml`, exact-pinned workspace deps), and repository hygiene. Branch reviewed: `next-phase-plan` at `28b5d3f` (clean tree, 147 commits).

---

## 1. Verdict up front

LocalPilot is the flagship of the LocalX set and it shows: this is the most disciplined repository of the four by a wide margin. The docs-first process (spec docs that own areas, ADRs that beat style rules, a checklist that tracks what is actually implemented), the three-OS CI gate with a blocking supply-chain job, the workspace-wide lint policy (`forbid(unsafe_code)`, deny `unwrap`/`expect`/`todo`/`dbg!`), and the exact-pinned dependency policy are all the kind of infrastructure most projects bolt on after their first incident, not before their first release.

The weaknesses are the inverted image of the strengths: process maturity is ahead of validation maturity. The one acknowledged release gate — a live run against a real provider — is still open; test depth is uneven across crates (the security-critical sandbox crate has the thinnest coverage relative to its risk); and v0.1.0 has never been tagged, so the release pipeline (`release.yml`, `update` command, install scripts) is plumbing that has never carried water.

---

## 2. What is good

### 2.1 Documentation architecture (best-in-class for a project this size)

- `CLAUDE.md` is a router, not a rulebook — it points at the owning spec instead of duplicating it. The 14 numbered spec docs cleanly partition the product: provenance, product spec, architecture, plan, provider contract, tools, harness, security, testing, release, ADRs, checklist, feature specs, engineering style, tooling.
- `docs/11-implementation-checklist.md` is dated, honest, and distinguishes "implemented and covered" from "needs live validation." Most projects' checklists are aspirational; this one tracks reality.
- ADR discipline (`docs/10-decisions.md`) with an explicit precedence rule ("ADRs win over style rules") prevents the common failure where a style guide silently contradicts a recorded decision.
- The clean-room provenance policy is blocking, documented first (`00-clean-room.md`), and reflected in `AGENTS.md`'s read-only behavior-reference rule. For a project that openly positions itself next to a vendor CLI, this is exactly the paper trail to have.

### 2.2 CI and supply chain

- The test job runs on Windows, Linux, and macOS with the pinned 1.82 toolchain — matching the ADR that all three are tier-1 — and covers `fmt`, `clippy -D warnings`, nextest, doctests (explicitly, because nextest skips them — a detail many CI configs miss), `cargo check`, plus a separate clippy/build pass for the feature-gated TUI so the optional surface can't rot.
- A separate blocking supply-chain job runs `cargo deny`, `cargo audit`, and `cargo machete`.
- `deny.toml` advisory ignores are individually justified with reachability analysis (e.g. RUSTSEC-2026-0009: why the `time` parsing DoS is unreachable, why the fix is blocked on MSRV, and when to revisit). This is how advisory ignores should be written and almost never are.
- Every workspace dependency is exact-pinned (`=x.y.z`) in one place; members reference `workspace = true`. No drift surface.

### 2.3 Workspace and code quality

- Crate decomposition follows dependency direction: `localpilot-core` is provider-free domain types; providers, tools, sandbox, store, harness, TUI, and the LocalMind bridge are separate crates with typed error enums at each boundary (`#[non_exhaustive]`, `thiserror`).
- `Secret` (core) is a textbook credential wrapper: redacted `Debug`/`Display`, deliberate non-implementation of `Serialize`, and a single audited `expose()` escape hatch with a comment that says exactly that.
- The retry policy (`localpilot-llm/src/retry.rs`) honours provider `retry_after`, uses capped exponential backoff with full jitter, and carries a module comment making the policy posture explicit (never retry against a provider's stated policy).
- Zero `TODO`/`FIXME` markers in `crates/`. `cargo fmt --check` passes. The 152 `unwrap`/`expect` hits are confined to test code (the workspace lints deny them on runtime paths; `clippy.toml` relaxes them in tests).
- Two protocol-distinct provider adapters (OpenAI-compatible 758 lines, Anthropic Messages 933 lines) plus a fake provider for deterministic tests — the right shape for a provider-neutral harness.
- The harness crate is the largest and best-tested (4,767 src / 1,907 test lines) and it is the differentiating feature: brief/progress parsing, rule engine, resume with bounded retry, quality-gate ratification, compaction that preserves tool-call/tool-result pairing.

### 2.4 Process tooling

- `tasks/` plan files with an explicit lifecycle (disposable, deleted before v1, never named after harness runtime files), a plan template, and recorded research notes (OpenCode comparison, code-graph design). The thinking is inspectable.
- `xtask` for repo automation, `install.ps1`/`install.sh` for both platforms, `SECURITY.md`, `CONTRIBUTING.md` with the PR provenance note.

---

## 3. What is bad

### 3.1 Test depth is uneven, and the gap is worst where risk is highest

Per-crate source vs. dedicated test volume:

| Crate | src lines | tests/ dir lines | Notes |
|---|---|---|---|
| harness | 4,767 | 1,907 | strong |
| cli | 3,392 | 583 | thin for its size |
| llm | 2,794 | 195 | most coverage is inline; stream parsing deserves more adversarial fixtures |
| tui | 1,638 | 346 | render snapshots exist |
| tools | 1,238 | 501 | reasonable |
| **sandbox** | **1,162** | **0** | inline tests in 4 files only |
| config | 893 | 477 | strong |
| localmind bridge | 650 | 0 | 1 file with inline tests |
| core | 574 | 0 | inline only (acceptable for pure types) |
| store | 543 | 0 | inline only — persistence + redaction-before-write deserves integration tests |
| skills / quota / recovery / mcp | 389/362/336/427 | 0/0/0/161 | inline only |

The sandbox crate (`command.rs` 428 lines, `permission.rs` 417 lines) is the permission boundary — command classification across POSIX/PowerShell/cmd.exe, path policy, destructive-command denial. It is exactly the code where a missed edge case is a security bug, and it has the least dedicated test surface relative to its risk. `proptest` is already a dev-dependency of this crate, which suggests property tests were intended; the follow-through matters more here than anywhere else.

### 3.2 The release pipeline has never run

Version is 0.1.0 across the workspace; no tag has ever been cut. That means `release.yml`, the `localpilot update [--check]` flow (which checks repository release tags), and the install scripts' "latest release" pathway are all untested against a real release. The first tag will be the integration test for all of them simultaneously — a classic source of broken-first-release embarrassment. The README itself says the only gate is a live provider run; the release plumbing is a second, unacknowledged gate.

### 3.3 The interactive surface is the least testable and most user-visible

`repl.rs`, `key_input.rs`, `trust.rs` are feature-gated behind `tui` and exercised by CI only as far as "clippy passes and it builds." The CHANGELOG's Unreleased section is dominated by interactive-input fixes (caret visibility, cursor editing, paste handling, truncated-stream recovery) — which is evidence this surface regresses and is being fixed by manual discovery. Snapshot tests exist for rendering (`localpilot-tui/tests/render.rs`), but the input/interaction state machine has no equivalent harness.

### 3.4 Inherited advisory exposure from the vendored LocalMind

The `time` 0.3.37 exact pin inside the LocalMind submodule forces the RUSTSEC-2026-0009 ignore in this repo's `deny.toml`. The justification is sound, but it demonstrates a structural coupling: LocalPilot's supply-chain posture is now downstream of a sibling repo that has no CI of its own (see the LocalMind review). A submodule that ships inside this product should be held to this repo's gate, and currently isn't.

---

## 4. What is missing

1. **Live provider validation** — acknowledged as the alpha gate; still open. Until one real end-to-end run against a hosted API and a local llama.cpp server is recorded, the stream-parsing and quota-classification code is validated only against fixtures written by the same authors.
2. **Coverage measurement** — no `cargo llvm-cov` (or equivalent) in CI. The unevenness in §3.1 is currently invisible to the gate; a coverage floor (even a modest one, enforced per-crate) would surface it.
3. **Sandbox integration tests** — dedicated `tests/` exercising the permission boundary end-to-end: a scripted approval run, destructive-command denial on all three shells, path-escape attempts (`..`, symlinks, UNC paths on Windows, case-insensitivity collisions).
4. **A tagged release + release-pipeline rehearsal** — cut `v0.1.0-alpha.1` (or a release-candidate tag on a fork) to exercise `release.yml`, `update --check`, and both install scripts before users do.
5. **Cross-repo contract tests** — LocalBox launches LocalPilot; LocalMind is embedded. There is no test anywhere in the four repos that exercises the boundary (e.g. LocalPilot consuming a LocalBox-served endpoint, or the localmind adapter mapping a real session bundle). Even one smoke fixture per boundary would catch drift.
6. **Performance/regression benchmarks** — no `criterion` or even a timing budget for the hot paths (compaction, stream parsing, transcript persistence). Pre-1.0 is the right time to record a baseline.
7. **Fuzzing for the stream parsers** — the OpenAI/Anthropic SSE parsers ingest untrusted bytes from the network. `cargo fuzz` targets for the two event parsers would be cheap and proportionate.

---

## 5. What can be made better

1. **Adopt `clippy::pedantic` deliberately**, as the Cargo.toml comment already plans. The codebase is clean enough that the diff will be small now; it only gets bigger.
2. **Promote the compaction pairing invariant to a property test.** "Compaction preserves tool-call/tool-result pairing" is stated in the checklist as covered; it is the perfect proptest invariant (arbitrary transcript → compact → assert pairing), and `proptest` is already in the dependency tree.
3. **Tighten the CLI crate.** At 3,392 lines `localpilot-cli/src` is the second-largest code mass and accretes a mixed bag (doctor, update, mcp, memory, learning, harness commands, REPL). Consider moving command implementations behind thin clap shims into the owning crates so the CLI crate trends toward pure wiring — it will also make the per-command logic testable without `assert_cmd`.
4. **Hold the submodule to the same gate.** Add a CI job (here or in LocalMind — ideally both) that runs the LocalMind workspace's fmt/clippy/test with this repo's toolchain. Today `exclude = ["external/localmind"]` means a broken LocalMind test suite would ship inside a green LocalPilot build.
5. **Record the live-run evidence when it happens.** The checklist items marked "needs live validation" should get a dated note (provider, model, what was exercised) so the alpha gate closes auditably, in the same style as the deny.toml justifications.
6. **README drift check against siblings.** LocalBox's README currently describes LocalPilot as "a fork of" Claude Code — directly contradicting this repo's central "not a fork, clean-room" positioning. That sentence lives in LocalBox, but the brand/legal exposure lands here. Worth a sweep of all sibling READMEs for positioning language after the rename waves (Unshackled→LocalPilot, BenchPilot→LocalBench left drift behind them).

---

## 6. Prioritized recommendations

| # | Severity | Item | Section |
|---|---|---|---|
| 1 | High | Close the live-provider gate with recorded evidence | §4.1, §5.5 |
| 2 | High | Dedicated sandbox/permission integration + property tests | §3.1, §4.3 |
| 3 | High | Rehearse the release pipeline with a real tag | §3.2, §4.4 |
| 4 | Medium | Run the LocalMind submodule's own test suite in CI | §3.4, §5.4 |
| 5 | Medium | Fix LocalBox's "fork of Claude Code" description of LocalPilot | §5.6 |
| 6 | Medium | Coverage measurement in CI with a floor | §4.2 |
| 7 | Medium | Fuzz targets for the two SSE parsers | §4.7 |
| 8 | Low | Adopt clippy pedantic now, while the diff is small | §5.1 |
| 9 | Low | Compaction pairing property test | §5.2 |
| 10 | Low | Slim the CLI crate toward pure wiring | §5.3 |

LocalPilot's engineering process is ahead of nearly every pre-release project of comparable scope. The remaining work is validation, not construction: prove the release path, prove the provider path, and put test weight where the security weight already is.
