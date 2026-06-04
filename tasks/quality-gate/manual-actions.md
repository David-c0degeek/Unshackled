# Manual actions

> Mirror of human-owned boxes. Keep in sync with the owning subject file.

| Box ID | Owner | Action | Source subject | Status | Deferral rationale |
|---|---|---|---|---|---|
| 06.4 | product-owner | Confirm ratification UX wording + default Rust gate | 06 | DEFERRED | Agent shipped a non-interactive surface (`gate propose` preview + `gate ratify` write); the per-check interactive accept/skip UX and final copy are a product-owner call. Default Rust gate proposed = `fmt, clippy, test, deps, audit` (see `quality/profiles.rs`); confirm names/commands/severities before v1. |
