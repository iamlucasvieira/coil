use coil_engine::{Config, Game, GameState};
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
        // Here you could update entities, state machines, etc.
        // For example:
        // for entity in &mut self.entities {
        //     if entity.is_active() {
        //         entity.update(delta_time);
        //     }
        // }
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

    fn render(&self) {
        // Clear and render
        print!("\x1B[2J\x1B[1;1H"); // Clear screen and move cursor to top-left
        println!("Coil of Fate - Frame: {}", self.frame_count);
        println!("Press Esc or Ctrl+C to exit");
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
