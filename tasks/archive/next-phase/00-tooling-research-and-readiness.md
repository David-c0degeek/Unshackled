# 00 — Tooling Research And Readiness

## Goal
Research the project stack, current best practices, official documentation,
assistant skills, MCPs, and local tooling before implementation starts. Convert
the findings into concrete plan rules, gates, enabled tools, and references so
later subjects execute with the right context already in place. This subject
absorbs the research doc's **W0** spikes: models.dev licensing (for 04), JSONL/
ACP protocol sources (for 05), and hook-isolation options (for 06).

## Boxes
> Subject 00 is required unless explicitly waived in the plan's §4 Decision log.
> No implementation subject starts until this subject is `DONE`, `ABANDONED`,
> or waived by decision.

- [x] **00.1** (agent) Read repo instructions and authoritative docs (AGENTS.md,
      CLAUDE.md, docs/00, 04, 05, 06, 07, 10, 13); list applicable constraints,
      clean-room/provenance rules, and existing conventions that bind subjects
      01–07. Record in this file.
- [x] **00.2** (agent) Inventory the crate graph, workspace deps, CI, existing
      commands, and the surfaces this plan touches (`localpilot-harness`
      session/compaction, `localpilot-llm` adapters/event model,
      `localpilot-sandbox` classifier/permission/path, `localpilot-tools`
      registry/builtins, `localpilot-store`, `localpilot-cli`/`-tui`,
      LocalMind embedding). Confirm the review's file/line references still
      match the current tree; note drift.
- [x] **00.3** (agent) Run the baseline gate and confirm or correct the plan's
      §2 Verification-commands table against the real repo, or record exact
      blockers and the command output needed to reproduce them.
- [x] **00.4** (agent) Research current Rust best practices for the surfaces
      this plan touches (async stream state machines, property testing —
      e.g. proptest vs quickcheck for the pairing invariant, JSONL framing,
      SQLite/append-log storage patterns); record only findings that affect
      this plan.
- [x] **00.5** (agent) Research external dependencies from official/primary
      sources and record links, versions, licenses, and date-sensitive notes:
      **models.dev** dataset license + schema (gates 04.1/04.2), **ACP (Agent
      Client Protocol)** spec location/version (gates 05.5), **agentskills.io**
      spec (gates 07.4), OpenAI/Anthropic streaming + rate-limit-header docs
      (gates 01.3–01.5). Provider work needs official-docs provenance.
- [x] **00.6** (agent) Review applicable repo skills (`clean-room-guard`,
      `add-provider`, `add-tool`, `implement-harness-step`,
      `write-golden-eval`) and any candidate MCP servers/tools; classify each
      `adopt`/`defer`/`reject` with rationale, trust notes, source URL,
      permissions, and setup cost.
- [x] **00.7** (agent) Set up only approved local tooling/config needed before
      coding; record install/config/provenance and keep security-sensitive
      permissions out of repo config unless narrowly justified.
- [x] **00.8** (agent) Bake adopted findings into the plan: update the §2
      Verification-commands table, §6 principles, §7 gates, subject boxes, §4
      decisions, and `tasks/next-phase/lessons.md`. Research that changes no
      plan artifact is not complete; bake in the finding or record why it was
      rejected/deferred. End with an implementation-readiness summary.

## Findings

### 00.1 — Binding constraints (recorded)

- **Clean-room (docs/00, ADR-0004/0005, blocking):** all code, prompts, tests,
  identifiers, UI copy original to this repo. Official public APIs or local
  servers only. OpenCode/Pi/local reference are read-only behavior references —
  no copied code, prompts, identifiers, JSON shapes, package structure.
  PR provenance note required when a behavior reference informed a change.
- **Engineering (docs/13):** MSRV 1.82, edition 2021, exact-pinned workspace
  deps, typed `thiserror` errors per crate boundary, `#![forbid(unsafe_code)]`,
  clippy denies `unwrap_used`/`expect_used`/`todo`/`dbg_macro` (workspace
  lints), no lock across `.await`, cancellation first-class
  (`CancellationToken`), fakes over mock frameworks, deterministic offline
  tests (`tempfile`, `wiremock`), live tests behind `LOCALPILOT_LIVE_TESTS`,
  `insta` snapshots, proptest for high-risk parsers/classifiers.
- **Tier-1 parity (ADR-0007):** Windows/Linux/macOS equal; per-OS shell and
  path policy; a single-OS box is not done.
- **Provider contract (docs/04):** capability-driven branching (never provider
  names), `ModelEvent` model, quota semantics honoring documented retry
  contracts, required offline test classes (translation, stream parsing,
  error classification, quota metadata, capabilities, redaction).
- **Tool system (docs/05):** `dispatch` is the only side-effect path; neither
  model nor harness can bypass the permission engine; outputs redacted before
  storage.
- **Security (docs/07):** command class table with interactive/non-interactive
  defaults; profiles `default`/`relaxed`/`bypass`; bypass never default, does
  not silently lift redaction/logging/workspace boundary; subjects 01/02 spec
  text lands here (permission half) and docs/06 (loop half).
- **ADR home (docs/10):** durable decisions promoted there; this plan owes at
  least three ADRs (reliability contract, memory convergence, bypass scope).

### 00.2 — Inventory and review-reference drift check

- Workspace: 14 crates (`cli`, `core`, `config`, `llm`, `tools`, `harness`,
  `tui`, `store`, `sandbox`, `mcp`, `skills`, `recovery`, `quota`,
  `localmind`); `external/localmind` is a vendored submodule excluded from the
  workspace. Lint policy is centralized in `[workspace.lints]`.
- **Every review file/line reference verified against the current tree — no
  drift.** Spot checks: `session.rs:516` (assistant message persisted with
  `ToolUse` blocks), `session.rs:522-526`/`530-531` (invalid-call and budget
  exit paths orphaning tool_use), `session.rs:489/524` (unpersisted
  `messages.push`), `event.rs:40` (stateless `split_inline_thinking`),
  `openai.rs:394`/`anthropic.rs:401` (`from_utf8_lossy` per chunk),
  `openai.rs:314-317` (integer-epoch quota parse), `openai.rs:267-273`
  (unconditional `reasoning_content` round-trip), `openai.rs:465` (index
  default 0), `anthropic.rs:33` (`DEFAULT_MAX_TOKENS = 4096`),
  `registry.rs:163` (`target_detail` key-guessing misses `program`/`args`),
  `builtins.rs:568-573` (`RunShellInput`), `builtins.rs:173` (lossy-read
  existence check), `permission.rs:124` (allowlist pre-empts class table).
- **One review correction (worse than reported):** review §2.2 says
  `env rm -rf …` classifies `Unknown` on POSIX. Actually `env` is in
  `is_read_only_program` (`command.rs`), so `env <anything>` classifies
  **ReadOnly → auto-Allow** on every profile. Box 02.3 must remove `env` from
  the read-only list in addition to wrapper classification.

### 00.3 — Baseline gate (2026-06-10)

All four §2 commands pass on the current tree: `cargo fmt --check` (clean),
`cargo clippy --workspace --all-targets -- -D warnings` (clean),
`cargo test --workspace` (exit 0), `cargo check --workspace` (clean).
Tooling present: rustc 1.82.0, cargo-machete 0.7.0, cargo-deny 0.19.6,
cargo-audit 0.22.1, cargo-nextest 0.9.92. §2 Verification-commands table is
correct as written; plan-specific gate rows await 01.2/02.2 test invocations.

### 00.4 — Rust-practice findings that affect this plan

- **Property testing:** `proptest =1.5.0` is already a pinned workspace dep and
  already used in `localpilot-sandbox` (`command.rs` totality test). Use
  proptest for the pairing invariant (01.2) and classifier floors (02.2/02.3);
  no new dependency.
- **Stream state machines:** both adapters already own per-stream stateful
  `SseDecoder` structs — the right home for byte buffering (01.4) and the
  think-tag stripper state (01.3). The shared stripper should be a struct with
  per-stream state in `localpilot-llm` (e.g. alongside `event.rs`), replacing
  the stateless `split_inline_thinking` free function.
- **UTF-8 holdback:** buffer raw `Vec<u8>`; after draining complete lines, a
  trailing incomplete UTF-8 sequence is ≤3 bytes (4-byte max code point), so a
  3-byte holdback is provably sufficient.
- **JSONL framing (for 03/05):** one serde_json value per `\n`-terminated
  line; internally tagged enums (`"type"` field) plus an explicit integer
  format-version field from day one (plan §6.21). Accept `\r\n` on read.

### 00.5 — External dependencies (official/primary sources, fetched 2026-06-10)

- **models.dev** — repo <https://github.com/anomalyco/models.dev>, **MIT
  license**. API: <https://models.dev/api.json> (full provider+model data),
  `models.json` (model metadata only), `catalog.json` (combined). Per-model
  fields: capabilities (reasoning, tool call, attachment, structured output,
  temperature), costs per Mtok (input/output/reasoning/cache read/write),
  limits (context window, max input, max output), metadata (release date,
  knowledge cutoff, open weights, modalities). Data maintained as TOML in-repo
  by community. MIT permits vendoring a snapshot with license/attribution
  preserved → unblocks 04.1; product-owner sign-off (04.2) still required.
- **ACP (Agent Client Protocol)** — spec <https://agentclientprotocol.com>,
  repo <https://github.com/agentclientprotocol/agent-client-protocol>.
  JSON-RPC; stable **protocol version 1** (integer), negotiated via
  `protocolVersion` during `initialize`. Public spec → implementing against it
  is clean-room-safe (plan §6.19). Gates 05.5.
- **agentskills.io** — spec <https://agentskills.io/specification>, repo
  <https://github.com/agentskills/agentskills>. A skill is a folder with
  `SKILL.md`: YAML frontmatter (required `name` ≤64 chars
  lowercase/digits/hyphens, `description`; optional `license`, `metadata`),
  free-form markdown body; optional `scripts/`, `references/`, `assets/` dirs;
  progressive disclosure (discovery → activation → execution). Gates 07.4.
- **OpenAI rate-limit headers** — official guide
  <https://developers.openai.com/api/docs/guides/rate-limits>:
  `x-ratelimit-{limit,remaining,reset}-{requests,tokens}`; **reset values are
  Go-style duration strings** (`"6s"`, `"1m30s"`, `"6m0s"`); `retry-after` is
  seconds (HTTP-date also legal per HTTP spec). Confirms review §3.6; gates
  01.5.
- **Anthropic rate-limit headers** — official doc
  <https://platform.claude.com/docs/en/api/rate-limits>: `retry-after` in
  seconds; `anthropic-ratelimit-{requests,tokens,input-tokens,output-tokens}-
  {limit,remaining,reset}`, with `-reset` values in **RFC 3339** format.
  Current `anthropic.rs` leaves `reset_at: None` — 01.5 should also parse the
  RFC 3339 reset on the Anthropic side, not only fix the OpenAI duration
  strings.
- **Streaming docs:** both adapters already carry provenance headers citing
  the public OpenAI/Anthropic API references; 01.3/01.4 changes stay within
  those documented surfaces.

### 00.6 — Skills / MCP classification

| Item | Verdict | Rationale |
|---|---|---|
| `clean-room-guard` (repo skill) | **adopt** | Mandatory before consulting the behavior reference, writing prompts/identifiers/UI copy, or opening PRs. Zero setup. |
| `implement-harness-step` (repo skill) | **adopt** | Harness loop changes in 01, 03, 06. |
| `add-tool` (repo skill) | **adopt** | Tool upgrades in 07 (and 02.1 detail-string change touches every builtin). |
| `add-provider` (repo skill) | **defer** | No new provider adapter in this plan; catalog (04) extends existing declarations. |
| `write-golden-eval` (repo skill) | **defer** | Eval-suite authoring out of this plan's scope. |
| New MCP servers/tools | **reject** | All work is local; no external MCP needed. Adding one would expand the trust surface for no box. |

### 00.7 — Tooling setup

No-op: every required tool is already installed and pinned (see 00.3); no new
config or security-sensitive permissions needed. Web research used built-in
fetch/search only.

### 00.8 — Baked into the plan

- §2 Verification-commands table confirmed correct; plan-specific gate rows
  remain TBD until 01.2/02.2 land their test invocations (as designed).
- Lessons entry added (`lessons.md`): review §2.2 understated the POSIX gap —
  `env` is classified ReadOnly, not Unknown; box 02.3 scope includes removing
  `env` from the read-only list.
- Note recorded against 01.5 (above): parse Anthropic RFC 3339 reset headers
  too, since the official doc confirms the format and `reset_at` is currently
  always `None` on that adapter.
- No §6/§7 changes needed: existing principles already cover proptest use,
  format versioning, and clean-room handling of public specs.

**Implementation-readiness summary:** gate green, review references verified
current, all external-source licensing/format questions answered from primary
sources, tooling installed. No blockers. Subjects 01 and 02 may start.

## Hindsight checkpoint
> Run after all boxes in this subject are complete and before marking the
> subject `DONE` in §5 of `tasks/NextPhase-Plan.md`. Use the plan's embedded
> "Appendix: Captain Hindsight Prompt". Record the review result here. An
> interim run after a large or risky box is allowed and recorded the same way;
> it does not replace the closing run.
>
> Required output sections: Keep; Fix before closing; Record; Risk; Verdict
> (`CLOSE` or `DO NOT CLOSE`). If the verdict is `DO NOT CLOSE`, leave the
> subject open, add/reopen boxes or update decisions/lessons, and rerun this
> checkpoint after the fixes.

- [x] Captain Hindsight review recorded
- [x] Verdict is `CLOSE`

**Captain Hindsight review (2026-06-10):**

1. **Keep:** Verifying every review line reference before trusting the defect
   inventory (caught the `env` underreport). Primary-source license/format
   research with URLs recorded (models.dev MIT, ACP v1, OpenAI duration
   strings, Anthropic RFC 3339) — each gates a concrete later box. Confirming
   proptest is already pinned avoids a needless dependency decision.
2. **Fix before closing:** none — gate green, findings recorded in-plan.
3. **Record:** the `env` classification correction (recorded in lessons.md and
   02.3 scope); the Anthropic reset_at note (recorded against 01.5). Neither
   is ADR-weight.
4. **Risk:** hook-isolation options (mentioned in the subject goal for 06)
   were not deeply researched — subject 06 is an *internal* hook fabric per
   the plan §1 scope, so isolation research is deferred to that subject's
   first box rather than blocking the reliability gate.
5. **Verdict:** CLOSE.

## Progress log
> One line per slice. Date · slice number (per-subject, starting at 1) · box
> IDs touched · what shipped · how verified · checkpoint commit/push status.
> Append a `tasks/next-phase/lessons.md` line here too whenever the slice
> taught something.

- 2026-06-10 · slice 1 · 00.1–00.8 · docs/constraints recorded; crate + review
  reference inventory (no drift; one correction); baseline gate run green;
  external-source research (models.dev/ACP/agentskills/rate-limit headers)
  recorded with URLs; skills classified; findings baked into plan + lessons.
  Verified: fmt/clippy/test/check all pass. Checkpoint: committed and pushed
  with the plan files.
