use crossterm::cursor::{MoveTo, Hide, Show};
use crossterm::style::Print;
use crossterm::queue;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use std::io::{Write, stdout};
use std::io::Error;

pub struct Terminal {}

impl Terminal {

    pub fn terminate() -> Result<(), Error> {
        disable_raw_mode()?;
        Ok(())
    }

    pub fn init() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(0, 0)?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), Error> {
        queue!(stdout(), Clear(ClearType::All))?;
        stdout().flush();
        Ok(())
    }

    pub fn move_cursor_to(x: u16, y: u16) -> Result<(), Error> {
        queue!(stdout(), MoveTo(x, y))?;
        stdout().flush();
        Ok(())
    }

    pub fn size() -> Result<(u16, u16), Error> {
        size()
    }

    pub fn hide() {
        queue!(stdout(), Hide).unwrap();
        stdout().flush();
    }

    pub fn show() {
        queue!(stdout(), Show).unwrap();
        stdout().flush();
    }

    pub fn print(text: &str) {
        queue!(stdout(), Print(text)).unwrap();
        stdout().flush();
    }

}
