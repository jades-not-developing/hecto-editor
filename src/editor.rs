use std::io::{Cursor, Stdout, Write};
use crate::TryDefault;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::time::Duration;
use crossterm::cursor::{DisableBlinking, EnableBlinking, Hide, MoveTo, MoveToNextLine, SetCursorStyle, Show};
use crossterm::queue;
use crossterm::style::{Print, ResetColor};
use crossterm::terminal::{Clear, ClearType};

pub struct Editor {
    rows: u16,
    columns: u16,
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

        let (columns, rows) = crossterm::terminal::size()?;
        self.columns = columns;
        self.rows = rows;

        self.render()?;

        Ok(true)
    }

    pub fn render(&self) -> anyhow::Result<()> {
        let mut stdout = ::std::io::stdout();
        queue!(
            stdout,
            Clear(ClearType::All),
            MoveTo(0, 0),
            Hide,
        )?;

        self.render_rows(&mut stdout)?;

        queue!(
            stdout,
            Show,
        )?;

        stdout.flush()?;

        Ok(())
    }

    pub fn render_rows(&self, stdout: &mut Stdout) -> anyhow::Result<()> {
        for _ in 0..self.rows-1 {
            queue!(
                stdout,
                Print("~\r"),
                MoveToNextLine(1),
            )?;
        }
        Ok(())
    }
}

impl TryDefault for Editor {
    fn try_default() -> anyhow::Result<Self> {
        crossterm::terminal::enable_raw_mode()?;
        let (columns, rows) = crossterm::terminal::size()?;
        Ok(Self { rows, columns })
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
