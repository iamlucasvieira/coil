use coil_engine::{
    Game,
    config::GameConfig,
    nodes::Node,
    renderer::{Cell, Renderer},
};
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
    width: u16,
    height: u16,
    cells: Vec<bool>,
}

impl Grid {
    fn new(width: u16, height: u16) -> Self {
        let mut rng = rand::rng();
        let mut cells = vec![false; (width * height) as usize];
        for cell in cells.iter_mut() {
            *cell = rng.random_bool(0.1);
        }
        Grid {
            width,
            height,
            cells,
        }
    }

    fn get(&self, x: u16, y: u16) -> bool {
        if x < self.width && y < self.height {
            self.cells[(y * self.width + x) as usize]
        } else {
            false
        }
    }

    fn coordinates(&self, index: usize) -> Option<(u16, u16)> {
        if index < self.cells.len() {
            Some((index as u16 % self.width, index as u16 / self.width))
        } else {
            None
        }
    }

    fn get_neighbors(&self, x: u16, y: u16) -> usize {
        let mut count = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if (dx != 0 || dy != 0)
                    && self.get((x as isize + dx) as u16, (y as isize + dy) as u16)
                {
                    count += 1;
                }
            }
        }
        count
    }

    fn set(&mut self, x: u16, y: u16, value: bool) {
        if x < self.width && y < self.height {
            self.cells[(y * self.width + x) as usize] = value;
        }
    }
}

struct PauseMenu {
    width: u16,
    height: u16,
    paused: bool,
}

impl PauseMenu {
    fn new(width: u16, height: u16) -> Self {
        PauseMenu {
            width,
            height,
            paused: false,
        }
    }

    fn is_paused(&self) -> bool {
        self.paused
    }
}

impl Node for PauseMenu {
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

    fn render(&self, renderer: &mut dyn Renderer) {
        if self.paused {
            let pause_text = "Game Paused. Press Space to Resume.";
            let x = (self.width / 2) - (pause_text.len() as u16 / 2);
            let y = self.height / 2;

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
    fn new(width: u16, height: u16) -> Self {
        GameOfLife {
            grid: Grid::new(width, height),
            pause_menu: PauseMenu::new(width, height),
        }
    }
}

impl Node for GameOfLife {
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
                let (x, y) = (column, row);
                if x < self.grid.width && y < self.grid.height {
                    let current_state = self.grid.get(x, y);
                    self.grid.set(x, y, !current_state);
                }
                false
            }
            _ => false, // Ignore other events
        }
    }

    fn render(&self, renderer: &mut dyn Renderer) {
        for y in 0..self.grid.height {
            for x in 0..self.grid.width {
                if self.grid.get(x, y) {
                    renderer.draw_cell(x, y, ALIVE_CELL).unwrap();
                } else {
                    renderer.draw_cell(x, y, DEAD_CELL).unwrap();
                }
            }
        }
        self.pause_menu.render(renderer);
    }
}

fn main() {
    let config = GameConfig {
        target_fps: 10,
        ..Default::default()
    };
    let (width, height) = config.screen_size;
    Game::with_config(GameOfLife::new(width, height), config).start();
}
