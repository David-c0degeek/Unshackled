# 05 - Test Approver And MCP Registry Hardening

## Goal

Fix misleading approval-test support and remove avoidable MCP registry string
leaks while preserving static built-in tools and dynamic MCP tool behavior.

## Boxes

- [x] **05.1** (agent) Trace `ScriptedApprover` callers and tests to determine
      whether exhausted scripted behavior should deny by default while
      `always()` approves.
- [x] **05.2** (agent) Implement an explicit default behavior or separate
      always-approver path so `ScriptedApprover::always()` always resolves to
      approval and scripted exhaustion remains intentional.
- [x] **05.3** (agent) Add tests for `always()`, exhausted scripted approval,
      and at least one `Ask` path that previously could have silently denied.
- [x] **05.4** (agent) Trace `Tool` trait static-name assumptions, built-in tool
      registry creation, MCP registry rebuilds, and CLI runtime construction.
- [x] **05.5** (agent) Remove `Box::leak` for MCP tool names/descriptions using
      an owned dynamic tool descriptor path or a trait signature change that
      does not regress built-in static tools.
- [x] **05.6** (agent) Add MCP tests proving repeated registry rebuilds preserve
      names/descriptions, permissions, and tool-call routing without leaking
      through static lifetime workarounds.
- [x] **05.7** (tech-lead) Review whether any `Tool` trait signature change is
      durable architecture and must be promoted to ADR-0001/ADR-0009 or a new
      ADR.

## Hindsight checkpoint

- [x] Captain Hindsight review recorded
- [x] Verdict is `CLOSE`

## Progress log

> One line per slice. Date - slice number - box IDs touched - what shipped -
> how verified - checkpoint commit/push status.

- 2026-06-05 - 1 - 05.1-05.7 - Made scripted approval exhaustion deny by
  default while `ScriptedApprover::always()` remains approving, changed tool
  metadata methods to return `&str`, removed MCP `Box::leak` metadata storage,
  and added permission/MCP registry rebuild regressions. Updated
  `docs/05-tool-system.md` to reflect the durable trait contract. Verified by
  focused sandbox/MCP tests and the final workspace gate. Checkpoint not
  committed/pushed by agent.

## Captain Hindsight

1. Keep: Changing `Tool` metadata lifetimes to borrowed strings avoids leaks
   while preserving static built-in implementations.
2. Fix before closing: None.
3. Record: No ADR is needed because the security and permission contract did
   not change; the durable tool-system doc now records the trait shape and MCP
   dynamic metadata expectation.
4. Risk: External code implementing `Tool` must update signatures from
   `&'static str` to `&str`, but existing built-ins continue to compile because
   string literals coerce to `&str`.
5. Verdict: CLOSE.
