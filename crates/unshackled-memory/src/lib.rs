//! Local project memory store for Unshackled.
//!
//! A flat, inspectable store of tagged entries. Memory is local-only by design:
//! no remote sync, no network. Entries are redacted before they are written, and
//! retrieval ranks by relevance with a token cap and a relevance threshold below
//! which stale entries are not injected. Graph/entity extraction is deliberately
//! deferred until the flat store proves useful.
#![forbid(unsafe_code)]

mod error;

use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use unshackled_config::redact::redact;

pub use error::MemoryError;

/// The kind of a memory entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum MemoryKind {
    ProjectFact,
    DurableDecision,
    RecurringWorkflow,
    DependencyNote,
    FailureFix,
    AcceptedSkill,
}

/// One memory entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryEntry {
    pub id: String,
    pub kind: MemoryKind,
    pub text: String,
    #[serde(default)]
    pub tags: Vec<String>,
    pub created_unix: u64,
    #[serde(default)]
    pub verified: bool,
}

/// Retrieval tuning.
#[derive(Debug, Clone, Copy)]
pub struct RetrievalConfig {
    pub token_cap: usize,
    pub threshold: f64,
}

impl Default for RetrievalConfig {
    fn default() -> Self {
        Self {
            token_cap: 512,
            threshold: 0.25,
        }
    }
}

const ENTRIES_FILE: &str = "entries.jsonl";
const DISABLED_FILE: &str = "disabled";

/// A handle to a workspace's local memory store under `.unshackled/memory`.
#[derive(Debug, Clone)]
pub struct MemoryStore {
    dir: PathBuf,
}

impl MemoryStore {
    /// Open the store under `workspace_root/.unshackled/memory`.
    #[must_use]
    pub fn open(workspace_root: &Path) -> Self {
        Self {
            dir: workspace_root.join(".unshackled").join("memory"),
        }
    }

    fn entries_path(&self) -> PathBuf {
        self.dir.join(ENTRIES_FILE)
    }

    fn disabled_path(&self) -> PathBuf {
        self.dir.join(DISABLED_FILE)
    }

    /// Whether memory injection is enabled (the project can opt out).
    #[must_use]
    pub fn is_enabled(&self) -> bool {
        !self.disabled_path().exists()
    }

    /// Disable memory injection for this project (opt-out).
    ///
    /// # Errors
    /// Returns [`MemoryError`] on filesystem failure.
    pub fn disable(&self) -> Result<(), MemoryError> {
        std::fs::create_dir_all(&self.dir).map_err(|e| MemoryError::io(&self.dir, e))?;
        std::fs::write(self.disabled_path(), b"memory disabled\n")
            .map_err(|e| MemoryError::io(self.disabled_path(), e))
    }

    /// Re-enable memory injection.
    ///
    /// # Errors
    /// Returns [`MemoryError`] on filesystem failure other than a missing flag.
    pub fn enable(&self) -> Result<(), MemoryError> {
        match std::fs::remove_file(self.disabled_path()) {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(MemoryError::io(self.disabled_path(), e)),
        }
    }

    /// Add an entry, redacting its text before persistence. Returns the new id.
    ///
    /// # Errors
    /// Returns [`MemoryError`] on serialization or filesystem failure.
    pub fn add(
        &self,
        kind: MemoryKind,
        text: &str,
        tags: Vec<String>,
        verified: bool,
    ) -> Result<String, MemoryError> {
        let mut entries = self.all()?;
        let id = format!("mem-{}-{}", now_unix(), entries.len() + 1);
        entries.push(MemoryEntry {
            id: id.clone(),
            kind,
            text: redact(text),
            tags,
            created_unix: now_unix(),
            verified,
        });
        self.persist(&entries)?;
        Ok(id)
    }

    /// All entries, in insertion order.
    ///
    /// # Errors
    /// Returns [`MemoryError`] if the store exists but cannot be read/parsed.
    pub fn all(&self) -> Result<Vec<MemoryEntry>, MemoryError> {
        let content = match std::fs::read_to_string(self.entries_path()) {
            Ok(c) => c,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
            Err(e) => return Err(MemoryError::io(self.entries_path(), e)),
        };
        let mut entries = Vec::new();
        for line in content.lines().filter(|l| !l.trim().is_empty()) {
            entries.push(serde_json::from_str(line)?);
        }
        Ok(entries)
    }

    /// Delete an entry by id. Returns whether it existed.
    ///
    /// # Errors
    /// Returns [`MemoryError`] on filesystem failure.
    pub fn delete(&self, id: &str) -> Result<bool, MemoryError> {
        let mut entries = self.all()?;
        let before = entries.len();
        entries.retain(|e| e.id != id);
        let removed = entries.len() != before;
        if removed {
            self.persist(&entries)?;
        }
        Ok(removed)
    }

    /// Score and rank entries for `query`, returning those above the threshold.
    ///
    /// # Errors
    /// Returns [`MemoryError`] if the store cannot be read.
    pub fn search(&self, query: &str) -> Result<Vec<MemoryEntry>, MemoryError> {
        let mut scored: Vec<(f64, MemoryEntry)> = self
            .all()?
            .into_iter()
            .map(|e| (relevance(&e, query), e))
            .filter(|(score, _)| *score > 0.0)
            .collect();
        scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        Ok(scored.into_iter().map(|(_, e)| e).collect())
    }

    /// Retrieve entries relevant to `query` to inject as context, respecting the
    /// relevance threshold and a token cap. Returns nothing when disabled.
    ///
    /// # Errors
    /// Returns [`MemoryError`] if the store cannot be read.
    pub fn retrieve(
        &self,
        query: &str,
        config: RetrievalConfig,
    ) -> Result<Vec<MemoryEntry>, MemoryError> {
        if !self.is_enabled() {
            return Ok(Vec::new());
        }
        let mut scored: Vec<(f64, MemoryEntry)> = self
            .all()?
            .into_iter()
            .map(|e| (relevance(&e, query), e))
            .filter(|(score, _)| *score >= config.threshold)
            .collect();
        scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

        let mut out = Vec::new();
        let mut tokens = 0usize;
        for (_, entry) in scored {
            let cost = entry.text.len() / 4 + 1;
            if tokens + cost > config.token_cap {
                break;
            }
            tokens += cost;
            out.push(entry);
        }
        Ok(out)
    }

    fn persist(&self, entries: &[MemoryEntry]) -> Result<(), MemoryError> {
        std::fs::create_dir_all(&self.dir).map_err(|e| MemoryError::io(&self.dir, e))?;
        let mut body = String::new();
        for entry in entries {
            body.push_str(&serde_json::to_string(entry)?);
            body.push('\n');
        }
        let path = self.entries_path();
        let tmp = path.with_extension("tmp");
        std::fs::write(&tmp, body.as_bytes()).map_err(|e| MemoryError::io(&tmp, e))?;
        std::fs::rename(&tmp, &path).map_err(|e| MemoryError::io(&path, e))
    }
}

/// Relevance of an entry to a query: the fraction of query words present in the
/// entry's text or tags, plus a small bonus for verified entries.
fn relevance(entry: &MemoryEntry, query: &str) -> f64 {
    let query_words: Vec<String> = words(query);
    if query_words.is_empty() {
        return 0.0;
    }
    let haystack = format!(
        "{} {}",
        entry.text.to_ascii_lowercase(),
        entry.tags.join(" ").to_ascii_lowercase()
    );
    let matched = query_words
        .iter()
        .filter(|w| haystack.contains(w.as_str()))
        .count();
    let overlap = matched as f64 / query_words.len() as f64;
    let bonus = if entry.verified { 0.15 } else { 0.0 };
    (overlap + bonus).min(1.0)
}

fn words(text: &str) -> Vec<String> {
    text.to_ascii_lowercase()
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|w| w.len() > 2)
        .map(str::to_string)
        .collect()
}

fn now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fresh() -> (tempfile::TempDir, MemoryStore) {
        let dir = tempfile::tempdir().unwrap();
        let store = MemoryStore::open(dir.path());
        (dir, store)
    }

    #[test]
    fn entries_round_trip_as_inspectable_jsonl() {
        let (_dir, store) = fresh();
        store
            .add(
                MemoryKind::ProjectFact,
                "uses tokio for async",
                vec!["async".into()],
                true,
            )
            .unwrap();
        let entries = store.all().unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].kind, MemoryKind::ProjectFact);
        // Stored as one JSON object per line.
        let raw = std::fs::read_to_string(store.entries_path()).unwrap();
        assert_eq!(raw.lines().count(), 1);
        let _: MemoryEntry = serde_json::from_str(raw.lines().next().unwrap()).unwrap();
    }

    #[test]
    fn text_is_redacted_before_write() {
        let (_dir, store) = fresh();
        let secret = "sk-abcdefghijklmnopqrstuvwxyz0123";
        store
            .add(
                MemoryKind::ProjectFact,
                &format!("key {secret}"),
                vec![],
                false,
            )
            .unwrap();
        let raw = std::fs::read_to_string(store.entries_path()).unwrap();
        assert!(!raw.contains(secret));
        assert!(raw.contains("[REDACTED]"));
    }

    #[test]
    fn retrieval_respects_the_token_cap() {
        let (_dir, store) = fresh();
        for i in 0..10 {
            store
                .add(
                    MemoryKind::ProjectFact,
                    &format!("parser detail number {i} about parsing"),
                    vec![],
                    false,
                )
                .unwrap();
        }
        let config = RetrievalConfig {
            token_cap: 12,
            threshold: 0.1,
        };
        let retrieved = store.retrieve("parser parsing detail", config).unwrap();
        assert!(!retrieved.is_empty());
        assert!(retrieved.len() < 10, "token cap should limit results");
    }

    #[test]
    fn stale_memory_below_threshold_is_not_injected() {
        let (_dir, store) = fresh();
        store
            .add(
                MemoryKind::ProjectFact,
                "the parser handles errors",
                vec![],
                false,
            )
            .unwrap();
        store
            .add(
                MemoryKind::ProjectFact,
                "completely unrelated gardening notes",
                vec![],
                false,
            )
            .unwrap();
        let retrieved = store
            .retrieve("parser errors", RetrievalConfig::default())
            .unwrap();
        assert_eq!(retrieved.len(), 1);
        assert!(retrieved[0].text.contains("parser"));
    }

    #[test]
    fn delete_removes_an_entry_and_disable_stops_injection() {
        let (_dir, store) = fresh();
        let id = store
            .add(MemoryKind::ProjectFact, "parser fact", vec![], false)
            .unwrap();
        assert!(store.delete(&id).unwrap());
        assert!(!store.delete(&id).unwrap());

        store
            .add(
                MemoryKind::ProjectFact,
                "another parser fact",
                vec![],
                false,
            )
            .unwrap();
        store.disable().unwrap();
        assert!(!store.is_enabled());
        assert!(store
            .retrieve("parser", RetrievalConfig::default())
            .unwrap()
            .is_empty());
        store.enable().unwrap();
        assert!(store.is_enabled());
    }
}
