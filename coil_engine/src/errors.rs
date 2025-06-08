use thiserror::Error;

/// Error types that can occur during engine operations.
#[derive(Debug, Error)]
pub enum EngineError {
    /// Error occurred during input handling operations.
    #[error("input error: {0}")]
    InputError(String),

    /// I/O error occurred during engine operations.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
