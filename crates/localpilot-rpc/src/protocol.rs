//! The headless-drive wire protocol: newline-delimited JSON over stdio.
//!
//! One JSON object per LF-terminated line in each direction. Every record
//! carries an explicit protocol version; an optional `id` correlates a reply
//! with the command that caused it. The protocol exposes the *existing*
//! runtime — commands in, streamed session events out — and never widens what
//! the permission engine would allow.

use serde::{Deserialize, Serialize};

/// The current RPC protocol version. Version negotiation happens in `hello`:
/// a client built for a newer version than this build receives a typed error
/// rather than silently mismatched records.
pub const RPC_PROTOCOL_VERSION: u32 = 1;

/// One client-to-agent record.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClientRecord {
    /// Protocol version the client speaks.
    pub v: u32,
    /// Optional correlation id, echoed on the direct reply.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub command: ClientCommand,
}

/// A typed command from the client.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum ClientCommand {
    /// Handshake: the agent replies with its protocol version and session info.
    Hello,
    /// Submit user input.
    Prompt {
        text: String,
        #[serde(default)]
        disposition: InputDisposition,
    },
    /// Cancel the running turn.
    Cancel,
    /// Answer a pending permission ask.
    PermissionReply { ask_id: String, allow: bool },
    /// Inspect session, harness-step, and permission state.
    Status,
    /// End the connection.
    Shutdown,
}

/// When queued input is admitted.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InputDisposition {
    /// Run now; rejected while a turn is already running (cancel first).
    #[default]
    Immediate,
    /// Inject into the running turn at the next safe provider-turn boundary;
    /// runs immediately when idle.
    Steer,
    /// Queue until the session is idle, then run as its own turn.
    FollowUp,
}

/// One agent-to-client record.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServerRecord {
    pub v: u32,
    /// Correlation id of the command this record directly replies to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub event: ServerEvent,
}

/// A typed event to the client. Streaming events mirror the runtime's session
/// events; the durable forms of the same vocabulary live in the session event
/// log.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum ServerEvent {
    Hello {
        protocol_version: u32,
        session_id: String,
        model: String,
    },
    TextDelta {
        text: String,
    },
    ReasoningDelta {
        text: String,
    },
    ToolStarted {
        id: String,
        name: String,
    },
    ToolFinished {
        id: String,
        name: String,
        is_error: bool,
        output: String,
    },
    Usage {
        input_tokens: u64,
        output_tokens: u64,
    },
    ContextUsage {
        used: usize,
        limit: usize,
    },
    Warning {
        message: String,
    },
    Plan {
        steps: Vec<PlanStepWire>,
    },
    QuotaPaused {
        reset: String,
    },
    Recovery {
        health: String,
    },
    /// The turn loop stopped; `reason` is the stop reason in snake case.
    Stopped {
        reason: String,
    },
    /// A permission decision needs the client: answer with
    /// `permission_reply`. An unanswered ask is denied, exactly like
    /// non-interactive mode.
    PermissionAsk {
        ask_id: String,
        tool: String,
        detail: String,
        risk: String,
    },
    /// Reply to `status`.
    Status {
        session_id: String,
        model: String,
        profile: String,
        busy: bool,
        pending_asks: Vec<String>,
        next_step: Option<String>,
    },
    /// Queued input was accepted with the given disposition.
    Queued {
        disposition: InputDisposition,
    },
    /// A command could not be honored; the connection stays open.
    Error {
        message: String,
    },
    /// The agent is closing the connection.
    Closed,
    /// A tool has failed repeatedly; the agent is switching strategy.
    ToolStuck {
        name: String,
        count: u32,
    },
}

/// One plan step on the wire.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanStepWire {
    pub title: String,
    pub status: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_records_roundtrip() {
        let records = vec![
            ClientRecord {
                v: RPC_PROTOCOL_VERSION,
                id: Some("1".to_string()),
                command: ClientCommand::Hello,
            },
            ClientRecord {
                v: RPC_PROTOCOL_VERSION,
                id: None,
                command: ClientCommand::Prompt {
                    text: "fix the bug".to_string(),
                    disposition: InputDisposition::Steer,
                },
            },
            ClientRecord {
                v: RPC_PROTOCOL_VERSION,
                id: None,
                command: ClientCommand::PermissionReply {
                    ask_id: "ask-1".to_string(),
                    allow: true,
                },
            },
        ];
        for record in records {
            let line = serde_json::to_string(&record).unwrap();
            let back: ClientRecord = serde_json::from_str(&line).unwrap();
            assert_eq!(record, back);
        }
    }

    #[test]
    fn disposition_defaults_to_immediate() {
        let record: ClientRecord =
            serde_json::from_str(r#"{"v":1,"command":{"type":"prompt","text":"hi"}}"#).unwrap();
        assert_eq!(
            record.command,
            ClientCommand::Prompt {
                text: "hi".to_string(),
                disposition: InputDisposition::Immediate,
            }
        );
    }

    #[test]
    fn server_records_roundtrip() {
        let record = ServerRecord {
            v: RPC_PROTOCOL_VERSION,
            id: None,
            event: ServerEvent::PermissionAsk {
                ask_id: "ask-1".to_string(),
                tool: "run_shell".to_string(),
                detail: "rm -rf build".to_string(),
                risk: "run a command".to_string(),
            },
        };
        let line = serde_json::to_string(&record).unwrap();
        let back: ServerRecord = serde_json::from_str(&line).unwrap();
        assert_eq!(record, back);
    }
}
