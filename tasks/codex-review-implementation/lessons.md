# Lessons

Append lessons during implementation. Durable lessons that should outlive this
disposable plan must be migrated to `tasks/lessons.md` before final sign-off.

- Harness-owned commands should be represented as normal quality checks as soon
  as possible; duplicating command execution creates permission drift.
- Submodule dependency hygiene has two closeout layers: fix and verify inside
  the submodule, then separately decide whether to commit the submodule and move
  the superproject pointer.
- Dynamic tool metadata belongs in owned registry entries with borrowed `&str`
  accessors; leaking strings to satisfy a static trait hides lifecycle bugs.
