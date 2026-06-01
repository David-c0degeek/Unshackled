# Lessons — `unshackled` Run Notes

> Append a line the moment a slice teaches something — not at the gate. These
> are disposable run-notes; durable lessons migrate to the permanent
> `tasks/lessons.md` at the §7 gate before this folder is deleted.

| Date | Slice | Lesson | Box / file |
|---|---|---|---|
| 2026-06-02 | 00/1 | MSRV-1.82 pin blocks newest dev tools: `cargo-nextest` ≥0.9.98 needs rustc 1.91, `cargo-machete` ≥0.8 needs `edition2024`. Pin `nextest 0.9.92` (the `0.9.97-b.2` beta segfaults on Windows), `machete 0.7.0`, `insta 1.47.2`. | 00.2 / D004 |
| 2026-06-02 | 00/1 | `cargo deny check` already reports `advisories FAILED` on the scaffold's `Cargo.lock` — a pinned dep carries a RustSec advisory. Resolve in the subject 01 supply-chain gate before it blocks CI. | 01 / deny.toml |
