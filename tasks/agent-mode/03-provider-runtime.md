# 03 — Provider Runtime: Env Compatibility, Timeouts, Thinking

## Goal
Make agent mode work reliably against hosted and local providers, including
drop-in launch from external launchers that set the documented public env vars,
slow local models, and reasoning models that emit `<think>` content.

## Boxes

- [x] **03.1** (agent) Honor the documented public provider env vars as a config
      fallback: when a configured provider lacks a `base_url`/credential, fill
      from `ANTHROPIC_BASE_URL` / `ANTHROPIC_API_KEY` (anthropic kind) and
      `OPENAI_BASE_URL` / `OPENAI_API_KEY` (openai kind). A model env var
      (`ANTHROPIC_MODEL`) resolves the default model when none is set. Artefact:
      config/registry tests for env-fallback resolution.
- [x] **03.2** (agent) Result: an external launcher that exports those env vars
      and runs `unshackled` reaches the configured local endpoint without a
      `.unshackled.toml` edit. Artefact: a documented, tested resolution path (no
      private endpoints; localhost/official only).
- [x] **03.3** (agent) Add a configurable request timeout suited to slow local
      models (a sane default, overridable via config and/or the documented env
      convention), applied to the provider HTTP client. Artefact: a config test;
      the default is generous enough for local inference.
- [x] **03.4** (agent) Handle reasoning/thinking robustly: strip or route inline
      `<think>…</think>` content that arrives as message text (not as a reasoning
      field) so it never pollutes the final answer or tool-call parsing, on both
      adapters. Artefact: stream-parse tests with inline think tags.
- [x] **03.5** (agent) Provide a documented way to suppress model thinking where
      the provider supports it (request shaping / option pass-through), without
      adopting any third-party-proprietary env-var name. Artefact: request-body
      test.
- [ ] **03.6** (agent) Document the local-model and gateway setup (provider
      config, env-var path, timeout) in `docs/providers.md`; verify against a
      local OpenAI-compatible server and an Anthropic-compatible gateway.

## Hindsight checkpoint
- [ ] Captain Hindsight review recorded
- [ ] Verdict is `CLOSE`

## Progress log
> One line per slice. Date · slice · box IDs · what shipped · how verified.

- 2026-06-03 · provider runtime · 03.1-03.5 · added public env fallbacks, request timeout config/default, inline thinking routing, and suppress-thinking request shaping · verified with config and provider adapter tests.
- 2026-06-03 · provider docs · 03.6 · documented setup paths in `docs/providers.md`; live local OpenAI-compatible and Anthropic-compatible gateway verification remains open.

- 2026-06-03 · localbox drop-in · 03.1, 03.2 · env-synthesized provider, ANTHROPIC_AUTH_TOKEN credential fallback, and /v1/messages endpoint normalization · verified with config + anthropic endpoint tests.
