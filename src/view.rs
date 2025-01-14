use crate::editor::Location;
use crate::editorcommand::{EditorCommand, Direction, EditorCommand::{Move, Insert, Backspace, Delete, Enter, Resize, Save}};
use crate::terminal::{Terminal, Position, Size};
use crate::buffer::Buffer;
use std::{cmp, io::Error};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    top_left: Location,
    location: Location,
}

impl View {

    pub fn load(&mut self, file_path: &str) {
        if let Ok(buffer) = Buffer::load(file_path) {
            self.buffer = buffer;
        }
    }

    pub fn handle_command(&mut self, command: EditorCommand) {
        match command {
            Resize => {
                self.needs_redraw = true;
            },
            Move(direction) => {
                self.move_cursor(&direction);
                self.needs_redraw = true;
            },
            Insert(character) => {
                self.add_character(character);
                self.needs_redraw = true;
            },
            Backspace if ((self.location.x > 0) | (self.location.y > 0)) => {
                self.move_cursor(&Direction::Left);
                self.handle_command(Delete);
                self.needs_redraw = true;
            },
            Delete if ((self.location.x as usize) < self.buffer.get_num_columns(self.location.y as usize)) |
                        ((self.location.y as usize) < self.buffer.get_num_rows() - 1) => {
                self.delete_character();
                self.needs_redraw = true;
            },
            Enter => {
                self.enter();
                self.needs_redraw = true;
            },
            Save => {
                self.save_file();
            }
            _ => (),
        }
    }

    fn move_cursor(&mut self, direction: &Direction) {
        let Size{num_rows, ..} = Terminal::size().unwrap_or_default();        
        match direction {
            Direction::Up if self.location.y > 0 => {
                self.location.y -= 1;
                self.location.x = cmp::min(self.location.x, self.buffer.get_num_columns(self.location.y as usize) as u16);
            },
            Direction::Down if (self.location.y as usize) < self.buffer.get_num_rows() => {
                self.location.y += 1;
                self.location.x = cmp::min(self.location.x, self.buffer.get_num_columns(self.location.y as usize) as u16);
            },
            Direction::Left => {
                if self.location.x > 0 {
                    self.location.x -= 1;
                }
                else if self.location.y > 0 {
                    self.location.y -= 1;
                    self.location.x = self.buffer.get_num_columns(self.location.y as usize) as u16;
                }
            },
            Direction::Right => {
                if (self.location.x as usize) < self.buffer.get_num_columns(self.location.y as usize) {
                    self.location.x += 1;
                }
                else if (self.location.y as usize) < self.buffer.get_num_rows() {
                    self.location.y += 1;
                    self.location.x = 0;
                }
            },
            Direction::PageUp => {
                self.location.y = cmp::max(0, self.location.y - num_rows);
                self.location.x = cmp::min(self.location.x, self.buffer.get_num_columns(self.location.y as usize) as u16);
            },
            Direction::PageDown => {
                self.location.y = cmp::min(self.buffer.get_num_rows() as u16, self.location.y + num_rows);
                self.location.x = cmp::min(self.location.x, self.buffer.get_num_columns(self.location.y as usize) as u16);
            },
            Direction::Home => {
                self.location.x = 0;
            },
            Direction::End => {
                self.location.x = self.buffer.get_num_columns(self.location.y as usize) as u16;
            },
            _ => (),
        }
        self.update_cursor_position();
    }

    fn update_cursor_position(&mut self) {
        let Size{num_rows, num_columns} = Terminal::size().unwrap_or_default();
        if self.location.x < self.top_left.x {
            self.top_left.x = self.location.x;
        }
        if self.location.y < self.top_left.y {
            self.top_left.y = self.location.y;
        }
        if self.location.x >= self.top_left.x + num_columns {
            self.top_left.x = self.location.x - num_columns + 1;
        }
        if self.location.y >= self.top_left.y + num_rows {
            self.top_left.y = self.location.y - num_rows + 1;
        }
        self.needs_redraw = true;
    }

    pub fn get_cursor_position(&self) -> Position {
        Position{row: self.location.y - self.top_left.y, column: self.location.x - self.top_left.x}
    }

    fn add_character(&mut self, character: char) {
        self.buffer.insert_character(self.location, character);
        self.move_cursor(&Direction::Right);
    }

    fn delete_character(&mut self){
        self.buffer.delete_character(self.location);
    }

    fn enter(&mut self) {
        self.buffer.enter(self.location);
        self.move_cursor(&Direction::Down);
        self.move_cursor(&Direction::Home);
    }

    fn save_file(&mut self) {
        let _ = self.buffer.save_file();
    }

    // TODO: Check for invisible characters and render them appropriately
    pub fn render(&mut self) {
        if self.needs_redraw {
            if self.buffer.is_empty() {
                Self::render_welcome_message();
            }
            else {
                self.render_lines();
            }
        }
        self.needs_redraw = false;
    }

    fn render_lines(&self) {
        // TODO: Make size a parameter of View and update it when Resize is called rather than using Terminal::size() each time
        let Size{num_rows, num_columns} = Terminal::size().unwrap_or_default();
        let Location{x, y} = self.top_left;

        for row in 0..num_rows {
            let _ = Terminal::move_cursor_to(Position{row, column: 0});
            let _ = Terminal::clear_line();
            if let Some(line) = self.buffer.get_line((row + y) as usize) {
                let result = Terminal::print(&line.get(x as usize..(x+num_columns) as usize));
                debug_assert!(result.is_ok(), "Error: Error occurred while printing line.");
            };
        }
    }

    fn render_welcome_message() {
        let Size{num_rows, ..} = Terminal::size().unwrap_or_default();
        for row in 0..num_rows {
            let _ = Terminal::move_cursor_to(Position{row, column: 0});
            let _ = Terminal::clear_line();
            let _ = Terminal::print("~");
        }
        let result = Self::draw_welcome_message();
        debug_assert!(result.is_ok(), "Error: Error occurred while printing welcome message.");
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