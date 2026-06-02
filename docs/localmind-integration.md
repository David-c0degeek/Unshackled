# LocalMind integration contract (forward-looking)

> Status: forward integration contract for the next development track. It records
> how Unshackled's learning surfaces evolve toward **LocalMind**, the extracted,
> host-neutral local learning engine. It changes no shipped behavior in this
> alpha and does not reopen completed work; it is the handoff that the next track
> implements.

## Why

Learning (candidate lessons, review queues, memory promotion, retrieval, skill
generation and maintenance, audit, self-improvement) is a coherent capability
that should not be re-implemented inside every coding agent. LocalMind owns that
core as a standalone engine usable by Claude Code, OpenAI Codex, and generic
transcript workflows. **Unshackled is LocalMind's first native host**: the
learning feature is built into the Unshackled binary and its commands — users do
not install LocalMind separately.

## What ships in this alpha (bridge surfaces)

The current crates are deliberately small **alpha bridge surfaces**, not the
final learning system:

- `unshackled-memory` — a flat, redacted, inspectable memory store with
  relevance-ranked retrieval, a token cap/threshold, and `memory` commands.
- `unshackled-skills` — a skill manifest/loader plus disabled suggestion drafts
  with a per-pattern cooldown.

These should be **wrapped or replaced by LocalMind core through an adapter**, not
expanded into a separate rich learning system.

## Ownership boundary

- **LocalMind core is host-neutral and must not depend on Unshackled.** It owns
  session closeout, redaction-on-import, summarization, candidate-lesson
  extraction, the terminal review queue, accepted-lesson persistence
  (Markdown-backed memory with a SQLite audit trail), search, agent-ready context
  export, and `SKILL.md` draft emission.
- **Unshackled owns the native host role**: capturing the evidence below,
  enforcing permissions and redaction at capture time, the TUI/footer/review
  surfaces, and the built-in commands. Unshackled may depend on LocalMind core
  through an adapter crate (a future `unshackled-localmind`); LocalMind core
  never depends back.

## Signal mapping (Unshackled → LocalMind core)

| Unshackled signal (already produced) | Where today | LocalMind core input |
| --- | --- | --- |
| Session transcript bundle | `unshackled-store` JSONL + `export_session` | imported, redacted session for summarization |
| Tool events (name, args, result, error) | `RuntimeEvent::ToolStarted/Finished`, `ContentBlock::ToolUse/ToolResult` | per-session tool trace for lesson extraction |
| Code diffs | per-step `git` commits (harness `resume`) | change evidence for a candidate lesson |
| Test output | `suite_green` / `test_command` result in the completion gate | pass/fail signal attached to a lesson |
| Commits | `harness: <step>` per-step commits, `StepTrace` | durable outcome anchor (commit hash) |
| Recovery events | `unshackled-recovery::RecoveryDiagnostic` | "frequent failure + fix" candidate lessons |
| Memory retrieval | `unshackled-memory::retrieve` | replaced by LocalMind retrieval/export |
| Review queue | (bridge: drafts created disabled) | LocalMind terminal review queue |
| Skill drafts | `unshackled-skills::SkillDraft` (disabled, cooldown) | LocalMind `SKILL.md` draft emission |

All capture stays redacted-before-persistence and inside the permission boundary;
LocalMind never bypasses either.

## Migration shape (next track, not this alpha)

1. Add a host-neutral `localmind-core` (its own repo/crate) implementing the core
   above; it depends on nothing from Unshackled.
2. Add an `unshackled-localmind` adapter that feeds the signals in the table into
   `localmind-core` and renders its review queue / memory in the existing TUI and
   `memory` / `skill` commands.
3. Reduce `unshackled-memory` / `unshackled-skills` to thin shims over the adapter
   (or remove them) once parity is reached — keeping the feature built-in.
4. No separate install: `localmind-core` is bundled into the Unshackled binary.

New rich-learning behavior (closeout, review, promotion, self-improvement) lands
in LocalMind, not by expanding these crates: learning is a host-neutral concern
that LocalMind owns.
