//! Seed retrieved LocalMind context into a session before a turn.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use localpilot_config::{CliOverrides, ConfigPaths, IngestMode};
use localpilot_harness::{ContextHook, SessionRuntime};

/// Cap on the auto-seeded accepted-memory block, so the always-on context stays
/// lean regardless of how large the memory store grows.
const ACCEPTED_MEMORY_CHAR_CAP: usize = 1_200;

/// LocalMind retrieval as a pre-turn context hook. Only accepted, review-gated
/// project memory is contributed as always-on context (lean, and replaced rather
/// than accumulated by the harness). Ingested folder knowledge is pulled on
/// demand through the `knowledge_search` tool instead of being seeded every turn,
/// unless the project opts back into the legacy push behavior via
/// `[ingest] mode = "push"`. Best-effort — a miss or error contributes nothing
/// and never fails the turn.
pub struct LocalMindContext {
    root: PathBuf,
}

impl ContextHook for LocalMindContext {
    fn name(&self) -> &str {
        "localmind-context"
    }

    fn context_for(&self, prompt: &str) -> Option<String> {
        let accepted = localpilot_localmind::context_for(&self.root, prompt)
            .ok()
            .flatten()
            .map(|text| bound(&text, ACCEPTED_MEMORY_CHAR_CAP));
        let ingested = match self.ingest_config() {
            Some(config) if config.enabled && config.mode == IngestMode::Push => {
                localpilot_localmind::ingest_context_for(&self.root, prompt)
                    .ok()
                    .flatten()
            }
            _ => None,
        };
        match (accepted, ingested) {
            (Some(accepted), Some(ingested)) => Some(format!("{accepted}\n{ingested}")),
            (Some(accepted), None) => Some(accepted),
            (None, Some(ingested)) => Some(ingested),
            (None, None) => None,
        }
    }
}

impl LocalMindContext {
    fn ingest_config(&self) -> Option<localpilot_config::IngestConfig> {
        localpilot_config::load(&ConfigPaths::standard(&self.root), &CliOverrides::default())
            .ok()
            .map(|config| config.ingest)
    }
}

/// Truncate `text` to at most `cap` characters, adding a marker when it was cut.
fn bound(text: &str, cap: usize) -> String {
    if text.chars().count() <= cap {
        return text.to_string();
    }
    let truncated: String = text.chars().take(cap).collect();
    format!("{truncated}\n… (memory truncated)")
}

/// Register the LocalMind context hook on a runtime.
pub fn register(cwd: &Path, runtime: &mut SessionRuntime) {
    runtime
        .hooks_mut()
        .register_context_hook(Arc::new(LocalMindContext {
            root: cwd.to_path_buf(),
        }));
}

/// Close out a finished session into LocalMind: extract candidate lessons and
/// enqueue them for review. Best-effort and non-fatal; a no-op when the session
/// produced no transcript. The interactive REPL (the `tui` feature) is the
/// consumer.
#[cfg_attr(not(feature = "tui"), allow(dead_code))]
pub fn close_out(cwd: &Path, session: localpilot_core::SessionId) {
    let store = localpilot_store::Store::open(cwd);
    // Skip an empty session so opening and closing the REPL leaves no artifacts.
    if store
        .read_transcript(session)
        .map(|m| m.is_empty())
        .unwrap_or(true)
    {
        return;
    }
    match localpilot_localmind::closeout_session(cwd, &store, session) {
        Ok(summary) => eprintln!(
            "learning: closed out session — {} candidate(s), {} enqueued for review",
            summary.candidate_count, summary.enqueued_count
        ),
        Err(error) => eprintln!("learning: closeout skipped ({error})"),
    }

    // Keep the code graph current while the workspace is quiet. Bounded so a
    // large edit burst cannot stall shutdown; leftovers wait for the next
    // session close, and an up-to-date graph is a cheap no-op.
    match localpilot_localmind::codegraph_reindex(cwd, CODEGRAPH_BATCH_LIMIT) {
        Ok(summary) if summary.reindexed + summary.pruned > 0 => eprintln!(
            "learning: code graph updated — {} file(s) reindexed, {} pruned{}",
            summary.reindexed,
            summary.pruned,
            if summary.remaining > 0 {
                ", more queued for next session"
            } else {
                ""
            }
        ),
        Ok(_) => {}
        Err(error) => eprintln!("learning: code graph reindex skipped ({error})"),
    }
}

/// How many files one session-close reindex pass may touch.
const CODEGRAPH_BATCH_LIMIT: usize = 64;
