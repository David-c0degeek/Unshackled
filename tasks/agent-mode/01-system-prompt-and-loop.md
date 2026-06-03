# 01 — Original Agent System Prompt + Tool-Use Scaffold

## Goal
Give agent mode an original, capability-focused system prompt and a tool-use
scaffold so a capable model reliably plans, calls tools, reads results, and
finishes a task — instead of chatting. Everything here is written from first
principles; nothing is copied or paraphrased from the behavior reference.

## Boxes

- [ ] **01.1** (agent) Author an original agent-mode system prompt (a new
      first-party string/resource in `unshackled-harness`) covering: the agent's
      role; the available tools and when to use each; the read→reason→act→verify
      loop; the workspace and permission boundaries; commit/verify expectations;
      and the instruction to keep the final answer separate from reasoning.
      Artefact: a `system_prompt` module + a test asserting it names every
      registered tool and contains no reference-derived text.
- [ ] **01.2** (agent) Seed the system prompt into every agent-mode session
      (chat, print, and the shared loop) and verify it is sent once, ahead of the
      first user turn. Artefact: a session test asserting the first request
      carries the system prompt.
- [ ] **01.3** (agent) Make tool-call coaxing robust: ensure the loop continues
      after a tool result, re-prompts on an empty/malformed tool call within the
      attempt budget, and tells the model the concrete error so it can correct.
      Artefact: tests for the empty-call and malformed-call paths.
- [ ] **01.4** (agent) Standardize tool-result formatting fed back to the model
      (clear success/error framing, truncation markers, file/line context) so the
      model can act on it; redaction stays applied. Artefact: a formatting unit
      test.
- [ ] **01.5** (agent) Add an explicit "finish" contract: the model signals task
      completion (no tool calls + a final answer) and the loop ends cleanly with a
      concise summary; distinguish "done" from "stuck/looping". Artefact: a test
      that a tool-then-final-answer sequence ends as `Done`.
- [ ] **01.6** (agent) Tune the loop limits and the existing recovery ladder for
      real multi-step tasks (turn/tool caps, repair prompt wording — original),
      verifying the degenerate-output guard and recovery still fire. Artefact:
      updated session tests.

## Hindsight checkpoint
- [ ] Captain Hindsight review recorded
- [ ] Verdict is `CLOSE`

## Progress log
> One line per slice. Date · slice · box IDs · what shipped · how verified.
