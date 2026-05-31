# Provider Contract

## Goals

Providers connect Unshackled to models. They must hide API differences behind a
single internal stream contract while preserving provider capabilities.

## Requirements

Every provider must declare:

- id
- display name
- source type: `official_api`, `local_server`, or `custom_user_endpoint`
- supported input blocks
- supported output events
- supported tool-call shape
- max context tokens if known
- auth requirements
- rate-limit behavior if known

## Allowed Provider Types

### Official API

Uses a provider's documented API and authentication method.

Examples:

- OpenAI API
- Google Vertex AI
- AWS Bedrock
- other official provider APIs

### Local Server

Uses an endpoint running on the user's machine or infrastructure.

Examples:

- Ollama
- vLLM
- llama.cpp server
- local OpenAI-compatible gateways

### Custom User Endpoint

Allowed only when the user explicitly configures it. The docs must state that
the user is responsible for authorization and terms compliance.

## Prohibited Provider Types

- private consumer-product endpoints
- scraped web sessions
- undocumented subscription backends
- endpoints requiring browser cookie reuse unless the provider explicitly
  documents that as supported

## Internal Request

```rust
pub struct ModelRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub tools: Vec<ToolSpec>,
}
```

Future fields:

- temperature
- max output tokens
- reasoning effort
- response format
- provider metadata
- cache policy

Provider-specific options must be namespaced.

## Internal Events

```rust
pub enum ModelEvent {
    TextDelta(String),
    ToolCall { id: String, name: String, input_json: Value },
    Done,
}
```

Future events:

- reasoning summary
- usage update
- refusal
- structured output delta
- provider warning

## Error Taxonomy

Providers return errors classified as:

- auth
- rate_limit
- quota
- invalid_request
- model_not_found
- server
- network
- stream_decode
- unsupported_feature

The UI should show concise messages. Debug logs may include request IDs but must
not log secrets.

## Provider Tests

Provider tests must not require real credentials by default.

Required:

- request translation tests
- stream parsing tests
- error classification tests
- redaction tests

Optional:

- live tests gated by env var

## Configuration Example

```toml
[provider]
default = "local"

[providers.local]
kind = "openai-compatible"
base_url = "http://localhost:11434/v1"
api_key_env = "UNSHACKLED_LOCAL_API_KEY"

[providers.openai]
kind = "openai"
api_key_env = "OPENAI_API_KEY"
```
