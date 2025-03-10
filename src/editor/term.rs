use crossterm::cursor::{MoveTo, Hide, Show};
use crossterm::style::Print;
use crossterm::queue;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use std::io::{Error, Write, stdout};

pub struct Terminal {}

pub struct Size {
    pub heigth: u16,
    pub width: u16,
}

pub struct Coordinates {
    pub x: u16,
    pub y: u16,
}

impl Terminal {

    pub fn terminate() -> Result<(), Error> {
        disable_raw_mode()?;
        Ok(())
    }

    pub fn init() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(&Coordinates::from(0, 0))?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), Error> {
        queue!(stdout(), Clear(ClearType::All))?;
        stdout().flush()?;
        Ok(())
    }

    pub fn move_cursor_to(coor: &Coordinates) -> Result<(), Error> {
        queue!(stdout(), MoveTo(coor.x, coor.y))?;
        stdout().flush()?;
        Ok(())
    }

    pub fn size() -> Size {
        let (x, y) = size().unwrap();
        Size { heigth: x, width: y }
    }

    pub fn hide() -> Result<(), Error> {
        queue!(stdout(), Hide)?;
        stdout().flush()?;
        Ok(())
    }

    pub fn show() -> Result<(), Error> {
        queue!(stdout(), Show)?;
        stdout().flush()?;
        Ok(())
    }

    pub fn print(text: &str) -> Result<(), Error> {
        queue!(stdout(), Print(text))?;
        stdout().flush()?;
        Ok(())
    }

}

impl Coordinates {

    pub fn from(x: u16, y: u16) -> Coordinates {
        Coordinates { x, y }
    }

}
