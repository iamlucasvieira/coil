use crate::errors::EngineError;
use crate::input::InputStrategy;
use std::time::Duration;

/// Configuration for the game engine.
///
/// This struct contains all the settings needed to configure the engine's behavior,
/// including frame rate, input handling, and other engine parameters.
#[derive(Debug, Clone)]
pub struct GameConfig {
    /// Target frames per second for the game loop
    pub target_fps: u32,
    /// Input handling strategy
    pub input_strategy: InputStrategy,
    /// Maximum frame time to prevent spiral of death (prevents spiral of death in complex scenes)
    pub max_frame_time: Duration,
    /// Whether to enable debug logging
    pub debug_mode: bool,
    /// Whether to enable vsync-like behavior
    pub vsync: bool,
}

impl GameConfig {
    /// Creates a new game configuration with sensible defaults.
    pub fn new() -> Self {
        Self {
            target_fps: 60,
            input_strategy: InputStrategy::default(),
            max_frame_time: Duration::from_millis(50), // Cap at 20 FPS minimum
            debug_mode: false,
            vsync: true,
        }
    }

    /// Sets the target frames per second.
    pub fn with_target_fps(mut self, fps: u32) -> Self {
        self.target_fps = fps;
        self
    }

    /// Sets the input strategy.
    pub fn with_input_strategy(mut self, strategy: InputStrategy) -> Self {
        self.input_strategy = strategy;
        self
    }

    /// Sets the maximum frame time.
    pub fn with_max_frame_time(mut self, max_time: Duration) -> Self {
        self.max_frame_time = max_time;
        self
    }

    /// Enables or disables debug mode.
    pub fn with_debug_mode(mut self, debug: bool) -> Self {
        self.debug_mode = debug;
        self
    }

    /// Enables or disables vsync-like behavior.
    pub fn with_vsync(mut self, vsync: bool) -> Self {
        self.vsync = vsync;
        self
    }

    /// Validates the configuration.
    pub fn validate(&self) -> Result<(), EngineError> {
        if self.target_fps == 0 {
            return Err(EngineError::Config(
                "Target FPS must be greater than zero".to_string(),
            ));
        }
        if self.max_frame_time.is_zero() {
            return Err(EngineError::Config(
                "Max frame time must be greater than zero".to_string(),
            ));
        }
        Ok(())
    }

    /// Gets the frame duration for the target FPS.
    pub fn frame_duration(&self) -> Duration {
        Duration::from_secs_f32(1.0 / self.target_fps as f32)
    }
}

impl Default for GameConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_config_defaults() {
        let config = GameConfig::new();
        assert_eq!(config.target_fps, 60);
        assert!(!config.debug_mode);
        assert!(config.vsync);
        assert_eq!(config.max_frame_time, Duration::from_millis(50));
    }

    #[test]
    fn test_game_config_builder() {
        let config = GameConfig::new()
            .with_target_fps(120)
            .with_debug_mode(true)
            .with_max_frame_time(Duration::from_millis(100));

        assert_eq!(config.target_fps, 120);
        assert!(config.debug_mode);
        assert_eq!(config.max_frame_time, Duration::from_millis(100));
    }

    #[test]
    fn test_config_validation() {
        let valid_config = GameConfig::new();
        assert!(valid_config.validate().is_ok());

        let invalid_config = GameConfig::new().with_target_fps(0);
        assert!(invalid_config.validate().is_err());

        let zero_frame_time_config = GameConfig::new().with_max_frame_time(Duration::ZERO);
        assert!(zero_frame_time_config.validate().is_err());
    }

    #[test]
    fn test_frame_duration() {
        let config = GameConfig::new().with_target_fps(60);
        let expected = Duration::from_secs_f32(1.0 / 60.0);
        assert_eq!(config.frame_duration(), expected);

        let config = GameConfig::new().with_target_fps(30);
        let expected = Duration::from_secs_f32(1.0 / 30.0);
        assert_eq!(config.frame_duration(), expected);
    }
}

