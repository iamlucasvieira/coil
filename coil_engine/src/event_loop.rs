use crate::errors::EngineError;
use crate::input::InputHandler;
use crossterm::event::Event;
use std::time::{Duration, Instant};

pub trait GameState {
    fn update(&mut self, delta_time: f32);
    fn on_even(&mut self, event: Event) -> bool;
    fn render(&self);
}

pub struct EventLoop {
    target_fps: u32,
    input_handler: InputHandler,
}

impl EventLoop {
    pub fn new(target_fps: u32) -> Result<Self, EngineError> {
        Ok(Self {
            target_fps,
            input_handler: InputHandler::new()?,
        })
    }

    pub fn run<T: GameState>(&mut self, mut state: T) -> Result<(), EngineError> {
        let mut previous_time = Instant::now();
        let mut lag_time = Duration::ZERO;
        let frame_duration = Duration::from_secs_f32(1.0 / self.target_fps as f32);

        self.input_handler.poll(frame_duration)?;

        for event in self.input_handler.drain() {
            if state.on_even(event) {
                return Ok(());
            }
        }

        loop {
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
    use crossterm::event::KeyCode;

    struct MockState;

    impl GameState for MockState {
        fn update(&mut self, _delta_time: f32) {}

        fn on_even(&mut self, event: Event) -> bool {
            if let Event::Key(key_event) = event {
                if key_event.code == KeyCode::Esc {
                    return true; // Exit on Esc key
                }
            }
            false
        }

        fn render(&self) {}
    }

    #[test]
    fn test_event_loop() {
        let mut event_loop = EventLoop::new(60).unwrap();
        let state = MockState;
        event_loop.run(state).unwrap();
    }
}
