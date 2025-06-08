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
    fn on_event(&mut self, event: Event) -> bool;

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
        Self::validate_target_fps(target_fps)?;
        Ok(Self {
            target_fps,
            input_handler: InputHandler::new()?,
        })
    }

    fn validate_target_fps(target_fps: u32) -> Result<(), EngineError> {
        if target_fps == 0 {
            Err(EngineError::EventLoop(
                "Target FPS must be greater than zero".to_string(),
            ))
        } else {
            Ok(())
        }
    }

    /// Runs the main game loop with the provided game state.
    ///
    /// This method implements a fixed timestep loop with lag compensation.
    /// It will continue running until the game state's `on_event` method
    /// returns `true` for any input event.
    ///
    /// # Arguments
    /// * `state` - The game state implementation to run
    ///
    /// # Returns
    /// * `Ok(())` when the game exits normally
    /// * `Err(EngineError)` if an error occurs during execution
    pub fn run<G: GameState>(&mut self, state: &mut G) -> Result<(), EngineError> {
        let mut previous_time = Instant::now();
        let mut lag_time = Duration::ZERO;
        let frame_duration = Duration::from_secs_f32(1.0 / self.target_fps as f32);

        loop {
            self.input_handler.poll(frame_duration)?;

            for event in self.input_handler.drain() {
                if state.on_event(event) {
                    return Ok(());
                }
            }

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
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    use std::sync::{Arc, Mutex};

    struct MockState {
        update_count: Arc<Mutex<u32>>,
        render_count: Arc<Mutex<u32>>,
        exit_after_n_events: Option<u32>,
    }

    impl MockState {
        fn new() -> Self {
            Self {
                update_count: Arc::new(Mutex::new(0)),
                render_count: Arc::new(Mutex::new(0)),
                exit_after_n_events: None,
            }
        }

        fn new_exit_after_n_events(n: u32) -> Self {
            Self {
                update_count: Arc::new(Mutex::new(0)),
                render_count: Arc::new(Mutex::new(0)),
                exit_after_n_events: Some(n),
            }
        }

        fn get_update_count(&self) -> u32 {
            *self.update_count.lock().unwrap()
        }

        fn get_render_count(&self) -> u32 {
            *self.render_count.lock().unwrap()
        }
    }

    impl GameState for MockState {
        fn update(&mut self, _delta_time: f32) {
            let mut count = self.update_count.lock().unwrap();
            *count += 1;
        }

        fn on_event(&mut self, event: Event) -> bool {
            if let Some(exit_after) = self.exit_after_n_events {
                if self.get_render_count() >= exit_after {
                    return true;
                }
            }

            if let Event::Key(key_event) = event {
                if key_event.code == KeyCode::Esc {
                    return true;
                }
            }
            false
        }

        fn render(&self) {
            let mut count = self.render_count.lock().unwrap();
            *count += 1;
        }
    }

    #[test]
    fn test_event_loop_creation() {
        assert!(EventLoop::new(60).is_ok());
        assert!(EventLoop::new(30).is_ok());
        assert!(EventLoop::new(120).is_ok());
    }

    #[test]
    fn test_event_loop_with_zero_fps() {
        let result = EventLoop::new(0);
        match result {
            Err(EngineError::EventLoop(_)) => {}
            _ => panic!("Expected an error for zero FPS"),
        }
    }

    #[test]
    fn test_game_state_trait_implementation() {
        let mut state = MockState::new();

        state.update(1.0 / 60.0);
        assert_eq!(state.get_update_count(), 1);

        state.render();
        assert_eq!(state.get_render_count(), 1);

        let key_event = Event::Key(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
        assert!(!state.on_event(key_event));

        let esc_event = Event::Key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
        assert!(state.on_event(esc_event));
    }

    #[test]
    fn test_game_state_update_counting() {
        let mut state = MockState::new();

        for i in 1..=10 {
            state.update(1.0 / 60.0);
            assert_eq!(state.get_update_count(), i);
        }
    }

    #[test]
    fn test_game_state_render_counting() {
        let state = MockState::new();

        for i in 1..=5 {
            state.render();
            assert_eq!(state.get_render_count(), i);
        }
    }

    #[test]
    fn test_game_state_exit_conditions() {
        let mut state = MockState::new();

        let char_event = Event::Key(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE));
        assert!(!state.on_event(char_event));

        let enter_event = Event::Key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        assert!(!state.on_event(enter_event));

        let esc_event = Event::Key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
        assert!(state.on_event(esc_event));
    }

    #[test]
    fn test_exit_immediately_state() {
        let mut state = MockState::new_exit_after_n_events(0);

        let any_event = Event::Key(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
        assert!(state.on_event(any_event));
    }
}
