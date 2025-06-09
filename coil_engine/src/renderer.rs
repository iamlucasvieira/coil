//! Rendering subsystem for the engine
//!
//! Defines a cell-based API and a Crossterm-backed implementation.
use crate::errors::EngineError;
use crossterm::cursor;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::style::Color;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use log::warn;
use std::io::{Write, stdout};

/// A single character cell with foreground and background colors.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cell {
    pub ch: char,
    pub fg: Color,
    pub bg: Color,
}

/// Abstract renderer API for games.
pub trait Renderer {
    /// Clear the back‑buffer.
    fn clear(&mut self) -> Result<(), EngineError>;

    /// Draw one cell at (x,y).
    fn draw_cell(&mut self, x: u16, y: u16, cell: Cell) -> Result<(), EngineError>;

    /// Draw a string starting at (x,y).
    fn draw_str(
        &mut self,
        x: u16,
        y: u16,
        text: &str,
        fg: Color,
        bg: Color,
    ) -> Result<(), EngineError>;

    /// Flush all pending draws to the terminal.
    fn flush(&mut self) -> Result<(), EngineError>;
}

pub struct BasicRenderer {
    width: u16,
    height: u16,
    back_buffer: Vec<Cell>,
    front_buffer: Vec<Cell>,
}

impl BasicRenderer {
    pub fn new(width: u16, height: u16) -> Result<Self, EngineError> {
        execute!(
            stdout(),
            EnterAlternateScreen,
            EnableMouseCapture,
            cursor::Hide
        )
        .map_err(|e| EngineError::Input(e.to_string()))?;
        let back_buffer = vec![
            Cell {
                ch: ' ',
                fg: Color::Reset,
                bg: Color::Reset,
            };
            (width * height) as usize
        ];
        let front_buffer = back_buffer.clone();
        Ok(Self {
            width,
            height,
            back_buffer,
            front_buffer,
        })
    }

    pub fn size(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    /// Return the index of the cell at (x,y) in the back buffer.
    pub fn index(&self, x: u16, y: u16) -> Result<usize, EngineError> {
        if x >= self.width || y >= self.height {
            return Err(EngineError::Render(format!(
                "Coordinates out of bounds: ({}, {})",
                x, y
            )));
        }
        Ok((y as usize * self.width as usize) + x as usize)
    }

    pub fn coordinates(&self, index: usize) -> Result<(u16, u16), EngineError> {
        if index >= self.back_buffer.len() {
            return Err(EngineError::Render(format!(
                "Index out of bounds: {}",
                index
            )));
        }
        let x = (index % self.width as usize) as u16;
        let y = (index / self.width as usize) as u16;
        Ok((x, y))
    }
}

impl Renderer for BasicRenderer {
    fn clear(&mut self) -> Result<(), EngineError> {
        self.back_buffer.fill(Cell {
            ch: ' ',
            fg: Color::Reset,
            bg: Color::Reset,
        });
        Ok(())
    }

    fn draw_cell(&mut self, x: u16, y: u16, cell: Cell) -> Result<(), EngineError> {
        let index = self.index(x, y)?;
        self.back_buffer[index] = cell;
        Ok(())
    }

    fn draw_str(
        &mut self,
        x: u16,
        y: u16,
        text: &str,
        fg: Color,
        bg: Color,
    ) -> Result<(), EngineError> {
        for (i, ch) in text.chars().enumerate() {
            match self.draw_cell(x + i as u16, y, Cell { ch, fg, bg }) {
                Ok(_) => {}
                Err(EngineError::Render(e)) => {
                    warn!("Failed to draw string at ({}, {}): {}", x + i as u16, y, e);
                }
                Err(e) => return Err(e),
            }
            self.draw_cell(x + i as u16, y, Cell { ch, fg, bg })?;
        }
        Ok(())
    }

    fn flush(&mut self) -> Result<(), EngineError> {
        let mut out = stdout();
        for (i, back_cell) in self.back_buffer.iter().enumerate() {
            let front_cell = self.front_buffer[i];
            let (x, y) = self.coordinates(i)?;
            if back_cell != &front_cell {
                execute!(
                    out,
                    crossterm::cursor::MoveTo(x, y),
                    crossterm::style::SetForegroundColor(back_cell.fg),
                    crossterm::style::SetBackgroundColor(back_cell.bg),
                    crossterm::style::Print(back_cell.ch)
                )
                .map_err(|e| EngineError::Render(e.to_string()))?;
                self.front_buffer[i] = *back_cell; // Update front buffer
            }
        }
        out.flush()
            .map_err(|e| EngineError::Render(e.to_string()))?;
        Ok(())
    }
}

impl Drop for BasicRenderer {
    fn drop(&mut self) {
        // Leave alternate screen and show cursor
        let _ = execute!(
            stdout(),
            LeaveAlternateScreen,
            DisableMouseCapture,
            cursor::Show
        );
    }
}
