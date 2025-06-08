use coil_engine::{EventLoop, GameState};
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

    // Create an event loop to run the game
    let mut event_loop = EventLoop::new(FRAME_RATE).unwrap_or_else(|e| {
        eprintln!("Failed to create event loop: {}", e);
        std::process::exit(1);
    });

    // Run the event loop
    event_loop.run(&mut game_state).unwrap_or_else(|e| {
        eprintln!("Event loop error: {}", e);
        std::process::exit(1);
    });
}
