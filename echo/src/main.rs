use coil_engine::{Game, config::Config, nodes::Node, renderer::Renderer};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::Color;

struct EchoGame {
    message: String,
}

impl EchoGame {
    fn new() -> Self {
        EchoGame {
            message: String::from("Press any key to echo it. Press Esc to exit."),
        }
    }
}

impl Node for EchoGame {
    fn update(&mut self, _delta_time: f32) {}

    fn on_event(&mut self, event: Event) -> bool {
        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Esc, ..
            }) => true,
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => true,
            Event::Key(key) => {
                self.message = format!("Key pressed: {}", key.code);
                false
            }
            _ => false, // Ignore other events
        }
    }

    fn render(&self, renderer: &mut dyn Renderer) {
        renderer
            .draw_str(0, 0, &self.message, Color::Black, Color::White)
            .unwrap();
    }
}

fn main() {
    env_logger::init();

    Game::new(EchoGame::new())
        .add_config(Config::TargetFps(60))
        .start();
}
