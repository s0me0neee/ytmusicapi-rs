use thiserror::Error;

#[derive(Debug, Error)]
pub enum YtMusicError {
    #[error("Python error: {0}")]
    Python(String),
    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),
}
