//! A structured, deterministic summary format.
//!
//! One shape serves every place the runtime condenses history into a bounded
//! digest: context compaction (trimmed exchanges) and harness branch closures
//! (abandoned step attempts). Keeping the format shared means a reader of the
//! event log or a compacted conversation learns one shape, and tests can pin
//! it once.

use serde::{Deserialize, Serialize};

/// A bounded, factual digest: a title line and itemized entries. Deterministic
/// by construction — no model involvement.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct StructuredSummary {
    /// What this summary condenses (one line, ends with a colon by convention).
    pub title: String,
    /// Itemized entries, most relevant first. Rendered as `- ` bullets.
    pub entries: Vec<String>,
}

impl StructuredSummary {
    /// A summary with a title and entries.
    #[must_use]
    pub fn new(title: impl Into<String>, entries: Vec<String>) -> Self {
        Self {
            title: title.into(),
            entries,
        }
    }

    /// Render as plain text: the title line followed by `- ` bullets.
    #[must_use]
    pub fn render(&self) -> String {
        let mut out = self.title.clone();
        for entry in &self.entries {
            out.push('\n');
            out.push_str("- ");
            out.push_str(entry);
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_title_and_bullets() {
        let summary = StructuredSummary::new(
            "Conversation summary for trimmed history:",
            vec![
                "user asked: fix the bug".to_string(),
                "tools used: read_file".to_string(),
            ],
        );
        assert_eq!(
            summary.render(),
            "Conversation summary for trimmed history:\n- user asked: fix the bug\n- tools used: read_file"
        );
    }

    #[test]
    fn empty_entries_render_title_only() {
        let summary = StructuredSummary::new("Nothing to report:", Vec::new());
        assert_eq!(summary.render(), "Nothing to report:");
    }

    #[test]
    fn roundtrips_through_serde() {
        let summary = StructuredSummary::new("t:", vec!["a".to_string()]);
        let json = serde_json::to_string(&summary).unwrap();
        let back: StructuredSummary = serde_json::from_str(&json).unwrap();
        assert_eq!(summary, back);
    }
}
