# 00 — Tooling Research And Readiness

## Goal
Establish repo context, baseline verification, and the clean-room ground rules
before any agent-mode work, so later subjects execute with the right context and
provenance discipline already in place.

## Boxes

- [ ] **00.1** (agent) Read `AGENTS.md`, `CLAUDE.md`, `docs/00-clean-room.md`,
      ADR-0004/0005, and the read-only-reference policy; record the exact
      clean-room constraints that bind this plan (no copied prompts/code/
      identifiers/UI copy; behavior reference only; official APIs/local servers).
- [ ] **00.2** (agent) Distill any needed mature-CLI observations into neutral
      behavior requirements in `tasks/agent-mode/behavior-requirements.md`.
      Prefer black-box observation; write only observable outcomes and edge
      cases. Do not transcribe, paraphrase, or derive prompts, schemas, tests,
      identifiers, user-facing text, file layout, or implementation details from
      the reference.
- [ ] **00.3** (agent) Inventory the agent-mode surfaces this plan touches:
      `unshackled-harness` (`session.rs` loop, recovery, compaction),
      `unshackled-tools`, `unshackled-llm` (provider trait + adapters),
      `unshackled-cli` (chat/print). Note the existing tool list and the current
      agent system prompt (or its absence).
- [ ] **00.4** (agent) Run the local gate (`fmt --check`, clippy, per-crate
      tests, check) and record the baseline green state and any windows-gnu
      caveats.
- [ ] **00.5** (agent) Research current best practices for agentic tool-use
      loops, tool-result formatting, and system-prompt design from primary
      sources; record only findings that change this plan. **Do not** transcribe
      any third-party prompt.
- [ ] **00.6** (agent) Confirm the documented public env-var/header conventions
      to support (Anthropic `ANTHROPIC_BASE_URL`/`ANTHROPIC_API_KEY`; OpenAI
      `OPENAI_BASE_URL`/`OPENAI_API_KEY`) with links and versions.
- [ ] **00.7** (agent) Decide which assistant skills / MCP servers / local tools
      aid this work; classify `adopt`/`defer`/`reject` with rationale.
- [ ] **00.8** (agent) Set up only approved tooling; keep secrets out of repo
      config.
- [ ] **00.9** (agent) Bake findings into the plan (§6 principles, §7 gates,
      subject boxes, §4 decisions, `lessons.md`); end with a readiness summary.

## Hindsight checkpoint
- [ ] Captain Hindsight review recorded
- [ ] Verdict is `CLOSE`

## Progress log
> One line per slice. Date · slice · box IDs · what shipped · how verified.
