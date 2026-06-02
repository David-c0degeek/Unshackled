//! Memory error type.

/// Errors from the local memory store.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum MemoryError {
    /// A filesystem operation failed.
    #[error("memory io error at {path}: {source}")]
    Io {
        path: String,
        #[source]
        source: std::io::Error,
    },

    /// An entry could not be serialized or deserialized.
    #[error("memory serialization error: {0}")]
    Serde(#[from] serde_json::Error),
}

impl MemoryError {
    pub(crate) fn io(path: impl AsRef<std::path::Path>, source: std::io::Error) -> Self {
        Self::Io {
            path: path.as_ref().display().to_string(),
            source,
        }
    }
}
