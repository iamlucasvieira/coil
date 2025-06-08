use crate::errors::EngineError;
use crossterm::{
    event::{self, Event, poll},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::time::Duration;

use std::collections::VecDeque;

pub(crate) struct InputHandler {
    queue: VecDeque<Event>,
}

impl InputHandler {
    pub fn new() -> Result<Self, EngineError> {
        enable_raw_mode().map_err(|e| EngineError::InputError(e.to_string()))?;
        Ok(Self {
            queue: VecDeque::new(),
        })
    }

    pub fn poll(&mut self, timeout: Duration) -> Result<(), EngineError> {
        if poll(timeout).map_err(|e| EngineError::InputError(e.to_string()))? {
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
