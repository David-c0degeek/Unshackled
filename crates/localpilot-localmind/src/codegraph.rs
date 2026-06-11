//! Code-graph maintenance through the host boundary.
//!
//! The host owns workspace access: candidate files are enumerated here with
//! the same ignore discipline as the rest of capture (gitignore-aware walk,
//! no hidden files), and the engine's own boundary applies the project's
//! `excluded_paths` on top. The engine never walks the filesystem itself.

use crate::LearningError;
use localmind_codegraph::{IngestBoundary, Reindexer};
use localmind_store::{GraphStore, ProjectConfig};
use std::path::{Path, PathBuf};

/// Outcome of one bounded reindex pass.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CodeGraphSummary {
    pub reindexed: usize,
    pub pruned: usize,
    pub unchanged: usize,
    pub rejected: usize,
    /// Plan entries left for a later pass when the batch budget ran out.
    pub remaining: usize,
}

/// Runs one bounded, incremental code-graph reindex of the project. Change
/// detection is content-based, so calling this at any lifecycle point is
/// safe: an up-to-date graph is a fast no-op. `batch_limit` caps how many
/// files one pass may touch; leftover work is picked up by the next pass.
pub fn codegraph_reindex(
    project_root: &Path,
    batch_limit: usize,
) -> Result<CodeGraphSummary, LearningError> {
    let config = ProjectConfig::discover(project_root)
        .map_err(|error| LearningError::Config(error.to_string()))?;
    let excluded = config.config.learning.excluded_paths.clone();

    let candidates = source_candidates(project_root);
    let boundary = IngestBoundary::new(project_root, excluded)
        .map_err(|error| LearningError::Graph(error.to_string()))?;
    let store = GraphStore::open_project(project_root)
        .map_err(|error| LearningError::Graph(error.to_string()))?;

    let mut reindexer =
        Reindexer::new().map_err(|error| LearningError::Graph(error.to_string()))?;
    let mut plan = reindexer
        .plan(&boundary, &candidates, &store)
        .map_err(|error| LearningError::Graph(error.to_string()))?;
    let report = reindexer
        .run(&boundary, &store, &mut plan, batch_limit)
        .map_err(|error| LearningError::Graph(error.to_string()))?;

    Ok(CodeGraphSummary {
        reindexed: report.reindexed,
        pruned: report.pruned,
        unchanged: plan.unchanged,
        rejected: plan.rejected.len(),
        remaining: plan.remaining(),
    })
}

/// Source and documentation files under the project root, walked with the
/// host's capture discipline: gitignore-aware and skipping hidden entries.
fn source_candidates(project_root: &Path) -> Vec<PathBuf> {
    let mut candidates = Vec::new();
    for entry in ignore::WalkBuilder::new(project_root).build().flatten() {
        let path = entry.into_path();
        if !path.is_file() {
            continue;
        }
        let indexable = path
            .extension()
            .and_then(|extension| extension.to_str())
            .map(|extension| matches!(extension, "rs" | "md"))
            .unwrap_or(false);
        if indexable {
            candidates.push(path);
        }
    }
    candidates.sort();
    candidates
}

#[cfg(test)]
mod tests {
    use super::codegraph_reindex;
    use std::fs;

    #[test]
    fn reindex_is_incremental_and_honours_exclusions() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = tempfile::tempdir()?;
        let root = temp_dir.path();
        fs::write(
            root.join(".localmind.toml"),
            "[learning]\nenabled = true\nexcluded_paths = [\"private\"]\n",
        )?;
        fs::create_dir_all(root.join("src"))?;
        fs::create_dir_all(root.join("private"))?;
        fs::write(root.join("src/lib.rs"), "pub fn answer() -> u8 { 42 }\n")?;
        fs::write(root.join("private/secret.rs"), "pub fn hidden() {}\n")?;

        let first = codegraph_reindex(root, usize::MAX)?;
        assert_eq!(first.reindexed, 1);
        assert_eq!(first.rejected, 1);
        assert_eq!(first.remaining, 0);

        // Nothing changed: the second pass is a no-op.
        let second = codegraph_reindex(root, usize::MAX)?;
        assert_eq!(second.reindexed, 0);
        assert_eq!(second.unchanged, 1);

        // An edit is picked up; the budget bounds the pass.
        fs::write(
            root.join("src/lib.rs"),
            "pub fn answer() -> u8 { 41 + 1 }\n",
        )?;
        let third = codegraph_reindex(root, usize::MAX)?;
        assert_eq!(third.reindexed, 1);
        Ok(())
    }
}
