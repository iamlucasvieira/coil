use crate::config::GameConfig;
use crate::errors::EngineError;
use crate::input::InputHandler;
use crossterm::event::Event;
use log::{debug, warn};
use std::time::{Duration, Instant};

/// Trait that defines the interface for game logic implementation.
///
/// Game implementations must implement this trait to define how the game
/// updates, handles events, and renders. This trait supports state machines
/// and entity-specific loops.
pub trait GameState {
    /// Updates the game state based on the elapsed time since the last frame.
    ///
    /// This method can contain state machine logic and update entities
    /// that have their own update loops.
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

/// Trait for entities that can have their own update logic within the main loop.
/// This allows for character AI, animations, or other entity-specific behavior.
pub trait Entity {
    /// Updates the entity's state.
    fn update(&mut self, delta_time: f32);

    /// Returns true if the entity is active and should continue updating.
    fn is_active(&self) -> bool;
}

/// Trait for state machines that can transition between different states.
pub trait StateMachine {
    type State;

    /// Gets the current state.
    fn current_state(&self) -> &Self::State;

    /// Transitions to a new state.
    fn transition_to(&mut self, state: Self::State);

    /// Updates the current state.
    fn update_state(&mut self, delta_time: f32);
}

/// Main event loop that manages game timing and coordinates game state updates.
///
/// The event loop uses a fixed timestep with lag compensation to ensure
/// consistent game timing regardless of frame rate variations. It supports
/// state machines and entity loops for complex game logic.
pub struct EventLoop {
    input_handler: InputHandler,
}

impl EventLoop {
    /// Creates a new event loop.
    ///
    /// # Returns
    /// * `Ok(EventLoop)` on success
    /// * `Err(EngineError)` if input handler initialization fails
    pub fn new() -> Result<Self, EngineError> {
        debug!("Creating event loop");
        Ok(Self {
            input_handler: InputHandler::new()?,
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
    /// * `state` - The game state implementation to run
    /// * `config` - Configuration for the game loop
    ///
    /// # Returns
    /// * `Ok(())` when the game exits normally
    /// * `Err(EngineError)` if an error occurs during execution
    pub fn run<G: GameState>(
        &mut self,
        state: &mut G,
        config: &GameConfig,
    ) -> Result<(), EngineError> {
        debug!("Starting event loop with config: {:?}", config);
        config.validate()?;
        let mut previous_time = Instant::now();
        let mut lag_time = Duration::ZERO;
        let frame_duration = config.frame_duration();

        loop {
            self.input_handler.poll(config.input_strategy.timeout())?;

            for event in self.input_handler.drain() {
                if state.on_event(event) {
                    return Ok(());
                }
            }

            let now = Instant::now();
            let mut elapsed = now.duration_since(previous_time);
            previous_time = now;

            // Prevent spiral of death by capping frame time
            if elapsed > config.max_frame_time {
                warn!(
                    "Frame time exceeded maximum: {:?}, capping to {:?}",
                    elapsed, config.max_frame_time
                );
                elapsed = config.max_frame_time;
            }

            lag_time += elapsed;

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
    use crate::config::GameConfig;
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

    // Example entity implementation for testing
    struct MockEntity {
        active: bool,
        update_count: u32,
    }

    impl MockEntity {
        fn new() -> Self {
            Self {
                active: true,
                update_count: 0,
            }
        }

        fn deactivate(&mut self) {
            self.active = false;
        }
    }

    impl Entity for MockEntity {
        fn update(&mut self, _delta_time: f32) {
            if self.active {
                self.update_count += 1;
            }
        }

        fn is_active(&self) -> bool {
            self.active
        }
    }

    // Example state machine implementation
    #[derive(Debug, PartialEq)]
    enum GameStateEnum {
        Menu,
        Playing,
        Paused,
    }

    struct MockStateMachine {
        current_state: GameStateEnum,
    }

    impl MockStateMachine {
        fn new() -> Self {
            Self {
                current_state: GameStateEnum::Menu,
            }
        }
    }

    impl StateMachine for MockStateMachine {
        type State = GameStateEnum;

        fn current_state(&self) -> &Self::State {
            &self.current_state
        }

        fn transition_to(&mut self, state: Self::State) {
            self.current_state = state;
        }

        fn update_state(&mut self, _delta_time: f32) {
            // State-specific update logic would go here
        }
    }

    #[test]
    fn test_event_loop_creation() {
        // Test creation - may fail in CI environments without terminal access
        match EventLoop::new() {
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
    fn test_entity_trait_implementation() {
        let mut entity = MockEntity::new();

        assert!(entity.is_active());
        entity.update(1.0 / 60.0);
        assert_eq!(entity.update_count, 1);

        entity.deactivate();
        assert!(!entity.is_active());
        entity.update(1.0 / 60.0);
        assert_eq!(entity.update_count, 1); // Should not increment when inactive
    }

    #[test]
    fn test_state_machine_trait_implementation() {
        let mut state_machine = MockStateMachine::new();

        assert_eq!(*state_machine.current_state(), GameStateEnum::Menu);

        state_machine.transition_to(GameStateEnum::Playing);
        assert_eq!(*state_machine.current_state(), GameStateEnum::Playing);

        state_machine.transition_to(GameStateEnum::Paused);
        assert_eq!(*state_machine.current_state(), GameStateEnum::Paused);
    }

    #[test]
    fn test_game_config_with_event_loop() {
        let config = GameConfig::new().with_target_fps(30);
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

