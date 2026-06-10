//! Context compaction.
//!
//! When the conversation approaches the context limit, the oldest exchanges are
//! dropped — but never in a way that separates a tool call from its result. We
//! group messages into exchanges bounded by user turns and drop whole oldest
//! exchanges, always keeping any leading system messages and the most recent
//! exchange.

use localpilot_core::{ContentBlock, Message, Role, StructuredSummary};

/// Title line for a compaction summary. Shared shape with harness branch
/// summaries via [`StructuredSummary`].
const SUMMARY_TITLE: &str = "Conversation summary for trimmed history:";

/// Floor for a truncated tool-result output during the last-resort truncation
/// pass: enough to keep the head of the output meaningful.
const TRUNCATED_OUTPUT_CHARS: usize = 240;

/// A rough token estimate (~4 characters per token) over message text.
#[must_use]
pub fn estimate_tokens(messages: &[Message]) -> usize {
    messages.iter().map(message_chars).sum::<usize>() / 4
}

fn message_chars(message: &Message) -> usize {
    message
        .content
        .iter()
        .map(|block| match block {
            ContentBlock::Text { text } | ContentBlock::Reasoning { text, .. } => text.len(),
            ContentBlock::ToolUse(call) => call.name.len() + call.input.to_string().len(),
            ContentBlock::ToolResult(result) => result.output.len(),
            _ => 0,
        })
        .sum()
}

/// Compact `messages` to fit under `token_limit`, preserving tool-call/result
/// pairing and leading system messages.
#[must_use]
pub fn compact(messages: Vec<Message>, token_limit: usize) -> Vec<Message> {
    compact_with_summary(messages, token_limit).messages
}

/// Result of compacting a conversation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompactionResult {
    /// Messages to send to the provider.
    pub messages: Vec<Message>,
    /// Whether older messages were removed (or oversized outputs truncated).
    pub compacted: bool,
    /// The structured digest of what was trimmed, when anything was.
    pub summary: Option<StructuredSummary>,
}

/// Compact `messages` and inject a bounded summary when old exchanges are
/// removed. The summary is deterministic and intentionally factual: it keeps the
/// task state visible without asking another model to summarize.
#[must_use]
pub fn compact_with_summary(messages: Vec<Message>, token_limit: usize) -> CompactionResult {
    if estimate_tokens(&messages) <= token_limit {
        return CompactionResult {
            messages,
            compacted: false,
            summary: None,
        };
    }

    let system_count = messages
        .iter()
        .take_while(|m| m.role == Role::System)
        .count();
    let mut system = messages[..system_count].to_vec();
    let body = &messages[system_count..];

    // Iterative compaction: a summary injected by an earlier compaction is
    // carried forward into the next digest instead of accumulating as extra
    // system messages.
    let mut carried: Vec<String> = Vec::new();
    system.retain(|message| match first_text(message) {
        Some(text) if text.starts_with(SUMMARY_TITLE) => {
            carried.extend(
                text.lines()
                    .skip(1)
                    .map(|line| line.trim_start_matches("- ").to_string()),
            );
            false
        }
        _ => true,
    });
    let fold_carried = |summary: Option<StructuredSummary>| -> Option<StructuredSummary> {
        const MAX_ENTRIES: usize = 8;
        if carried.is_empty() {
            return summary;
        }
        let mut entries = carried.clone();
        entries.extend(summary.map(|s| s.entries).unwrap_or_default());
        let excess = entries.len().saturating_sub(MAX_ENTRIES);
        entries.drain(..excess);
        Some(StructuredSummary::new(SUMMARY_TITLE, entries))
    };

    // Group the body into exchanges that each start at a user message, so a tool
    // call and its result always live in the same exchange.
    let mut exchanges: Vec<Vec<Message>> = Vec::new();
    for message in body {
        if message.role == Role::User || exchanges.is_empty() {
            exchanges.push(Vec::new());
        }
        if let Some(last) = exchanges.last_mut() {
            last.push(message.clone());
        }
    }

    let mut dropped = Vec::new();

    // Drop oldest exchanges until under the limit, always keeping the last one.
    while exchanges.len() > 1 {
        let candidate: Vec<Message> = system
            .iter()
            .cloned()
            .chain(exchanges.iter().flatten().cloned())
            .collect();
        if estimate_tokens(&candidate) <= token_limit {
            break;
        }
        dropped.push(exchanges.remove(0));
    }

    let mut summary = fold_carried(structured_summary(&dropped));

    // If a single very large recent window still exceeds the limit, keep
    // removing whole oldest exchanges before considering the summary. Removing
    // individual messages here can strand a tool_result without its tool_use.
    while exchanges.len() > 1
        && estimate_tokens(&build_messages(&system, summary.as_ref(), &exchanges)) > token_limit
    {
        dropped.push(exchanges.remove(0));
        summary = fold_carried(structured_summary(&dropped));
    }

    let mut out = build_messages(&system, summary.as_ref(), &exchanges);
    if estimate_tokens(&out) > token_limit && !dropped.is_empty() {
        for (max_exchanges, max_user_chars) in [(4, 60), (2, 60), (1, 60), (1, 30)] {
            summary = fold_carried(structured_summary_with(
                &dropped,
                max_exchanges,
                max_user_chars,
            ));
            out = build_messages(&system, summary.as_ref(), &exchanges);
            if estimate_tokens(&out) <= token_limit {
                break;
            }
        }
        if estimate_tokens(&out) > token_limit {
            summary = None;
            out = build_messages(&system, None, &exchanges);
        }
    }

    // Last resort: nothing left to drop (a single oversized kept exchange —
    // typically one huge tool result) can still exceed the limit. Truncate
    // tool-result outputs, oldest first, rather than giving up over budget.
    // Truncating only outputs never separates a tool_use from its result.
    if estimate_tokens(&out) > token_limit {
        truncate_oldest_tool_results(&mut out, token_limit);
    }

    // Even when the summary message was dropped to fit the budget, the result
    // digest keeps the carried entries so the event log loses nothing.
    let digest = summary.or_else(|| fold_carried(None)).unwrap_or_else(|| {
        StructuredSummary::new(
            SUMMARY_TITLE,
            vec!["older exchanges were trimmed".to_string()],
        )
    });

    CompactionResult {
        messages: out,
        compacted: true,
        summary: Some(digest),
    }
}

/// Truncate kept tool-result outputs, oldest first, stopping as soon as the
/// conversation fits (or every output is already truncated). The pairing
/// invariant is untouched: only the *content* of results shrinks, never their
/// presence.
fn truncate_oldest_tool_results(messages: &mut [Message], token_limit: usize) {
    for index in 0..messages.len() {
        if estimate_tokens(messages) <= token_limit {
            return;
        }
        for block in &mut messages[index].content {
            if let ContentBlock::ToolResult(result) = block {
                if result.output.chars().count() > TRUNCATED_OUTPUT_CHARS {
                    let mut kept = truncate(&result.output, TRUNCATED_OUTPUT_CHARS);
                    kept.push_str("\n[output truncated during context compaction]");
                    result.output = kept;
                }
            }
        }
    }
}

/// Merge runs of consecutive `Role::System` messages into a single system
/// message, preserving order and content. Compaction injects its summary as a
/// system message right after the agent prompt, which would otherwise reach the
/// provider as two consecutive system messages — fine for the Anthropic adapter
/// (it concatenates all system blocks) but surfaced verbatim by the OpenAI-style
/// adapter. Folding them keeps a single leading system block on every wire.
/// Only *adjacent* system messages merge, so a lone system message elsewhere in
/// the history is left untouched.
#[must_use]
pub fn merge_consecutive_system(messages: Vec<Message>) -> Vec<Message> {
    let mut out: Vec<Message> = Vec::with_capacity(messages.len());
    for message in messages {
        match out.last_mut() {
            Some(last) if last.role == Role::System && message.role == Role::System => {
                last.content.extend(message.content);
            }
            _ => out.push(message),
        }
    }
    out
}

fn build_messages(
    system: &[Message],
    summary: Option<&StructuredSummary>,
    exchanges: &[Vec<Message>],
) -> Vec<Message> {
    system
        .iter()
        .cloned()
        .chain(summary.map(|digest| Message::text(Role::System, digest.render())))
        .chain(exchanges.iter().flatten().cloned())
        .collect()
}

fn structured_summary(dropped: &[Vec<Message>]) -> Option<StructuredSummary> {
    structured_summary_with(dropped, 4, 120)
}

fn structured_summary_with(
    dropped: &[Vec<Message>],
    max_exchanges: usize,
    max_user_chars: usize,
) -> Option<StructuredSummary> {
    if dropped.is_empty() {
        return None;
    }
    let entries = dropped
        .iter()
        .rev()
        .take(max_exchanges)
        .rev()
        .filter_map(|exchange| summarize_exchange(exchange, max_user_chars))
        .collect();
    Some(StructuredSummary::new(SUMMARY_TITLE, entries))
}

fn summarize_exchange(exchange: &[Message], max_user_chars: usize) -> Option<String> {
    let user = exchange
        .iter()
        .find(|message| message.role == Role::User)
        .and_then(first_text)
        .map(|text| truncate(text, max_user_chars));
    let tools: Vec<String> = exchange
        .iter()
        .flat_map(|message| &message.content)
        .filter_map(|block| match block {
            ContentBlock::ToolUse(call) => Some(call.name.clone()),
            _ => None,
        })
        .collect();
    match (user, tools.is_empty()) {
        (Some(user), true) => Some(format!("user asked: {user}")),
        (Some(user), false) => Some(format!(
            "user asked: {user}; tools used: {}",
            tools.join(", ")
        )),
        (None, false) => Some(format!("tools used: {}", tools.join(", "))),
        (None, true) => None,
    }
}

fn first_text(message: &Message) -> Option<&str> {
    message.content.iter().find_map(|block| match block {
        ContentBlock::Text { text } => Some(text.as_str()),
        _ => None,
    })
}

fn truncate(text: &str, max_chars: usize) -> String {
    let mut out = String::new();
    for ch in text.chars().take(max_chars) {
        out.push(ch);
    }
    if text.chars().count() > max_chars {
        out.push_str("...");
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use localpilot_core::{ToolCall, ToolResult, ToolUseId};

    fn user(text: &str) -> Message {
        Message::text(Role::User, text)
    }

    fn tool_exchange(id: &str) -> Vec<Message> {
        vec![
            Message::new(
                Role::Assistant,
                vec![ContentBlock::ToolUse(ToolCall::new(
                    ToolUseId::from(id),
                    "read_file",
                    serde_json::json!({ "path": "a" }),
                ))],
            ),
            Message::new(
                Role::Tool,
                vec![ContentBlock::ToolResult(ToolResult::success(
                    ToolUseId::from(id),
                    "x".repeat(400),
                ))],
            ),
        ]
    }

    #[test]
    fn merge_consecutive_system_folds_only_adjacent_system_messages() {
        let messages = vec![
            Message::text(Role::System, "agent prompt"),
            Message::text(Role::System, "summary"),
            Message::text(Role::User, "hi"),
            Message::text(Role::System, "late note"),
        ];
        let merged = merge_consecutive_system(messages);

        assert_eq!(
            merged.iter().map(|m| m.role).collect::<Vec<_>>(),
            vec![Role::System, Role::User, Role::System]
        );
        let leading: Vec<&str> = merged[0]
            .content
            .iter()
            .filter_map(|block| match block {
                ContentBlock::Text { text } => Some(text.as_str()),
                _ => None,
            })
            .collect();
        assert_eq!(leading, vec!["agent prompt", "summary"]);
    }

    #[test]
    fn under_limit_is_unchanged() {
        let messages = vec![user("hi")];
        assert_eq!(compact(messages.clone(), 1000), messages);
    }

    #[test]
    fn compaction_preserves_tool_result_pairing() {
        let mut messages = vec![Message::text(Role::System, "sys")];
        for i in 0..6 {
            messages.push(user(&format!("turn {i}")));
            messages.extend(tool_exchange(&format!("call_{i}")));
        }
        let compacted = compact(messages, 50);

        // Every tool result kept must have its tool call kept too, and vice versa.
        let call_ids: Vec<_> = compacted
            .iter()
            .flat_map(|m| &m.content)
            .filter_map(|b| match b {
                ContentBlock::ToolUse(c) => Some(c.id.clone()),
                _ => None,
            })
            .collect();
        let result_ids: Vec<_> = compacted
            .iter()
            .flat_map(|m| &m.content)
            .filter_map(|b| match b {
                ContentBlock::ToolResult(r) => Some(r.id.clone()),
                _ => None,
            })
            .collect();
        assert_eq!(call_ids, result_ids);
        // The system message is always kept.
        assert_eq!(compacted.first().map(|m| m.role), Some(Role::System));
        // It actually dropped something.
        assert!(call_ids.len() < 6);
    }

    #[test]
    fn final_trimming_does_not_orphan_tool_results() {
        let mut messages = vec![Message::text(Role::System, "sys")];
        for i in 0..4 {
            messages.push(user(&format!("turn {i} {}", "x".repeat(200))));
            messages.extend(tool_exchange(&format!("call_{i}")));
        }
        let compacted = compact(messages, 25);

        let call_ids: Vec<_> = compacted
            .iter()
            .flat_map(|m| &m.content)
            .filter_map(|b| match b {
                ContentBlock::ToolUse(c) => Some(c.id.clone()),
                _ => None,
            })
            .collect();
        let result_ids: Vec<_> = compacted
            .iter()
            .flat_map(|m| &m.content)
            .filter_map(|b| match b {
                ContentBlock::ToolResult(r) => Some(r.id.clone()),
                _ => None,
            })
            .collect();
        assert_eq!(call_ids, result_ids);
    }

    #[test]
    fn a_single_oversized_exchange_is_truncated_not_given_up_on() {
        // One huge tool result in the only (kept) exchange: nothing can be
        // dropped, so the output itself must shrink to fit the budget.
        let mut messages = vec![
            Message::text(Role::System, "sys"),
            user("read the big file"),
        ];
        messages.push(Message::new(
            Role::Assistant,
            vec![ContentBlock::ToolUse(ToolCall::new(
                ToolUseId::from("big"),
                "read_file",
                serde_json::json!({ "path": "big.txt" }),
            ))],
        ));
        messages.push(Message::new(
            Role::Tool,
            vec![ContentBlock::ToolResult(ToolResult::success(
                ToolUseId::from("big"),
                "x".repeat(64 * 1024),
            ))],
        ));

        let result = compact_with_summary(messages, 500);

        assert!(result.compacted);
        assert!(
            estimate_tokens(&result.messages) <= 500,
            "still over budget: {}",
            estimate_tokens(&result.messages)
        );
        // Pairing held: the tool_use and its (truncated) result both survive.
        let calls = result
            .messages
            .iter()
            .flat_map(|m| &m.content)
            .filter(|b| matches!(b, ContentBlock::ToolUse(_)))
            .count();
        let results: Vec<&str> = result
            .messages
            .iter()
            .flat_map(|m| &m.content)
            .filter_map(|b| match b {
                ContentBlock::ToolResult(r) => Some(r.output.as_str()),
                _ => None,
            })
            .collect();
        assert_eq!(calls, 1);
        assert_eq!(results.len(), 1);
        assert!(results[0].contains("[output truncated during context compaction]"));
    }

    #[test]
    fn compaction_result_carries_a_structured_summary() {
        let mut messages = vec![Message::text(Role::System, "sys")];
        for i in 0..6 {
            messages.push(user(&format!("turn {i} {}", "x".repeat(120))));
            messages.extend(tool_exchange(&format!("call_{i}")));
        }
        let result = compact_with_summary(messages, 160);
        assert!(result.compacted);
        let summary = result.summary.expect("a digest of what was trimmed");
        assert_eq!(summary.title, SUMMARY_TITLE);
        assert!(!summary.entries.is_empty());
    }

    #[test]
    fn a_previous_summary_feeds_the_next_compaction() {
        // First compaction produces a summary; a manually compacted history
        // carries it as a system message. The next compaction folds those
        // entries into the new digest instead of stacking summary messages.
        let mut messages = vec![
            Message::text(Role::System, "sys"),
            Message::text(
                Role::System,
                format!(
                    "{SUMMARY_TITLE}
- user asked: earlier work"
                ),
            ),
        ];
        for i in 0..6 {
            messages.push(user(&format!("turn {i} {}", "x".repeat(120))));
            messages.extend(tool_exchange(&format!("call_{i}")));
        }

        let result = compact_with_summary(messages, 320);
        assert!(result.compacted);
        let summary = result.summary.expect("a digest");
        assert!(
            summary.entries.iter().any(|e| e.contains("earlier work")),
            "carried entries: {:?}",
            summary.entries
        );
        // Exactly one summary system message in the output.
        let summary_messages = result
            .messages
            .iter()
            .filter(|m| {
                m.role == Role::System
                    && first_text(m).is_some_and(|t| t.starts_with(SUMMARY_TITLE))
            })
            .count();
        assert_eq!(summary_messages, 1);
    }

    #[test]
    fn compaction_injects_a_bounded_summary() {
        let mut messages = vec![Message::text(Role::System, "sys")];
        for i in 0..8 {
            messages.push(user(&format!("turn {i} {}", "x".repeat(80))));
            messages.extend(tool_exchange(&format!("call_{i}")));
        }

        let result = compact_with_summary(messages, 160);
        assert!(result.compacted);
        let system_text: Vec<_> = result
            .messages
            .iter()
            .filter(|message| message.role == Role::System)
            .filter_map(first_text)
            .collect();
        assert!(system_text
            .iter()
            .any(|text| text.contains("Conversation summary for trimmed history")));
        assert!(estimate_tokens(&result.messages) <= 160);
    }
}
