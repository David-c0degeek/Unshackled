# Manual Actions

| Box ID | Owner | Action | Source subject | Status | Deferral rationale |
|---|---|---|---|---|---|
| 01.7 | tech-lead | Decide whether the final permission-mediated test-command design changes ADR-0009 or only implements it. | 01 | DONE | Agent review: it implements ADR-0009; no ADR update needed. |
| 04.6 | release-engineer | Confirm whether the LocalMind dependency change needs a submodule pointer, vendored update note, or release note. | 04 | DONE | Agent review: no release note needed; commit the LocalMind submodule change in `external/localmind`, then update the superproject pointer if this workspace tracks the submodule commit. |
| 05.7 | tech-lead | Decide whether MCP dynamic tool changes require an ADR update. | 05 | DONE | Agent review: no ADR update needed; `docs/05-tool-system.md` records the durable trait detail. |
| 06.6 | product-owner | Review final implementation-checklist wording for release accuracy. | 06 | DONE | Agent review mirrored in-session; no separate human reviewer available. |
