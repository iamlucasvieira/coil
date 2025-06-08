use thiserror::Error;

#[derive(Debug, Error)]
pub enum EngineError {
    #[error("input error: {0}")]
    InputError(String),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
