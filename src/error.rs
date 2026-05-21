use thiserror::Error;

/// Error type returned by all [`YTMusic`](crate::YTMusic) methods.
#[derive(Debug, Error)]
pub enum YtMusicError {
    /// An error raised by the underlying Python library (e.g. network failure, auth error).
    #[error("Python error: {0}")]
    Python(String),
    /// JSON (de)serialization failed — should not occur in normal usage.
    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),
}
