use crate::config::{Config, GameConfig};
use crate::errors::EngineError;
use crate::event_loop::{EventLoop, GameState};
use std::process;

pub struct Game<S> {
    state: S,
    config: GameConfig,
}

impl<S: GameState> Game<S> {
    pub fn new(state: S) -> Self {
        Self {
            state,
            config: GameConfig::new(),
        }
    }

    pub fn add_config(mut self, config: Config) -> Self {
        self.config = self.config.add_config(config);
        self
    }

    pub fn start(&mut self) {
        if let Err(e) = (|| -> Result<(), EngineError> {
            let mut event_loop = EventLoop::new()?;
            event_loop.run(&mut self.state, &self.config)?;
            Ok(())
        })() {
            eprintln!("Error running game: {}", e);
            process::exit(1);
        }
    }
}
