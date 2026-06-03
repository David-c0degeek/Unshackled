# 00 — Tooling Research And Readiness

## Goal
Establish repo context, baseline verification, and the clean-room ground rules
before any agent-mode work, so later subjects execute with the right context and
provenance discipline already in place.

## Boxes

- [x] **00.1** (agent) Read `AGENTS.md`, `CLAUDE.md`, `docs/00-clean-room.md`,
      ADR-0004/0005, and the read-only-reference policy; record the exact
      clean-room constraints that bind this plan (no copied prompts/code/
      identifiers/UI copy; behavior reference only; official APIs/local servers).
- [x] **00.2** (agent) Distill any needed mature-CLI observations into neutral
      behavior requirements in `tasks/agent-mode/behavior-requirements.md`.
      Prefer black-box observation; write only observable outcomes and edge
      cases. Do not transcribe, paraphrase, or derive prompts, schemas, tests,
      identifiers, user-facing text, file layout, or implementation details from
      the reference.
- [x] **00.3** (agent) Inventory the agent-mode surfaces this plan touches:
      `unshackled-harness` (`session.rs` loop, recovery, compaction),
      `unshackled-tools`, `unshackled-llm` (provider trait + adapters),
      `unshackled-cli` (chat/print). Note the existing tool list and the current
      agent system prompt (or its absence).
- [x] **00.4** (agent) Run the local gate (`fmt --check`, clippy, per-crate
      tests, check) and record the baseline green state and any windows-gnu
      caveats.
- [x] **00.5** (agent) Research current best practices for agentic tool-use
      loops, tool-result formatting, and system-prompt design from primary
      sources; record only findings that change this plan. **Do not** transcribe
      any third-party prompt.
- [x] **00.6** (agent) Confirm the documented public env-var/header conventions
      to support (Anthropic `ANTHROPIC_BASE_URL`/`ANTHROPIC_API_KEY`; OpenAI
      `OPENAI_BASE_URL`/`OPENAI_API_KEY`) with links and versions.
- [x] **00.7** (agent) Decide which assistant skills / MCP servers / local tools
      aid this work; classify `adopt`/`defer`/`reject` with rationale.
- [x] **00.8** (agent) Set up only approved tooling; keep secrets out of repo
      config.
- [x] **00.9** (agent) Bake findings into the plan (§6 principles, §7 gates,
      subject boxes, §4 decisions, `lessons.md`); end with a readiness summary.

## Hindsight checkpoint
- [x] Captain Hindsight review recorded
- [x] Verdict is `CLOSE`

## Readiness summary

- Clean-room constraints: `AGENTS.md`, `CLAUDE.md`, `docs/00-clean-room.md`,
  ADR-0004, and ADR-0005 bind the work. The local behavior reference may be used
  only as a read-only source of observable workflow behavior and high-level
  product expectations. Do not copy, translate, paraphrase, port, summarize, or
  derive prompts, code, tests, identifiers, schemas, file structure, UI copy,
  branding, private endpoint behavior, or implementation details from it.
  Provider integrations remain limited to official public APIs, local servers,
  or explicit user-owned custom endpoints.
- Primary-source research: OpenAI function-calling docs describe the loop as
  request-with-tools, receive tool call, execute application code, send tool
  output, then receive final text or more tool calls
  (`https://developers.openai.com/api/docs/guides/function-calling`). OpenAI
  eval docs call out instruction following, functional correctness, tool
  selection, and argument precision as agent eval targets
  (`https://developers.openai.com/api/docs/guides/evaluation-best-practices`).
  OpenAI reasoning guidance favors simple direct prompts, delimiters, explicit
  success criteria, and avoiding chain-of-thought prompting
  (`https://developers.openai.com/api/docs/guides/reasoning-best-practices`).
  OpenAI agent safety guidance supports structured outputs, tool approvals, and
  eval/trace review for risky tool workflows
  (`https://developers.openai.com/api/docs/guides/agent-builder-safety`).
  Anthropic tool-use docs confirm client tools use `tool_use` and `tool_result`
  content blocks and that strict tool use can guarantee schema conformance
  (`https://platform.claude.com/docs/en/agents-and-tools/tool-use/overview`).
  Anthropic stop-reason docs require explicit handling of `end_turn`,
  `max_tokens`, `tool_use`, `pause_turn`, and `refusal`, and warn not to add text
  directly after tool results
  (`https://platform.claude.com/docs/en/build-with-claude/handling-stop-reasons`).
  These findings align with the existing plan; no new prompt text, tool schema,
  or third-party wording was imported.
- Tooling classification: adopt local `cargo`, `rg`, `git`, focused unit/eval
  tests, and the already documented optional cargo hygiene tools. Defer MCP
  servers and project skills until a concrete friction point appears. Reject
  behavior-reference code inspection and any private endpoint tooling for this
  plan.
- Setup: no MCP server, assistant skill, secret, provider key, or machine-local
  config was added to the repository for readiness. Existing docs and tests are
  enough to continue.
- Plan integration: §6 already encodes clean-room, official-API, typed-tool,
  eval, and plan-agnostic-code rules; §7 already requires final gates, clean-room
  audit, live-provider validation, and manual-action closure. Remaining open
  work is now limited to provider live/gateway verification and the evaluation
  maturity/live-run track.

## Progress log
> One line per slice. Date · slice · box IDs · what shipped · how verified.

- 2026-06-03 · readiness subset · 00.2, 00.3, 00.4, 00.6 · recorded neutral behavior requirements, inventoried touched runtime/tool/provider/CLI surfaces, verified local gates, and implemented public provider env conventions · verified with focused tests, `cargo check --workspace`, `cargo fmt --check`, and clippy.
- 2026-06-03 · remaining readiness · 00.1, 00.5, 00.7-00.9 · recorded clean-room constraints, current primary-source tool-loop/eval/safety findings, tooling classification, no-secret setup note, and final readiness summary · verified by doc review and plan cross-check.
- 2026-06-03 · hindsight close · 00.1-00.9 · readiness gates are documented, neutral behavior requirements exist, approved tooling is sufficient, and no unapproved source or secret entered the repo · verdict `CLOSE`.
