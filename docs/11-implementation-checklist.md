# Implementation Checklist

## Foundation

- [ ] Replace repository URL placeholders.
- [ ] Add GitHub Actions.
- [ ] Add `cargo-deny` CI step.
- [ ] Add `cargo-audit` CI step.
- [ ] Add changelog.
- [ ] Add contributor guide.

## Core

- [ ] Add typed IDs for sessions, turns, and tool calls.
- [ ] Add message metadata.
- [ ] Add usage accounting model.
- [ ] Add structured error hierarchy.

## Config

- [ ] Implement config loading.
- [ ] Implement user config path resolution.
- [ ] Implement project config path resolution.
- [ ] Implement env overrides.
- [ ] Implement config diagnostics.

## Providers

- [ ] Implement provider trait fully.
- [ ] Implement local OpenAI-compatible provider.
- [ ] Implement official hosted provider.
- [ ] Implement provider registry.
- [ ] Implement mock provider for tests.
- [ ] Add stream parser tests.

## Tools

- [ ] Implement tool registry.
- [ ] Implement path policy.
- [ ] Implement read tool.
- [ ] Implement write tool.
- [ ] Implement edit tool.
- [ ] Implement search tool.
- [ ] Implement shell tool.
- [ ] Implement git tools.
- [ ] Add approval interface.

## Harness

- [ ] Implement brief parser.
- [ ] Implement brief renderer.
- [ ] Implement progress parser.
- [ ] Implement progress renderer.
- [ ] Implement status command.
- [ ] Implement intake.
- [ ] Implement planner.
- [ ] Implement rules.
- [ ] Implement resume loop.
- [ ] Implement attempt logs.
- [ ] Implement replan loop.

## TUI

- [ ] Pick TUI crate stack.
- [ ] Implement message list.
- [ ] Implement input box.
- [ ] Implement streaming render.
- [ ] Implement approval modal.
- [ ] Implement status line.
- [ ] Implement slash commands.

## Store

- [ ] Implement transcript format.
- [ ] Implement atomic writes.
- [ ] Implement session index.
- [ ] Implement redaction.
- [ ] Implement export command.

## Security

- [ ] Implement secret detection.
- [ ] Implement command classification.
- [ ] Implement workspace trust prompts.
- [ ] Implement non-interactive denial policy.
- [ ] Add destructive command tests.

## Release

- [ ] Add install docs.
- [ ] Add alpha release checklist.
- [ ] Run clean-room audit.
- [ ] Tag `v0.1.0-alpha.1`.

