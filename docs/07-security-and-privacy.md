# Security and Privacy

## Security Model

The model is untrusted. Tool inputs are untrusted. Provider outputs are
untrusted. User-approved policy is trusted.

## Local Effects

Local side effects include:

- file writes
- file deletes
- shell commands
- package installs
- git mutations
- network access
- credential reads

Every local side effect must be mediated by the tool runtime and permission
engine.

## Workspace Trust

When opening a directory for the first time, Unshackled should ask whether the
workspace is trusted.

Trusted means:

- read normal project files
- run low-risk commands
- use configured tools

Trusted does not mean:

- read secrets without approval
- run destructive commands without approval
- write outside workspace without approval

## Secret Redaction

Redact:

- API keys
- bearer tokens
- private keys
- passwords
- cloud credentials
- connection strings with credentials

Redaction applies to:

- logs
- transcripts
- tool outputs
- error messages

## Shell Policy

Commands are classified as:

- read-only
- project-write
- external-write
- network
- destructive
- privileged
- unknown

Default decisions:

| Class | Interactive | Non-interactive |
| --- | --- | --- |
| read-only | allow | allow |
| project-write | ask | deny |
| external-write | ask | deny |
| network | ask | deny |
| destructive | ask with explicit warning | deny |
| privileged | ask with explicit warning | deny |
| unknown | ask | deny |

## Network Policy

The core app may call configured model providers. Tools need separate approval
for arbitrary network commands.

Provider clients must:

- use TLS for hosted APIs
- redact auth headers in logs
- expose request IDs when providers return them
- avoid logging raw prompts by default

## Telemetry

Default: no remote telemetry.

Allowed:

- local logs
- local performance timings
- user-exported debug bundles after review

If remote telemetry is ever added:

- it must be opt-in
- schema must be public
- redaction must happen before upload
- no prompts or source code by default

## Supply Chain

Required before public release:

- `cargo audit`
- `cargo deny`
- dependency license review
- release artifact reproducibility notes

## Abuse Resistance

Unshackled is a coding tool. It should not ship prompts or affordances aimed at:

- malware creation
- credential theft
- phishing
- evasion
- unauthorized access

The permission engine is a local safety layer, not a replacement for provider
usage policies.

