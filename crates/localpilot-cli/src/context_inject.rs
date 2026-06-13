//! Seed retrieved LocalMind context into a session before a turn.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use localpilot_config::{CliOverrides, ConfigPaths};
use localpilot_harness::{ContextHook, SessionRuntime};

/// LocalMind retrieval as a pre-turn context hook: relevant accepted project
/// memory is contributed as system context for the upcoming turn. Best-effort
/// — a retrieval miss or error contributes nothing and never fails the turn.
pub struct LocalMindContext {
    root: PathBuf,
    auto_ingest: bool,
}

impl ContextHook for LocalMindContext {
    fn name(&self) -> &str {
        "localmind-context"
    }

    fn context_for(&self, prompt: &str) -> Option<String> {
        let accepted = localpilot_localmind::context_for(&self.root, prompt)
            .ok()
            .flatten();
        let ingested = self.ingested_context_for(prompt);
        match (accepted, ingested) {
            (Some(accepted), Some(ingested)) => Some(format!("{accepted}\n{ingested}")),
            (Some(accepted), None) => Some(accepted),
            (None, Some(ingested)) => Some(ingested),
            (None, None) => None,
        }
    }
}

impl LocalMindContext {
    fn ingested_context_for(&self, prompt: &str) -> Option<String> {
        let config =
            localpilot_config::load(&ConfigPaths::standard(&self.root), &CliOverrides::default())
                .ok()?
                .ingest;
        if !config.enabled {
            return None;
        }
        if self.auto_ingest && matches!(localpilot_localmind::ingest_status(&self.root), Ok(None)) {
            let _ = localpilot_localmind::ingest_run(
                &self.root,
                &config,
                localpilot_localmind::RunMode::Full,
            );
        }
        localpilot_localmind::ingest_context_for(&self.root, prompt)
            .ok()
            .flatten()
    }
}

/// Register the LocalMind context hook on a runtime.
pub fn register(cwd: &Path, runtime: &mut SessionRuntime) {
    runtime
        .hooks_mut()
        .register_context_hook(Arc::new(LocalMindContext {
            root: cwd.to_path_buf(),
            auto_ingest: false,
        }));
}

/// Register LocalMind retrieval and allow one bounded first-use ingest pass.
///
/// This is intended for the trusted interactive REPL path. Non-interactive
/// modes use [`register`] so a plain prompt never creates project files.
#[cfg_attr(not(feature = "tui"), allow(dead_code))]
pub fn register_auto_ingest(cwd: &Path, runtime: &mut SessionRuntime) {
    runtime
        .hooks_mut()
        .register_context_hook(Arc::new(LocalMindContext {
            root: cwd.to_path_buf(),
            auto_ingest: true,
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
