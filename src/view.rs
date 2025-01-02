use crate::terminal::{Terminal, Position, Size};
use crate::buffer::Buffer;
use std::io::Error;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct View {
    buffer: Buffer,
}

impl View {

    pub fn load(&mut self, file_path: &str) {
        if let Ok(buffer) = Buffer::load(file_path) {
            self.buffer = buffer;
        }
    }

    pub fn render(&self) -> Result<(), Error> {
        let Size{num_rows, ..} = Terminal::size()?;
        for row in 0..num_rows {
            Terminal::move_cursor_to(Position{row, column: 0})?;
            Terminal::clear_line()?;
            if let Some(line) = self.buffer.get_line(row as usize) {
                Terminal::print(line)?;
            }
            else {
                Terminal::print("~")?;
            }
            Terminal::print("\r")?;
        }
        if self.buffer.is_empty() {
            Self::draw_welcome_message()?;
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
        Terminal::print(&message)?;
        Ok(())
    }

}