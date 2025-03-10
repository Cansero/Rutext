use crossterm::event::{read,Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use std::io::Error;

mod term;
use term::{Terminal, Coordinates};

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub const fn new() -> Self {
        Self { should_quit: false }
    }

    pub fn run(&mut self) {
        Terminal::init().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                _ => (),
            }
        }
    }
    
    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("See ya!!")?;
        } else {
            Self::draw_rows()?;
            let coor = Coordinates::from(0, 0);
            Terminal::move_cursor_to(&coor)?;
        }
        Terminal::show()?;
        Ok(())
    }

    fn draw_rows() -> Result<(), Error> {
        let size = Terminal::size();
        for current_row in 0..size.heigth {
            Terminal::print("~")?;
            if current_row + 1 < size.heigth {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }

}

