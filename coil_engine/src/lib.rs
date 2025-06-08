pub mod config;
pub mod core;
pub mod errors;
mod event_loop;
mod input;

pub use config::Config;
pub use core::Game;
pub use event_loop::{Entity, EventLoop, GameState, StateMachine};
pub use input::InputStrategy;
