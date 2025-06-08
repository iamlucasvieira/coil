use thiserror::Error;

/// Error types that can occur during engine operations.
#[derive(Debug, Error)]
pub enum EngineError {
    /// Error occurred during input handling operations.
    #[error("input error: {0}")]
    Input(String),

    /// I/O error occurred during engine operations.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    /// Event Loop error, typically related to event handling.
    #[error("event loop error: {0}")]
    EventLoop(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::io;

    fn get_test_cases() -> Vec<EngineError> {
        vec![
            EngineError::Input("test input error".to_string()),
            EngineError::Io(io::Error::new(io::ErrorKind::Other, "test io error")),
            EngineError::EventLoop("test event loop error".to_string()),
        ]
    }
    fn get_expected_debug_message(error: &EngineError) -> String {
        match error {
            EngineError::Input(_) => "input error".to_string(),
            EngineError::Io(_) => "io error".to_string(),
            EngineError::EventLoop(_) => "test event loop error".to_string(),
        }
    }

    #[test]
    fn test_engine_error_display() {
        for error in get_test_cases() {
            let error_string = format!("{}", error);
            assert!(error_string.contains(&get_expected_debug_message(&error)));
        }
    }

    #[test]
    fn test_io_error_conversion() {
        let io_error = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let engine_error = EngineError::from(io_error);

        let error_string = format!("{}", engine_error);
        assert!(error_string.contains("io error"));
        assert!(error_string.contains("file not found"));
    }

    #[test]
    fn test_error_trait_implementation() {
        let error = EngineError::Input("test".to_string());
        let _source = error.source(); // Should not panic

        let io_error = io::Error::new(io::ErrorKind::Other, "test");
        let engine_error = EngineError::from(io_error);
        let _source = engine_error.source(); // Should not panic
    }

    #[test]
    fn test_different_input_error_messages() {
        let errors = vec![
            EngineError::Input("keyboard error".to_string()),
            EngineError::Input("mouse error".to_string()),
            EngineError::Input("".to_string()),
        ];

        for error in errors {
            let error_string = format!("{}", error);
            assert!(error_string.starts_with("input error: "));
        }
    }

    #[test]
    fn test_different_io_error_kinds() {
        let io_errors = vec![
            io::Error::new(io::ErrorKind::NotFound, "not found"),
            io::Error::new(io::ErrorKind::PermissionDenied, "permission denied"),
            io::Error::new(io::ErrorKind::TimedOut, "timed out"),
        ];

        for io_error in io_errors {
            let engine_error = EngineError::from(io_error);
            let error_string = format!("{}", engine_error);
            assert!(error_string.starts_with("io error: "));
        }
    }
}
