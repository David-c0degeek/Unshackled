//! Review-queue, memory, and audit operations over the LocalMind store.
//!
//! These wrap LocalMind's project store and return plain Unshackled-owned types
//! so callers (the CLI) never name a LocalMind type directly.

use localmind_core::{ReviewAction, ReviewDecision, ReviewItemId};
use localmind_store::{MemoryPersistence, ReviewQueue, ReviewQueueItem};

use crate::LearningError;
use std::path::Path;

/// A review-queue item, flattened for display.
#[derive(Debug, Clone)]
pub struct ReviewSummary {
    pub id: String,
    pub state: String,
    pub session_id: String,
    pub summary: String,
    pub category: String,
    pub confidence: f32,
    pub note: Option<String>,
    pub replacement: Option<String>,
}

/// A reviewer's verdict on a queue item.
#[derive(Debug, Clone)]
pub enum ReviewVerdict {
    Accept,
    Reject,
    Defer,
    Edit { replacement: String },
}

/// A memory search hit.
#[derive(Debug, Clone)]
pub struct SearchHit {
    pub memory_id: String,
    pub score: i64,
    pub path: String,
    pub snippet: String,
}

/// An audit-log entry for a memory change.
#[derive(Debug, Clone)]
pub struct AuditEntry {
    pub id: i64,
    pub kind: String,
    pub actor: String,
    pub subject: String,
    pub at: String,
}

fn summarize(item: &ReviewQueueItem) -> ReviewSummary {
    ReviewSummary {
        id: item.id.to_string(),
        state: format!("{:?}", item.state),
        session_id: item.session_id.to_string(),
        summary: item.candidate.summary().to_string(),
        category: format!("{:?}", item.candidate.category),
        confidence: item.candidate.confidence.value(),
        note: item.note.clone(),
        replacement: item.replacement_summary.clone(),
    }
}

/// List every item in the project's review queue.
///
/// # Errors
/// Returns [`LearningError::Review`] if the queue cannot be opened or read.
pub fn review_list(project_root: &Path) -> Result<Vec<ReviewSummary>, LearningError> {
    let queue = open_queue(project_root)?;
    let items = queue.list().map_err(review_err)?;
    Ok(items.iter().map(summarize).collect())
}

/// Inspect a single review-queue item.
///
/// # Errors
/// Returns [`LearningError::Review`] if the queue cannot be opened or read.
pub fn review_show(
    project_root: &Path,
    item_id: &str,
) -> Result<Option<ReviewSummary>, LearningError> {
    let queue = open_queue(project_root)?;
    let item = queue.get(&ReviewItemId::new(item_id)).map_err(review_err)?;
    Ok(item.as_ref().map(summarize))
}

/// Record a reviewer's verdict on a queue item, returning the new state.
///
/// # Errors
/// Returns [`LearningError`] if the decision or its audit record fails.
pub fn review_decide(
    project_root: &Path,
    item_id: &str,
    verdict: ReviewVerdict,
    reviewer: &str,
    note: Option<String>,
) -> Result<String, LearningError> {
    let (action, replacement_summary) = match verdict {
        ReviewVerdict::Accept => (ReviewAction::Accept, None),
        ReviewVerdict::Reject => (ReviewAction::Reject, None),
        ReviewVerdict::Defer => (ReviewAction::MarkTemporary, None),
        ReviewVerdict::Edit { replacement } => (ReviewAction::Edit, Some(replacement)),
    };
    let persistence = open_memory(project_root)?;
    let queue = open_queue(project_root)?;
    let item = queue
        .decide(ReviewDecision {
            item_id: ReviewItemId::new(item_id),
            action,
            reviewer: reviewer.to_string(),
            decided_at: None,
            note,
            replacement_summary,
            evidence: Vec::new(),
        })
        .map_err(review_err)?;
    persistence
        .record_review_item_audit(&item)
        .map_err(memory_err)?;
    Ok(format!("{:?}", item.state))
}

/// Promote an accepted review item into durable Markdown memory, returning the
/// new memory entry id.
///
/// # Errors
/// Returns [`LearningError::Memory`] if promotion fails.
pub fn promote(project_root: &Path, item_id: &str) -> Result<String, LearningError> {
    let persistence = open_memory(project_root)?;
    let entry = persistence
        .promote_review_item(&ReviewItemId::new(item_id))
        .map_err(memory_err)?;
    Ok(entry.id.to_string())
}

/// Search accepted memory.
///
/// # Errors
/// Returns [`LearningError::Memory`] if the search fails.
pub fn search(project_root: &Path, query: &str) -> Result<Vec<SearchHit>, LearningError> {
    let persistence = open_memory(project_root)?;
    let results = persistence.search(query).map_err(memory_err)?;
    Ok(results
        .into_iter()
        .map(|result| SearchHit {
            memory_id: result.memory_id.to_string(),
            score: result.score,
            path: result.path.display().to_string(),
            snippet: result.snippet,
        })
        .collect())
}

/// Retrieve relevant accepted memory for `query`, formatted as a compact context
/// block to seed into an agent turn. Returns `None` when nothing matches, so the
/// caller injects nothing rather than noise.
///
/// # Errors
/// Returns [`LearningError::Context`] if memory cannot be searched.
pub fn context_for(project_root: &Path, query: &str) -> Result<Option<String>, LearningError> {
    use std::fmt::Write as _;
    let persistence = open_memory(project_root)?;
    let hits = persistence
        .search(query)
        .map_err(|e| LearningError::Context(e.to_string()))?;
    if hits.is_empty() {
        return Ok(None);
    }
    let mut context = String::from("Relevant accepted project memory:\n");
    for hit in hits.iter().take(5) {
        let _ = writeln!(context, "- {}", hit.snippet.trim());
    }
    Ok(Some(context))
}

/// The memory-change audit log.
///
/// # Errors
/// Returns [`LearningError::Memory`] if the audit log cannot be read.
pub fn audit(project_root: &Path) -> Result<Vec<AuditEntry>, LearningError> {
    let persistence = open_memory(project_root)?;
    let records = persistence.audit_records().map_err(memory_err)?;
    Ok(records
        .into_iter()
        .map(|record| AuditEntry {
            id: record.id,
            kind: record.kind,
            actor: record.actor,
            subject: record.subject,
            at: record.happened_at,
        })
        .collect())
}

/// Open the review queue, ensuring the project has a LocalMind config first so a
/// never-closed-out project opens an empty queue rather than erroring.
fn open_queue(project_root: &Path) -> Result<ReviewQueue, LearningError> {
    crate::initialize(project_root)?;
    ReviewQueue::open_project(project_root).map_err(review_err)
}

/// Open memory persistence, ensuring the project is initialized first.
fn open_memory(project_root: &Path) -> Result<MemoryPersistence, LearningError> {
    crate::initialize(project_root)?;
    MemoryPersistence::open_project(project_root).map_err(memory_err)
}

fn review_err(e: impl std::fmt::Display) -> LearningError {
    LearningError::Review(e.to_string())
}

fn memory_err(e: impl std::fmt::Display) -> LearningError {
    LearningError::Memory(e.to_string())
}
