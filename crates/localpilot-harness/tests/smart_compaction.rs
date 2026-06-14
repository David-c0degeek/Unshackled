//! Smart-mode context compaction: completed-only cutover and deterministic
//! fallback at the session boundary, plus long-session regression fixtures.
//!
//! These drive the runtime with the fake provider and a scripted summarizer so
//! every smart success and every fallback class is exercised offline.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use async_trait::async_trait;
use localpilot_core::{
    ContentBlock, Message, Role, StructuredSummary, SummarySection, SummarySectionKind, ToolCall,
    ToolResult, ToolUseId,
};
use localpilot_harness::{
    CompactionMode, FallbackReason, RuntimeEvent, SessionConfig, SessionRuntime, StopReason,
    Summarizer, SummarizerTuning,
};
use localpilot_llm::FakeProvider;
use localpilot_recovery::{RecoveryBudget, RecoveryEngine};
use localpilot_sandbox::{Interactivity, PermissionEngine, Profile, ScriptedApprover, Workspace};
use localpilot_store::Store;
use localpilot_tools::ToolRegistry;
use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;

const SUMMARY_TITLE: &str = "Conversation summary for trimmed history:";

/// A summarizer that returns the same scripted result for every call.
struct ScriptedSummarizer(Result<StructuredSummary, FallbackReason>);

#[async_trait]
impl Summarizer for ScriptedSummarizer {
    async fn summarize(
        &self,
        _dropped: &[Vec<Message>],
        _carried: &[String],
        _tuning: SummarizerTuning,
        _cancel: &CancellationToken,
    ) -> Result<StructuredSummary, FallbackReason> {
        self.0.clone()
    }
}

struct Harness {
    _dir: tempfile::TempDir,
    runtime: SessionRuntime,
    events: broadcast::Sender<RuntimeEvent>,
    cancel: CancellationToken,
}

fn smart_runtime(provider: Arc<FakeProvider>, limit: usize) -> Harness {
    let dir = tempfile::tempdir().unwrap();
    let runtime = SessionRuntime::new(
        provider,
        ToolRegistry::with_builtins(),
        PermissionEngine::new(Profile::Default, Vec::new()),
        Box::new(ScriptedApprover::always()),
        Store::open(dir.path()),
        Workspace::new(dir.path()).unwrap(),
        RecoveryEngine::new(RecoveryBudget::default()),
        SessionConfig {
            interactivity: Interactivity::NonInteractive,
            context_token_limit: limit,
            compaction_mode: CompactionMode::SmartWithFallback,
            ..SessionConfig::default()
        },
        Vec::new(),
    );
    let (events, _rx) = broadcast::channel(256);
    Harness {
        _dir: dir,
        runtime,
        events,
        cancel: CancellationToken::new(),
    }
}

fn smart_summary(marker: &str) -> StructuredSummary {
    let mut summary = StructuredSummary::new(
        SUMMARY_TITLE,
        vec![
            format!("goal: {marker}"),
            "progress: did the work".to_string(),
        ],
    );
    summary.sections = vec![SummarySection::new(
        SummarySectionKind::Goal,
        vec![marker.to_string()],
    )];
    summary
}

async fn fill_until_compacted(h: &mut Harness) {
    let filler = "context ".repeat(250);
    for label in ["first", "second", "third"] {
        let prompt = format!("{label} {filler}");
        let reason = h.runtime.run_turn(&prompt, &h.events, &h.cancel).await;
        assert_eq!(reason, StopReason::Done);
    }
}

fn last_request_text(provider: &FakeProvider) -> String {
    let requests = provider.requests();
    let messages = &requests.last().expect("a request").messages;
    messages
        .iter()
        .flat_map(|m| &m.content)
        .filter_map(|b| match b {
            ContentBlock::Text { text } => Some(text.clone()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[tokio::test]
async fn smart_cutover_replaces_the_deterministic_summary() {
    let provider = Arc::new(
        FakeProvider::new()
            .text("one")
            .text("two")
            .text("three")
            .text("after"),
    );
    let mut h = smart_runtime(Arc::clone(&provider), 900);
    h.runtime
        .set_summarizer(Arc::new(ScriptedSummarizer(Ok(smart_summary(
            "SMART_MARKER",
        )))));

    fill_until_compacted(&mut h).await;
    let result = h.runtime.compact_conversation().await;
    assert!(result.compacted);
    assert_eq!(result.used_mode, CompactionMode::SmartWithFallback);
    assert!(result.fallback_reason.is_none());

    // The smart digest reaches the next provider request.
    let reason = h.runtime.run_turn("after", &h.events, &h.cancel).await;
    assert_eq!(reason, StopReason::Done);
    assert!(last_request_text(&provider).contains("SMART_MARKER"));
}

#[tokio::test]
async fn smart_failure_falls_back_to_the_deterministic_projection() {
    let provider = Arc::new(
        FakeProvider::new()
            .text("one")
            .text("two")
            .text("three")
            .text("after"),
    );
    let mut h = smart_runtime(Arc::clone(&provider), 900);
    h.runtime
        .set_summarizer(Arc::new(ScriptedSummarizer(Err(FallbackReason::Timeout))));

    fill_until_compacted(&mut h).await;
    let result = h.runtime.compact_conversation().await;
    assert!(result.compacted);
    assert_eq!(result.used_mode, CompactionMode::Deterministic);
    assert_eq!(
        result.fallback_reason.as_deref(),
        Some("smart summarizer timed out")
    );

    // The deterministic digest (not a smart one) reaches the next request.
    let reason = h.runtime.run_turn("after", &h.events, &h.cancel).await;
    assert_eq!(reason, StopReason::Done);
    assert!(last_request_text(&provider).contains(SUMMARY_TITLE));
}

#[tokio::test]
async fn malformed_smart_output_leaves_active_history_unchanged() {
    // Completed-only cutover: a rejected smart attempt records a fallback and
    // keeps the deterministic projection — it never corrupts active history.
    let provider = Arc::new(
        FakeProvider::new()
            .text("one")
            .text("two")
            .text("three")
            .text("after"),
    );
    let mut h = smart_runtime(Arc::clone(&provider), 900);
    h.runtime
        .set_summarizer(Arc::new(ScriptedSummarizer(Err(FallbackReason::Malformed))));

    fill_until_compacted(&mut h).await;
    let result = h.runtime.compact_conversation().await;
    assert!(result.compacted);
    assert_eq!(result.used_mode, CompactionMode::Deterministic);
    assert_eq!(
        result.fallback_reason.as_deref(),
        Some("smart summary was malformed")
    );
}

#[tokio::test]
async fn long_session_with_repeated_failures_is_digested_under_budget() {
    // A realistic long session: a repeated failing command, a decision change,
    // touched files, and a pending action. Smart-with-fallback must produce a
    // valid request under budget and keep pairing intact.
    // `read_file` on a missing path returns an error result without running any
    // external process — a repeatable failure fixture.
    let provider = Arc::new(
        FakeProvider::new()
            .tool_call(
                "c1",
                "read_file",
                serde_json::json!({ "path": "src/parse.rs" }),
            )
            .text("the build failed; error in src/parse.rs")
            .tool_call(
                "c2",
                "read_file",
                serde_json::json!({ "path": "src/parse.rs" }),
            )
            .text("still failing; decided to switch to a recursive descent parser")
            .tool_call(
                "c3",
                "read_file",
                serde_json::json!({ "path": "src/lex.rs" }),
            )
            .text("next: rewrite the tokenizer")
            .text("after"),
    );
    let mut h = smart_runtime(Arc::clone(&provider), 700);
    h.runtime
        .set_summarizer(Arc::new(ScriptedSummarizer(Ok(smart_summary(
            "rewrite the tokenizer",
        )))));

    let filler = "context ".repeat(120);
    for prompt in ["build it", "build again", "look at the parser"] {
        let reason = h
            .runtime
            .run_turn(&format!("{prompt} {filler}"), &h.events, &h.cancel)
            .await;
        assert_eq!(reason, StopReason::Done);
    }

    let result = h.runtime.compact_conversation().await;
    assert!(result.compacted);
    assert!(result.context_used <= result.context_limit);

    let reason = h.runtime.run_turn("after", &h.events, &h.cancel).await;
    assert_eq!(reason, StopReason::Done);

    let requests = provider.requests();
    let messages = &requests.last().expect("a request").messages;
    // Pairing invariant survives compaction.
    let calls: Vec<_> = messages
        .iter()
        .flat_map(|m| &m.content)
        .filter_map(|b| match b {
            ContentBlock::ToolUse(c) => Some(c.id.clone()),
            _ => None,
        })
        .collect();
    let results: Vec<_> = messages
        .iter()
        .flat_map(|m| &m.content)
        .filter_map(|b| match b {
            ContentBlock::ToolResult(r) => Some(r.id.clone()),
            _ => None,
        })
        .collect();
    assert_eq!(calls, results);
}

#[tokio::test]
async fn media_payloads_never_reach_the_summarizer_or_digest() {
    // A tool result carrying a large base64-like blob is captured for the
    // summarizer input pack, which strips media; the digest never echoes it.
    let dropped = vec![vec![
        Message::text(Role::User, "load the image"),
        Message::new(
            Role::Assistant,
            vec![ContentBlock::ToolUse(ToolCall::new(
                ToolUseId::from("c1"),
                "read_file",
                serde_json::json!({ "path": "img.png" }),
            ))],
        ),
        Message::new(
            Role::Tool,
            vec![ContentBlock::ToolResult(ToolResult::success(
                ToolUseId::from("c1"),
                "AAAA".repeat(500),
            ))],
        ),
    ]];

    // A real provider-backed summarizer would strip the blob in its input pack;
    // here the contract we assert is that a digest echoing such a blob is
    // rejected by the validator, so it can never reach active history.
    let blob = "A".repeat(300);
    let body = serde_json::json!({
        "sections": [ { "kind": "progress", "items": [format!("decoded {blob}")] } ]
    })
    .to_string();
    let provider = Arc::new(FakeProvider::new().text(&body));
    let summarizer = localpilot_harness::ProviderSummarizer::new(provider, "m");
    let cancel = CancellationToken::new();
    let outcome = summarizer
        .summarize(&dropped, &[], SummarizerTuning::default(), &cancel)
        .await;
    assert_eq!(outcome, Err(FallbackReason::Unsupported));
}
