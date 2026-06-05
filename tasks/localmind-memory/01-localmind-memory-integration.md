# 01 — LocalMind Memory Integration

## Goal

Make LocalMind the only durable memory implementation used by Unshackled.

## Boxes

- [x] **01.1** (agent) Add LocalMind store APIs for listing and deleting accepted memory.
- [x] **01.2** (agent) Route `unshackled memory` through `unshackled-localmind`.
- [x] **01.3** (agent) Remove the `unshackled-memory` workspace crate and dependency.
- [x] **01.4** (agent) Update docs, repository metadata, and stale comments.
- [x] **01.5** (agent) Run formatting and focused verification.

## Hindsight checkpoint

- [x] Captain Hindsight review recorded
- [x] Verdict is `CLOSE`

Review: the integration stayed within the intended ownership boundary: LocalMind
gained store-level list/delete APIs, while Unshackled owns CLI formatting,
disable-injection policy, and session closeout wiring. The main correction was
to make context lookup non-initializing so read-only print paths do not create a
fresh `.localmind` project.

Verdict: `CLOSE`

## Progress log

- 2026-06-05 · Replaced CLI memory wiring and added LocalMind list/delete APIs ·
  verification pending · no checkpoint commit yet.
- 2026-06-05 · Removed the legacy memory crate, made LocalMind a default CLI
  dependency, updated docs/install metadata for the renamed repository, and
  completed verification: `cargo fmt --check`, LocalMind formatting,
  `cargo clippy -p localmind-store --manifest-path external/localmind/Cargo.toml --locked --all-targets -- -D warnings`,
  `cargo test -p localmind-store --manifest-path external/localmind/Cargo.toml --locked`,
  `cargo test -p unshackled-localmind`, `cargo test -p unshackled --test memory`,
  `cargo test -p unshackled --test print`, `cargo test --workspace`,
  `cargo clippy --workspace --all-targets -- -D warnings`, and
  `cargo check --workspace` all passed.
