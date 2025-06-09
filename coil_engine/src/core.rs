use crate::config::{Config, GameConfig};
use crate::errors::EngineError;
use crate::event_loop::EventLoop;
use crate::nodes::Node;
use std::process;

pub struct Game<N> {
    pub node: N,
    pub config: GameConfig,
}

impl<N: Node> Game<N> {
    pub fn new(node: N) -> Self {
        Self {
            node,
            config: GameConfig::new(),
        }
    }

    pub fn with_config(node: N, config: GameConfig) -> Self {
        Self { node, config }
    }

    pub fn add_config(mut self, config: Config) -> Self {
        self.config = self.config.add_config(config);
        self
    }

    pub fn start(&mut self) {
        if let Err(e) = (|| -> Result<(), EngineError> {
            let mut event_loop = EventLoop::new(&self.config)?;
            event_loop.run::<N>(&mut self.node)?;
            Ok(())
        })() {
            eprintln!("Error running game: {}", e);
            process::exit(1);
        }
    }
}
