use std::time::Duration;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

pub struct Editor;
impl Editor {
    pub fn new() -> anyhow::Result<Self> {
        crossterm::terminal::enable_raw_mode()?;
        Ok(Self)
    }

    pub fn tick(&mut self) -> anyhow::Result<bool> {
        if crossterm::event::poll(Duration::from_millis(100))? {
            match crossterm::event::read()? {
                Event::Key(KeyEvent {
                    code,
                    kind,
                    modifiers,
                    ..
                }) => match (kind, code, modifiers) {
                    (
                        KeyEventKind::Press,
                        KeyCode::Char(c),
                        KeyModifiers::CONTROL,
                    ) => match c {
                        'q' => return Ok(false),
                        c => println!("Press [Ctrl] + `{c}`"),
                    },

                    (KeyEventKind::Press, KeyCode::Char(c), _) => println!("Press `{c}`"),
                    _ => {}
                },
                _ => {}
            }
        }
        Ok(true)
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        crossterm::terminal::disable_raw_mode().expect("Failed to disable raw mode");
    }
}