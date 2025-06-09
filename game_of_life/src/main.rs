use coil_engine::{BasicRenderer, Cell, Config, Game, GameState, Renderer};
use crossterm::event::{Event, KeyCode, KeyEvent, MouseEvent, MouseEventKind};
use crossterm::style::Color;
use rand::Rng;

const ALIVE_CELL: Cell = Cell {
    ch: '█',
    fg: Color::Green,
    bg: Color::Reset,
};

const DEAD_CELL: Cell = Cell {
    ch: ' ',
    fg: Color::Reset,
    bg: Color::Reset,
};

struct Grid {
    width: usize,
    height: usize,
    cells: Vec<bool>,
}

impl Grid {
    fn new() -> Self {
        Grid {
            width: 0,
            height: 0,
            cells: Vec::new(),
        }
    }

    fn set_size(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.cells.resize(width * height, false);

        // Randomly initialize the grid
        let mut rng = rand::rng();
        for cell in self.cells.iter_mut() {
            *cell = rng.random_bool(0.1);
        }
    }

    fn get(&self, x: usize, y: usize) -> bool {
        if x < self.width && y < self.height {
            self.cells[y * self.width + x]
        } else {
            false
        }
    }

    fn coordinates(&self, index: usize) -> Option<(usize, usize)> {
        if index < self.cells.len() {
            Some((index % self.width, index / self.width))
        } else {
            None
        }
    }

    fn get_neighbors(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if (dx != 0 || dy != 0)
                    && self.get((x as isize + dx) as usize, (y as isize + dy) as usize)
                {
                    count += 1;
                }
            }
        }
        count
    }

    fn set(&mut self, x: usize, y: usize, value: bool) {
        if x < self.width && y < self.height {
            self.cells[y * self.width + x] = value;
        }
    }
}

struct PauseMenu {
    paused: bool,
}

impl PauseMenu {
    fn new() -> Self {
        PauseMenu { paused: false }
    }

    fn is_paused(&self) -> bool {
        self.paused
    }
}

impl GameState for PauseMenu {
    fn update(&mut self, _delta_time: f32) {}

    fn on_event(&mut self, event: Event) -> bool {
        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Char(' '),
                ..
            }) => {
                self.paused = !self.paused; // Toggle pause state
                false // Do not exit
            }
            _ => false, // Ignore other events
        }
    }

    fn render(&self, renderer: &mut BasicRenderer) {
        if self.paused {
            let pause_text = "Game Paused. Press Space to Resume.";
            let (width, height) = renderer.size();
            // scale to center the text
            let x = (width / 2) - (pause_text.len() as u16 / 2);
            let y = height / 2;

            renderer
                .draw_str(x, y, pause_text, Color::Reset, Color::DarkBlue)
                .unwrap();
        }
    }
}

struct GameOfLife {
    pub grid: Grid,
    pause_menu: PauseMenu,
}

impl GameOfLife {
    fn new() -> Self {
        GameOfLife {
            grid: Grid::new(),
            pause_menu: PauseMenu::new(),
        }
    }
}

impl GameState for GameOfLife {
    fn update(&mut self, _delta_time: f32) {
        if self.pause_menu.is_paused() {
            return; // Skip update if paused
        }
        for idx in 0..self.grid.cells.len() {
            let (x, y) = self.grid.coordinates(idx).unwrap();
            let neighbors = self.grid.get_neighbors(x, y);
            let alive = self.grid.get(x, y);
            let new_state = match (alive, neighbors) {
                (true, 2) | (true, 3) => true, // Stay alive
                (false, 3) => true,            // Become alive
                _ => false,                    // Die or stay dead
            };
            self.grid.set(x, y, new_state);
        }
    }

    fn on_event(&mut self, event: Event) -> bool {
        self.pause_menu.on_event(event.clone());
        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Esc, ..
            }) => true, // Exit on Esc key
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: crossterm::event::KeyModifiers::CONTROL,
                ..
            }) => true, // Exit on Ctrl+C
            Event::Mouse(MouseEvent {
                kind: MouseEventKind::Down(_),
                column,
                row,
                ..
            }) => {
                let x = column as usize;
                let y = row as usize;
                if x < self.grid.width && y < self.grid.height {
                    let current_state = self.grid.get(x, y);
                    self.grid.set(x, y, !current_state);
                }
                false
            }
            _ => false, // Ignore other events
        }
    }

    fn render(&self, renderer: &mut BasicRenderer) {
        for y in 0..self.grid.height {
            for x in 0..self.grid.width {
                if self.grid.get(x, y) {
                    renderer.draw_cell(x as u16, y as u16, ALIVE_CELL).unwrap();
                } else {
                    renderer.draw_cell(x as u16, y as u16, DEAD_CELL).unwrap();
                }
            }
        }
        self.pause_menu.render(renderer);
    }
}

fn main() {
    let mut game = Game::new(GameOfLife::new()).add_config(Config::TargetFps(2));
    let (width, height) = game.config.screen_size;
    game.state.grid.set_size(width as usize, height as usize);
    game.start();
}
