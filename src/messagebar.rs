use crate::terminal::Terminal;
use crate::terminal::{Size, Position};
use crate::uicomponent::UIComponent;
use std::io::Error;

#[derive(Default)]
pub struct MessageBar {
    needs_redraw: bool,
    message: String,
    size: Size,
}

impl MessageBar {

    pub fn set_message(&mut self, message: String) {
        self.message = message;
        self.set_redraw(true);
    }

}

impl UIComponent for MessageBar {
    
    fn set_redraw(&mut self, needs_redraw: bool) {
        self.needs_redraw = needs_redraw;
    }

    fn needs_redraw(&self) -> bool {
        self.needs_redraw
    }

    fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    fn draw(&mut self, start_row: usize) -> Result<(), Error> {
        Terminal::move_cursor_to(Position{row: start_row, column: 0})?;
        Terminal::print(&self.message)?;
        Ok(())
    }

}