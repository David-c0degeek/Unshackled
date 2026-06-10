//! The internal request model.

use indexmap::IndexMap;
use localpilot_core::Message;
use serde::{Deserialize, Serialize};

/// How much reasoning/thinking effort to request from the model. Mapped per
/// provider by the adapter: a protocol shape with a documented effort field
/// uses it; a model/protocol without one clamps to a no-op — never an error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReasoningEffort {
    Minimal,
    Low,
    Medium,
    High,
}

impl ReasoningEffort {
    /// The wire string used by effort-aware request shapes.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            ReasoningEffort::Minimal => "minimal",
            ReasoningEffort::Low => "low",
            ReasoningEffort::Medium => "medium",
            ReasoningEffort::High => "high",
        }
    }

    /// Parse a user-facing effort name.
    #[must_use]
    pub fn parse(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "minimal" | "min" | "off" => Some(ReasoningEffort::Minimal),
            "low" => Some(ReasoningEffort::Low),
            "medium" | "med" => Some(ReasoningEffort::Medium),
            "high" => Some(ReasoningEffort::High),
            _ => None,
        }
    }
}

/// A provider-neutral request. Provider-specific tuning lives under
/// [`ModelRequest::options`], namespaced, reserving room for future first-class
/// fields (temperature, max output tokens, response format).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelRequest {
    pub model: String,
    pub messages: Vec<Message>,
    #[serde(default)]
    pub tools: Vec<ToolSpec>,
    /// Requested reasoning effort; an explicit value overrides any provider
    /// option default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<ReasoningEffort>,
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub options: IndexMap<String, serde_json::Value>,
}

impl ModelRequest {
    /// A request with no tools and no options.
    #[must_use]
    pub fn new(model: impl Into<String>, messages: Vec<Message>) -> Self {
        Self {
            model: model.into(),
            messages,
            tools: Vec::new(),
            reasoning_effort: None,
            options: IndexMap::new(),
        }
    }

    /// Set the available tools.
    #[must_use]
    pub fn with_tools(mut self, tools: Vec<ToolSpec>) -> Self {
        self.tools = tools;
        self
    }

    /// Set the requested reasoning effort.
    #[must_use]
    pub fn with_reasoning_effort(mut self, effort: Option<ReasoningEffort>) -> Self {
        self.reasoning_effort = effort;
        self
    }
}

/// A tool exposed to the model: a name, a description, and a JSON input schema.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolSpec {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}
