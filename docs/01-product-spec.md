# Product Specification

## Product Definition

Unshackled is a terminal-based coding-agent harness. It helps a developer turn
an idea into an explicit brief, turn the brief into a stepwise plan, and execute
that plan through an LLM plus local tools under rules that preserve reviewability.

The product is not a general chatbot. It is an engineering workflow controller.

## Target Users

- individual developers building software locally
- maintainers who want repeatable agent workflows
- power users who run local models
- teams that want auditable agent sessions before adopting hosted automation

## Non-Goals

- no private consumer-product endpoint automation
- no vendor-specific clone behavior
- no hidden telemetry
- no cloud sync in v1
- no remote code execution service in v1
- no browser IDE replacement in v1
- no model training or fine-tuning in v1

## Core Jobs

### Job 1: Convert an Idea into a Brief

Input:

- a short idea from the user
- optional project files
- optional constraints

Output:

- `brief.md`
- structured requirements
- constraints
- non-goals
- acceptance criteria

The brief must be understandable without the transcript.

### Job 2: Convert a Brief into a Plan

Input:

- `brief.md`
- repository summary
- optional user-edited constraints

Output:

- `PROGRESS.md`
- numbered steps
- completion state
- branch name
- test strategy

The plan must be editable by the user. The next run treats the edited file as
the source of truth.

### Job 3: Execute One Step at a Time

Input:

- next incomplete plan step
- current repository state
- configured tools
- configured provider

Output:

- code changes
- test output
- one commit per completed step
- updated `PROGRESS.md`

The agent must not mark a step complete until the rule engine allows it.

### Job 4: Recover from Failed Attempts

When a step repeatedly fails:

- the attempt is logged
- the current working changes are discarded only inside the target workspace
- the model context is reset
- the planner reconceives the failed step with the attempt log
- a capped retry counter prevents infinite loops

This is the anti-sunk-cost behavior: do not let the same failing context keep
digging.

## Modes

### Interactive REPL

An always-on terminal session with:

- message history
- tool approvals
- slash commands
- progress display
- model switching

### Harness CLI

Scriptable commands:

- `unshackled init`
- `unshackled harness intake`
- `unshackled harness plan`
- `unshackled harness resume`
- `unshackled harness status`
- `unshackled harness feature`

### Print Mode

Single prompt in, answer out:

- no workspace mutation unless explicitly enabled
- useful for shell pipelines

## User-Facing Files

### `.unshackled.toml`

Project-local config.

### `brief.md`

Problem statement and contract.

### `PROGRESS.md`

Plan and execution state.

### `.unshackled/`

Ignored runtime state:

- transcripts
- attempt logs
- cache
- provider metadata
- tool-output snapshots

## Minimum Viable Product

MVP must support:

- config loading
- one official hosted provider
- one local provider
- text-only model calls
- file read/write/edit tools
- shell command tool with approval
- brief generation
- plan generation
- progress parsing
- status display
- deterministic rule engine
- tests for all parsers and rule decisions

MVP does not need:

- image input
- MCP
- remote agents
- voice
- plugin marketplace
- IDE integration
- complex terminal rendering

