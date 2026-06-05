# Codex Review

Date: 2026-06-05

Scope: full local repository review of Unshackled, including source, docs, CI
configuration, and local verification gates.

## Summary

Implementation status: resolved on 2026-06-05. The original findings below are
kept as the review record; the resolution table and verification section at the
end describe the current state.

The normal Rust build/test gate is healthy: formatting, clippy, tests, workspace
check, and the feature-gated TUI/LocalMind build all pass locally. The main
risks are around harness safety boundaries and supply-chain hygiene:

- harness resume has effect paths that do not consistently use the permission
  engine;
- harness commit staging can include unrelated user changes;
- quota handling misses mid-stream provider errors;
- current supply-chain checks fail with an up-to-date advisory database and
  local `cargo-machete`.

## Resolution Status

| Finding | Status | Resolution |
|---|---|---|
| `harness.test_command` bypasses permissions | Resolved | Legacy test commands are synthesized into mediated quality checks. |
| Harness resume can commit unrelated dirty changes | Resolved | Resume now runs session-start preflight and stages scoped paths only. |
| Mid-stream quota/rate-limit errors do not pause cleanly | Resolved | Provider stream errors return provider-error stops and persist quota pause state. |
| `cargo audit` fails on `time 0.3.37` | Resolved with temporary ignore | `.cargo/audit.toml` contains a narrow MSRV-bound ignore for RUSTSEC-2026-0009. |
| `cargo machete` fails on LocalMind MCP | Resolved in submodule working tree | The unused `localmind-core` dependency was removed from `external/localmind`; commit and push the submodule change separately. |
| `ScriptedApprover::always()` defaults to deny | Resolved | Script exhaustion denies by default; `always()` approves consistently. |
| MCP registry rebuild leaks descriptor strings | Resolved | Dynamic tool metadata is owned by MCP tool entries and borrowed as `&str`. |
| Implementation checklist is stale | Resolved | The checklist now separates implemented, hardening, live-validation, and release-blocker state. |

## Findings

### 1. High: `harness.test_command` bypasses the permission engine

Evidence:

- `crates/unshackled-harness/src/resume.rs:165` calls `run_test_command`
  directly for configured tests.
- `crates/unshackled-harness/src/resume.rs:292` implements `run_test_command`
  with `std::process::Command`.

Impact:

A repository-controlled `.unshackled.toml` can configure `harness.test_command`
and have it execute during non-interactive harness resume without going through
command classification, approval, workspace trust, or the sandbox permission
engine. This contradicts the architecture rule that harness work must not bypass
permission checks.

Possible fix:

- Route `harness.test_command` through the same `CheckRunner` and
  `PermissionEngine` path used by ratified quality checks.
- Represent legacy `test_command` as a synthesized `CheckConfig` with a stable
  name such as `test`.
- Preserve current behavior only after explicit ratification or permission
  approval.
- Add tests proving destructive, network, and unknown `test_command` values are
  denied in non-interactive default mode.

### 2. High: harness resume can commit unrelated dirty worktree changes

Evidence:

- `crates/unshackled-harness/src/rules.rs:130` defines the
  `no_stale_uncommitted` session-start rule.
- Production code does not evaluate `Trigger::SessionStart`; search found only
  rule definitions and tests.
- `crates/unshackled-harness/src/resume.rs:232` stages with `git add -A`.

Impact:

If a user has unrelated edits before running harness resume, the harness can
stage and commit them as part of the step. This is especially risky because the
product contract emphasizes reviewable, one-step commits.

Possible fix:

- Add a preflight at the start of each harness resume step that evaluates
  `RuleEngine` with `Trigger::SessionStart`.
- Populate `RuleContext::uncommitted_unrelated` from `git status --porcelain`
  before model work begins.
- Alternatively or additionally, track changed paths from tool calls and stage
  only those paths plus `PROGRESS.md`.
- Add a regression test with a pre-existing dirty file and verify resume blocks
  before model/tool execution.

### 3. High: mid-stream quota/rate-limit errors do not pause cleanly

Evidence:

- `crates/unshackled-harness/src/session.rs:439` stores quota metadata from
  stream errors.
- `crates/unshackled-harness/src/session.rs:458` converts any stream failure
  into `BadOutputKind::MalformedStructuredOutput`.
- `crates/unshackled-harness/src/resume.rs:131` only creates a paused run when
  the turn returns `StopReason::ProviderError`.

Impact:

Provider quota or rate-limit errors can occur after an SSE stream starts. In
that case, the runtime records quota metadata but enters the bad-output recovery
path instead of returning `ProviderError`. Harness wait/resume will not engage,
and recovery retries may waste attempts or mark the model degraded.

Possible fix:

- Distinguish stream transport/provider errors from malformed model output.
- If a stream error carries quota metadata, emit `RuntimeEvent::QuotaPaused` and
  return `StopReason::ProviderError`.
- Keep malformed/incomplete stream bodies in recovery only when the provider
  error is a decode/structured-output class.
- Add a session test where a stream yields text or nothing, then returns a quota
  error, and assert that resume persists `quota-paused.json`.

### 4. Release blocker: `cargo audit` fails on `time 0.3.37`

Evidence:

- `Cargo.lock:2156` pins `time 0.3.37`.
- `cargo tree -i time@0.3.37` shows it is pulled through:
  - `external/localmind/crates/localmind-core`
  - `external/localmind/crates/localmind-store`
  - `crates/unshackled-localmind`
- Local `cargo audit` reports `RUSTSEC-2026-0009`, "Denial of Service via Stack
  Exhaustion", fixed in `time >=0.3.47`.

Impact:

The CI workflow runs `cargo audit`, so current CI can fail with an up-to-date
advisory database. This also blocks a credible alpha/release gate.

Possible fix:

- Update the LocalMind dependency chain to use `time >=0.3.47`, if compatible
  with the pinned MSRV.
- If `time >=0.3.47` requires a newer Rust than the project allows, document
  the MSRV tradeoff and either:
  - raise MSRV deliberately, or
  - remove/replace the transitive dependency path, or
  - add a narrowly scoped temporary audit ignore with a concrete rationale and
    removal condition.
- Re-run `cargo audit` and CI supply-chain job after the lockfile update.

### 5. Release blocker: `cargo machete` fails on LocalMind MCP

Evidence:

- `external/localmind/crates/localmind-mcp/Cargo.toml:12` declares
  `localmind-core`.
- Local `cargo machete` reports `localmind-core` as unused for
  `localmind-mcp`.

Impact:

The CI workflow runs `cargo machete`, so the current workspace can fail the
supply-chain job even when Rust tests pass.

Possible fix:

- Remove the unused dependency from `localmind-mcp` if it is genuinely unused.
- If it is needed through generated code or a future feature that machete cannot
  see, add a `[package.metadata.cargo-machete] ignored = ["localmind-core"]`
  entry with a short rationale.
- Prefer fixing this inside the LocalMind submodule, then update the submodule
  pointer in Unshackled.

### 6. Medium: `ScriptedApprover::always()` actually defaults to deny

Evidence:

- `crates/unshackled-sandbox/src/permission.rs:230` documents
  `ScriptedApprover::always()` as "An approver that always approves."
- `crates/unshackled-sandbox/src/permission.rs:232` implements it as
  `Self::new(Vec::new())`.
- `crates/unshackled-sandbox/src/permission.rs:247` defaults an exhausted script
  to `false`.

Impact:

Tests that intend to exercise approved `Ask` paths can silently deny instead.
This can reduce coverage of approval flows and make tests misleading.

Possible fix:

- Add an explicit default behavior field to `ScriptedApprover`, or split it into
  `ScriptedApprover` and `AlwaysApprover`.
- Make `always()` return an approver whose `approve` future always resolves to
  `true`.
- Add tests for exhausted scripted approver behavior and for `always()`.

### 7. Low: MCP tool registry rebuild leaks descriptor strings

Evidence:

- `crates/unshackled-mcp/src/client.rs:136` leaks MCP tool names with
  `Box::leak`.
- The description uses the same pattern starting at
  `crates/unshackled-mcp/src/client.rs:137`.
- `crates/unshackled-cli/src/mcp.rs` rebuilds registries for fresh runtimes,
  including per harness step.

Impact:

Long-running harness sessions with MCP tools leak a small amount of memory per
registry rebuild. It is bounded in typical use, but avoidable.

Possible fix:

- Change the `Tool` trait to return `&str` instead of `&'static str` for names
  and descriptions, or add an owned/dynamic tool-spec path.
- Store MCP names/descriptions as `String` or `Arc<str>` in `McpTool`.
- Keep builtins static while allowing dynamic tools to borrow from owned fields.

### 8. Low: implementation checklist is stale

Evidence:

- `docs/11-implementation-checklist.md` still marks many completed items as
  unchecked, including GitHub Actions, changelog, config loading, providers,
  tools, TUI, MCP, tests, and security surfaces.
- The repository contains CI workflows and the tested implementations for many
  of those items.

Impact:

The stale checklist makes release status harder to trust and conflicts with the
README's current alpha-status claims.

Possible fix:

- Reconcile `docs/11-implementation-checklist.md` against current code.
- Mark implemented items complete only where tests cover the behavior.
- Split remaining gaps into "not implemented", "implemented but needs hardening",
  and "implemented but needs live validation".
- Link release blockers to concrete issues or task-plan entries.

## Verification Results

Passed locally:

```powershell
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo check --workspace
cargo build -p unshackled --features tui,learning
cargo clippy -p unshackled --features tui,learning --all-targets -- -D warnings
cargo machete
cargo deny check
cargo audit
cargo run -p unshackled -- doctor
```

`cargo deny check` passes, but emits warnings about duplicate transitive
dependencies and wildcard path dependencies, mostly from workspace and LocalMind
path dependencies. `cargo audit` passes with accepted warnings and the documented
temporary `time` advisory ignore.

The workspace contains the implementation changes and the nested LocalMind
submodule change after the review.
