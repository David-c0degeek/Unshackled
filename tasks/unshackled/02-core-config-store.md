# 02 — Core Domain, Config, Store

## Goal
> Phase 1 (`docs/03`) plus the store foundation. Make `unshackled-core` a
> complete provider-neutral, UI-neutral domain layer; make `unshackled-config`
> load config with deterministic precedence and redaction; make
> `unshackled-store` persist transcripts as inspectable plain files with atomic
> writes. No HTTP, no terminal UI, no provider names beyond generic enum
> variants (`docs/02` §`unshackled-core`). These three crates underpin every
> later subject.

## Boxes
> ID = `02.<box-number>`. All agent-owned.

- [ ] **02.1** (agent) Expand `unshackled-core` domain types with **newtypes**
      (`docs/13` §3): `SessionId`, `TurnId`, `ToolUseId` (and `MessageId` if
      needed) wrapping `Uuid`, each distinct so they cannot be confused.
      (Verified: unit test that the IDs are distinct types; `Debug` derived.)
- [ ] **02.2** (agent) Complete the message/content model: `Message` with
      `role`, `content: Vec<ContentBlock>`, and **metadata**; `ContentBlock`
      enum covering Text, Reasoning (text + optional signature + provider
      metadata, per `docs/04`), ToolUse, ToolResult. Mark growable public enums
      `#[non_exhaustive]`. (Verified: serde round-trip test for each variant.)
- [ ] **02.3** (agent) Add normalized `ToolCall`/`ToolResult` model (id, name,
      JSON input, result text, error flag) per `docs/02` §Tool Calls, decoupled
      from any provider format. (Verified: round-trip + error-flag test.)
- [ ] **02.4** (agent) Add a usage-accounting model (input/output tokens, and
      room for tokens/sec + cost estimate the TUI footer needs, `docs/12`).
      (Verified: serde round-trip; arithmetic uses checked/saturating ops where
      it touches untrusted numbers, `docs/13` §12.)
- [ ] **02.5** (agent) Add a structured error hierarchy: one `thiserror` enum
      per crate boundary (`docs/02` §Error Handling, `docs/13` §4). Replace the
      placeholder `UnshackledError::Message(String)` in core with a real typed
      `CoreError` (or remove it if core needs none). `#[non_exhaustive]`; sources
      via `#[from]`. (Verified: each enum compiles; no `anyhow` in a library
      public signature.)
- [ ] **02.6** (agent) Add a **secret wrapper** type in `core` (or `config`)
      whose `Debug`/`Display` prints `***`, raw value only via `expose()`
      (`docs/13` §8, `docs/07`). (Verified: test asserts `format!("{:?}")` and
      `Display` never contain the secret.)
- [ ] **02.7** (agent) Implement the full config schema in `unshackled-config`
      reflecting `docs/06`/`docs/04`: `[provider]` + `[providers.*]`,
      `[harness]` (mode, attempts_per_step, auto_commit, test_command,
      `[harness.rules]`), `[permissions] profile`, `[quota]`. Use `figment`
      (already a dep) for layering. (Verified: a representative `.unshackled.toml`
      deserializes; unknown-but-namespaced provider options preserved.)
- [ ] **02.8** (agent) Implement config precedence (`docs/02` §`unshackled-config`):
      CLI flags > env vars > project `.unshackled.toml` > user config > built-in
      defaults. Implement user-config-directory resolution (cross-platform, no
      hardcoded paths, `docs/13` §7) and project-config resolution. (Verified:
      precedence is deterministic — see 02.10.)
- [ ] **02.9** (agent) Implement env-var override mapping and redaction helpers
      in config; api keys come from env (`SECURITY.md`, `docs/04` config
      example), wrapped in the secret type. (Verified: env override test; key
      never appears in debug output.)
- [ ] **02.10** (agent) Add the `docs/08` "Required MVP Tests / Config":
      default config loads; project overrides user; env overrides project; CLI
      overrides env; secrets redacted in debug output. Use snapshot tests
      (`insta`) for precedence outcomes and proptest for precedence invariants
      (`docs/13` §10). Invalid config produces precise diagnostics naming the
      offending key/section. (Verified: all five tests pass; an invalid-config
      test asserts the diagnostic text.)
- [ ] **02.11** (agent) Implement `unshackled-store` transcript persistence:
      inspectable plain files (JSONL or similar), a session index, atomic writes
      (temp-then-rename so an interrupted write leaves no corrupt session,
      `docs/13` §5), and redaction applied **before** persistence (`docs/07`,
      `docs/13` §8). (Verified: `docs/08` Store tests — write/read round trip;
      interrupted write leaves no corrupt session; redaction before persistence.)
- [ ] **02.12** (agent) Implement a **reusable secret-detection** primitive
      (best-effort, `docs/07` Secret Redaction, `docs/11` Security
      "Implement secret detection") as a shared function/helper: detect API
      keys, bearer tokens, private keys, passwords, cloud credentials, and
      connection strings with credentials. It is the single detector that store
      redaction (02.11), tool-output redaction (04.11), logging (`docs/13` §11),
      and memory writes (07.9) all call — not re-implemented per crate. Doc
      states detection is best-effort and inspect/delete is the backstop
      (`docs/07`). Lives with the config redaction helpers (`docs/02`
      §`unshackled-config`). (Verified: unit tests detect each secret class and
      pass clean text through; a redaction-applies-everywhere test references
      this detector from store + tool paths.)
- [ ] **02.13** (agent) Implement the store **export** command/path (`docs/11`
      Store "Implement export command", `docs/07` Telemetry
      "user-exported debug bundles after review"): export a session/transcript
      as an inspectable bundle, redacted before export (calls 02.12). (Verified:
      `assert_cmd` export test — bundle written; redaction applied; no secret in
      output.)
- [ ] **02.14** (agent) Implement the broader `.unshackled/` runtime-state
      persistence the store owns (`docs/01` §`.unshackled/`, `docs/02`
      §`unshackled-store`, `docs/11` Store): file-backed cache, tool-output
      snapshots, and provider-metadata persistence — all under the ignored
      `.unshackled/` dir, inspectable plain files where possible, atomic writes
      (temp-then-rename), redacted before persistence. (Memory store, skill
      drafts, and quota wait records are persisted by their owning subjects
      07.15 using this store layer.) (Verified: round-trip + atomic-write +
      redaction tests for cache, tool-output snapshot, and provider metadata.)
- [ ] **02.15** (agent) Confirm Phase 1 "Done when": config precedence
      deterministic, invalid config has precise diagnostics, `unshackled-core`
      has no provider dependencies (grep its `Cargo.toml`). Document each crate
      with a `//!` responsibility doc and `///` on public items with `# Errors`
      (`docs/13` §13). (Verified: dependency check passes; doc-build clean.)

## Progress log
> One line per slice. Date · slice · box IDs · what shipped · how verified.
