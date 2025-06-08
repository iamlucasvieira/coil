pub mod config;
pub mod errors;
mod event_loop;
mod input;

pub use config::GameConfig;
pub use event_loop::{Entity, EventLoop, GameState, StateMachine};
pub use input::InputStrategy;
