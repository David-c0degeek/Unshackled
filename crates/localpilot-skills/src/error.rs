//! Skills error type.

/// Errors from loading or parsing skills.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum SkillError {
    /// A `skill.toml` manifest was invalid; the message names the bad field.
    #[error("invalid skill manifest: {0}")]
    InvalidManifest(String),

    /// A prompt template was invalid or could not be rendered.
    #[error("invalid prompt template: {0}")]
    InvalidTemplate(String),

    /// A filesystem operation failed.
    #[error("skills io error at {path}: {source}")]
    Io {
        path: String,
        #[source]
        source: std::io::Error,
    },
}
