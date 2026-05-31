# Contributing

## Ground Rules

Unshackled is original Rust software. Contributions must follow
[docs/00-clean-room.md](docs/00-clean-room.md).

Do not submit code, prompts, tests, endpoint adapters, docs, or UI copy copied
from proprietary or leaked projects.

## Development Setup

```powershell
cargo check --workspace
cargo test --workspace
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
```

## Pull Request Requirements

Each PR should include:

- what changed
- why it changed
- tests added or updated
- provenance note for API behavior or protocol details

Example provenance note:

```text
Provider request shape implemented from public API docs at <url>.
No private endpoint behavior used.
```

## Coding Style

- Keep crate boundaries narrow.
- Prefer typed data over stringly contracts.
- Put provider-specific code only in provider modules.
- Put local side effects only in tools.
- Keep prompts in harness modules and test them as product behavior.
- Use `tracing` for diagnostics.
- Redact secrets before persistence or logging.

## Review Checklist

- [ ] Code is original.
- [ ] Public docs are cited where protocol behavior matters.
- [ ] Tests cover failure paths.
- [ ] No private endpoints.
- [ ] No vendor branding as product identity.
- [ ] No secrets in fixtures.
- [ ] No broad unrelated refactors.

