use std::io::{Cursor, Stdout, Write};
use crate::{Terminal, TryDefault};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::time::Duration;
use crossterm::cursor::{DisableBlinking, EnableBlinking, Hide, MoveTo, MoveToNextLine, SetCursorStyle, Show};
use crossterm::queue;
use crossterm::style::{Print, ResetColor};
use crossterm::terminal::{Clear, ClearType};

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
        self.terminal.clear();
        self.terminal.hide_cursor();

        self.render_rows()?;

        self.terminal.show_cursor();
        self.terminal.flush();

        Ok(())
    }

    pub fn render_rows(&mut self) -> anyhow::Result<()> {
        for _ in 0..self.terminal.rows-1 {
            self.terminal.print("~\r")?;
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
        let mut stdout = ::std::io::stdout();
        queue!(
            stdout,
            Clear(ClearType::All),
            MoveTo(0, 0),
            ResetColor,
        ).expect("Failed to reset screen");
        crossterm::terminal::disable_raw_mode().expect("Failed to disable raw mode");
    }
}
