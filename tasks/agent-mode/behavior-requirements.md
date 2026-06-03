# Behavior requirements — agent-mode

> Disposable notes for neutral, observable requirements distilled from product
> use, repo specs, and public documentation. This file may mention that a local
> read-only behavior reference was consulted, but it must not contain copied or
> paraphrased prompts, schemas, code, identifiers, tests, UI copy, branding, file
> layout, or implementation details from that reference.

## Rules

- Write requirements as observable outcomes: "when X happens, agent mode does Y."
- Prefer black-box observation of workflows, failures, and edge cases.
- Keep source-specific details out unless they come from this repo or official
  public documentation.
- Include a short provenance note when a requirement was cross-checked against
  the local read-only behavior reference.

## Requirements

| ID | Area | Observable requirement | Provenance |
|---|---|---|---|
| BR-001 | Prompting | When an agent-mode turn starts, the first provider request includes one system message that describes the workspace, permission boundary, tool loop, finish contract, and active tool names. | Repo specs; first-party implementation |
| BR-002 | Tools | Tool results delivered back to the model use a consistent success/error envelope and remain redacted. | Repo specs |
| BR-003 | Tools | A multi-edit request either applies every scoped replacement to one file or leaves the file unchanged with a concrete error. | Repo specs |
| BR-004 | Tools | Filename discovery is separate from content search and respects workspace and ignore boundaries. | Repo specs |
| BR-005 | Provider runtime | Provider configuration can be launched from documented public env vars for OpenAI-compatible and Anthropic endpoints without committing secrets. | Official provider documentation |
| BR-006 | Provider runtime | Inline `<think>...</think>` text from local or gateway models is routed as reasoning metadata and does not pollute final answer text. | Local-model compatibility requirement |
| BR-007 | Context | When history exceeds the configured context budget, older exchanges are compacted, a bounded factual summary is retained, and the next request proceeds. | Repo specs |

## Maturity scenario matrix

| ID | Task class | Failure mode | Expected outcome | Coverage | Provenance |
|---|---|---|---|---|---|
| MS-001 | Read/edit/verify | Bad edit | Agent reads or edits a target file and the automatic check observes the expected final file state. | Offline golden task | Repo dogfood |
| MS-002 | Multi-file update | Context loss | Agent completes coordinated edits across documentation and source-like files. | Offline golden task | Repo dogfood |
| MS-003 | Malformed tool call | Malformed tool call | Runtime reports the concrete malformed-call issue and gives the model another chance within the turn budget. | Session test | Repo dogfood |
| MS-004 | Permission denial | Permission denial | Denied tool calls become error results, not crashes, and the loop can continue to a final answer. | Session/tool tests | Repo specs |
| MS-005 | Timeout/local model | Timeout | Slow provider requests use the configured timeout rather than a short fixed default. | Config/adapter test and manual live check | Official provider documentation |
| MS-006 | Live hosted model | Local-model quality limit | Hosted and local live runs are recorded separately so local quality limits are visible instead of masking runtime bugs. | Manual action | Repo release gate |
