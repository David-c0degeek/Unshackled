//! Cross-source context-pack budget allocation.
//!
//! A context pack draws from several derived sources — accepted memory anchors,
//! recent session facts, ingest hits, and code-graph neighbors — that must
//! *compete under one token budget* rather than each getting a fixed slice that
//! crowds the others out. Allocation is two-phase and fully deterministic:
//!
//! 1. **Reserves.** Each source is guaranteed up to a small fraction of the
//!    budget, filled highest-trust source first and highest score within a
//!    source. This keeps a flood of ingest hits from starving a single
//!    high-value accepted-memory anchor.
//! 2. **Shared pool.** Whatever budget the reserves leave is filled by global
//!    score across every remaining candidate, so a strong hit from any source
//!    can still win the leftover space.
//!
//! Every candidate ends up either selected or skipped *with a reason*, so a pack
//! is always inspectable: why each entry is in, and why a high-ranking near-miss
//! is out.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

/// Where a context-pack candidate originated. Declaration order is the
/// reserve-fill priority: earlier sources are filled first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackSource {
    /// A context entry the user explicitly pinned; highest precedence.
    ManualPin,
    /// An accepted, review-gated LocalMind memory.
    AcceptedMemory,
    /// A fact carried from the recent session (compaction digest, etc.).
    RecentSession,
    /// A derived ingest chunk matching the task query.
    Ingest,
    /// A code-graph neighbor of a task-relevant symbol.
    CodeGraph,
}

impl PackSource {
    /// Fill priority within the reserve phase (lower wins).
    fn priority(self) -> u8 {
        match self {
            PackSource::ManualPin => 0,
            PackSource::AcceptedMemory => 1,
            PackSource::RecentSession => 2,
            PackSource::Ingest => 3,
            PackSource::CodeGraph => 4,
        }
    }

    /// Source-quality weight contributed to a candidate's rank. Trusted,
    /// review-gated, or user-pinned sources outrank lexical ingest hits.
    fn quality_weight(self) -> i64 {
        match self {
            PackSource::ManualPin => 40,
            PackSource::AcceptedMemory => 30,
            PackSource::RecentSession => 20,
            PackSource::Ingest => 10,
            PackSource::CodeGraph => 5,
        }
    }

    /// Fraction of the total budget reserved for this source. Reserves
    /// deliberately sum to less than one so a shared pool remains for global
    /// competition.
    fn reserve_fraction(self) -> f64 {
        match self {
            PackSource::ManualPin => 0.15,
            PackSource::AcceptedMemory => 0.20,
            PackSource::RecentSession => 0.15,
            PackSource::Ingest => 0.25,
            PackSource::CodeGraph => 0.10,
        }
    }

    /// Every source, for reserve accounting and reporting.
    pub(crate) fn all() -> [PackSource; 5] {
        [
            PackSource::ManualPin,
            PackSource::AcceptedMemory,
            PackSource::RecentSession,
            PackSource::Ingest,
            PackSource::CodeGraph,
        ]
    }
}

/// One candidate competing for space in a context pack.
#[derive(Debug, Clone)]
pub struct PackCandidate {
    pub source: PackSource,
    pub id: String,
    pub path: Option<String>,
    /// Raw relevance score from the originating search/index.
    pub score: u64,
    pub token_estimate: u64,
    pub snippet: String,
    pub stale: bool,
    /// Recency rank (higher is more recent). Recent-session and current-turn
    /// candidates set this; static derived sources leave it zero.
    pub recency: u64,
    /// The task explicitly names this candidate's file path.
    pub file_match: bool,
    /// Source confidence in `0.0..=1.0` (accepted memory and extraction set
    /// this; lexical sources leave it at the neutral default).
    pub confidence: f32,
}

/// The components of a candidate's composite rank, kept so a pack is auditable:
/// a reader can see exactly why one entry outranked another.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RankSignals {
    pub relevance: i64,
    pub source_quality: i64,
    pub recency: i64,
    pub file_match: i64,
    pub confidence: i64,
    pub stale_penalty: i64,
    pub redundancy_penalty: i64,
    pub final_score: i64,
}

/// A candidate after allocation, carrying the reason it was kept or skipped.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackEntry {
    pub source: PackSource,
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    pub score: u64,
    pub token_estimate: u64,
    pub snippet: String,
    pub stale: bool,
    /// Human-readable inclusion or skip reason.
    pub reason: String,
    /// The rank-signal breakdown that decided this entry's competition.
    #[serde(default)]
    pub signals: RankSignals,
}

impl PackCandidate {
    fn into_entry(self, reason: String, signals: RankSignals) -> PackEntry {
        PackEntry {
            source: self.source,
            id: self.id,
            path: self.path,
            score: self.score,
            token_estimate: self.token_estimate,
            snippet: self.snippet,
            stale: self.stale,
            reason,
            signals,
        }
    }

    /// Dedup key: same path and same leading snippet text is the same content,
    /// even across sources.
    fn dedup_key(&self) -> String {
        let snippet: String = self
            .snippet
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            .chars()
            .take(80)
            .collect::<String>()
            .to_ascii_lowercase();
        format!("{}|{snippet}", self.path.as_deref().unwrap_or(""))
    }
}

/// The outcome of competing candidates under one budget.
#[derive(Debug, Clone, Default)]
pub struct Allocation {
    pub selected: Vec<PackEntry>,
    pub skipped: Vec<PackEntry>,
    pub token_estimate: u64,
    pub per_source_tokens: BTreeMap<PackSource, u64>,
}

/// Reserve token amounts per source for `budget`.
pub(crate) fn reserves(budget: u64) -> BTreeMap<PackSource, u64> {
    PackSource::all()
        .into_iter()
        .map(|source| {
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let amount = (budget as f64 * source.reserve_fraction()) as u64;
            (source, amount)
        })
        .collect()
}

/// Penalty applied to each repeat of a file path already seen higher in the
/// ranking, so repeated files/memories collapse toward one useful entry.
const REDUNDANCY_PENALTY: i64 = 15;
/// Penalty for a stale (superseded) candidate, so newer evidence wins.
const STALE_PENALTY: i64 = 25;

/// Compete `candidates` for `budget` tokens. Each candidate is scored from
/// explicit signals (relevance, source quality, recency, stale and redundancy
/// penalties), then selected reserve-first and shared-by-rank. Deterministic:
/// ties break by source priority then id.
pub(crate) fn allocate(candidates: Vec<PackCandidate>, budget: u64) -> Allocation {
    let signals = rank_all(&candidates);

    // Indices sorted for the reserve phase: source precedence, then rank, then id.
    let mut by_reserve: Vec<usize> = (0..candidates.len()).collect();
    by_reserve.sort_by(|&a, &b| {
        candidates[a]
            .source
            .priority()
            .cmp(&candidates[b].source.priority())
            .then(signals[b].final_score.cmp(&signals[a].final_score))
            .then_with(|| candidates[a].id.cmp(&candidates[b].id))
    });

    let reserves = reserves(budget);
    let mut allocation = Allocation::default();
    let mut used_total = 0_u64;
    let mut used_by_source: BTreeMap<PackSource, u64> = BTreeMap::new();
    let mut seen = BTreeSet::new();
    let mut selected = BTreeSet::new();
    let mut duplicate = BTreeSet::new();

    // Phase 1: reserves. Fill each source up to its guaranteed share.
    for &idx in &by_reserve {
        let candidate = &candidates[idx];
        if !seen.insert(candidate.dedup_key()) {
            duplicate.insert(idx);
            allocation.skipped.push(
                candidate
                    .clone()
                    .into_entry("duplicate content".to_string(), signals[idx]),
            );
            continue;
        }
        let reserve = reserves.get(&candidate.source).copied().unwrap_or(0);
        let src_used = used_by_source.get(&candidate.source).copied().unwrap_or(0);
        let cost = candidate.token_estimate;
        if src_used.saturating_add(cost) <= reserve && used_total.saturating_add(cost) <= budget {
            used_total = used_total.saturating_add(cost);
            *used_by_source.entry(candidate.source).or_default() += cost;
            selected.insert(idx);
            allocation.selected.push(candidate.clone().into_entry(
                format!("included from {} within reserve", label(candidate.source)),
                signals[idx],
            ));
        }
    }

    // Phase 2: shared pool. Compete leftovers globally by rank.
    let mut leftovers: Vec<usize> = (0..candidates.len())
        .filter(|idx| !selected.contains(idx) && !duplicate.contains(idx))
        .collect();
    leftovers.sort_by(|&a, &b| {
        signals[b]
            .final_score
            .cmp(&signals[a].final_score)
            .then(
                candidates[a]
                    .source
                    .priority()
                    .cmp(&candidates[b].source.priority()),
            )
            .then_with(|| candidates[a].id.cmp(&candidates[b].id))
    });
    for idx in leftovers {
        let candidate = &candidates[idx];
        let cost = candidate.token_estimate;
        if used_total.saturating_add(cost) <= budget {
            used_total = used_total.saturating_add(cost);
            *used_by_source.entry(candidate.source).or_default() += cost;
            allocation.selected.push(candidate.clone().into_entry(
                format!(
                    "included from {} within shared budget",
                    label(candidate.source)
                ),
                signals[idx],
            ));
        } else {
            allocation.skipped.push(candidate.clone().into_entry(
                format!("skipped: budget exhausted ({cost} tokens did not fit)"),
                signals[idx],
            ));
        }
    }

    allocation.token_estimate = used_total;
    allocation.per_source_tokens = used_by_source;
    allocation
}

/// Score every candidate. Redundancy is counted over a canonical rank order so a
/// repeated file path is demoted on its second and later appearances.
fn rank_all(candidates: &[PackCandidate]) -> Vec<RankSignals> {
    let base: Vec<RankSignals> = candidates.iter().map(base_signals).collect();
    let mut order: Vec<usize> = (0..candidates.len()).collect();
    order.sort_by(|&a, &b| {
        base[b]
            .final_score
            .cmp(&base[a].final_score)
            .then(
                candidates[a]
                    .source
                    .priority()
                    .cmp(&candidates[b].source.priority()),
            )
            .then_with(|| candidates[a].id.cmp(&candidates[b].id))
    });
    let mut path_seen: BTreeMap<String, i64> = BTreeMap::new();
    let mut signals = base;
    for &idx in &order {
        let path = candidates[idx].path.clone().unwrap_or_default();
        let repeats = *path_seen.get(&path).unwrap_or(&0);
        let penalty = if path.is_empty() {
            0
        } else {
            repeats * REDUNDANCY_PENALTY
        };
        path_seen.insert(path, repeats + 1);
        signals[idx].redundancy_penalty = -penalty;
        signals[idx].final_score -= penalty;
    }
    signals
}

/// Bonus for a task-named file path.
const FILE_MATCH_BONUS: i64 = 20;

/// The order-independent part of a candidate's rank.
fn base_signals(candidate: &PackCandidate) -> RankSignals {
    let relevance = i64::try_from(candidate.score).unwrap_or(i64::MAX);
    let source_quality = candidate.source.quality_weight();
    let recency = i64::try_from(candidate.recency).unwrap_or(i64::MAX).min(50);
    let file_match = if candidate.file_match {
        FILE_MATCH_BONUS
    } else {
        0
    };
    #[allow(clippy::cast_possible_truncation)]
    let confidence = (candidate.confidence.clamp(0.0, 1.0) * 15.0) as i64;
    let stale_penalty = if candidate.stale { -STALE_PENALTY } else { 0 };
    RankSignals {
        relevance,
        source_quality,
        recency,
        file_match,
        confidence,
        stale_penalty,
        redundancy_penalty: 0,
        final_score: relevance + source_quality + recency + file_match + confidence + stale_penalty,
    }
}

fn label(source: PackSource) -> &'static str {
    match source {
        PackSource::ManualPin => "manual pin",
        PackSource::AcceptedMemory => "accepted memory",
        PackSource::RecentSession => "recent session",
        PackSource::Ingest => "ingest",
        PackSource::CodeGraph => "code graph",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn candidate(source: PackSource, id: &str, score: u64, tokens: u64) -> PackCandidate {
        PackCandidate {
            source,
            id: id.to_string(),
            path: Some(format!("{id}.rs")),
            score,
            token_estimate: tokens,
            snippet: format!("snippet {id}"),
            stale: false,
            recency: 0,
            file_match: false,
            confidence: 1.0,
        }
    }

    #[test]
    fn reserves_sum_to_less_than_the_budget() {
        let reserves = reserves(1_000);
        let total: u64 = reserves.values().sum();
        assert!(total < 1_000, "a shared pool must remain, got {total}");
    }

    #[test]
    fn every_candidate_is_selected_or_skipped() {
        let candidates = vec![
            candidate(PackSource::Ingest, "a", 10, 30),
            candidate(PackSource::AcceptedMemory, "b", 5, 30),
            candidate(PackSource::CodeGraph, "c", 8, 30),
        ];
        let n = candidates.len();
        let out = allocate(candidates, 1_000);
        assert_eq!(out.selected.len() + out.skipped.len(), n);
        assert!(out.selected.iter().all(|e| !e.reason.is_empty()));
    }

    #[test]
    fn a_reserve_protects_a_high_value_anchor_from_an_ingest_flood() {
        // Many cheap ingest hits plus one accepted-memory anchor; the anchor
        // must survive on its reserve even though ingest has far more hits.
        let mut candidates = vec![candidate(PackSource::AcceptedMemory, "anchor", 1, 20)];
        for i in 0..50 {
            candidates.push(candidate(PackSource::Ingest, &format!("i{i}"), 100, 20));
        }
        let out = allocate(candidates, 100);
        assert!(
            out.selected
                .iter()
                .any(|e| e.source == PackSource::AcceptedMemory),
            "the anchor must be protected by its reserve"
        );
    }

    #[test]
    fn the_shared_pool_goes_to_the_highest_score() {
        // Two sources, tight budget: after reserves, the leftover goes to the
        // highest-scoring candidate regardless of source.
        let candidates = vec![
            candidate(PackSource::Ingest, "low", 1, 40),
            candidate(PackSource::CodeGraph, "high", 99, 40),
        ];
        let out = allocate(candidates, 60);
        // Budget 60: reserves are ingest 18, code graph 6; neither fits a 40 in
        // reserve, so the shared pool (60) takes exactly one — the higher score.
        assert_eq!(out.selected.len(), 1);
        assert_eq!(out.selected[0].id, "high");
        assert!(out.selected[0].reason.contains("shared budget"));
        assert_eq!(out.skipped.len(), 1);
        assert!(out.skipped[0].reason.contains("budget exhausted"));
    }

    #[test]
    fn duplicates_across_sources_are_skipped_once() {
        let mut a = candidate(PackSource::Ingest, "a", 10, 10);
        let mut b = candidate(PackSource::AcceptedMemory, "b", 10, 10);
        a.path = Some("same.rs".to_string());
        b.path = Some("same.rs".to_string());
        a.snippet = "identical body text".to_string();
        b.snippet = "identical body text".to_string();
        let out = allocate(vec![a, b], 1_000);
        assert_eq!(out.selected.len(), 1);
        assert_eq!(out.skipped.len(), 1);
        assert_eq!(out.skipped[0].reason, "duplicate content");
    }

    #[test]
    fn allocation_never_exceeds_the_budget() {
        let candidates: Vec<_> = (0..20)
            .map(|i| candidate(PackSource::Ingest, &format!("i{i}"), i, 25))
            .collect();
        let out = allocate(candidates, 100);
        assert!(out.token_estimate <= 100);
        let summed: u64 = out.selected.iter().map(|e| e.token_estimate).sum();
        assert_eq!(summed, out.token_estimate);
    }

    #[test]
    fn accepted_memory_quality_can_outrank_a_higher_raw_ingest_score() {
        // Tight shared budget for one slot: an accepted memory with a modest raw
        // score still beats an ingest hit because of its source-quality weight.
        let memory = candidate(PackSource::AcceptedMemory, "m", 5, 40); // 5 + 30 = 35
        let ingest = candidate(PackSource::Ingest, "i", 20, 40); // 20 + 10 = 30
                                                                 // Budget 40 leaves no reserve room (reserves are < 40 each here at 60),
                                                                 // so they compete in the shared pool by final score.
        let out = allocate(vec![ingest, memory], 40);
        assert_eq!(out.selected.len(), 1);
        assert_eq!(out.selected[0].id, "m");
        assert!(out.selected[0].signals.source_quality >= 30);
    }

    #[test]
    fn a_stale_candidate_loses_to_fresher_evidence() {
        let mut stale = candidate(PackSource::Ingest, "old", 30, 40);
        stale.stale = true; // 30 + 10 - 25 = 15
        let fresh = candidate(PackSource::Ingest, "new", 20, 40); // 20 + 10 = 30
        let out = allocate(vec![stale, fresh], 40);
        assert_eq!(out.selected.len(), 1);
        assert_eq!(out.selected[0].id, "new");
        let dropped = out.skipped.iter().find(|e| e.id == "old").unwrap();
        assert_eq!(dropped.signals.stale_penalty, -STALE_PENALTY);
    }

    #[test]
    fn repeated_files_are_demoted_by_a_redundancy_penalty() {
        // Two hits from the same path: the second is penalized so it competes
        // worse than a distinct-file hit of equal raw score.
        let mut a = candidate(PackSource::Ingest, "a", 50, 30);
        let mut b = candidate(PackSource::Ingest, "b", 50, 30);
        a.path = Some("same.rs".to_string());
        b.path = Some("same.rs".to_string());
        let other = candidate(PackSource::Ingest, "c", 45, 30); // distinct file
        let signals = rank_all(&[a, b, other]);
        // One of the same-path entries carries a redundancy penalty.
        assert!(signals.iter().any(|s| s.redundancy_penalty < 0));
    }

    #[test]
    fn an_exact_file_match_lifts_an_otherwise_lower_candidate() {
        let mut named = candidate(PackSource::Ingest, "named", 10, 40); // 10+10+20 = 40
        named.file_match = true;
        let other = candidate(PackSource::Ingest, "other", 25, 40); // 25+10 = 35
        let out = allocate(vec![other, named], 40);
        assert_eq!(out.selected.len(), 1);
        assert_eq!(out.selected[0].id, "named");
        assert_eq!(out.selected[0].signals.file_match, FILE_MATCH_BONUS);
    }

    #[test]
    fn manual_pins_take_the_highest_precedence() {
        // A low-relevance manual pin survives a flood of high-score ingest hits
        // because its reserve is filled first.
        let mut candidates = vec![candidate(PackSource::ManualPin, "pin", 1, 20)];
        for i in 0..50 {
            candidates.push(candidate(PackSource::Ingest, &format!("i{i}"), 100, 20));
        }
        let out = allocate(candidates, 150);
        assert!(
            out.selected
                .iter()
                .any(|e| e.source == PackSource::ManualPin),
            "the manual pin must be protected by its reserve"
        );
    }
}
