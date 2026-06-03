# AgentMode-Plan.md — Bridge agent mode to a full-fledged agentic CLI

> **Disposable.** This plan and its `tasks/agent-mode/` folder are deleted once
> the work ships. Shipped code, comments, tests, identifiers, and commit messages
> must not reference the plan, subjects, box IDs, decision IDs, or `slice` — see
> §6.

## 1. Subject

Bring **agent mode** (the `chat` / `print` conversational loop) up to the
practical capability of established agentic coding CLIs: a strong, original
system-prompt and tool-use scaffold; a broader, well-described tool surface;
robust handling of local and hosted providers (env-var compatibility, timeouts,
thinking/`<think>` handling); tuned context management; and a real
end-to-end evaluation against a capable model. The architecture (tool loop,
permission engine, recovery, providers) already exists — this plan closes the
**proof-and-polish** gap so a strong model reliably reads, edits, runs, and
finishes real repo tasks.

**Out of scope:** the harness (rule-enforced) mode beyond what agent mode shares;
the LocalMind learning subsystem (already integrated); multimodal/image input;
sub-agent orchestration beyond a single optional delegation primitive (§5 may
defer it); any GUI. No new operating mode.

## 2. Authoritative inputs

| Source | Contribution |
|---|---|
| `docs/01-product-spec.md`, `docs/06-harness-spec.md` | Agent-mode definition, shared loop, operating-mode boundary |
| `docs/04-provider-contract.md`, `docs/05-tool-system.md` | Provider trait + tool trait contracts the new work extends |
| `docs/00-clean-room.md`, `AGENTS.md`, ADR-0004/0005 | Clean-room provenance rules; the read-only behavior reference policy |
| `crates/unshackled-harness/src/session.rs` | The existing `run_turn` loop, recovery, compaction to extend |
| Read-only behavior reference (`AGENTS.md`-documented path) | **Behavior only.** Observable capabilities of a mature agent CLI. Not a source of prompts, code, identifiers, or UI copy. |
| Official Anthropic / OpenAI API docs | The documented env-var and request conventions to support (e.g. `ANTHROPIC_BASE_URL`) |
| `<base>` | TBD (record the branch/commit this plan branches from) |

## 3. Subject file index

| # | File | Subject |
|---|---|---|
| 00 | `tasks/agent-mode/00-tooling-research-and-readiness.md` | Tooling research and readiness |
| 01 | `tasks/agent-mode/01-system-prompt-and-loop.md` | Original agent system prompt + tool-use scaffold |
| 02 | `tasks/agent-mode/02-tool-surface.md` | Tool-surface expansion |
| 03 | `tasks/agent-mode/03-provider-runtime.md` | Provider runtime: env compat, timeouts, thinking |
| 04 | `tasks/agent-mode/04-context-management.md` | Context, compaction, and long-session handling |
| 05 | `tasks/agent-mode/05-evaluation.md` | Live validation + agentic eval suite |

## 4. Decision log

| ID | Date | Title | Decision | Rationale | Refs (box IDs / files) |
|---|---|---|---|---|---|
| D001 | | | | | |

## 5. Master progress tracker

| Done | # | File | Status | Owner summary | Human actions mirrored? |
|---|---|---|---|---|---|
| [ ] | 00 | `tasks/agent-mode/00-tooling-research-and-readiness.md` | TODO | agent: 8 | n/a |
| [ ] | 01 | `tasks/agent-mode/01-system-prompt-and-loop.md` | TODO | TBD | TBD |
| [ ] | 02 | `tasks/agent-mode/02-tool-surface.md` | TODO | TBD | TBD |
| [ ] | 03 | `tasks/agent-mode/03-provider-runtime.md` | TODO | TBD | TBD |
| [ ] | 04 | `tasks/agent-mode/04-context-management.md` | TODO | TBD | TBD |
| [ ] | 05 | `tasks/agent-mode/05-evaluation.md` | TODO | TBD | TBD |

## 6. Cross-cutting principles

> Violations are blockers, not nits.

1. KISS · YAGNI · CLEAN · SOLID · DRY (in that order when conflicting).
2. **Clean-room provenance is blocking and is the defining constraint of this
   plan.** The behavior reference is a *patched third-party product*: consult it
   only for **observable behavior**, never as a source. Do **not** copy or
   paraphrase its prompts, code, tests, identifiers, file structure, UI copy, or
   branding. Every prompt, tool, and message shipped here is written from first
   principles and original to this repo. Where the reference informed a design
   decision, record a provenance note in the Decision log naming *what behavior*
   was matched — never *what text* was read. See `docs/00-clean-room.md`.
3. **Official public APIs and local servers only.** No private or undocumented
   endpoints. Env-var/header conventions adopted must be the documented public
   ones (e.g. the Anthropic SDK's `ANTHROPIC_BASE_URL`), used as integration
   labels, not as product identity.
4. **Engineering rules from `docs/13-rust-best-practices.md` hold** — MSRV 1.82,
   exact-pinned deps, `#![forbid(unsafe_code)]`, typed per-crate errors, no
   `unwrap`/`expect`/`panic!` on library runtime paths, cross-platform discipline.
5. **Keep modules small and locally understandable.** Split a prompt/tool/loop
   file before it becomes a dumping ground; cover coordinated behavior with tests.
6. **Low cyclomatic complexity.** Prefer guard clauses, extracted decision
   helpers, and table-driven cases over deep branching.
7. **Spec at the contract level, not the SDK level.** State what must be
   observably true (the model completes the task, the tool is gated, the flood is
   aborted), not how to call a crate.
8. **Tests pin observable behavior, not implementation.** Each test prevents a
   nameable bug; offline `FakeProvider`/`wiremock` for determinism, with the live
   path validated separately in subject 05.
9. **Every box has an owner and a stable ID** (`<subject-id>.<box-number>`).
10. **Lessons land in `tasks/agent-mode/lessons.md` as they happen.**
11. **Code is plan-agnostic.** No box IDs, `D###`, `slice`, or plan references in
    shipped code, comments, identifiers, or commit messages.
12. **Captain Hindsight review before each subject close** (verdict `CLOSE`).
13. **Tooling research before implementation** — subject 00 first.
14. **Behavior parity across Windows, Linux, macOS** (ADR-0007); the interactive
    paths are validated where they can run (TUI on MSVC).

## 7. Gate review (run last; tick everything)

- [ ] All §5 subjects `DONE` (or `ABANDONED` with a §4 row)
- [ ] Subject 00 completed or waived with a §4 row
- [ ] `cargo check --workspace`, `cargo build --workspace` (3-OS via CI) pass
- [ ] `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings` clean
- [ ] `cargo test --workspace` green on Windows, Ubuntu, macOS
- [ ] `cargo deny check`, `cargo audit`, `cargo machete` pass
- [ ] §6 cross-cutting reviewed; agent-mode behavior tests hold
- [ ] Every non-abandoned subject has a recorded Captain Hindsight checkpoint `CLOSE`
- [ ] **Clean-room audit**: text scan for prohibited framing and for any prompt,
      identifier, or UI string traceable to the behavior reference; provenance
      notes present where the reference was consulted
- [ ] Shipped artefacts plan-agnostic — grep (excluding `tasks/`) for box IDs,
      `\bD\d{3}\b`, `tasks/agent-mode/`, `AgentMode-Plan.md`, `\bslices?\b`
- [ ] Commit messages plan-agnostic
- [ ] `tasks/agent-mode/manual-actions.md` — every human-owned box resolved or deferred
- [ ] **Live-provider validation passed** (subject 05): agent mode completes a
      small real repo task against a capable hosted model and a capable local model
- [ ] `tasks/agent-mode/lessons.md` reconciled; lasting lessons migrated to `tasks/lessons.md`
- [ ] Plan handed to reviewer for §8 sign-off

## 8. Acceptance / sign-off

| Date | Reviewer | Result | Notes |
|---|---|---|---|
| | | | |
