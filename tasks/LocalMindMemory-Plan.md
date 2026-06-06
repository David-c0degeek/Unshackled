# LocalMindMemory-Plan.md

> Disposable build-process plan for replacing the legacy LocalPilot memory crate
> with the bundled LocalMind implementation. Shipped code, tests, comments, and
> commit metadata stay plan-agnostic.

## Collaboration model

| Field | Value |
|---|---|
| Mode | `solo` |
| Primary owner | Codex |
| Coordinator | Codex |
| Resume safety | required |
| Parallel branches | `no` |
| Notes | Continue current implementation; no parallel owners. |

## 1. Subject

Replace LocalPilot's standalone flat memory crate with the LocalMind-backed
learning and memory implementation. In scope: CLI memory routing, LocalMind
store APIs needed by LocalPilot, workspace/docs cleanup, and verification. Out
of scope: new TUI review screens, graph database expansion, or autonomous
memory writes.

## 2. Authoritative inputs

| Source | Contribution |
|---|---|
| `AGENTS.md` | Planning, clean-room, and workspace constraints. |
| `docs/localmind-integration.md` | Ownership boundary and intended LocalMind host integration. |
| `docs/02-architecture.md` | Crate ownership model to update after crate removal. |
| `docs/13-rust-best-practices.md` | Rust quality expectations. |
| `external/localmind` | First-party LocalMind engine consumed by LocalPilot. |

## 3. Subject file index

| # | File | Subject |
|---|---|---|
| 00 | `tasks/localmind-memory/00-tooling-research-and-readiness.md` | Tooling research and readiness |
| 01 | `tasks/localmind-memory/01-localmind-memory-integration.md` | LocalMind-backed memory integration |

## 4. Decision log

| ID | Date | Title | Decision | Rationale | Refs |
|---|---|---|---|---|---|
| D001 | 2026-06-05 | Waive separate tooling subject gate | Record current repo/context inspection as subject 00 instead of restarting from a clean baseline. | Work was already in progress before the Tier L trigger was checked; redo would add churn without new signal. | 00 |

## 5. Master progress tracker

| Done | # | File | Status | Owner summary | Human actions mirrored? |
|---|---|---|---|---|---|
| [x] | 00 | `tasks/localmind-memory/00-tooling-research-and-readiness.md` | DONE | agent: 4 | n/a |
| [x] | 01 | `tasks/localmind-memory/01-localmind-memory-integration.md` | DONE | agent: 5 | n/a |

## 6. Cross-cutting principles

1. Keep LocalMind as the single durable memory implementation.
2. Keep LocalMind host-neutral; LocalPilot owns only adapter and CLI behavior.
3. Memory writes remain local, inspectable, reviewed, and auditable.
4. Update docs and metadata so no removed crate is advertised.
5. Preserve clean-room provenance; no proprietary source or prompt copying.

## 7. Gate review

- [x] Subject 01 completed
- [x] `cargo fmt --check` clean
- [x] `cargo test -p localmind-store --manifest-path external/localmind/Cargo.toml` passes
- [x] `cargo test -p localpilot-localmind -p localpilot` passes
- [x] `cargo check --workspace` clean
- [x] Stale `localpilot-memory` references removed outside deletion history
- [x] Hindsight checkpoint recorded with verdict `CLOSE`

## 8. Acceptance / sign-off

| Date | Reviewer | Result | Notes |
|---|---|---|---|
| 2026-06-05 | Codex | PASS | LocalMind is now the only durable memory implementation in LocalPilot; full workspace verification passed. |
