# Architecture

## System Shape

Unshackled is a set of Rust crates with a thin CLI binary.

```text
CLI/TUI
  |
  v
Session Runtime
  |
  +-- Harness Orchestrator
  +-- Tool Runtime
  +-- Provider Runtime
  +-- Store
  +-- Permission Engine
```

The runtime owns conversation flow. The provider runtime owns model calls. The
tool runtime owns local effects. The harness orchestrator owns project workflow.

## Crate Responsibilities

### `unshackled-cli`

Owns:

- command parsing
- top-level dispatch
- process exit codes
- human-readable command output

Must not own:

- business logic
- provider payload construction
- tool execution policy

### `unshackled-core`

Owns:

- domain types
- provider-neutral message model
- content blocks
- session IDs
- shared error types

Must remain:

- free of HTTP clients
- free of terminal UI code
- free of provider-specific names except generic enum variants

### `unshackled-config`

Owns:

- config schema
- config layering
- env var mapping
- redaction helpers

Config precedence:

1. command-line flags
2. environment variables
3. project `.unshackled.toml`
4. user config
5. built-in defaults

### `unshackled-llm`

Owns:

- provider trait
- stream event model
- provider registry
- official provider implementations
- local provider implementations

Provider implementations must live behind the same trait.

### `unshackled-tools`

Owns:

- tool trait
- tool registry
- JSON schema generation
- dispatch
- builtin tools

Builtin v1 tools:

- `read_file`
- `write_file`
- `edit_file`
- `list_files`
- `search_text`
- `run_shell`
- `git_status`
- `git_commit`

### `unshackled-harness`

Owns:

- brief parser/renderer
- progress parser/renderer
- intake role
- planner role
- worker role
- rule engine
- retry/discard/replan loop

The harness may call tools through interfaces. It must not bypass permission
checks.

### `unshackled-tui`

Owns:

- terminal layout
- message rendering
- keyboard input
- approval dialogs
- status lines

Recommended crates:

- `ratatui`
- `crossterm`
- `tui-textarea`

### `unshackled-store`

Owns:

- transcript persistence
- session indexes
- file-backed cache
- attempt logs
- redaction before persistence

Storage must be inspectable plain files where possible.

### `unshackled-sandbox`

Owns:

- permission rules
- workspace path policy
- command risk classification
- platform sandbox integration

V1 should implement conservative policy without relying on OS sandboxing:

- never write outside allowed workspace roots without approval
- never delete recursively without explicit approval
- never run network commands without approval unless allowlisted
- never read secret-like files without approval

### `unshackled-mcp`

Owns:

- MCP client protocol
- server lifecycle
- tool discovery
- resource reads
- permission integration

MCP is post-MVP.

## Runtime Flow

### Normal Chat Turn

1. User submits message.
2. Runtime builds provider-neutral messages.
3. Tool registry exposes allowed tool schemas.
4. Provider streams response events.
5. Tool calls are routed through permission checks.
6. Tool results are appended to the conversation.
7. Loop continues until provider emits final answer.
8. Store persists transcript.

### Harness Resume

1. Load config.
2. Load `brief.md`.
3. Load `PROGRESS.md`.
4. Validate repo state.
5. Select next incomplete step.
6. Build worker prompt from the step and current state.
7. Run agent loop with tools.
8. Run post-step rules.
9. Run tests if configured.
10. Commit if rules pass.
11. Mark step done and commit progress update.
12. Stop or continue based on mode.

## Data Model

### Messages

Messages are provider-neutral:

- role
- content blocks
- metadata

Provider adapters translate messages to the provider's official API format.

### Tool Calls

Tool calls are normalized:

- id
- tool name
- JSON input
- result text
- error flag

Provider adapters translate between provider tool-call formats and this model.

### Session State

Session state is split:

- durable transcript
- volatile runtime state
- project files
- provider metadata

Project files are authoritative for harness work. The transcript is supporting
context, not source of truth.

## Error Handling

Errors must be typed at crate boundaries:

- config errors
- provider errors
- tool errors
- permission errors
- harness validation errors
- store errors

The CLI converts errors to:

- short user message
- optional debug detail behind `--verbose`
- stable non-zero exit code

## Observability

Use `tracing`.

Default behavior:

- no remote telemetry
- local debug logs only when enabled
- redact tokens and secrets by default

Log levels:

- `error`: failed operation
- `warn`: recoverable risk or degraded mode
- `info`: major lifecycle events
- `debug`: payload metadata, never raw secrets
- `trace`: local-only deep diagnostics

