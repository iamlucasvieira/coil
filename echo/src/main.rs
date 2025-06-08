use coil_engine::{EventLoop, GameState, InputStrategy};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

struct EchoGame {
    message: String,
    should_print: bool,
}

impl EchoGame {
    fn new() -> Self {
        EchoGame {
            message: String::from("Press any key to echo it. Press Esc to exit."),
            should_print: true,
        }
    }
}

impl GameState for EchoGame {
    fn update(&mut self, _delta_time: f32) {
        self.should_print = false;
    }

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
                self.should_print = true;
                self.message = format!("Key pressed: {}", key.code);
                false
            }
            _ => false, // Ignore other events
        }
    }

    fn render(&self) {
        if self.should_print {
            println!("{}\n", self.message);
        }
    }
}

const FRAME_RATE: u32 = 60;

fn main() {
    env_logger::init();

    // Create an instance of the game state
    let mut game_state = EchoGame::new();

    EventLoop::new(FRAME_RATE)
        .unwrap()
        .with_input_strategy(InputStrategy::NonBlocking)
        .run(&mut game_state)
        .unwrap();
}
