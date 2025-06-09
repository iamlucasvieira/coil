use coil_engine::{BasicRenderer, Config, Game, GameState, Renderer};
use crossterm::event::Event;

struct MyGame {
    // Example state - you could have a state machine here
    frame_count: u32,
}

impl MyGame {
    fn new() -> Self {
        Self { frame_count: 0 }
    }
}

impl GameState for MyGame {
    fn update(&mut self, _delta_time: f32) {
        self.frame_count += 1;
    }

    fn on_event(&mut self, event: Event) -> bool {
        // Handle input events
        match event {
            Event::Key(key_event) => {
                use crossterm::event::{KeyCode, KeyModifiers};
                match (key_event.code, key_event.modifiers) {
                    (KeyCode::Char('c'), KeyModifiers::CONTROL) => true, // Exit on Ctrl+C
                    (KeyCode::Esc, _) => true,                           // Exit on Escape
                    _ => false,
                }
            }
            _ => false,
        }
    }

    fn render(&self, renderer: &mut BasicRenderer) {
        renderer
            .draw_str(
                0,
                0,
                &format!("Frame: {}", self.frame_count),
                crossterm::style::Color::Black,
                crossterm::style::Color::White,
            )
            .unwrap();
        renderer
            .draw_str(
                0,
                1,
                "Press Esc or Ctrl+C to exit",
                crossterm::style::Color::Black,
                crossterm::style::Color::White,
            )
            .unwrap();
    }
}

fn main() {
    Game::new(MyGame::new())
        .add_config(Config::TargetFps(60))
        .add_config(Config::InputStrategy(
            coil_engine::InputStrategy::NonBlocking,
        ))
        .start();
}
