# 02 — Tool-Surface Expansion

## Goal
Broaden the builtin tool set to what real coding tasks need, with original
implementations, generated schemas, permission gating, and redaction. Each tool
must be discoverable from its schema and safe by construction.

## Boxes

- [x] **02.1** (agent) Add a precise multi-edit / patch-apply tool that applies
      several scoped edits to one file (or a unified-diff-style patch) atomically,
      with clear failure when context does not match. Permission-gated as a
      workspace write. Artefact: apply/partial-failure tests.
- [x] **02.2** (agent) Add a glob/find tool (filename pattern listing) distinct
      from content `search_text`, respecting the workspace boundary and ignore
      rules. Artefact: a glob test.
- [x] **02.3** (agent) Broaden git tooling for the agent loop (diff, log, add,
      restore/checkout of a path) as classified commands through the permission
      engine; destructive ones require approval. Artefact: classification +
      gating tests.
- [x] **02.4** (agent) Decide on a network fetch tool (read a URL) — `adopt` or
      `defer` via a §4 decision. If adopted: official HTTP only, gated as a
      network effect, output redacted and size-capped; off unless enabled.
      Artefact: gating test or a decision-log deferral.
- [x] **02.5** (agent) Decide on a single optional sub-task delegation primitive
      (run a focused sub-prompt with a tool subset) — `adopt` or `defer` via §4.
      Keep it minimal; full sub-agent orchestration is out of scope (§1).
- [x] **02.6** (agent) Ensure every new tool has a typed input schema, a clear
      model-facing description, declared effects, and redacted output; update the
      builtin-count and schema-stability tests. Artefact: registry tests green.

## Hindsight checkpoint
- [x] Captain Hindsight review recorded
- [x] Verdict is `CLOSE`

## Progress log
> One line per slice. Date · slice · box IDs · what shipped · how verified.

- 2026-06-03 · tool surface · 02.1-02.6 · added multi-edit, file finding, git diff/log/add/restore, stable schemas, and deferred network/delegation scope · verified with `cargo test -p unshackled-tools --test tools`.
- 2026-06-03 · Captain Hindsight · 02 · CLOSE: added tools are permission-gated and schema-covered; network fetch and delegation are intentionally deferred by decision log.
