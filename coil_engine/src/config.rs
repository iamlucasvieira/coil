use crate::errors::EngineError;
use crate::input::InputStrategy;
use std::time::Duration;

pub enum Config {
    TargetFps(u32),
    InputStrategy(InputStrategy),
    MaxFrameTime(Duration),
    DebugMode(bool),
    Vsync(bool),
}
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

    pub fn add_config(mut self, config: Config) -> Self {
        match config {
            Config::TargetFps(fps) => self.target_fps = fps,
            Config::InputStrategy(strategy) => self.input_strategy = strategy,
            Config::MaxFrameTime(max_time) => self.max_frame_time = max_time,
            Config::DebugMode(debug) => self.debug_mode = debug,
            Config::Vsync(vsync) => self.vsync = vsync,
        }
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
}

