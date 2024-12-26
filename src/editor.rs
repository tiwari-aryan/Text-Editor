use crate::terminal::{Terminal, Position, Size};

use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use std::io::Error;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
}

impl Editor {

    pub const fn default() -> Self {
        Self { should_quit: false }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
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
        }) = event {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                _ => (),
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            print!("Goodbye! \r\n");
        }
        else {
            Self::draw_rows()?;
            Self::draw_welcome_message()?;
            Terminal::move_cursor_to(Position{row: 0, column: 0})?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }

    fn draw_rows() -> Result<(), Error> {
        let Size{num_rows, ..} = Terminal::size()?;
        for row in 0..num_rows {
            Terminal::move_cursor_to(Position{row, column: 0})?;
            Terminal::clear_line()?;
            Terminal::print("~\r")?;
        }
        Ok(())
    }

    fn draw_welcome_message() -> Result<(), Error> {
        let Size{num_rows, num_columns} = Terminal::size()?;
        let row = num_rows / 3;
        let mut message = format!("{NAME} Editor -- v{VERSION}");
        message.truncate((num_columns - 1) as usize);
        let column = (num_columns - (message.len() as u16)) / 2;
        Terminal::move_cursor_to(Position{row, column})?;
        Terminal::print(message)?;
        Ok(())
    }

}