use crate::editor::DocumentStatus;
use crate::terminal::Terminal;
use crate::terminal::{Size, Position};

pub struct StatusBar {
    pub needs_redraw: bool,
    document_status: DocumentStatus,
}

impl StatusBar {

    pub fn set_status(&mut self, document_status: DocumentStatus) {
        if self.document_status != document_status {
            self.document_status = document_status;
            self.needs_redraw = true;
        }
    }

    pub fn render(&mut self) {
        if self.needs_redraw {
            let Size{num_rows, ..} = Terminal::size().unwrap_or_default();
            let DocumentStatus{file_path, current_line, total_lines, is_modified} = &self.document_status;
            let _ = Terminal::move_cursor_to(Position{row: num_rows - 2, column: 0});
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
            let _ = Terminal::reverse_colour();
            let _ = Terminal::print(&string);
            let _ = Terminal::reset_colour();
            self.needs_redraw = false;
        }
    }
}

impl Default for StatusBar {
    fn default() -> Self {
        Self{needs_redraw: true, document_status: DocumentStatus::default()}
    }
}