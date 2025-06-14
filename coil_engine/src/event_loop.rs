use crate::config::GameConfig;
use crate::errors::EngineError;
use crate::input::InputHandler;
use crate::nodes::Node;
use crate::renderer::{BasicRenderer, Renderer};
use log::{debug, warn};
use std::time::{Duration, Instant};

/// Main event loop that manages game timing and coordinates game state updates.
///
/// The event loop uses a fixed timestep with lag compensation to ensure
/// consistent game timing regardless of frame rate variations. It supports
/// state machines and entity loops for complex game logic.
pub struct EventLoop<'a> {
    input_handler: InputHandler,
    renderer: BasicRenderer,
    config: &'a GameConfig,
}

impl<'a> EventLoop<'a> {
    /// Creates a new event loop.
    ///
    /// # Returns
    /// * `Ok(EventLoop)` on success
    /// * `Err(EngineError)` if input handler initialization fails
    pub fn new(config: &'a GameConfig) -> Result<Self, EngineError> {
        debug!("Creating event loop");
        config.validate()?;
        let (width, height) = config.screen_size;
        Ok(Self {
            input_handler: InputHandler::new()?,
            renderer: BasicRenderer::new(width, height)?,
            config,
        })
    }

    /// Runs the main game loop with the provided game state and configuration.
    ///
    /// This method implements a fixed timestep loop with lag compensation.
    /// It will continue running until the game state's `on_event` method
    /// returns `true` for any input event.
    ///
    /// The loop supports state machines and entity updates through the GameState trait.
    ///
    /// # Arguments
    /// * `node` - The initial game state implementing the Node trait
    /// * `config` - Configuration for the game loop
    ///
    /// # Returns
    /// * `Ok(())` when the game exits normally
    /// * `Err(EngineError)` if an error occurs during execution
    pub fn run<N: Node>(&mut self, node: &mut dyn Node) -> Result<(), EngineError> {
        debug!("Starting event loop with config: {:?}", self.config);
        let mut previous_time = Instant::now();
        let mut lag_time = Duration::ZERO;
        let frame_duration = self.config.frame_duration();

        loop {
            self.input_handler
                .poll(self.config.input_strategy.timeout())?;

            for event in self.input_handler.drain() {
                if node.on_event(event) {
                    return Ok(());
                }
            }

            let now = Instant::now();
            let mut elapsed = now.duration_since(previous_time);
            previous_time = now;

            // Prevent spiral of death by capping frame time
            if elapsed > self.config.max_frame_time {
                warn!(
                    "Frame time exceeded maximum: {:?}, capping to {:?}",
                    elapsed, self.config.max_frame_time
                );
                elapsed = self.config.max_frame_time;
            }

            lag_time += elapsed;

            while lag_time >= frame_duration {
                node.update(frame_duration.as_secs_f32());
                lag_time -= frame_duration;
            }

            self.renderer.clear()?;
            node.render(&mut self.renderer);
            self.renderer.flush()?;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Config, GameConfig};
    use crossterm::event::Event;
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

    impl Node for MockState {
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

        fn render(&self, _renderer: &mut dyn Renderer) {
            let mut count = self.render_count.lock().unwrap();
            *count += 1;
        }
    }

    #[test]
    fn test_event_loop_creation() {
        let config = GameConfig::new();
        match EventLoop::new(&config) {
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
    fn test_game_state_trait_implementation() {
        let mut state = MockState::new();
        let mut renderer = BasicRenderer::new(80, 24).unwrap();

        state.update(1.0 / 60.0);
        assert_eq!(state.get_update_count(), 1);

        state.render(&mut renderer);
        assert_eq!(state.get_render_count(), 1);

        let key_event = Event::Key(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
        assert!(!state.on_event(key_event));

        let esc_event = Event::Key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
        assert!(state.on_event(esc_event));
    }

    #[test]
    fn test_game_config_with_event_loop() {
        let config = GameConfig::new().add_config(Config::TargetFps(30));

        assert!(config.validate().is_ok());
        assert_eq!(config.target_fps, 30);
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
        let mut renderer = BasicRenderer::new(80, 24).unwrap();

        for i in 1..=5 {
            state.render(&mut renderer);
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
