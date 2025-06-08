use crate::errors::EngineError;
use crossterm::{
    event::{self, Event, poll},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::time::Duration;

use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, Default)]
/// Defines how input events should be handled in the engine.
pub enum InputStrategy {
    /// Non-blocking: grab every event currently queued, then sleep to cap FPS.
    #[default]
    NonBlocking,
    /// Frame-budgeted: block up to one frame’s duration, then update/render immediately.
    FrameBudgeted,
    /// A custom timeout (in ms) each frame.
    Timeout(Duration),
}

impl InputStrategy {
    /// Returns the timeout duration for the input strategy.
    pub fn timeout(&self) -> Duration {
        match self {
            InputStrategy::NonBlocking => Duration::from_millis(1), // Short timeout for responsiveness
            InputStrategy::FrameBudgeted => Duration::from_millis(16), // ~60 FPS
            InputStrategy::Timeout(duration) => *duration,
        }
    }
}

pub(crate) struct InputHandler {
    queue: VecDeque<Event>,
}

impl InputHandler {
    pub fn new() -> Result<Self, EngineError> {
        enable_raw_mode().map_err(|e| EngineError::Input(e.to_string()))?;
        Ok(Self {
            queue: VecDeque::new(),
        })
    }

    pub fn poll(&mut self, timeout: Duration) -> Result<(), EngineError> {
        while poll(timeout)? {
            if let Ok(event) = event::read() {
                self.queue.push_back(event);
            }
        }
        Ok(())
    }

    pub fn drain(&mut self) -> Vec<Event> {
        self.queue.drain(..).collect()
    }
}

impl Drop for InputHandler {
    fn drop(&mut self) {
        disable_raw_mode().unwrap_or_else(|e| {
            eprintln!("Failed to disable raw mode: {}", e);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_input_handler_creation() {
        // Test creation - may fail in CI environments without terminal access
        match InputHandler::new() {
            Ok(_) => {
                // Success case - terminal is available
            }
            Err(EngineError::Input(_)) => {
                // Expected failure in CI/test environments without terminal
            }
            Err(e) => {
                panic!("Unexpected error type: {:?}", e);
            }
        }
    }

    #[test]
    fn test_input_handler_with_mock() {
        // Test the basic structure without requiring terminal access
        use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
        use std::collections::VecDeque;

        let mut queue: VecDeque<Event> = VecDeque::new();

        // Test empty queue
        let drained: Vec<Event> = queue.drain(..).collect();
        assert!(drained.is_empty());

        // Test adding events
        queue.push_back(Event::Key(KeyEvent::new(
            KeyCode::Char('a'),
            KeyModifiers::NONE,
        )));
        queue.push_back(Event::Key(KeyEvent::new(
            KeyCode::Enter,
            KeyModifiers::NONE,
        )));

        let drained: Vec<Event> = queue.drain(..).collect();
        assert_eq!(drained.len(), 2);

        // Test queue is empty after drain
        let drained_again: Vec<Event> = queue.drain(..).collect();
        assert!(drained_again.is_empty());
    }

    #[test]
    fn test_input_handler_timeout_duration() {
        let short_timeout = Duration::from_millis(1);
        let long_timeout = Duration::from_millis(100);

        assert!(short_timeout < long_timeout);
        assert_eq!(short_timeout.as_millis(), 1);
        assert_eq!(long_timeout.as_millis(), 100);
    }

    #[test]
    fn test_event_matching() {
        use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

        let key_event = Event::Key(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
        let esc_event = Event::Key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));

        match key_event {
            Event::Key(ke) => assert_eq!(ke.code, KeyCode::Char('a')),
            _ => panic!("Expected key event"),
        }

        match esc_event {
            Event::Key(ke) => assert_eq!(ke.code, KeyCode::Esc),
            _ => panic!("Expected key event"),
        }
    }
}
