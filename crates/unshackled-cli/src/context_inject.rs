//! Seed retrieved LocalMind context into a session before a turn.
//!
//! Compiled as a no-op without the `learning` feature, so call sites need no
//! conditional compilation.

use std::path::Path;

use unshackled_harness::SessionRuntime;

/// Inject relevant accepted project memory for `query` as a system message. A
/// no-op when learning is disabled, when nothing matches, or on any retrieval
/// error (best-effort context, never fatal to the turn).
#[cfg(feature = "learning")]
pub fn seed(cwd: &Path, runtime: &mut SessionRuntime, query: &str) {
    if let Ok(Some(context)) = unshackled_localmind::context_for(cwd, query) {
        runtime.seed_system(context);
    }
}

#[cfg(not(feature = "learning"))]
pub fn seed(_cwd: &Path, _runtime: &mut SessionRuntime, _query: &str) {}

/// Close out a finished session into LocalMind: extract candidate lessons and
/// enqueue them for review. Best-effort and non-fatal; a no-op when learning is
/// disabled or the session produced no transcript. The interactive REPL (the
/// `tui` feature) is the consumer.
#[cfg(feature = "learning")]
#[cfg_attr(not(feature = "tui"), allow(dead_code))]
pub fn close_out(cwd: &Path, session: unshackled_core::SessionId) {
    let store = unshackled_store::Store::open(cwd);
    // Skip an empty session so opening and closing the REPL leaves no artifacts.
    if store
        .read_transcript(session)
        .map(|m| m.is_empty())
        .unwrap_or(true)
    {
        return;
    }
    match unshackled_localmind::closeout_session(cwd, &store, session) {
        Ok(summary) => eprintln!(
            "learning: closed out session — {} candidate(s), {} enqueued for review",
            summary.candidate_count, summary.enqueued_count
        ),
        Err(error) => eprintln!("learning: closeout skipped ({error})"),
    }
}

#[cfg(not(feature = "learning"))]
#[cfg_attr(not(feature = "tui"), allow(dead_code))]
pub fn close_out(_cwd: &Path, _session: unshackled_core::SessionId) {}
