use crossterm::terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType};
use crossterm::event::{read,Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use crossterm::execute;
use std::io::stdout;

pub struct Editor {}

impl Editor {
    pub fn new() -> Self {
        Editor{}
    }

    pub fn run(&self) {
        if let Err(err) = self.repl() {
            panic!("{err:#?}");
        }
        print!("\x1b[2J");
        print!("Goodbye.\r\n");
    }

    fn repl(&self) -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        loop {
            if let Key(event) = read()? {
                println!("{event:?} \r");
                if let Char(c) = event.code {
                    if c == 'q' {
                        break;
                    }
                }
            }
        }
        disable_raw_mode()?;
        Ok(())
    }
}

