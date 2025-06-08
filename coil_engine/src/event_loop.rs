use crate::errors::EngineError;
use crate::input::InputHandler;
use crossterm::event::Event;
use std::time::{Duration, Instant};

/// Trait that defines the interface for game logic implementation.
/// 
/// Game implementations must implement this trait to define how the game
/// updates, handles events, and renders.
pub trait GameState {
    /// Updates the game state based on the elapsed time since the last frame.
    /// 
    /// # Arguments
    /// * `delta_time` - Time elapsed since the last update in seconds
    fn update(&mut self, delta_time: f32);
    
    /// Handles input events from the terminal.
    /// 
    /// # Arguments
    /// * `event` - The input event to handle
    /// 
    /// # Returns
    /// * `true` if the game should exit, `false` to continue running
    fn on_even(&mut self, event: Event) -> bool;
    
    /// Renders the current game state to the terminal.
    fn render(&self);
}

/// Main event loop that manages game timing and coordinates game state updates.
/// 
/// The event loop uses a fixed timestep with lag compensation to ensure
/// consistent game timing regardless of frame rate variations.
pub struct EventLoop {
    target_fps: u32,
    input_handler: InputHandler,
}

impl EventLoop {
    /// Creates a new event loop with the specified target frame rate.
    /// 
    /// # Arguments
    /// * `target_fps` - The target frames per second for the game loop
    /// 
    /// # Returns
    /// * `Ok(EventLoop)` on success
    /// * `Err(EngineError)` if input handler initialization fails
    pub fn new(target_fps: u32) -> Result<Self, EngineError> {
        Ok(Self {
            target_fps,
            input_handler: InputHandler::new()?,
        })
    }

    /// Runs the main game loop with the provided game state.
    /// 
    /// This method implements a fixed timestep loop with lag compensation.
    /// It will continue running until the game state's `on_even` method
    /// returns `true` for any input event.
    /// 
    /// # Arguments
    /// * `state` - The game state implementation to run
    /// 
    /// # Returns
    /// * `Ok(())` when the game exits normally
    /// * `Err(EngineError)` if an error occurs during execution
    pub fn run<T: GameState>(&mut self, mut state: T) -> Result<(), EngineError> {
        let mut previous_time = Instant::now();
        let mut lag_time = Duration::ZERO;
        let frame_duration = Duration::from_secs_f32(1.0 / self.target_fps as f32);

        self.input_handler.poll(frame_duration)?;

        for event in self.input_handler.drain() {
            if state.on_even(event) {
                return Ok(());
            }
        }

        loop {
            let current_time = Instant::now();
            let elapsed_time = current_time.duration_since(previous_time);
            lag_time += elapsed_time;
            previous_time = current_time;

            while lag_time >= frame_duration {
                state.update(frame_duration.as_secs_f32());
                lag_time -= frame_duration;
            }

            state.render();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::KeyCode;

    struct MockState;

    impl GameState for MockState {
        fn update(&mut self, _delta_time: f32) {}

        fn on_even(&mut self, event: Event) -> bool {
            if let Event::Key(key_event) = event {
                if key_event.code == KeyCode::Esc {
                    return true; // Exit on Esc key
                }
            }
            false
        }

        fn render(&self) {}
    }

    #[test]
    fn test_event_loop() {
        let mut event_loop = EventLoop::new(60).unwrap();
        let state = MockState;
        event_loop.run(state).unwrap();
    }
}
