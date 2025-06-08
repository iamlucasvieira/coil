# Coil Engine

A simple game engine for terminal-based games written in Rust.

## Features

- **Event Loop**: Fixed timestep game loop with lag compensation for consistent timing
- **Input Handling**: Cross-platform terminal input using crossterm with automatic raw mode management
- **Game State Management**: Trait-based architecture for implementing game logic
- **Error Handling**: Custom error types for engine operations

## Architecture

The engine is built around a few core concepts:

- `EventLoop`: Manages the main game loop, frame timing, and coordinates game state updates
- `GameState`: A trait that game implementations must implement to define update, event handling, and rendering logic
- `InputHandler`: Handles terminal input events with automatic cleanup
- `EngineError`: Custom error types for engine operations

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
coil_engine = "0.1.0"
```

### Basic Example

```rust
use coil_engine::{EventLoop, GameState};
use crossterm::event::{Event, KeyCode};

struct MyGame;

impl GameState for MyGame {
    fn update(&mut self, delta_time: f32) {
        // Update game logic here
    }

    fn on_even(&mut self, event: Event) -> bool {
        // Handle input events
        // Return true to exit the game loop
        if let Event::Key(key_event) = event {
            if key_event.code == KeyCode::Esc {
                return true; // Exit on Esc key
            }
        }
        false
    }

    fn render(&self) {
        // Render your game here
        println!("Game is running!");
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut event_loop = EventLoop::new(60)?; // 60 FPS
    let game = MyGame;
    event_loop.run(game)?;
    Ok(())
}
```

## Dependencies

- `crossterm`: Cross-platform terminal manipulation
- `thiserror`: Derive macros for error handling

## License

This project is licensed under the MIT License.