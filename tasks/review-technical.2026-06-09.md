# LocalPilot — Technical Code & Functionality Review

**Date:** 2026-06-09
**Reviewer:** automated deep review (Claude)
**Scope:** line-level review of the load-bearing runtime code: the session loop (`localpilot-harness/src/session.rs`), compaction, the worker decision logic, both provider adapters and the shared event model (`localpilot-llm`), the tool registry and builtins (`localpilot-tools`), and the sandbox (command classifier, permission engine, path containment). This complements the repo-level review (`review.2026-06-09.md`); it is about whether the code is *correct*, not whether the repo is well-run.

Verdict up front: the architecture is right and most of the code is genuinely careful — the path containment, the permission-engine shape, the completion-marker discipline in the SSE decoders, and the anti-sunk-cost worker logic are above the bar for this category of tool. But there are five findings I'd class as **flat-out wrong** (three will produce provider 400s or corrupted output in real streaming use), and a cluster of permission-model inconsistencies that undercut the security story the docs tell.

---

## 1. Flat-out wrong

### 1.1 (Critical) Orphaned `tool_use` blocks make the next request invalid — two paths

`session.rs` persists the assistant message *including its `ToolUse` blocks* (line 516) before validating or executing the calls. Two paths then fail to ever attach a `ToolResult` for them:

- **Invalid tool calls** (`session.rs:522-526`): if `invalid_tool_calls` rejects the batch (empty id, empty name, non-object input), a corrective user message is pushed and the loop `continue`s. The assistant message with its `tool_use` blocks is already in history; no `tool_result` will ever exist for them.
- **Tool-call budget exhaustion** (`session.rs:530-531`): if `max_tool_calls` runs out midway through a call list, the remaining calls' `tool_use` blocks stay in the persisted assistant message with no results.

The Anthropic Messages API **requires** every `tool_use` block to be answered by a `tool_result` block in the immediately following user turn; the next `run_turn` (or the very next loop iteration, in the invalid-calls case) sends a history that violates this and gets a 400 `invalid_request_error`. OpenAI-compatible servers have the same rule for `tool_calls`/`role:"tool"` pairing (hosted OpenAI enforces it; llama.cpp templates vary). The compaction module goes to great lengths to never orphan a pairing — and the session loop then orphans them itself.

**Fix:** synthesize an error `ToolResult` for every unexecuted call before continuing or stopping — "tool call rejected: <reason>" / "tool budget exhausted; call not executed". That keeps the wire contract intact on every path and is also better signal for the model than a bare user message.

### 1.2 (Critical) `split_inline_thinking` is stateless across deltas — reasoning leaks into the final answer

`event.rs:40` is called per `content`/`text_delta` chunk (`openai.rs:450`, `anthropic.rs:525`) but holds no state between calls. In real streaming, a `<think>` block spans many deltas. What happens today:

- Delta 1 = `"<think>Let me look at"` → correctly routed to `ReasoningDelta`.
- Delta 2 = `"the error handling…"` → no tag found in *this* delta → emitted as **`TextDelta`** — hidden reasoning leaks into the visible final answer, into the persisted transcript, and into the next request's assistant text.
- A tag split across deltas (`"<thi"` + `"nk>"`) is never recognized at all.

The unit tests pass because they put the whole tagged block inside a single delta — the one case streaming rarely produces. The sibling LocalBox proxy solved exactly this with a stateful stripper and tag holdback; this adapter needs the same: per-stream stripper state (in `SseDecoder`, which already exists and is stateful) with holdback of a potential partial tag at chunk tails.

### 1.3 (High) UTF-8 characters split across network chunks are corrupted in both SSE decoders

`SseDecoder::push` does `String::from_utf8_lossy(bytes)` per network chunk (`openai.rs:394`, `anthropic.rs:401`). TCP/HTTP chunk boundaries do not respect UTF-8 boundaries; a multi-byte character (CJK, emoji, accented text — routine output from local models) split across two chunks decodes as U+FFFD replacement characters. The fix is mechanical: buffer raw bytes (`Vec<u8>`), and only decode complete lines — or keep the undecodable tail bytes (max 3) for the next push, the byte-level analogue of the tag holdback in 1.2.

### 1.4 (High) The approval prompt for `run_shell` shows no detail — users approve commands blind

`registry.rs:163` builds the approval prompt's `detail` from the input keys `command`, `path`, `url`, `pattern`, `query`. `RunShellInput`'s fields are `program` and `args` (`builtins.rs:568-573`). Result: the single riskiest tool in the system prompts "permission: run_shell" with an **empty** target description; the user cannot see what command they are approving. Add `program` (and a joined preview of `args`) to `target_detail` — or better, have tools provide their own detail string instead of key-guessing from JSON.

### 1.5 (High) The relaxed-profile allowlist bypasses command classification entirely

`permission.rs:124`: under `Profile::Relaxed`, a tool whose *name* is allowlisted gets `Decision::Allow` before `base_decision` ever runs. Allowlisting `run_shell` (the obvious thing a user does to stop prompt fatigue) therefore auto-approves **Destructive and Privileged** commands — `sudo`, `rm -rf`, `format` — with no prompt. The class table (`command_decision`) is dead code for allowlisted tools. The allowlist should be a *floor-aware* override: it may relax `Ask` to `Allow` for ReadOnly/ProjectWrite/Network classes, but Destructive/Privileged (and arguably Unknown) should still ask. Until then, the docs should say plainly that allowlisting `run_shell` disables command gating.

---

## 2. Permission-model inconsistencies (the story and the code disagree)

### 2.1 `bypass` does not actually keep the workspace boundary for commands

`permission.rs:114-121` (and the `Profile::Bypass` doc comment) promise that bypass "does not lift the workspace boundary." That holds only for the *file tools*, whose effects carry `inside_workspace`. `RunCommand` effects carry no path information, so under bypass any shell command — `cat /etc/shadow`, `cp ~/.ssh/id_rsa /tmp` — is `Allow`. The boundary claim is true for `read_file`/`write_file` and false for `run_shell`, which is the tool that matters. Either scope the claim in the docs/comment, or give `run_shell` a workspace-confined execution story (cwd is already the root; path-bearing args are the gap).

### 2.2 The POSIX classifier doesn't see through shell wrappers; Windows does

`classify_windows` parses `powershell`/`cmd` argument strings for destructive/privileged patterns. `classify_posix` has no equivalent: `bash -c "rm -rf /"`, `sh -c …`, `env rm -rf …`, `python -c "shutil.rmtree(...)"` all classify as `Unknown`. `Unknown → Ask/Deny` saves the default profile, but combined with 1.5 (allowlist) or bypass it's a clean bypass, and the asymmetry between platforms is unjustified. Minimum fix: classify `bash/sh/zsh/dash/ksh -c`, `env`, and interpreter `-c`/`-e` invocations as at-least-Unknown-never-ReadOnly explicitly, and document that wrapper commands are never auto-allowed.

### 2.3 Destructive git is reachable at ProjectWrite severity via `run_shell`

The builtin `git_restore` tool is classed `Destructive` (approval required). But `classify_git` classes `restore`, `reset`, `checkout` — including `git reset --hard`, which destroys uncommitted work — as `ProjectWrite`. A model can simply call `run_shell("git", ["reset", "--hard"])` and face a weaker gate than the purpose-built tool. Flags should escalate: `reset --hard`, `checkout/restore` with pathspecs, `clean -f` → `Destructive`.

### 2.4 Sequencing nit: effects are all approved before any are executed

`dispatch` resolves every effect for a call up front, which is right for one call — but multi-effect tools (e.g. `run_shell` with Network) prompt twice with the same empty detail (see 1.4). Cosmetic once 1.4 is fixed.

---

## 3. Real but lesser defects

1. **Transcript persistence diverges from what the model saw.** `REPAIR_PROMPT` and the invalid-tool-call feedback are `self.messages.push(...)` (`session.rs:489, 524`), not `self.append(...)` — they shape the conversation but are never persisted. A resumed or replayed session reconstructs a different history than the one the model actually received. If the omission is deliberate (synthetic messages), mark them; don't silently fork the two histories.
2. **Compaction cannot shrink a single oversized exchange.** `compact_with_summary` only drops whole exchanges and always keeps the last one; one huge tool result (a 64 KB capped output is ~16k estimated tokens against the 24k default budget) can exceed the limit with nothing left to drop. There is no per-tool-result truncation pass. Add one (truncate oldest tool-result *outputs* inside the kept window before giving up).
3. **Token estimation is bytes/4** (`compaction.rs:13-28`). For CJK text this over-counts ~3×; for dense code it under-counts. Combined with a fixed 24k default it means the effective window varies wildly by content. Fine as a heuristic — but it also feeds the user-visible `ContextUsage` events, so the UI shows numbers that can be far from reality. Consider a per-provider correction or at least document the bias.
4. **O(n²) work per turn.** Each loop iteration clones the full message history, re-runs compaction, and re-estimates (`session.rs:367-369`); the degenerate-output guard re-scans the whole accumulated text every 32 bytes (`is_slash_flood(&text)` on the full buffer, `session.rs:413-415`). Both are quadratic in long turns. Incrementalize the flood check (scan only the new tail with bounded look-back) and cache the compaction result until history changes.
5. **`write_file` overwrite=false is bypassed for non-UTF-8 targets.** `existing` is `read_to_string(...).ok()` (`builtins.rs:173`); a binary file yields `None`, so the "exists and overwrite is false" check never fires and the file is clobbered. Use `path.exists()` for the existence check, `read_to_string` only for newline detection.
6. **OpenAI quota header parsing won't match the real API.** `x-ratelimit-reset` is parsed as integer epoch seconds (`openai.rs:314-317`); OpenAI sends duration strings (`"1s"`, `"6m0s"`) under `x-ratelimit-reset-requests`/`-tokens`. `retry-after` as HTTP-date is also unparsed. Quota metadata will simply be absent against the hosted API — exactly the case the wait/resume feature exists for. (Live-provider validation, already the acknowledged release gate, would have caught this.)
7. **Non-standard `reasoning_content`/`reasoning_signature` keys are sent to every OpenAI-compatible endpoint** (`openai.rs:267-273`), including hosted OpenAI, which does not document them (they're a DeepSeek/vLLM convention). Some strict servers reject unknown message fields. Gate the round-trip on a provider capability flag.
8. **`max_tokens` defaults to 4096 on the Anthropic adapter** (`anthropic.rs:33`) — low for a coding agent writing files; expect routine `max_tokens` truncation warnings. Make it config-prominent or raise the default.
9. **Tool-call accumulator index defaults to 0** (`openai.rs:465`): a server that omits `index` on parallel tool calls merges all fragments into one corrupted accumulator. Rare, but worth a guard (fall back to id-keyed accumulation when ids are present).
10. **Late system messages are silently reordered by the Anthropic adapter.** `translate_messages` hoists *every* system message into the top-level system string, so a mid-conversation `seed_system` injection (the documented host pattern, `session.rs:281`) time-travels to the front of the conversation on the Anthropic wire while staying in place on the OpenAI wire. Behavioral divergence between providers for the same history.

---

## 4. What is genuinely good (verified, not assumed)

- **Path containment (`path.rs`) is the strongest file in the sandbox**: lexical `..` normalization that deliberately *preserves* escaping `..` for the containment check, canonicalization of the deepest existing ancestor (symlinks, 8.3 names, case), tail re-append for not-yet-existing files, and tests covering symlink escape and traversal. This is how it should be done.
- **Completion-marker discipline.** Both decoders refuse to treat a transport close as success: text without a terminal `finish_reason`/lifecycle yields a typed `StreamDecode` error, and the Anthropic decoder tracks open/closed content blocks so `message_stop` can't bless a truncated block. Most clients silently persist truncated output; this design (and `stream_error_stops_turn` routing decode errors into the recovery ladder rather than killing the turn) is genuinely better than the norm.
- **The recovery ladder is well-conceived**: live degenerate-output detection during streaming, post-stream detection, retry-without-tool-schemas for flood loops, health degradation with a hard stop, and persisted diagnostics — with redaction applied twice on the way out.
- **The worker/`StepLoop`** is small, pure, and exhaustively tested; verdict ranking (`block > retry > commit`), capped replans with attempt logs, and the gate refusing commits on failing tests or secret-bearing messages are all correct.
- **`dispatch` is a true single chokepoint**: every effect decided, every output redacted (including under bypass), denials returned as model-visible error results. The `Secret` wrapper, argument-list-only `run_shell` (no shell interpretation), atomic temp-file writes with newline preservation, unique-match `edit_file` semantics, and output capping at char boundaries are all the right calls.
- **Compaction's pairing invariant** (never separate a `tool_use` from its result *within* the kept window) is correctly implemented and tested — which is what makes 1.1 stand out: the loop violates the invariant the compactor protects.

---

## 5. What is missing (functionality level)

1. **Tool-results discipline on every exit path** (the 1.1 fix) plus a test asserting the invariant: *after any `run_turn` return, every `tool_use` id in history has a matching `tool_result` id*. This is a five-line property check that would have caught both paths.
2. **A stateful inline-thinking filter** shared by both adapters (the 1.2 fix), with split-tag and cross-delta fixtures ported from the LocalBox proxy's test suite (same problem, already solved in-ecosystem — clean-room rules permitting, re-derive the cases, not the code).
3. **Cancellation during tool execution.** `cancel` is checked at loop top and during streaming, but a running tool (60 s default shell timeout) cannot be interrupted; Ctrl-C waits out the child. Race tool futures against `cancel.cancelled()` and kill on cancellation.
4. **Per-model context limits.** `max_context_tokens` exists on `ProviderDeclaration` but is `None` everywhere and nothing maps model → window; the only budget is the global config default. The plumbing exists — wire it.
5. **A no-tools prompt path for weak local models.** `needs_no_tool_prompt_path: false` is declared but there's no implementation behind the capability for models that can't do native tool calls — a core scenario for the local-first audience.
6. **Streaming tool-argument display.** `incremental_tool_json: true` is declared, but `ToolCall` events are only emitted fully assembled; the UI can't show a long-running tool call forming. Minor, but the capability flag overpromises.

---

## 6. Vision assessment

The vision is the clearest of the four repos and the spec discipline shows: `01-product-spec.md` defines five concrete jobs (idea→brief, brief→plan, step execution, anti-sunk-cost recovery, bad-model recovery), explicit non-goals, and a real differentiator — *the rule-enforced harness mode* — rather than "Claude Code but local." The implementation genuinely tracks the spec: every job has a corresponding tested module.

Two vision-level critiques:

1. **The differentiator is the least-proven part.** The harness's value claim is unattended multi-step execution, which is exactly where 1.1/1.5/2.x bite hardest (no human watching the gate). The vision would be strengthened by an explicit *reliability contract* for harness mode — invariants the loop guarantees (tool pairing, no severity downgrade via `run_shell`, transcript = model-visible history) — stated in the spec and enforced by property tests. Right now the spec defines workflow, not invariants.
2. **Two memory systems, one product.** LocalPilot has its own store/redaction/memory commands *and* embeds LocalMind (with its own store/redaction/markdown memory). The product spec doesn't say which owns what long-term; redaction logic exists in both stacks with different pattern sets. Decide the convergence story (LocalMind as the only memory backend? LocalPilot store as transcript-only?) before both grow further apart.

---

## 7. Prioritized recommendations

| # | Severity | Item | Section |
|---|---|---|---|
| 1 | Critical | Synthesize `tool_result`s on invalid-call and budget-exhaustion paths; add pairing invariant test | §1.1, §5.1 |
| 2 | Critical | Stateful cross-delta `<think>` stripping in the adapters | §1.2, §5.2 |
| 3 | High | Byte-buffer the SSE decoders; never `from_utf8_lossy` mid-stream | §1.3 |
| 4 | High | Show program+args in `run_shell` approval prompts | §1.4 |
| 5 | High | Allowlist must not bypass Destructive/Privileged classes | §1.5 |
| 6 | High | Escalate destructive git flags; classify POSIX shell wrappers | §2.2, §2.3 |
| 7 | Medium | Scope or implement the bypass workspace-boundary claim for commands | §2.1 |
| 8 | Medium | Persist synthetic messages (or mark them); fix `write_file` binary overwrite; truncation pass in compaction | §3.1, §3.5, §3.2 |
| 9 | Medium | Fix OpenAI quota-header parsing; gate `reasoning_content` on capability | §3.6, §3.7 |
| 10 | Low | Cancellation through tool execution; per-model context limits; incremental flood check | §5.3, §5.4, §3.4 |

The foundations are right — chokepoint dispatch, typed effects, contained paths, honest stream termination. The critical fixes are all localized (one file each) and none require redesign. Fix the five §1 items before the first live alpha run; three of them will otherwise be discovered by the first user with a reasoning model and a multibyte locale.
