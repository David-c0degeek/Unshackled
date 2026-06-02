# Plan-Template.md — Skeleton For A Multi-Slice Plan In This Monorepo

> **Purpose.** Reusable shape for an autonomous-agent-driven multi-slice
> plan. Distilled from `Messaging-Plan.md` after a 61-slice run.
> Copy this file to `tasks/<Name>-Plan.md` and replace placeholders.
>
> **Disposable.** The plan and its `tasks/<name>/` folder are deleted (or
> archived out of the repo) once the subject ships to production. Shipped
> code, comments, tests, identifiers, and commit messages must not depend on or
> reference them — see §6.11.

> **Terms.**
> - **Subject** — one file under `tasks/<name>/` (`NN-<slug>.md`). 5-8 per plan.
>   Subject `00` is reserved for tooling research and readiness.
> - **Box** — one `[ ]` checklist item inside a subject file, ID
>   `<subject-id>.<box-number>`, where `<subject-id>` is the §3 `#` / `NN`
>   filename prefix (e.g. `01`) and `<box-number>` is the box ordinal in
>   that file. Stable; never renumbered.
> - **Slice** — one agent work-session, recorded as one line in a
>   subject's Progress log. A box may span several slices; a slice may
>   tick several boxes.

> **Placeholder convention.**
> - `<Name>` — PascalCase plan name in the plan filename (e.g. `Messaging`).
> - `<name>` — lowercase folder name for the plan's subjects.
> - `<Solution>` — you replace now, before the agent runs.
> - `NN` — two-digit subject number (`01`, `02`, `03`), matches the §3 `#`.
> - `<slug>` — short kebab-case subject name, e.g. `auth-boundaries`.
> - `<box-id>` — shorthand for `<subject-id>.<box-number>`, e.g. `01.3`.
> - `<base>` — merge base / target branch this plan branches from; record it in §2.
> - `TBD` — the agent fills during the run.
> - Do not use literal ellipses in filenames.

---

## How to use this template

1. Copy this file to `tasks/<Name>-Plan.md`.
2. Create a sibling folder `tasks/<name>/` to hold subject files.
3. Fill §1 (Subject), §2 (Inputs), §3 (Subject index).
4. Create `tasks/<name>/00-tooling-research-and-readiness.md` from the "00 subject-file shape" below before the agent starts.
5. Create each other indexed subject file from the "Subject-file shape" below before the agent starts.
6. Fill §6 (Cross-cutting principles) — keep these short, blocking, non-negotiable.
7. Leave §4 (Decision log) and the §5 (Master tracker) starter rows for the agent to complete.
8. Leave §7 (Gate review) as-is; the agent ticks it last.
9. Leave §8 (Acceptance / sign-off) empty for the user or reviewer.
10. Create `tasks/<name>/manual-actions.md` from the start — split agent-owned boxes from human-owned boxes. (Lesson from messaging plan: do this on day 1, not day 60.)
11. Create `tasks/<name>/lessons.md` empty. Append to it *during* the run, the moment a slice teaches something — not at the gate. These are disposable run-notes; durable lessons migrate to the permanent `tasks/lessons.md` at §7.
12. Run the Captain Hindsight checkpoint in each subject file before marking that subject `DONE` in §5. A `DO NOT CLOSE` verdict means the subject stays open until the required fixes, decisions, or lessons are resolved.

---

## 1. Subject

> One paragraph. What is being built. What is explicitly out of scope.
> The "out of scope" half is as important as the "in scope" half — it
> stops the plan from sprawling.

---

## 2. Authoritative inputs

> Table. One row per input doc / branch / spec / archived plan.
> Contribution column says exactly what each input gives.

| Source | Contribution |
|---|---|
| | |

---

## 3. Subject file index

> One row per subject file. **Aim for 5-8 subjects, not 15.** If you
> need more, the work is probably two plans.

| # | File | Subject |
|---|---|---|
| 00 | `tasks/<name>/00-tooling-research-and-readiness.md` | Tooling research and readiness |
| 01 | `tasks/<name>/01-<slug>.md` | |
| 02 | `tasks/<name>/02-<slug>.md` | |
| TBD | TBD | TBD |

---

## 4. Decision log

> Append-only. Every time the agent (or reviewer) deviates from a
> spec literal — even slightly — a row goes here. **A spec amendment
> never disappears into a slice; it is recorded as a decision with
> rationale.** This is what makes the plan survive a context-window
> reset.
>
> **Cite the box.** The Refs column names the box ID(s) (`<subject-id>.<box-number>`) and/or
> file(s) the decision touches. A decision that can't name what it
> amends is too vague — sharpen it.
>
> **Retiring a subject or box.** If a subject or box proves wrong
> mid-run, do **not** delete it. Mark it done and struck
> (`- [x] ~~<box-id> <box text>~~ ABANDONED; see D###`), mark the subject
> `ABANDONED` in §5, and add a Decision-log row with the rationale.
> History must survive a reset; a silently deleted box looks like
> unfinished work after compaction.

| ID | Date | Title | Decision | Rationale | Refs (box IDs / files) |
|---|---|---|---|---|---|
| D001 | | | | | |

---

## 5. Master progress tracker

> One row per subject file. A subject is `DONE` when **every** box in
> its file is `[x]` and its Hindsight checkpoint verdict is `CLOSE`.
> Abandoned boxes must use the exact struck-and-done
> format from §4. Mark a retired subject `ABANDONED` (see §4); an
> abandoned subject still gets its Done cell `[x]` (abandoned counts as
> resolved).
> §5 is "fully ticked" when every Done cell is `[x]` and every Status is
> `DONE` or `ABANDONED`.
>
> **Owner labels and summary.** Every box in every subject file MUST have
> an owner. Owners are one of: `agent`, `release-engineer`,
> `product-owner`, `tech-lead`, `domain-sme`. (Lesson from messaging
> plan: not having owner labels meant the master tracker stalled at
> "almost done" with a residue of human-owned items confused for
> agent-owned. Split them upfront.) The Owner-summary column names the
> specific roles present (e.g. `agent: 4; tech-lead: 1`), never a generic
> `human`.
>
> **Human-owned boxes** are mirrored into `tasks/<name>/manual-actions.md`
> so they don't get lost between agent-owned boxes. Keep the two in sync.

| Done | # | File | Status | Owner summary | Human actions mirrored? |
|---|---|---|---|---|---|
| [ ] | 00 | `tasks/<name>/00-tooling-research-and-readiness.md` | TODO | agent: 8 | n/a |
| [ ] | 01 | `tasks/<name>/01-<slug>.md` | TODO | TBD | TBD |
| [ ] | 02 | `tasks/<name>/02-<slug>.md` | TODO | TBD | TBD |

---

## 6. Cross-cutting principles

> These apply to every subject file. Violations are blockers, not nits.
> 1-11 are **defaults distilled from prior runs** — keep, cut, or trim
> each to what actually bites this plan. Lift any others from
> `AGENTS.md` / `CLAUDE.md`. Principles 2-4 are **C#/.NET-specific** —
> drop them for a non-.NET plan.

1. KISS · YAGNI · CLEAN · SOLID · DRY (in that order when conflicting).
2. *(C#)* One class per file.
3. *(C#)* Nullable reference types enabled.
4. *(.NET)* No new build warnings.
5. **Keep code modular and locally understandable.** Prefer small, cohesive modules with explicit boundaries. Split files before they become catch-all dumping grounds; split methods when branching, nesting, or mixed responsibilities make them hard to scan. If a file or method has to be large, record why and pin the coordinated behaviour with tests.
6. **Cyclomatic complexity stays low.** Branch-heavy methods are blockers unless the branching is inherent to the domain and covered by focused tests. Prefer guard clauses, extracted decision helpers, table-driven cases, or explicit strategy objects over deep `if`/`else`, nested `switch`, or boolean-flag flows. If the repo has a cyclomatic-complexity threshold, meet it; otherwise treat double-digit complexity in a method as design pressure to simplify.
7. **Spec at the contract level, not the SDK level.** State requirements as testable contracts on observable behaviour. Do not prescribe SDK call shapes — that's the implementer's choice. (Lesson from messaging plan: the spec said "one `SendAsync` per message" and the SB SDK couldn't honour the per-message error semantics implied. Three Decision-log amendments resulted. State *what* must be true, not *how* to call.)
8. **Coverage % is a smell-detector, not a goal.** A test exists to pin observable behaviour, not to chase a number. If you can't articulate "this test prevents future-X bug", delete it. Coverage gate failures should provoke "why is this code path untested?" — not "let's write a test that touches it". Architecture tests pin contracts; coverage gates flag gaps. Don't conflate them.
9. **Every plan box has an owner and a stable ID.** Owner from the §5 enum; ID `<subject-id>.<box-number>`. See §5.
10. **Lessons land in `tasks/<name>/lessons.md` as they happen**, not at the gate — per-plan run-notes that die with the folder. Durable lessons migrate to the permanent `tasks/lessons.md` at §7.
11. **Code is plan-agnostic.** Comments, test names, commit messages, and identifiers must read as permanent project artefacts. Never reference the plan, slices, box IDs (`<subject-id>.<box-number>`), decision-log entries (`D###`), or this file — they vanish when the plan is deleted after go-live (see Purpose). A dangling `// see 03.2` or `// per Decision D007` becomes a broken reference in production. Put the *why* directly in the comment.
12. **Captain Hindsight review before subject close.** Before marking a subject `DONE`, run the subject's Hindsight checkpoint. Record Keep / Fix before closing / Record / Risk / Verdict in the subject file. A `DO NOT CLOSE` verdict is a blocker: add or reopen boxes, update §4 decisions, or append lessons before closing.
13. **Tooling research before implementation.** Every plan starts with subject `00 - Tooling Research And Readiness` unless explicitly waived in §4. No implementation subject starts until subject 00 records repo context, baseline verification, stack best practices, official sources, and any approved Skills/MCP/tooling setup. A waiver must explain why the research step is unnecessary or unsafe for this plan.
14. *(plan-specific principles below)*

---

## 7. Gate review (run last; tick everything)

> Run only when §5 is fully ticked. §7 is the engineering gate the agent
> ticks; §8 is human acceptance that follows it.

- [ ] All §5 subjects done (or explicitly `ABANDONED` with a §4 row)
- [ ] Subject 00 completed, or explicitly waived/abandoned with a §4 row
- [ ] Build command for this plan passes with 0 errors and 0 new warnings. Default for .NET: `dotnet build <Solution>.sln`
- [ ] Test command for this plan passes. Default for .NET: `dotnet test <Solution>.sln`; coverage gate met if the repo enforces one
- [ ] Cross-cutting principles from §6 reviewed; any plan-specific architecture tests/rules hold
- [ ] Every non-abandoned subject has a recorded Captain Hindsight checkpoint with verdict `CLOSE`
- [ ] Shipped code/tests/comments/identifiers are plan-agnostic — grep the repo **excluding `tasks/`** for box IDs (`\b\d\d\.\d+\b`), decision IDs (`\bD\d{3}\b`), the literal `tasks/<name>/`, the plan filename `<Name>-Plan.md`, and `\bslices?\b`; zero hits after triaging false positives (version strings can match the box-ID pattern; "slice" may be a legit domain term). See §6.11
- [ ] Commit messages are plan-agnostic — `git log <base>..HEAD` mentions no box IDs, decision IDs, `tasks/<name>/`, `<Name>-Plan.md`, or `slice`/`slices` (apply the same false-positive triage; file grep does not cover commit messages)
- [ ] `tasks/<name>/manual-actions.md` — every human-owned box resolved or explicitly deferred
- [ ] *(plan-specific gates below)*
- [ ] `tasks/<name>/lessons.md` reconciled; lasting lessons migrated to the permanent `tasks/lessons.md` (create it if missing) before the `tasks/<name>/` folder is deleted (see Disposable note)
- [ ] Plan handed to reviewer for §8 sign-off

---

## 8. Acceptance / sign-off

> Filled by the user or reviewer after §7 passes. This is intentionally
> separate from §4: sign-off is acceptance, not a spec amendment.

| Date | Reviewer | Result | Notes |
|---|---|---|---|
| | | | |

---

## 00 subject-file shape (copy into `tasks/<name>/00-tooling-research-and-readiness.md`)

```markdown
# 00 — Tooling Research And Readiness

## Goal
Research the project stack, current best practices, official documentation,
assistant skills, MCPs, and local tooling before implementation starts. Convert
the findings into concrete plan rules, gates, enabled tools, and references so
later subjects can execute with the right context already in place.

## Boxes
> Subject 00 is required unless explicitly waived in the plan's §4 Decision log.
> No implementation subject starts until this subject is `DONE`, `ABANDONED`,
> or waived by decision.

- [ ] **00.1** (agent) Read repo instructions and authoritative project docs;
      list applicable constraints, clean-room/provenance rules, and existing
      conventions.
- [ ] **00.2** (agent) Inventory language, framework, runtime, package manager,
      CI, existing commands, and repo structure; identify the implementation
      surfaces this plan will touch.
- [ ] **00.3** (agent) Run baseline build/test/lint commands, or record exact
      blockers and the command output needed to reproduce them.
- [ ] **00.4** (agent) Research current best practices for the detected
      language/framework/runtime/testing stack using official or primary
      sources; record only findings that affect this plan.
- [ ] **00.5** (agent) Research APIs, libraries, providers, or standards this
      plan depends on using official or primary sources; record links,
      versions, and date-sensitive notes.
- [ ] **00.6** (agent) Research applicable `SKILL.md` workflows, assistant
      skills, MCP servers, and local tools; classify each as `adopt`, `defer`,
      or `reject` with rationale, trust notes, source URL, permissions,
      install/config command, and setup cost.
- [ ] **00.7** (agent) Set up only approved local tooling/config needed before
      coding; record install/config/provenance and keep security-sensitive
      permissions out of repo config unless narrowly justified.
- [ ] **00.8** (agent) Bake adopted findings into the plan: update §6
      cross-cutting principles, §7 gates, subject boxes, §4 decisions, and
      `tasks/<name>/lessons.md` as needed. Research that changes no plan
      artefact is not complete; either bake in the finding or record why it was
      rejected/deferred. End with an implementation-readiness summary.

## Hindsight checkpoint
> Run after all boxes in this subject are complete and before marking
> the subject `DONE` in §5. Use the embedded prompt in "Appendix: Captain
> Hindsight Prompt". Record the review result here.
>
> Required output sections: Keep; Fix before closing; Record; Risk;
> Verdict (`CLOSE` or `DO NOT CLOSE`). If the verdict is `DO NOT CLOSE`,
> leave the subject open, add/reopen boxes or update decisions/lessons,
> and rerun this checkpoint after the fixes.

- [ ] Captain Hindsight review recorded
- [ ] Verdict is `CLOSE`

## Progress log
> One line per slice. Date · slice number · box IDs touched · what
> shipped · how verified. Append a `tasks/<name>/lessons.md` line here too
> whenever the slice taught something.
```

---

## Subject-file shape (copy into each `tasks/<name>/NN-<slug>.md`)

```markdown
# <subject-id> — <Subject Title>

## Goal
> One paragraph. What this subject delivers.

## Boxes
> ID = `<subject-id>.<box-number>` (subject index ID · box ordinal). Stable — never
> renumber; mark retired boxes done and struck with `ABANDONED; see D###`
> (see §4), don't delete.
> Owner MUST be one of the §5 enum:
> agent · release-engineer · product-owner · tech-lead · domain-sme.
> Examples below use `01`; replace it with this file's `<subject-id>`.

- [ ] **01.1** (agent) Box description — outcome wording. Include the
      smallest verifiable artefact (test name, file path, log line).
- [ ] **01.2** (agent) Next box.
- [ ] **01.3** (release-engineer) Box that needs human action — also
      mirror into `tasks/<name>/manual-actions.md`.

## Hindsight checkpoint
> Run after all boxes in this subject are complete and before marking
> the subject `DONE` in §5. Use the embedded prompt in "Appendix: Captain
> Hindsight Prompt". Record the review result here.
>
> Required output sections: Keep; Fix before closing; Record; Risk;
> Verdict (`CLOSE` or `DO NOT CLOSE`). If the verdict is `DO NOT CLOSE`,
> leave the subject open, add/reopen boxes or update decisions/lessons,
> and rerun this checkpoint after the fixes.

- [ ] Captain Hindsight review recorded
- [ ] Verdict is `CLOSE`

## Progress log
> One line per slice. Date · slice number · box IDs touched · what
> shipped · how verified. Append a `tasks/<name>/lessons.md` line here too
> whenever the slice taught something.
```

---

## manual-actions.md shape (`tasks/<name>/manual-actions.md`)

> Mirror of every human-owned box from the subject files (see §5). One
> row per human action; keep in sync with the owning subject file.
> Status is one of `TODO`, `DONE`, `DEFERRED` (a deferral needs a rationale).
> Owner is a non-`agent` role from the §5 enum.

| Box ID | Owner | Action | Source subject | Status | Deferral rationale |
|---|---|---|---|---|---|
| 01.3 | release-engineer | TBD | 01 | TODO | |

---

## Appendix: Captain Hindsight Prompt

> This appendix is embedded so copied plans are self-standing and do not depend
> on machine-local prompt files.

```text
You are now Captain Hindsight.

Review the completed subject, phase, box, or major plan section with hindsight.
Assume the work is already done, then identify what is clearer now than it was
before the work started.

Check specifically for:
- Scope drift or missed requirements.
- Spec deviations that need a Decision-log row.
- Lessons that should be recorded before context is lost.
- Tests that pin implementation details instead of observable behavior.
- Complexity, duplication, brittle design, or awkward naming that should be fixed now.
- Human-owned actions that need to be mirrored or resolved.
- Plan references that leaked into shipped code, tests, comments, identifiers, or commit messages.

Return exactly these sections:

1. Keep: what was correct and should remain.
2. Fix before closing: concrete issues, missing tests, spec drift, plan hygiene, or design problems.
3. Record: decisions or lessons that must be added to the plan files.
4. Risk: anything still uncertain after verification.
5. Verdict: CLOSE or DO NOT CLOSE.

If the verdict is DO NOT CLOSE, list the smallest concrete actions needed before
the work can be closed.
```
