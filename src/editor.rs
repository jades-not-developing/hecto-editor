use crate::{Terminal, TryDefault};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::time::Duration;
use crossterm::queue;
use crossterm::style::ResetColor;
use crossterm::terminal::{Clear, ClearType};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    terminal: Terminal,
}
impl Editor {
    pub fn tick(&mut self) -> anyhow::Result<bool> {
        if crossterm::event::poll(Duration::from_millis(100))? {
            match crossterm::event::read()? {
                Event::Key(KeyEvent {
                               code,
                               kind,
                               modifiers,
                               ..
                           }) => match (kind, code, modifiers) {
                    (KeyEventKind::Press, KeyCode::Char(c), KeyModifiers::CONTROL) => match c {
                        'q' => return Ok(false),
                        _ => {},
                    },

                    (KeyEventKind::Press, KeyCode::Char(c), _) => {},
                    _ => {}
                },
                _ => {}
            }
        }

        self.terminal.update()?;

        self.render()?;

        Ok(true)
    }

    pub fn render(&mut self) -> anyhow::Result<()> {
        self.terminal.clear()?;
        self.terminal.move_to(0, 0)?;
        self.terminal.hide_cursor()?;

        self.render_rows()?;

        self.terminal.show_cursor()?;
        self.terminal.flush()?;

        Ok(())
    }

    pub fn render_rows(&mut self) -> anyhow::Result<()> {
        for row in 0..self.terminal.rows-1 {
            if row == self.terminal.rows / 3 {
                let mut welcome_msg = format!("-=- Hecto version {VERSION} -=-");
                let width = self.terminal.columns as usize;
                let len = welcome_msg.len();
                let padding = width.saturating_sub(len) / 2;
                let spaces = " ".repeat(padding.saturating_sub(1));
                welcome_msg = format!("~{}{}", spaces, welcome_msg);
                welcome_msg.truncate(width);
                self.terminal.print(format!("{}\r", welcome_msg))?;
            } else {
                self.terminal.print("~\r")?;
            }
            self.terminal.next_line()?;
        }
        Ok(())
    }
}

impl TryDefault for Editor {
    fn try_default() -> anyhow::Result<Self> {
        crossterm::terminal::enable_raw_mode()?;
        Ok(Self {
            terminal: Terminal::try_default()?,
        })
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        self.terminal.clear().expect("Failed to clear the screen");
        self.terminal.move_to(0, 0).expect("Failed to reset cursor position");
        self.terminal.show_cursor().expect("Failed to reset show cursor");

        crossterm::terminal::disable_raw_mode().expect("Failed to disable raw mode");
    }
}
