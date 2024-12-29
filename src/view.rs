use crate::terminal::{Terminal, Position, Size};
use std::io::Error;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {

}

impl View {
    pub fn render() -> Result<(), Error> {
        let Size{num_rows, ..} = Terminal::size()?;
        for row in 0..num_rows {
            Terminal::move_cursor_to(Position{row, column: 0})?;
            Terminal::clear_line()?;
            Terminal::print("~\r")?;
        }
        Self::draw_welcome_message()?;
        Terminal::move_cursor_to(Position{row: 0, column: 2})?;
        Terminal::print("Hello World!")?;
        Ok(())
    }

    fn draw_welcome_message() -> Result<(), Error> {
        let Size{num_rows, num_columns} = Terminal::size()?;
        let row = num_rows / 3;
        let mut message = format!("{NAME} Editor -- v{VERSION}");
        message.truncate((num_columns - 1) as usize);
        let column = (num_columns - (message.len() as u16)) / 2;
        Terminal::move_cursor_to(Position{row, column})?;
        Terminal::print(&message)?;
        Ok(())
    }

}