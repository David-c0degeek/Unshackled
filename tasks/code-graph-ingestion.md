# Code-Graph Ingestion: Design and Clean-Room Plan

Date: 2026-06-08

A standalone, source-level design study for giving LocalMind a **code-structure
knowledge graph** — built by reading the workspace's files, docs, and folders —
and joining it to LocalMind's existing learned-knowledge graph so retrieval can
traverse *code* and *lessons* together.

It is self-contained: no external document is required to read or act on it. It
is a **design/decision doc**, not an execution plan. When a workstream below is
ready to build, copy `tasks/plan-template.md` to `tasks/CodeGraph-Plan.md` and
run it through the `plan-large-task` ceremony (this is a new-crate, multi-crate,
multi-session change — it earns the full template).

The concrete trigger: graphify (`github.com/safishamsi/graphify`) turns
codebases into queryable knowledge graphs. It is open source and inspectable,
so it is a **behavior/concept reference** for this work — studied, not copied.
The provenance rules are in §2.

---

## 0. North Star fit — does this earn a place?

This work is filtered through the identity contract in
`tasks/localpilot-next-phase-research.md` §0 ("accountable autonomy"), using its
decision filter:

> **Does this make the agent more trustworthy to run unsupervised?**

**Verdict: SERVES.** A code-structure graph attacks the single most expensive
agent failure mode — *acting on wrong assumptions about code it has not read*
(the first item in Karpathy's critique set: models "make wrong assumptions on
your behalf and just run along with them without checking"). Concretely it lets
the harness and LocalMind:

- anchor accepted lessons, bugs, and decisions to **real symbols** (this
  function, this test, this module) instead of free-text file paths;
- retrieve by **structure** ("what calls this", "what tests this", "what does
  this module depend on") rather than only by keyword or embedding similarity;
- give the planner grounded context before it edits, reducing blind edits to
  code it does not understand.

**The bend that keeps it SERVES, not dilution** (every borrowed idea must be
rebuilt to fit the harness):

- ingestion runs **through LocalPilot's permission + redaction boundary** and
  honors `.localmind.toml` `excluded_paths` — the graph never contains anything
  capture was not allowed to read;
- the graph is **local, deterministic, and attributable** — code-structure
  extraction is AST-based with no network and no LLM in the hot path;
- it stays **offline and reproducible** — no external API, no telemetry, no
  cloud graph service.

**DANGER to refuse** (where this stops being LocalPilot): graphify's pattern of
sending documents to **external AI APIs for semantic extraction**, any
cloud-hosted graph store, and copying another tool's schema/identifiers. See §2
and §6.

---

## 1. Where this fits — LocalMind owns it, LocalPilot hosts it

### 1.1 The schema is already specified

LocalMind's vision (`external/localmind/vision.md` §5, "Graph Knowledge Layer")
already names the entities and relationships this work must produce. Code-graph
ingestion does not invent a schema — it **populates the one already on paper**:

- **Entities** include `Project`, `Repository`, `File`, `Class`, `Test`,
  `Framework`, `Tool`, `Dependency` — alongside the learned-knowledge entities
  `Bug`, `Decision`, `Lesson`, `Skill`, `Command`, `Error`.
- **Relationships** include `implemented_by`, `tested_by`, `documented_in`,
  `uses`, `belongs_to_project`, `requires_skill`, `fixed_by`, `caused_by`.

The payoff is the **join**: a tree-sitter walk produces the `File`/`Class`/
`Test`/`Function` nodes and `implemented_by`/`tested_by`/`uses` edges; the
existing learning loop produces `Lesson`/`Bug`/`Decision` nodes. Linking them
(`lesson —about→ Function`, `bug —caused_by→ Dependency`) is the unified graph
the vision asks for, and it is the thing neither graphify (code only) nor the
Obsidian-style PKM tools (notes only) have.

### 1.2 Current state — greenfield

- LocalMind crates today: `localmind-core`, `localmind-store`,
  `localmind-review`, `localmind-search`, `localmind-skills`, `localmind-mcp`,
  `localmind-cli`. MIT, MSRV 1.82, exact-pinned deps, `unsafe_code = "forbid"`,
  `unwrap`/`expect` denied, `rusqlite` (bundled SQLite) already present.
- `localmind-core` modules: `lesson`, `memory`, `evidence`, `session`, `skill`,
  `summary`, `audit`, `review`, `context`, `adapter`. **No entity/graph
  module.**
- The only "graph" today is a `graph: bool` flag in `localmind-search`,
  defaulting to `false`. The graph layer is unbuilt.
- `localmind-mcp` already exists — the surface for exposing graph queries to a
  host or other agents is in place.

So this is a clean build on a known schema, not a retrofit.

### 1.3 Ownership boundary (do not violate)

Per `docs/localmind-integration.md`, the learning engine is host-neutral and
LocalPilot is its first native host. That split holds here:

- **LocalMind core owns**: the graph schema, the ingestion contract, the parser
  pipeline, graph persistence, and graph-aware retrieval. This is a new
  capability in the **LocalMind workspace** (`external/localmind`), not in
  LocalPilot.
- **LocalPilot (host) owns**: workspace access, the permission engine, redaction
  before persistence, trust gating, and the CLI/TUI surfaces. It feeds files to
  the ingester through the same boundary that already governs capture.
- The edge stays one-way: LocalPilot depends on LocalMind, never the reverse.

---

## 2. Clean-room framing (blocking)

graphify is open source and inspectable. That makes it a **behavior/concept
reference governed by `docs/00-clean-room.md`** — the *same* status the repo
already assigned to the inspectable agents OpenCode and Pi in
`tasks/localpilot-next-phase-research.md` §10. The user's framing ("use it as a
reference, don't reinvent the wheel, but don't flat-out steal") is exactly this
existing precedent, not a new exception.

What that permits and forbids:

- **Permitted**: studying graphify to understand the *concept* — AST-to-graph
  ingestion, typed nodes/edges, confidence-tagged provenance, graph query
  primitives. Re-derive each from the public idea and the LocalMind vision.
- **Forbidden** (hard rules, §"Hard Rules" of `00-clean-room.md`): copying,
  translating, porting, or mechanically transforming graphify's source code,
  identifiers, function/class names, JSON shapes, graph-schema field names, UI
  copy, or package structure. Name everything in LocalMind's own vocabulary.
- **Dependencies**: `tree-sitter` and language grammars are open-source crates
  usable under their own licenses — using them is fine (it is the standard AST
  toolchain, not graphify-specific). Verify each grammar's license in the
  readiness step.
- **graphify's own license**: not yet confirmed (the canonical `LICENSE` path
  did not resolve). Because the clean-room stance is "re-derive the concept,
  copy nothing," the license does not gate *using graphify as a reference*. It
  **does** gate ever vendoring any graphify file or generated data — which this
  plan does not do. Confirm the license in CG0 anyway, and record it.
- **PR provenance note** (required when a change was reference-informed):

  ```text
  Concept cross-checked against the open-source graphify project as a
  behavior reference. Schema, code, identifiers, and graph shapes are original
  to LocalMind and re-derived from its own vision spec.
  ```

This work also lands in the **LocalMind** repo, which carries its own provenance
expectations; the same discipline applies there.

---

## 3. graphify's transferable concepts — adopt / adapt / reject

A concept inventory (from reading graphify's public description), each tagged for
LocalMind. **Concepts only — no code, identifiers, or shapes are carried over.**

| graphify concept | What it is | LocalMind decision |
| --- | --- | --- |
| tree-sitter AST parse, ~28 languages | Local, deterministic symbol extraction | **Adapt.** Start with the languages LocalMind users actually work in (Rust first). tree-sitter is the right toolchain; treat the C-grammar build dependency as a portability spike (§5, CG0). |
| Typed nodes + edges | Graph of entities and relationships | **Adopt the concept; use the vision §5 schema.** Our node/edge *names* come from `vision.md`, not graphify. |
| Confidence tags (`EXTRACTED` / `INFERRED` / `AMBIGUOUS`) | Provenance on each relationship | **Adapt and unify.** LocalMind lessons already carry confidence + evidence + provenance; extend that one vocabulary to code-derived nodes rather than inventing a parallel one. Convergent design — validates the approach. |
| Local for code, **external AI APIs for documents** | Semantic doc extraction off-machine | **Reject the external-AI half.** Code-structure extraction is deterministic AST work — no LLM, no network. Any semantic/doc enrichment uses **local models only**, opt-in, and never in the offline default path. This is the §0 DANGER line. |
| Leiden community detection ("god nodes", clusters, insight report) | Graph clustering for human-facing insights | **Defer.** Not needed for retrieval v1. Revisit as an optional analysis pass once the graph exists and there is a use for cluster summaries. |
| `graph.html` / `graph.json` / report outputs | Local artifacts for browsing the graph | **Adapt, local-only.** A redacted local HTML/JSON export fits LocalPilot's "local bundle, never cloud" stance (`localpilot-next-phase-research.md` §5.11). The report is optional polish. |
| MCP server tools (`query_graph`, `shortest_path`, `get_neighbors`) | Programmatic graph access for agents | **Adopt the concept via `localmind-mcp`** (already exists). Our tool names and request/response shapes are original. |

---

## 4. Design sketch

### 4.1 Graph schema (LocalMind-native)

Define node and edge types in `localmind-core` from the `vision.md` §5 list.
Minimum viable code-structure subset:

- **Nodes**: `Repository`, `File`, `Module`, `Type` (struct/enum/class/interface),
  `Function`, `Test`, `Dependency`. Each node carries: stable id, kind, name,
  source location (file + span), a content hash (for incremental reindex), and a
  **provenance/confidence** record shared with the existing lesson provenance.
- **Edges**: `implemented_by`, `tested_by`, `documented_in`, `uses`,
  `belongs_to_project`. Each edge carries a confidence tag and the evidence
  (the span/import that justifies it).
- **Join edges to learned knowledge**: `about` / `anchored_to` linking a
  `Lesson`/`Bug`/`Decision` to a code node. This is the differentiator.

### 4.2 Persistence

Stay inside the existing dependency surface: **SQLite via `rusqlite`** (already
bundled). Nodes and edges as tables with indexes on kind/name/file; traversal via
recursive CTEs. Do **not** add a separate embedded graph engine for v1 — keep the
dependency count low (`docs/13-rust-best-practices.md`). Adopt a **graph format
version + migrate-on-load** contract from the first release, mirroring the
session-format discipline recommended in `localpilot-next-phase-research.md` §5.3.

### 4.3 Ingestion pipeline

1. **Walk** the workspace through the host's permission boundary; honor
   `.localmind.toml` `excluded_paths` and redaction. The ingester never sees a
   path capture was not allowed to read.
2. **Parse** each supported file with tree-sitter; extract the node set.
3. **Resolve edges** within and across files (calls, imports, test-to-target).
   Tag each edge `EXTRACTED` (direct from AST) or `INFERRED` (heuristic) — reuse
   the confidence vocabulary, do not clone graphify's tag names verbatim.
4. **Persist** nodes/edges with content hashes and provenance.
5. **Index** for retrieval.

Everything in steps 2–4 is deterministic and offline.

### 4.4 Incremental, git-aware reindex

Full reparse on every change does not scale and fights the "designed for local
hardware" principle (`vision.md` §11). Reindex only files whose content hash
changed (git diff or mtime), prune nodes for deleted files, and mark superseded
nodes rather than hard-deleting (provenance survives). Indexing is bounded and
backgroundable.

### 4.5 Retrieval integration

Extend `localmind-search` to flip the placeholder `graph: bool` into a real
capability: combine **graph traversal** (neighbors, shortest path, callers/
callees, tests-of) with the existing keyword path and any future vector path,
weighted by **recency and confidence** as the vision's retrieval layer (§6)
specifies. The headline query the vision asks for —

> "We are working on message processing. What do we know about this user's
> preferences, previous bugs, architectural constraints, and relevant skills?"

— becomes answerable because the code nodes (what the message-processing module
*is*) and the learned nodes (what we *learned* about it) are one graph.

### 4.6 Surfaces

- `localmind-mcp`: graph query tools (original names/shapes) so a host or other
  agent can ask structural questions.
- LocalPilot CLI/TUI: a `localpilot memory` subcommand area for graph inspection,
  permission- and redaction-gated like the rest of memory.
- Optional local HTML/JSON export — file on disk, redacted, never cloud.

---

## 5. Build path — recommendation

Two ways to get a code graph; they were the two options raised in discussion.

**Path A — native LocalMind ingester (recommended).** A new crate
(e.g. `localmind-codegraph`) in the LocalMind workspace using tree-sitter,
writing into `localmind-store`, queried through `localmind-search` and
`localmind-mcp`.

- *For*: the schema is already specified; it is offline, deterministic, and
  local-first by construction; it stays clean-room-clean (no coupling to another
  tool's output format); it produces the **join** with learned knowledge that is
  the whole point.
- *Against*: more to build; tree-sitter brings a C-grammar build dependency that
  must be proven across tier-1 platforms (Windows/Linux/macOS, ADR-0007).

**Path B — consume an external code graph over MCP (fallback / spike only).**
Point LocalMind/LocalPilot at graphify (or any tool) emitting a graph over MCP.

- *For*: fastest way to *validate* whether structural retrieval is worth the
  build before committing to Path A.
- *Against*: graphify uses **external AI APIs for document extraction** (network
  — violates the offline default), couples us to an external tool's output
  shape, and adds a runtime dependency that pulls against local-first and
  no-telemetry. Not a shippable end state for LocalPilot.

**Recommendation: build Path A.** Optionally run a **throwaway Path B spike**
first, purely to measure retrieval-quality lift on a real repo and de-risk the
investment — then discard it. Do not ship a dependency on an external graph tool.

---

## 6. Clean-room cautions (graphify-specific)

- graphify is a **concept reference**, governed by `docs/00-clean-room.md`. Do
  not copy code, prompts, identifiers, node/edge field names, JSON shapes, MCP
  tool names, or package layout. Re-derive from the public concept and the
  LocalMind vision; name things in LocalMind's vocabulary.
- The **external-AI document-extraction** pattern is a non-goal. Code-structure
  ingestion is deterministic AST work; any semantic enrichment is local-model
  only and opt-in.
- Cloud-hosted graph storage/sharing is a non-goal; local artifacts only.
- tree-sitter and grammars are public open-source deps — usable under their
  licenses; confirm each in CG0.

---

## 7. Recommended workstreams (seeds a future plan)

`W`-numbered to slot alongside the existing next-phase workstreams; these become
subjects when promoted into `tasks/CodeGraph-Plan.md`.

**CG0 — Readiness (required first).** Confirm graphify's license and record it.
Evaluate tree-sitter crates + grammar licenses; prove the C-grammar build across
Windows/Linux/macOS (or evaluate a pure-Rust parser alternative) against MSRV
1.82 and the exact-pinned-dep rule. Draft the node/edge schema from `vision.md`
§5. Decide A-vs-B; optionally run the Path B spike and discard it.

**CG1 — Graph schema + store.** Node/edge types in `localmind-core`; SQLite
tables + indexes in `localmind-store`; provenance/confidence aligned with
existing lesson provenance; format version + migrate-on-load.

**CG2 — Ingester crate.** `localmind-codegraph`: permission-/redaction-gated
workspace walk, tree-sitter parse (Rust first), extract
`File`/`Module`/`Type`/`Function`/`Test` nodes and
`implemented_by`/`tested_by`/`uses`/`documented_in` edges, confidence-tag,
persist.

**CG3 — Incremental, git-aware reindex.** Content-hash diffing, stale-node
pruning with supersession, bounded background indexing.

**CG4 — Graph-aware retrieval + the join.** Real graph traversal in
`localmind-search`; recency/confidence weighting; `lesson ↔ code-node` anchoring
so accepted memory attaches to real symbols.

**CG5 — Surfaces.** `localmind-mcp` query tools (original names), LocalPilot CLI
inspection, optional local HTML/JSON export — all permission-/redaction-gated.

---

## 8. Open questions

- Which languages after Rust? (Driven by the repos LocalMind users actually
  work in.)
- tree-sitter's C-grammar build dependency vs a pure-Rust parser — which wins on
  tier-1 cross-platform + MSRV + dependency hygiene?
- Code embeddings for semantic code search — local-model only, and deferred
  until a vector layer lands?
- Is Leiden-style community detection worth it for insight reports, or skip for
  v1?
- New crate `localmind-codegraph`, or fold ingestion into `localmind-store`?
  (Leaning new crate for a clean boundary.)
- SQLite + recursive CTEs vs an embedded graph engine — confirm SQLite holds up
  on traversal performance before adding any dependency.

---

## 9. References

- **graphify** (`github.com/safishamsi/graphify`) — open-source code-to-graph
  tool; **concept reference only** under `docs/00-clean-room.md` §"Local Behavior
  Reference" and the §10 precedent in `tasks/localpilot-next-phase-research.md`.
- **tree-sitter** — public AST parsing toolchain; candidate dependency.
- `external/localmind/vision.md` §5 (Graph Knowledge Layer), §6 (Retrieval), §11
  (local-hardware optimization) — the authoritative schema and intent.
- `docs/localmind-integration.md` — LocalMind/LocalPilot ownership boundary.
- `docs/00-clean-room.md` — provenance rules (blocking).
- `tasks/localpilot-next-phase-research.md` §0 (identity filter), §10 (clean-room
  precedent for inspectable open-source references).
