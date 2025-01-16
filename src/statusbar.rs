use crate::editor::DocumentStatus;
use crate::terminal::Terminal;
use crate::terminal::{Size, Position};
use crate::uicomponent::UIComponent;
use std::io::Error;

pub struct StatusBar {
    needs_redraw: bool,
    document_status: DocumentStatus,
    size: Size,
}

impl StatusBar {

    pub fn set_status(&mut self, document_status: DocumentStatus) {
        if self.document_status != document_status {
            self.document_status = document_status;
            self.set_redraw(true);
        }
    }

}

impl Default for StatusBar {
    fn default() -> Self {
        Self{
            needs_redraw: true,
            document_status: DocumentStatus::default(),
            size: Size::default(),
        }
    }
}

impl UIComponent for StatusBar {
    
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
        let DocumentStatus{file_path, current_line, total_lines, is_modified} = &self.document_status;
        Terminal::move_cursor_to(Position{row: start_row, column: 0})?;
        let string: String = if let Some(file_name) = file_path {
            if *is_modified {
                format!("{file_name} - {total_lines} lines (modified) {:^15}", format!("{current_line}/{total_lines}"))
            }
            else {
                format!("{file_name} - {total_lines} lines {:^15}", format!("{current_line}/{total_lines}"))
            }
        }
        else {
            "".to_string()
        };
        Terminal::reverse_colour()?;
        Terminal::print(&string)?;
        Terminal::reset_colour()?;
        Ok(())
    }

}