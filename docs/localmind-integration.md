# LocalMind Integration Contract

## Why

Learning (candidate lessons, review queues, memory promotion, retrieval, skill
generation and maintenance, audit, self-improvement) is a coherent capability
that should not be re-implemented inside every coding agent. LocalMind owns that
core as a standalone engine usable by native hosts and generic transcript
workflows.

LocalPilot is LocalMind's first native host. The LocalMind crates are bundled
into the LocalPilot binary through `localpilot-localmind`; users do not install
LocalMind separately.

## Ownership Boundary

The store split is fixed by ADR-0011: the LocalPilot store (`.localpilot/`) is
the execution record only (transcripts, the session event log, caches,
diagnostics); LocalMind (`.localmind/`) is the only memory/learning backend.
LocalPilot's redaction stack is the canonical redactor at the host boundary;
LocalMind's import redaction is defense in depth, not a second authority.

- **LocalMind core is host-neutral and must not depend on LocalPilot.** It owns
  session closeout, redaction-on-import, summarization, candidate-lesson
  extraction, the review queue, accepted-lesson persistence, Markdown-backed
  memory with a SQLite audit/search index, agent-ready context export, and
  `SKILL.md` draft emission.
- **LocalPilot owns the native host role.** It captures session evidence,
  enforces permissions and redaction before persistence, drives TUI/CLI
  surfaces, and adapts LocalPilot session records into LocalMind contracts.

## Bundling

LocalMind is vendored as a git submodule at `external/localmind` and excluded
from the LocalPilot workspace because it is its own workspace. The
`localpilot-localmind` adapter depends on `localmind-core` and `localmind-store`
by path.

```sh
git clone --recurse-submodules <repo>
git submodule update --init --recursive
```

CI checks out submodules recursively. The adapter is a one-way edge: LocalPilot
depends on LocalMind, never the reverse.

## Current Surfaces

- `localpilot-localmind::closeout_session` imports an LocalPilot transcript into
  LocalMind, extracts candidate lessons, and enqueues them for review.
- `localpilot learning` exposes the rich LocalMind loop: `closeout`, `review`,
  `promote`, `search`, `skills`, and `audit`.
- `localpilot memory` uses LocalMind accepted memory for status, inspect, search,
  delete, and context-injection disable.
- Agent turns seed relevant accepted LocalMind memory as best-effort context.
- Interactive sessions close out into LocalMind on exit, then run one bounded,
  incremental pass of the code-graph reindex (content-hash change detection;
  leftovers wait for the next close).
- `localpilot memory graph <symbol>` inspects a symbol's graph neighborhood,
  tests, and anchored lessons; `localpilot memory export <path> [--html]`
  writes a redacted, local-only snapshot of the graph (host redaction stack
  applied before write; no network).
- Promoting an accepted review item anchors the new memory to the code nodes
  its hints resolve to, so graph retrieval can surface it by structure.
- Folder ingestion writes rebuildable derived knowledge under
  `.localmind/ingest/`: manifests, redacted chunks, job state, review
  candidates, and task context packs. Normal turns may receive compact
  high-ranking ingested chunks as local context, but that context is not accepted
  memory. Promotion from ingestion enqueues LocalMind review items first.

State is project-local under `.localmind/`. Durable memory is readable Markdown;
queue, audit, search index, and the code-structure graph live in SQLite.

## Code Graph

LocalMind owns a code-structure knowledge graph (schema, tree-sitter ingestion,
persistence, traversal, ranked retrieval) populated from files the host feeds
it through the capture boundary; the engine never walks the filesystem itself.
The graph honours `.localmind.toml` `excluded_paths`, is offline and
deterministic (no model, no network in the pipeline), and joins code nodes to
accepted memory through anchor edges so retrieval traverses code and lessons
together. Reindexing is incremental and content-hash gated; removed sources
are superseded rather than deleted, so provenance and anchored knowledge
survive. The engine also exposes transport-agnostic MCP tool contracts
(`localmind-mcp`) for structural queries a host MCP server can mount.

## Signal Mapping

| LocalPilot signal | LocalMind use |
| --- | --- |
| Session transcript bundle | imported, redacted session for summarization |
| Tool events in transcript | evidence for lesson extraction |
| Code diffs and commits | future durable outcome anchors |
| Test output and quality gate results | future pass/fail signal attached to lessons |
| Recovery events | future frequent-failure candidate lessons |
| Accepted memory | LocalMind retrieval and context injection |
| Skill drafts | LocalMind disabled `SKILL.md` draft emission |

All capture stays redacted-before-persistence and inside the permission boundary;
LocalMind never bypasses either.

## Commands

```sh
localpilot learning closeout --session <id>
localpilot learning review list
localpilot learning review accept <item-id>
localpilot learning promote <item-id>
localpilot learning search "<query>"
localpilot memory inspect
localpilot memory delete <memory-id>
localpilot memory graph <symbol>
localpilot memory export graph.json
```

New rich-learning behavior lands in LocalMind, not by expanding host-local memory
implementations.
