use std::fmt::Display;
use std::io::{Stdout, Write};
use crossterm::cursor::{Hide, MoveTo, MoveToNextLine, Show};
use crossterm::queue;
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};
use crate::{Position, TryDefault};

pub struct Terminal {
    pub rows: u16,
    pub columns: u16,
    stdout: Stdout,
}

impl Terminal {
    pub fn update(&mut self) -> anyhow::Result<()> {
        (self.columns, self.rows) = crossterm::terminal::size()?;

        Ok(())
    }

    pub fn clear(&mut self) -> anyhow::Result<()> {
        queue!(
            self.stdout,
            Clear(ClearType::All),
        )?;
        Ok(())
    }

    pub fn clear_line(&mut self) -> anyhow::Result<()> {
        queue!(
            self.stdout,
            Clear(ClearType::CurrentLine),
        )?;
        Ok(())
    }

    pub fn hide_cursor(&mut self) -> anyhow::Result<()> {
        queue!(
            self.stdout,
            Hide,
        )?;

        Ok(())
    }

    pub fn show_cursor(&mut self) -> anyhow::Result<()> {
        queue!(
            self.stdout,
            Show,
        )?;

        Ok(())
    }

    pub fn move_to(&mut self, x: u16, y: u16) -> anyhow::Result<()> {
        queue!(
            self.stdout,
            MoveTo(x, y),
        )?;

        Ok(())
    }

    pub fn move_to_position(&mut self, position: &Position) -> anyhow::Result<()> {
        self.move_to(position.x, position.y)
    }

    pub fn next_line(&mut self) -> anyhow::Result<()> {
        queue!(
            self.stdout,
            MoveToNextLine(1),
        )?;

        Ok(())
    }

    pub fn print<T: Display>(&mut self, content: T) -> anyhow::Result<()> {
        queue!(
            self.stdout,
            Print(content),
        )?;

        Ok(())
    }

    pub fn flush(&mut self) -> anyhow::Result<()> {
        self.stdout.flush()?;
        Ok(())
    }
}

impl TryDefault for Terminal {
    fn try_default() -> anyhow::Result<Self> {
        let (columns, rows) = crossterm::terminal::size()?;
        let mut terminal = Self { columns, rows, stdout: ::std::io::stdout() };
        terminal.clear()?;
        Ok(terminal)
    }
}