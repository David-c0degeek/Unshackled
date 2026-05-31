# Test Plan

## Test Layers

### Unit Tests

Required for:

- parsers
- config precedence
- redaction
- path normalization
- command classification
- rule verdicts
- provider event parsing

### Integration Tests

Required for:

- fake provider + tool loop
- harness intake with fake provider
- harness plan with fake provider
- harness resume with fake provider
- permission prompts with scripted decisions
- session persistence

### Snapshot Tests

Useful for:

- CLI help
- error messages
- TUI render output
- `brief.md` rendering
- `PROGRESS.md` rendering

### Live Tests

Live provider tests must be opt-in:

```powershell
$env:UNSHACKLED_LIVE_TESTS = "1"
cargo test --test live_provider
```

Live tests must:

- skip when credentials are absent
- avoid destructive tools
- keep prompts minimal
- never run in default CI

## Fixture Policy

Fixtures must be authored for this repository. Do not copy fixtures from
closed-source tools or leaked projects.

Allowed fixtures:

- hand-written API responses based on public docs
- fake provider event streams
- small temporary repos
- generated files used only for tests

## Required MVP Tests

### Config

- default config loads
- project config overrides user config
- env overrides project config
- CLI overrides env
- secrets are redacted in debug output

### Provider

- text request translates correctly
- tool schema translates correctly
- streaming text parses correctly
- streaming tool call parses correctly
- malformed stream returns typed error

### Tools

- read file in workspace
- deny read outside workspace
- write file in workspace
- deny write outside workspace
- edit exact match
- reject ambiguous edit
- shell read-only allowed
- shell destructive denied in non-interactive mode

### Harness

- parse valid brief
- reject brief missing required section
- parse valid progress
- reject progress with duplicate step number
- next incomplete step selection
- mark step complete
- attempt counter increment
- rule retry path
- rule discard path
- replan cap

### Store

- transcript write/read round trip
- interrupted write leaves no corrupt session
- redaction before persistence

## CI Matrix

Platforms:

- Windows latest
- Ubuntu latest
- macOS latest

Commands:

```powershell
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo check --workspace
```

Optional:

```powershell
cargo audit
cargo deny check
```

