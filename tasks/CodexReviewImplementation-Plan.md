# CodexReviewImplementation Plan

> Disposable build-plan artifact. Keep shipped code, public docs, tests,
> identifiers, branch names, PR text, and commit messages independent of this
> plan. This file and the sibling folder are removed or archived before v1.

## Collaboration model

| Field | Value |
|---|---|
| Mode | `solo` |
| Primary owner | agent |
| Coordinator | agent |
| Resume safety | required |
| Parallel branches | no |
| Notes | Large task because it spans harness, sandbox/permissions, provider runtime, MCP, docs, CI/supply-chain gates, and external LocalMind dependencies. |

### Parallel work tracker

| Subject | Owner | Branch | Dependencies | Conflict-risk files | Status | Handoff notes |
|---|---|---|---|---|---|---|
| n/a | | | | | | |

### Checkpoint gate

Before considering a box complete, update the relevant subject file, this plan
tracker, lessons/manual actions when applicable, run the focused verification
or record the exact blocker, commit the coherent checkpoint, and push. Commit
messages and PR metadata must describe the product change, not this plan.

## 1. Subject

Fully implement the actionable findings in `docs/codex-review.md`: close
harness permission and worktree safety gaps, make mid-stream quota failures
pause cleanly, restore supply-chain hygiene gates, fix misleading test support
and MCP dynamic tool memory handling, then reconcile release documentation.
Out of scope: changing product direction, adding new provider families,
replacing the quality-gate architecture, or consulting the read-only behavior
reference unless subject 00 finds the repository specs incomplete.

## 2. Authoritative inputs

| Source | Contribution |
|---|---|
| `docs/codex-review.md` | Defines the findings, severity, evidence, impact, possible fixes, and current verification state. |
| `docs/00-clean-room.md` and ADR-0004/ADR-0005 in `docs/10-decisions.md` | Clean-room constraints, allowed inputs, local reference limits, and PR provenance expectations. |
| ADR-0007 in `docs/10-decisions.md` | Windows, Linux, and macOS parity requirement. |
| ADR-0009 in `docs/10-decisions.md` | Ratified quality checks must run through the permission engine and sandbox. |
| `docs/04-provider-contract.md` | Provider error taxonomy, stream-event expectations, and quota/rate-limit contract. |
| `docs/05-tool-system.md` and `docs/07-security-and-privacy.md` | Tool, permission, sandbox, and redaction contracts. |
| `docs/06-harness-spec.md` and `docs/08-testing.md` | Harness runtime behavior, resume semantics, bad-output recovery, and regression-test expectations. |
| `docs/13-rust-best-practices.md` and `docs/14-dev-tooling.md` | Rust style, cross-platform path/process rules, checkpoint gates, and Tier L planning rules. |
| `.github/workflows/*.yml`, `deny.toml`, workspace `Cargo.toml` / `Cargo.lock` | Actual CI and supply-chain gates to restore. |
| Local source under `crates/` and `external/localmind/` | Current implementation surfaces and regression-test locations. |

## 3. Subject file index

| # | File | Subject |
|---|---|---|
| 00 | `tasks/codex-review-implementation/00-tooling-research-and-readiness.md` | Tooling research and readiness |
| 01 | `tasks/codex-review-implementation/01-harness-command-permissions.md` | Permission-mediated harness checks |
| 02 | `tasks/codex-review-implementation/02-harness-worktree-safety.md` | Resume preflight and scoped staging |
| 03 | `tasks/codex-review-implementation/03-provider-quota-pauses.md` | Mid-stream quota pause behavior |
| 04 | `tasks/codex-review-implementation/04-supply-chain-gates.md` | Audit and machete release blockers |
| 05 | `tasks/codex-review-implementation/05-test-and-mcp-hardening.md` | Test approver and MCP registry hardening |
| 06 | `tasks/codex-review-implementation/06-docs-and-final-gate.md` | Checklist reconciliation and final gate |

## 4. Decision log

| ID | Date | Title | Decision | Rationale | Refs |
|---|---|---|---|---|---|
| D001 | 2026-06-05 | Tier L plan | Track this effort as Tier L in `tasks/` with solo ownership. | The review spans more than three crates and touches permission engine, provider runtime, tool trait implications, and release gates. | all subjects |
| D002 | 2026-06-05 | Legacy test command mediation | Represent legacy `harness.test_command` as a synthesized quality check named `test`. | This preserves the legacy config surface while routing command execution through the existing permission, classification, and sandbox path required by ADR-0009. | 01 |
| D003 | 2026-06-05 | Temporary `time` advisory ignore | Keep a narrow `cargo audit` ignore for RUSTSEC-2026-0009 until the workspace MSRV can adopt `time >=0.3.47`. | The fixed crate version requires edition 2024 metadata unsupported by the current Rust/Cargo 1.82 toolchain, and the affected LocalMind path does not parse untrusted RFC 2822 timestamps. | 04 |
| D004 | 2026-06-05 | Dynamic tool metadata | Change tool metadata accessors from `&'static str` to `&str` and store dynamic MCP metadata in owned tool entries. | This removes the MCP `Box::leak` workaround while keeping static built-in tool implementations simple. The durable trait contract is documented in `docs/05-tool-system.md`; no ADR change is required. | 05 |

## 5. Master progress tracker

| Done | # | File | Status | Owner summary | Human actions mirrored? |
|---|---|---|---|---|---|
| [x] | 00 | `tasks/codex-review-implementation/00-tooling-research-and-readiness.md` | DONE | agent: 8 | n/a |
| [x] | 01 | `tasks/codex-review-implementation/01-harness-command-permissions.md` | DONE | agent: 6; tech-lead: 1 | yes |
| [x] | 02 | `tasks/codex-review-implementation/02-harness-worktree-safety.md` | DONE | agent: 6 | n/a |
| [x] | 03 | `tasks/codex-review-implementation/03-provider-quota-pauses.md` | DONE | agent: 6 | n/a |
| [x] | 04 | `tasks/codex-review-implementation/04-supply-chain-gates.md` | DONE | agent: 5; release-engineer: 1 | yes |
| [x] | 05 | `tasks/codex-review-implementation/05-test-and-mcp-hardening.md` | DONE | agent: 6; tech-lead: 1 | yes |
| [x] | 06 | `tasks/codex-review-implementation/06-docs-and-final-gate.md` | DONE | agent: 5; product-owner: 1 | yes |

## 6. Cross-cutting principles

1. Clean-room provenance is blocking. Do not copy code, prompts, tests,
   identifiers, UI copy, or private endpoint details from any external or local
   behavior reference. Use public docs and this repo's specs first.
2. Permission and sandbox mediation is non-negotiable for repository-controlled
   commands, discovered checks, synthesized checks, and tool execution paths.
3. Runtime behavior must be cross-platform. Prefer `Path`/`PathBuf`, process
   argument lists, and existing command classification abstractions over shell
   string construction.
4. Tests pin observable behavior: denied commands stay denied, unrelated dirty
   work stays unstaged, quota pauses persist resumable state, and supply-chain
   blockers fail or pass in CI the same way they do locally.
5. Keep implementation surfaces narrow. Changes belong in the owning crate:
   permission decisions in sandbox paths, provider taxonomy in LLM/runtime
   paths, harness orchestration in harness paths, MCP registry changes in MCP
   paths.
6. Avoid durable architecture drift hidden in this plan. If a subject changes
   the provider trait, tool trait, quality-gate contract, or release policy in a
   way future contributors need to know, promote it to `docs/10-decisions.md`.
7. Every subject closes with a Captain Hindsight checkpoint before its tracker
   row is marked `DONE`.
8. The final gate must include the normal four-command Rust gate plus
   supply-chain hygiene: `cargo machete`, `cargo deny check`, and `cargo audit`.
9. If `external/localmind` must change, prefer committing the minimal original
   change in that tree and updating this workspace consistently; record any
   submodule or vendored-dependency workflow found in subject 00.
10. The stale checklist may be updated only after code and tests prove the
    corresponding behavior, or after the remaining gap is explicitly labeled.

## 7. Gate review

- [x] All §5 subjects done or explicitly abandoned with a §4 row
- [x] Subject 00 completed or explicitly waived/abandoned with a §4 row
- [x] `cargo fmt --check` clean
- [x] `cargo clippy --workspace --all-targets -- -D warnings` clean
- [x] `cargo test --workspace` passes
- [x] `cargo check --workspace` clean
- [x] `cargo machete` clean or every remaining report has an accepted, documented ignore
- [x] `cargo deny check` clean
- [x] `cargo audit` clean or every remaining advisory has a narrow temporary ignore with owner, rationale, and removal condition
- [x] Focused regression tests from subjects 01-05 pass by package
- [x] `cargo build -p unshackled --features tui,learning` passes
- [x] `cargo clippy -p unshackled --features tui,learning --all-targets -- -D warnings` passes
- [x] `cargo run -p unshackled -- doctor` succeeds or records a real environment-only blocker
- [x] Every non-abandoned subject has Captain Hindsight verdict `CLOSE`
- [x] Durable architecture decisions reviewed; no `docs/10-decisions.md` update required
- [x] Public docs and checklist reflect the implemented state without plan references
- [x] `tasks/codex-review-implementation/manual-actions.md` has no unresolved required action
- [x] `tasks/codex-review-implementation/lessons.md` reconciled with `tasks/lessons.md`
- [ ] Git status is clean except explicitly deferred artifacts. Not checked:
      implementation changes are intentionally left in the working tree because
      this session did not request commits or pushes.

## 8. Acceptance / sign-off

| Date | Reviewer | Result | Notes |
|---|---|---|---|
| 2026-06-05 | agent | PASS | Implementation complete and locally verified. No read-only behavior reference was used. Changes are left uncommitted in the working tree. |

## Appendix: Captain Hindsight Prompt

Run this at each subject close before marking that subject `DONE`.

```text
You are now Captain Hindsight.

Review the completed subject with hindsight. Assume the work is already done,
then identify what is clearer now than it was before the work started.

Check specifically for:
- Scope drift or missed requirements.
- Spec deviations that need a Decision-log row, and whether the decision is
  durable enough to promote to an ADR in docs/10-decisions.md.
- Lessons that should be recorded before context is lost.
- Tests that pin implementation details instead of observable behavior.
- Complexity, duplication, brittle design, or awkward naming that should be
  fixed now.
- Human-owned actions that need to be mirrored or resolved.
- Plan references that leaked into shipped code, tests, comments, identifiers,
  branch names, PR text, or commit messages.
- Clean-room provenance violations, copied prompts or identifiers, or private
  and undocumented endpoint use.
- Cross-platform parity for Windows, Linux, and macOS.

Return exactly these sections:

1. Keep: what was correct and should remain.
2. Fix before closing: concrete issues, missing tests, spec drift, plan hygiene,
   or design problems.
3. Record: decisions or lessons that must be added to the plan files, noting any
   that should become an ADR.
4. Risk: anything still uncertain after verification.
5. Verdict: CLOSE or DO NOT CLOSE.

If the verdict is DO NOT CLOSE, list the smallest concrete actions needed before
the subject can be closed.
```
