pub mod errors;
mod event_loop;
mod input;

pub use event_loop::{EventLoop, GameState};
pub use input::InputStrategy;
