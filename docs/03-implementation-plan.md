# Implementation Plan

## Phase 0: Repository Foundation

Goal: stable clean-room Rust workspace.

Tasks:

- create Cargo workspace
- add crate boundaries
- add clean-room policy
- add license files
- add CI skeleton
- add formatting and lint configuration
- add dependency policy

Done when:

- `cargo check --workspace` passes
- `cargo test --workspace` passes
- docs explain provenance rules

## Phase 1: Core Domain and Config

Goal: load config and represent sessions without provider-specific logic.

Tasks:

- define `Message`, `ContentBlock`, `ToolCall`, `ToolResult`
- define `SessionId`, `TurnId`, `ToolUseId`
- implement `.unshackled.toml` parser
- implement user config directory resolution
- implement env var overrides
- implement secret redaction
- add snapshot tests for config precedence

Done when:

- config precedence is deterministic
- invalid config has precise diagnostics
- core crate has no provider dependencies

## Phase 2: Provider Runtime

Goal: call official APIs and local servers through one trait.

Tasks:

- finalize `ModelProvider` trait
- implement request/response stream model
- implement local OpenAI-compatible provider
- implement one official hosted provider
- add provider capability descriptors
- add provider registry
- add retry/backoff policy
- add rate-limit error classification

Done when:

- text-only `unshackled ask "..."` works
- provider tests use recorded local fixtures or mock HTTP
- no private endpoint is present in code or tests

## Phase 3: Tool Runtime

Goal: expose safe local tools to the agent.

Tasks:

- implement tool registry
- implement JSON schema generation
- implement `read_file`
- implement `write_file`
- implement `edit_file`
- implement `list_files`
- implement `search_text`
- implement `run_shell`
- implement `git_status`
- implement `git_commit`
- add path normalization and workspace-root checks
- add approval hooks

Done when:

- tools can be called by a fake model loop
- writes outside workspace are blocked by default
- dangerous shell commands require approval

## Phase 4: Session Runtime

Goal: complete a tool-using agent loop.

Tasks:

- build conversation state machine
- stream model events into UI events
- execute tool calls
- append tool results
- persist transcript
- support cancellation
- support max turn/tool limits
- support print mode

Done when:

- a fake provider can request file reads and shell commands
- cancellation leaves consistent persisted state
- all loop limits are tested

## Phase 5: Harness Documents

Goal: parse and render project workflow files.

Tasks:

- define `brief.md` schema
- define `PROGRESS.md` schema
- implement lossless parser/renderer where possible
- implement validation rules
- implement `unshackled init`
- implement `unshackled harness status`

Done when:

- user-edited files are accepted if semantically valid
- malformed files report exact sections/lines
- status works without a model provider

## Phase 6: Intake and Planning

Goal: create brief and plan through model calls.

Tasks:

- write original Unshackled intake prompt
- write original Unshackled planner prompt
- implement `harness intake`
- implement `harness plan`
- persist intake transcript
- support `--auto`
- support `--refine`
- support `--replan`
- validate generated artifacts before writing

Done when:

- idea -> `brief.md` works
- `brief.md` -> `PROGRESS.md` works
- invalid model output is retried with validation feedback

## Phase 7: Rule Engine

Goal: make execution governed by deterministic rules.

Tasks:

- define trigger types
- define verdict types
- define rule registry
- implement pre-edit rules
- implement post-edit rules
- implement pre-shell rules
- implement pre-commit rules
- implement step-complete rules
- implement config overrides
- implement attempt counters

Done when:

- each rule is unit tested
- rule failures are visible to the model and user
- config can tighten policy but cannot silently bypass critical rules

## Phase 8: Harness Worker

Goal: execute plan steps with model and tools.

Tasks:

- implement worker role
- select next incomplete step
- run agent loop for one step
- run tests
- commit step
- update progress
- log attempt failures
- discard failed attempts within workspace
- replan after capped failures

Done when:

- a sample repo can complete a small task end to end
- one commit is created per completed step
- repeated failures trigger context reset and replan

## Phase 9: Terminal UI

Goal: usable interactive experience.

Tasks:

- implement message viewport
- implement prompt input
- implement streaming response rendering
- implement tool approval dialogs
- implement status line
- implement slash commands
- implement model/provider picker
- implement transcript search

Done when:

- user can chat, approve tools, and run harness commands inside the TUI
- screen rendering is tested with snapshots
- no text overlap in common terminal sizes

## Phase 10: MCP and Extensions

Goal: integrate external tools without weakening safety.

Tasks:

- implement MCP client
- discover MCP tools
- route MCP tool calls through permission checks
- read MCP resources
- persist server configs
- add server health status

Done when:

- MCP tools behave like builtin tools from the model's perspective
- permissions apply uniformly

## Phase 11: Release Hardening

Goal: ship a public alpha.

Tasks:

- add installers
- add GitHub Actions CI
- add Windows/macOS/Linux smoke tests
- add cargo-deny
- add cargo-audit
- add release notes
- add public docs
- run clean-room audit

Done when:

- fresh install works on supported platforms
- no prohibited framing or private endpoint remains
- release artifact includes licenses

