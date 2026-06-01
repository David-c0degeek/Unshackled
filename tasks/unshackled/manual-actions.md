# Manual Actions — Human-Owned Boxes

> Mirror of every non-`agent` box from the subject files (§5 of
> `tasks/Unshackled-Plan.md`). Keep in sync with the owning subject. Status is
> one of `TODO`, `DONE`, `DEFERRED` (a deferral needs a rationale). Owner is a
> non-`agent` role from the §5 enum.

| Box ID | Owner | Action | Source subject | Status | Deferral rationale |
|---|---|---|---|---|---|
| 00.6 | tech-lead | Decide MCP posture for the run (default: start without MCP; add OpenAI Docs MCP at subject 03 only after audit). Record in Decision log. | 00 | TODO | |
| 00.7 | tech-lead | Security-review every authored repo skill `SKILL.md` + bundled scripts before the repo is trusted; confirm no broad tool allowlists. | 00 | TODO | |
| 00.8 | release-engineer | Confirm credentials policy: `.env` gitignored, env-var → key mapping documented, no real keys in CI. | 00 | TODO | |
| 03.13 | tech-lead | Choose the first official hosted provider (official public API, ADR-0004) and confirm its public-docs source. | 03 | TODO | |
| 03.14 | release-engineer | Provide local-only credentials for the opt-in live provider test (`UNSHACKLED_LIVE_TESTS`); never commit keys. | 03 | TODO | |
| 06.18 | tech-lead | Review intake + planner prompts and rule verdict severities (block vs warn) for correctness + clean-room provenance before locking. | 06 | TODO | |
| 07.16 | tech-lead | Security-review MCP permission integration + any shipped/sample skill manifests; confirm no permission bypass / broad allowlists. | 07 | TODO | |
| 09.10 | tech-lead | Human clean-room audit sign-off (provenance, not just correctness) before the §7 clean-room gate is ticked. | 09 | TODO | |
| 09.11 | release-engineer | Run the full `docs/09` release checklist before tagging. | 09 | TODO | |
| 09.12 | release-engineer | Confirm Public-Alpha Criteria and tag `v0.1.0-alpha.1`. | 09 | TODO | |
| 09.13 | release-engineer | Set up the nightly channel build from main. | 09 | TODO | |
