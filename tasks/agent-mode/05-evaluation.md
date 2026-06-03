# 05 — Live Validation + Agentic Eval Suite

## Goal
Prove agent mode actually completes real repo tasks — the single biggest unknown
— against both a capable hosted model and a capable local model, and lock the
result with a repeatable eval so regressions are caught.

## Boxes

- [ ] **05.1** (agent) Define a small set of golden agent tasks on a throwaway
      repo (e.g. "add a failing test then make it pass", "fix a named bug",
      "rename a symbol across files"), each with an automatic pass check.
      Artefact: the task definitions + checks under the eval suite.
- [ ] **05.2** (agent) Add an opt-in live eval runner (behind an env flag, no key
      committed) that runs the golden tasks end to end and reports a success rate
      and a per-task scorecard. Artefact: the runner + a scorecard format.
- [ ] **05.3** (release-engineer) Run the live eval once against a capable hosted
      model (real key, local only) and record the scorecard. Mirror into
      `manual-actions.md`.
- [ ] **05.4** (release-engineer) Run the live eval against a capable local model
      (≥ Q4 via a local OpenAI-compatible server or the Anthropic gateway) and
      record the scorecard; note where local-model quality limits results.
      Mirror into `manual-actions.md`.
- [ ] **05.5** (agent) Triage failures from 05.3/05.4 into concrete fixes
      (prompt, tool, loop) or recorded limitations (§4 decisions / `lessons.md`);
      re-run until the hosted-model success rate meets the agreed bar.
- [ ] **05.6** (agent) Document how to run the eval and interpret the scorecard;
      ensure the offline suite stays green and the live path stays opt-in.

## Hindsight checkpoint
- [ ] Captain Hindsight review recorded
- [ ] Verdict is `CLOSE`

## Progress log
> One line per slice. Date · slice · box IDs · what shipped · how verified.
