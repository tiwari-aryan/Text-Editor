use crate::editor::Location;
use crate::terminal::{Terminal, Position, Size};
use crate::buffer::Buffer;
use std::{cmp::min, io::Error};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct View {
    buffer: Buffer,
    pub needs_redraw: bool,
}

impl View {

    pub fn load(&mut self, file_path: &str) {
        if let Ok(buffer) = Buffer::load(file_path) {
            self.buffer = buffer;
        }
    }

    pub fn render(&mut self, top_left: Location) -> Result<(), Error> {
        if self.needs_redraw {
            if self.buffer.is_empty() {
                Self::render_welcome_message()?;
            }
            else {
                self.render_lines(top_left)?;
            }
        }
        self.needs_redraw = false;
        Ok(())
    }

    fn render_lines(&self, top_left: Location, ) -> Result<(), Error> {
        let Size{num_rows, num_columns} = Terminal::size()?;
        let Location{x, y} = top_left;

        for row in 0..num_rows {
            Terminal::move_cursor_to(Position{row, column: 0})?;
            Terminal::clear_line()?;
            if let Some(line) = self.buffer.get_line((row + y) as usize) {
                let start_index: usize = min(x as usize, line.len());
                let end_index: usize = min((x + num_columns) as usize, line.len());
                Terminal::print(&line[start_index..end_index])?;
            }
            else {
                Terminal::print("~")?;
            }
        }
        Ok(())
    }

    fn render_welcome_message() -> Result<(), Error> {
        let Size{num_rows, ..} = Terminal::size()?;
        for row in 0..num_rows {
            Terminal::move_cursor_to(Position{row, column: 0})?;
            Terminal::clear_line()?;
            Terminal::print("~")?;
        }
        Self::draw_welcome_message()?;
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