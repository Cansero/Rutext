use crossterm::terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType};
use crossterm::event::{read,Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use crossterm::execute;
use std::io::stdout;
use std::io::Error;

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn new() -> Self {
        Editor{ should_quit: false }
    }

    pub fn run(&mut self) {
        Self::init().unwrap();
        let result = self.repl();
        Self::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            let event = read()?;
            self.evaluate_event(&event);
            self.refresh_screen()?;
            
            if self.should_quit {
                break;
            }
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
        if self.should_quit {
            Self::clear_screen()?;
            print!("See ya!! \r\n");
        }
        Ok(())
    }

    fn init() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()
    }

    fn terminate() -> Result<(), Error> {
        disable_raw_mode()
    }

    fn clear_screen() -> Result<(), Error> {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All))
    }
}

